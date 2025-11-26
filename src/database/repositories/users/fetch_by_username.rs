use sqlx::query_as;

use crate::database::{DatabaseError, DatabasePool, schemas::users::User};

/// Fetch [`User`] item by username
pub async fn fetch_by_username(
    username: &str,
    pool: &DatabasePool,
) -> Result<Option<User>, DatabaseError> {
    let item = query_as(
        "
            SELECT id, username, password, role, created
            FROM users
            WHERE username = ?
        ",
    )
    .bind(username.to_lowercase())
    .fetch_optional(pool)
    .await?;

    Ok(item)
}
