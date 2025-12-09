use sqlx::{Executor, Sqlite, query_as};

use crate::models::users::User;

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

/// Fetch [`User`] item by session id
pub async fn fetch_by_session_id<'a, E>(
    id: &uuid::Uuid,
    exec: E,
) -> Result<Option<User>, sqlx::Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let item = query_as(
        "
            SELECT u.id, u.username, u.password, u.role, u.created
              FROM users as u
            INNER JOIN sessions as s
              ON s.owner_id = u.id
            WHERE s.id = ?
        ",
    )
    .bind(id)
    .fetch_optional(exec)
    .await?;

    Ok(item)
}

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
