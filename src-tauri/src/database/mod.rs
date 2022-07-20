mod migrator;

use crate::database::migrator::Migrator;
use sea_orm::{Database, DbErr};
use sea_orm_migration::prelude::*;

const DATABASE_URL: &str = if cfg!(debug_assertions) {
    "sqlite::memory:"
} else {
    "sqlite://data.db"
};

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let schema_manager = SchemaManager::new(&db);

    Migrator::refresh(&db).await?;
    println!("{:?}", schema_manager.has_table("bakery").await?);

    Ok(())
}

pub async fn db_main() {
    if let Err(err) = run().await {
        panic!("{}", err);
    }
}
