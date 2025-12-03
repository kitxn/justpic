use sqlx::{Executor, Sqlite, sqlite::SqliteQueryResult};

pub async fn remove<'a, E>(id: &uuid::Uuid, exec: E) -> Result<SqliteQueryResult, sqlx::Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let res = sqlx::query(
        "
            DELETE FROM sessions
            WHERE id = ?
          ",
    )
    .bind(id)
    .execute(exec)
    .await?;

    Ok(res)
}
