use sqlx::SqliteConnection;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

use super::DatabasePool;

async fn connect(opts: SqliteConnectOptions) -> Result<DatabasePool, sqlx::Error> {
    let pool = SqlitePool::connect_with(opts).await?;
    Ok(pool)
}

pub async fn open_db(path: &std::path::Path) -> Result<DatabasePool, sqlx::Error> {
    if let Some(path) = path.parent() {
        std::fs::create_dir_all(path)?;
    }

    let opts = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);

    connect(opts).await
}

pub async fn open_in_memory_db() -> Result<SqliteConnection, sqlx::Error> {
    use sqlx::Connection;

    let opts = SqliteConnectOptions::new()
        .in_memory(true)
        .shared_cache(true);

    let mut conn = SqliteConnection::connect_with(&opts).await?;
    sqlx::migrate!().run(&mut conn).await?;

    Ok(conn)
}

pub async fn apply_migrations(pool: &DatabasePool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!().run(pool).await?;

    tracing::info!("Migrations applied!");

    Ok(())
}
