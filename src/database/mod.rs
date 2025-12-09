pub mod sqlite;

pub type DatabasePool = sqlx::Pool<sqlx::sqlite::Sqlite>;
