use sqlx::{Executor, Sqlite, sqlite::SqliteQueryResult};

use crate::{error::Error, models::files::internal::File};

/// Insert card information into the database
///
/// If the associated file
/// is not inserted into the database
/// before the card, an error will occur.
pub async fn insert<'a, E>(file: &File, exec: E) -> Result<SqliteQueryResult, Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let res = sqlx::query(
        "
            INSERT INTO files (
              id, uploader_id, mimetype, filesize, 
              width, height, created, color,
              state
            ) VALUES (
              ?, ?, ?, ?,
              ?, ?, ?, ?,
              ?
            )
          ",
    )
    .bind(file.id())
    .bind(file.uploader_id())
    .bind(file.mimetype())
    .bind(file.filesize())
    .bind(file.width())
    .bind(file.height())
    .bind(file.created())
    .bind(file.color())
    .bind(file.state())
    .execute(exec)
    .await?;

    Ok(res)
}
