use actix_web::{HttpResponse, http::StatusCode};

use crate::models::error::ApiError;

/// Error type for the application
///
/// Contains the Kind and a possible error details
#[derive(Debug, derive_more::Display)]
pub enum Error {
    /// Access to the resource is denied.
    ///
    /// (For example, viewing a private profile without authorization)
    #[display("ACCESS_DENIED")]
    AccessDenied,

    /// The required resource was not found
    #[display("RESOURCE_NOT_FOUND")]
    ResourceNotFound,

    /// The sought object was not found
    ///
    /// (for example, the sought profile)
    #[display("ITEM_NOT_FOUND")]
    ItemNotFound,

    /// The server address is already in use
    #[display("HOST_SOCKET_IN_USE")]
    SocketInUse,

    /// The object already exists and cannot be created again
    #[display("ALREADY_EXISTS")]
    Conflict,

    /// Incorrect data
    #[display("BAD_INPUT")]
    BadInput,

    /// IO error
    #[display("IO_ERROR: {_0}")]
    Io(std::io::Error),

    /// Db error
    #[display("DB_ERROR: {_0}")]
    Database(sqlx::Error),

    /// Cryptography error
    #[display("CRYPTO_ERROR")]
    CryptoError(bcrypt::BcryptError),

    /// File storage error
    #[display("STORAGE_ERROR: {_0}")]
    Storage(crate::storage::StorageError),
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Error::Database(value.into())
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Error::ItemNotFound,
            _ => Error::Database(value),
        }
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(value: bcrypt::BcryptError) -> Self {
        match value {
            bcrypt::BcryptError::Io(error) => Error::from(error),
            bcrypt::BcryptError::CostNotAllowed(..)
            | bcrypt::BcryptError::InvalidCost(..)
            | bcrypt::BcryptError::InvalidPrefix(..)
            | bcrypt::BcryptError::InvalidSaltLen(..)
            | bcrypt::BcryptError::InvalidBase64(..)
            | bcrypt::BcryptError::InvalidHash(..) => Error::BadInput,
            _ => Error::CryptoError(value),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::NotFound => Error::ResourceNotFound,
            std::io::ErrorKind::PermissionDenied => Error::AccessDenied,
            std::io::ErrorKind::AddrInUse => Error::SocketInUse,
            std::io::ErrorKind::AlreadyExists => Error::Conflict,
            std::io::ErrorKind::InvalidData | std::io::ErrorKind::InvalidInput => Error::BadInput,
            _ => Error::Io(value),
        }
    }
}

impl From<crate::storage::StorageError> for Error {
    fn from(value: crate::storage::StorageError) -> Self {
        Error::Storage(value)
    }
}

/// [Error] Result alias
pub type Result<T> = std::result::Result<T, Error>;

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::ItemNotFound | Error::ResourceNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let code = self.status_code();
        let code_u16 = code.as_u16();
        HttpResponse::build(code).json(ApiError {
            code: code_u16,
            message: self.to_string(),
        })
    }
}
