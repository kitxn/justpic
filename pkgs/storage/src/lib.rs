pub mod storage;

/// File storage error
#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum StorageError {
    /// Incorrect key length
    #[display("BAD_FILE_KEY_LENGTH")]
    #[from(skip)]
    BadKeyLen,

    /// Incorrect key type
    ///
    /// (for example, invalid characters in the key)
    #[display("BAD_FILE_KEY_SPECIFICATION")]
    #[from(skip)]
    BadKeySpec,

    // TODO: flatten io errors
    /// I/O error
    #[display("IO: {_0}")]
    #[from]
    Io(std::io::Error),
}
