use sqlx::{Executor, Sqlite, sqlite::SqliteQueryResult};

use crate::{error::Error, models::files::state::FileState};

pub async fn update_file_state<'a, E>(
    file_id: &str,
    state: FileState,
    exec: E,
) -> Result<SqliteQueryResult, Error>
where
    E: Executor<'a, Database = Sqlite>,
{
    let res = sqlx::query(
        "
        UPDATE files
        SET state = ?
        WHERE id = ?
      ",
    )
    .bind(state)
    .bind(file_id)
    .execute(exec)
    .await?;

    Ok(res)
}
