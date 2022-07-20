mod entity;
use sea_orm::{ConnectionTrait, Database, DbErr, Schema};
use sea_orm_migration::prelude::*;
use std::fs::File;
use std::path;

async fn run() -> Result<(), DbErr> {
    let database_path = "data.sqlite3";

    let database_url = if cfg!(debug_assertions) {
        "sqlite::memory:".to_owned()
    } else {
        format!("sqlite://{}", database_path)
    };

    let db = if !cfg!(debug_assertions) && !path::Path::new(&database_path).exists() {
        File::create(&database_path).unwrap();
        Database::connect(database_url).await?
    } else {
        Database::connect(database_url).await?
    };

    let backend = db.get_database_backend();
    let schema = Schema::new(backend);

    let st = backend.build(&schema.create_table_from_entity(entity::users::Entity));
    db.execute(st).await?;

    let schema_manager = SchemaManager::new(&db);

    println!("{:?}", schema_manager.has_table("users").await?);

    Ok(())
}

pub async fn db_main() {
    if let Err(err) = run().await {
        panic!("{}", err);
    }
}
