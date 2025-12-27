use crate::{error::Error, repositories, users::models::User, utils};

/// Create a User model and insert it into the database
pub async fn create_and_insert(
    username: &str,
    password: &str,
    pool: &sqlx::SqlitePool,
) -> Result<User, Error> {
    let password = utils::crypto::hash_password(password)?;

    let user = User::new(username, password);

    repositories::users::insert(&user, pool)
        .await
        .map_err(map_user_insert_error)?;

    Ok(user)
}

fn map_user_insert_error(err: sqlx::Error) -> Error {
    match err {
        sqlx::Error::Database(e) if e.is_unique_violation() => Error::BadInput,
        _ => Error::from(err),
    }
}
