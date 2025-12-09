use sqlx::{Executor, Sqlite, query, sqlite::SqliteQueryResult};

/// Delete [`User`] item by id
pub async fn delete_by_id<'a, E>(id: &uuid::Uuid, exec: E) -> Result<SqliteQueryResult, sqlx::Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let res = query(
        "
            DELETE FROM users
            WHERE id = ?
        ",
    )
    .bind(id)
    .execute(exec)
    .await?;

    Ok(res)
}
