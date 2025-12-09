use sqlx::{Executor, Sqlite, sqlite::SqliteQueryResult};

use crate::models::sessions::Session;

pub async fn insert<'a, E>(item: &Session, exec: E) -> Result<SqliteQueryResult, sqlx::Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let res = sqlx::query(
        "
            INSERT INTO sessions (
              id, owner_id, agent, created, expires
            ) VALUES (
              ?, ?, ?, ?, ?
            )
          ",
    )
    .bind(item.id())
    .bind(item.owner_id())
    .bind(item.agent())
    .bind(item.created())
    .bind(item.expires())
    .execute(exec)
    .await?;

    Ok(res)
}
