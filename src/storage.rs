use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub type FileStream = ReaderStream<File>;

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

#[derive(Debug, Clone)]
pub struct Storage {
    root: std::path::PathBuf,
}

// TODO: Add key validation
impl Storage {
    pub fn new(root: std::path::PathBuf) -> Self {
        Storage { root }
    }

    pub fn init(&self) -> Result<(), StorageError> {
        std::fs::create_dir_all(&self.root)?;

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<FileStream, StorageError> {
        let path = self.root.join(&key[..2]).join(&key[2..4]).join(key);
        let file = File::open(path).await?;

        let stream = FileStream::new(file);

        Ok(stream)
    }

    pub async fn set(&self, key: &str) -> Result<(), StorageError> {
        let path = self.root.join(&key[..2]).join(&key[2..4]).join(key);

        todo!();
    }

    pub async fn remove(&self, key: &str) -> Result<(), StorageError> {
        let path = self.root.join(&key[..2]).join(&key[2..4]).join(key);

        tokio::fs::remove_file(path).await?;

        Ok(())
    }
}
