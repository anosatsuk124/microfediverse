use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::{
    convert::Infallible,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tauri::State;

pub struct ServerState(pub Arc<Mutex<bool>>);

impl Default for ServerState {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(false)))
    }
}

#[tauri::command]
pub fn shutdown_server(state: State<'_, ServerState>) {
    *state.0.lock().unwrap() = false;
}

#[tauri::command]
pub async fn start_server(state: State<'_, ServerState>) -> Result<(), ()> {
    {
        *state.0.lock().unwrap() = true;
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

    let graceful = server.with_graceful_shutdown(async {
        let state = state.0.clone();
        tokio::spawn(async move {
            loop {
                if !state.lock().unwrap().clone() {
                    break;
                }
            }
        })
        .await
        .unwrap();
    });

    match graceful.await {
        Ok(_) => println!("shutdown"),
        Err(e) => println!("error: {e}"),
    }
    Ok(())
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}
