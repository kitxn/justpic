use chrono::{DateTime, Utc};
use sqlx::{Executor, Sqlite, query, sqlite::SqliteQueryResult};

pub async fn update_expire_datetime<'a, E>(
    id: &uuid::Uuid,
    new_expiration: &DateTime<Utc>,
    exec: E,
) -> Result<SqliteQueryResult, sqlx::Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let res = query(
        "
            UPDATE sessions
            SET expires = ?
            WHERE id = ?
        ",
    )
    .bind(new_expiration)
    .bind(id)
    .execute(exec)
    .await?;

    Ok(res)
}
