pub mod database;

pub type DatabasePool = sqlx::Pool<sqlx::sqlite::Sqlite>;

/// File storage error
#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum DatabaseError {
    // TODO: flatten sqlx errors
    /// Sqlx error
    #[display("SQLX: {_0}")]
    #[from]
    Sqlx(sqlx::error::Error),

    /// Sql migration error
    #[display("Migration: {_0}")]
    #[from]
    Migration(sqlx::migrate::MigrateError),

    // TODO: flatten io errors
    /// I/O error
    #[display("IO: {_0}")]
    #[from]
    Io(std::io::Error),
}
