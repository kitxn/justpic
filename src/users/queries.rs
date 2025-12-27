use sqlx::SqlitePool;

use crate::{error::Error, repositories, types::pagination::PaginationParams, users::models::User};

pub async fn try_get_by_username(
    username: impl AsRef<str>,
    pool: &SqlitePool,
) -> Result<Option<User>, Error> {
    repositories::users::get_by_username(username.as_ref(), pool)
        .await
        .map_err(Error::from)
}

pub async fn get_by_username(username: impl AsRef<str>, pool: &SqlitePool) -> Result<User, Error> {
    match try_get_by_username(username, pool).await? {
        Some(v) => Ok(v),
        None => Err(Error::ItemNotFound),
    }
}

pub async fn try_get_by_id(id: &uuid::Uuid, pool: &SqlitePool) -> Result<Option<User>, Error> {
    repositories::users::get_by_id(id, pool)
        .await
        .map_err(Error::from)
}

pub async fn get_by_id(id: &uuid::Uuid, pool: &SqlitePool) -> Result<User, Error> {
    match try_get_by_id(id, pool).await? {
        Some(v) => Ok(v),
        None => Err(Error::ItemNotFound),
    }
}

pub async fn get_all(pagination: PaginationParams, pool: &SqlitePool) -> Result<Vec<User>, Error> {
    repositories::users::get_many(pagination, pool)
        .await
        .map_err(Error::from)
}
