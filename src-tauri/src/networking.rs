use hyper::{
    http,
    server::{conn::AddrIncoming, Builder},
    service::{make_service_fn, service_fn, Service},
    Body, Request, Response, Server, StatusCode,
};
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Mutex;
use tauri::State;
use tokio::sync::watch;

pub struct ServerState(pub watch::Sender<bool>, pub Mutex<watch::Receiver<bool>>);

impl Default for ServerState {
    fn default() -> Self {
        let (tx1, rx1) = watch::channel(false);
        Self(tx1, Mutex::from(rx1))
    }
}

async fn shutdown(state: State<'_, ServerState>) {
    let mut recv = state.1.lock().unwrap();
    recv.changed().await.expect("msg");
}

#[tauri::command]
pub fn shutdown_server(state: State<'_, ServerState>) {
    state.0.send(false);
}

#[tauri::command]
pub async fn start_server(state: State<'_, ServerState>) -> Result<(), ()> {
    {
        state.0.send(true);
    }
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown(state));

    println!("started");

    match graceful.await {
        Ok(_) => println!("shutdown"),
        Err(e) => println!("error: {e}"),
    }
    Ok(())
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}
