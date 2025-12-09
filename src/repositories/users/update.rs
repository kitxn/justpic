use sqlx::{Executor, Sqlite, query, sqlite::SqliteQueryResult};

pub async fn update_password<'a, E>(
    id: &uuid::Uuid,
    new_password: &str,
    exec: E,
) -> Result<SqliteQueryResult, sqlx::Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let res = query(
        "
            UPDATE users
            SET password = ?
            WHERE id = ?
        ",
    )
    .bind(new_password)
    .bind(id)
    .execute(exec)
    .await?;

    Ok(res)
}

pub async fn update_username<'a, E>(
    id: &uuid::Uuid,
    new_username: &str,
    exec: E,
) -> Result<SqliteQueryResult, sqlx::Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let res = query(
        "
            UPDATE users
            SET username = ?
            WHERE id = ?
        ",
    )
    .bind(new_username)
    .bind(id)
    .execute(exec)
    .await?;

    Ok(res)
}
