/// Error type for the application
///
/// Contains the Kind and a possible error details
#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum Error {
    #[from]
    #[display("IO_ERROR: {_0}")]
    Io(std::io::Error),

    #[from(skip)]
    #[display("UNDEFINED_BACKEND_ERROR: {_0}")]
    Custom(String),
}

/// [Error] Result alias
pub type Result<T> = std::result::Result<T, Error>;
