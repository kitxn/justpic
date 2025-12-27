use std::path::Path;

use sqlx::{
    Connection, SqliteConnection,
    sqlite::{SqliteConnectOptions, SqlitePool},
};

pub type DatabasePool = sqlx::Pool<sqlx::sqlite::Sqlite>;

pub async fn open(path: &Path) -> sqlx::Result<DatabasePool> {
    if let Some(path) = path.parent() {
        std::fs::create_dir_all(path)?;
    }

    let opts = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);

    SqlitePool::connect_with(opts).await
}

pub async fn open_in_memory() -> sqlx::Result<SqliteConnection> {
    let opts = SqliteConnectOptions::new()
        .in_memory(true)
        .shared_cache(true);

    SqliteConnection::connect_with(&opts).await
}

pub async fn migrate(pool: &DatabasePool) -> sqlx::Result<()> {
    sqlx::migrate!().run(pool).await?;

    tracing::info!("Migrations applied!");

    Ok(())
}
