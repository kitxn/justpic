use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{error::Error, repositories, sessions::models::Session};

pub async fn create_and_insert(user_id: Uuid, pool: &SqlitePool) -> Result<Session, Error> {
    let session = Session::new(user_id);
    repositories::sessions::insert(&session, pool).await?;

    Ok(session)
}

pub async fn delete_by_id(id: Uuid, pool: &SqlitePool) -> Result<(), Error> {
    repositories::sessions::remove_by_id(&id, pool).await?;

    Ok(())
}
