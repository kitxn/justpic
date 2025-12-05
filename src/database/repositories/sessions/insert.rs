use sqlx::{Executor, Sqlite, sqlite::SqliteQueryResult};

use crate::database::schemas::sessions::DbSession;

pub async fn insert<'a, E>(item: &DbSession, exec: E) -> Result<SqliteQueryResult, sqlx::Error>
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
    .bind(item.id)
    .bind(item.owner_id)
    .bind(&item.agent)
    .bind(item.created)
    .bind(item.expires)
    .execute(exec)
    .await?;

    Ok(res)
}
