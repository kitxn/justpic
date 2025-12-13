use sqlx::{Executor, Sqlite, query_as};

use crate::models::sessions::Session;

pub async fn get_by_id<'a, E>(id: &uuid::Uuid, exec: E) -> Result<Option<Session>, sqlx::Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let item = query_as(
        "
            SELECT id, owner_id, agent, created, expires
            FROM sessions
            WHERE id = ?
        ",
    )
    .bind(id)
    .fetch_optional(exec)
    .await?;

    Ok(item)
}
