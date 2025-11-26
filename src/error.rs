use actix_web::{HttpResponse, http::StatusCode};

use crate::models::error::ApiError;

/// Error type for the application
///
/// Contains the Kind and a possible error details
#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum Error {
    #[from]
    #[display("IO_ERROR: {_0}")]
    Io(std::io::Error),

    #[from]
    #[display("DB_ERROR: {_0}")]
    Database(crate::database::DatabaseError),

    #[from]
    #[display("STORAGE_ERROR: {_0}")]
    Storage(crate::storage::StorageError),

    #[from(skip)]
    #[display("UNDEFINED_BACKEND_ERROR: {_0}")]
    Custom(String),
}

/// [Error] Result alias
pub type Result<T> = std::result::Result<T, Error>;

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
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
