use sqlx::{Executor, Sqlite, sqlite::SqliteQueryResult};

use crate::models::users::User;

/// Insert [`User`] item into database
pub async fn insert<'a, E>(item: &User, exec: E) -> Result<SqliteQueryResult, sqlx::Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let res = sqlx::query(
        "
            INSERT INTO users (
              id, username, password, role
            ) VALUES (
              ?, ?, ?, ?
            )
          ",
    )
    .bind(item.id())
    .bind(item.username())
    .bind(item.password())
    .bind(item.role())
    .execute(exec)
    .await?;

    tracing::info!("User {}:{} inserted in db!", item.id(), item.username());
    Ok(res)
}
