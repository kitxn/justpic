pub mod repositories;
pub mod schemas;

pub mod sqlite;

pub type DatabasePool = sqlx::Pool<sqlx::sqlite::Sqlite>;
