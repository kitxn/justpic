use sqlx::{Executor, Sqlite, query_as};

use crate::models::users::User;

/// Fetch [`User`] item by username
pub async fn fetch_by_username<'a, E>(username: &str, exec: E) -> Result<Option<User>, sqlx::Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let item = query_as(
        "
            SELECT id, username, password, role, created
            FROM users
            WHERE username = ?
        ",
    )
    .bind(username.to_lowercase())
    .fetch_optional(exec)
    .await?;

    Ok(item)
}
