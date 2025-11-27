use sqlx::{Executor, Sqlite, query_as};

use crate::database::schemas::users::User;

/// Fetch [`User`] item by id
pub async fn fetch_by_id<'a, E>(id: &uuid::Uuid, exec: E) -> Result<Option<User>, sqlx::Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let item = query_as(
        "
            SELECT id, username, password, role, created
            FROM users
            WHERE id = ?
        ",
    )
    .bind(id)
    .fetch_optional(exec)
    .await?;

    Ok(item)
}
