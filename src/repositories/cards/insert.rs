use sqlx::{Executor, Sqlite, sqlite::SqliteQueryResult};

use crate::{error::Error, models::cards::internal::Card};

/// Insert card information into the database
///
/// If the associated file
/// is not inserted into the database
/// before the card, an error will occur.
pub async fn insert<'a, E>(card: &Card, exec: E) -> Result<SqliteQueryResult, Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let res = sqlx::query(
        "
            INSERT INTO cards (
              id, file_key, owner_id, title, 
              description, created, source_url, is_private
            ) VALUES (
              ?, ?, ?, ?,
              ?, ?, ?, ?
            )
          ",
    )
    .bind(card.id())
    .bind(card.file().id())
    .bind(card.owner_id())
    .bind(card.title())
    .bind(card.description())
    .bind(card.created())
    .bind(card.source_url())
    .bind(card.is_private())
    .execute(exec)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => Error::BrokenRelation {
            from: "card-file_key",
            to: "file-key",
        },
        _ => Error::from(e),
    })?;

    Ok(res)
}
