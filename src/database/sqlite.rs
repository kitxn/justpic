use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

use super::{DatabaseError, DatabasePool};

pub async fn open_db(path: &std::path::Path) -> Result<DatabasePool, DatabaseError> {
    if let Some(path) = path.parent() {
        std::fs::create_dir_all(path)?;
    }

    let opts = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(opts).await?;

    Ok(pool)
}

pub async fn apply_migrations(pool: &DatabasePool) -> Result<(), DatabaseError> {
    sqlx::migrate!("./migrations").run(pool).await?;

    Ok(())
}
