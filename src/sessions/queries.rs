use sqlx::SqlitePool;

use crate::{error::Error, sessions::models::Session};

pub async fn try_get_by_id(id: uuid::Uuid, pool: &SqlitePool) -> Result<Option<Session>, Error> {
    todo!()
}

pub async fn get_by_id(id: uuid::Uuid, pool: &SqlitePool) -> Result<Session, Error> {
    match try_get_by_id(id, pool).await? {
        Some(v) => Ok(v),
        None => Err(Error::ItemNotFound),
    }
}
