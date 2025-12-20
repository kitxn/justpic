use std::path::PathBuf;

use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::{error::Error, util};

pub type FileStream = ReaderStream<File>;

const MAX_UPLOADING_FILE_SIZE: usize = 128;

#[derive(Clone)]
pub struct Storage {
    root: std::path::PathBuf,

    use_subdir: bool,
}

// TODO: Add key validation
impl Storage {
    pub fn new(root: std::path::PathBuf, use_sub: bool) -> Self {
        Storage {
            root,
            use_subdir: use_sub,
        }
    }

    /// Initialize storage
    pub fn init(&self) -> Result<(), Error> {
        std::fs::create_dir_all(&self.root)?;

        tracing::info!("File storage opened!");

        Ok(())
    }

    /// Get a file stream using the specified key
    pub async fn get(&self, key: &str) -> Result<FileStream, Error> {
        let path = self.generate_path(key);
        let file = File::open(path).await?;

        let stream = FileStream::new(file);

        Ok(stream)
    }

    /// Save file to storage by stream
    pub async fn set_from_stream<T, B, E>(&self, key: &str, stream: &mut T) -> Result<(), Error>
    where
        T: futures::Stream<Item = Result<B, E>> + Unpin,
        B: AsRef<[u8]>,
        Error: From<E>,
    {
        let path = self.generate_path(key);

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        util::stream::write_file_from_stream(path, stream, MAX_UPLOADING_FILE_SIZE).await?;

        Ok(())
    }

    /// Destroys the storage and deletes all files from it
    pub async fn destroy_storage(self) -> Result<(), Error> {
        todo!()
    }

    /// Move the item with the specified key to another storage
    pub async fn move_to_another(&self, storage: &Storage, key: &str) -> Result<(), Error> {
        let path = self.generate_path(key);

        todo!();
    }

    /// Accept an item from another repository
    async fn accept_from_another(&self, key: &str) -> Result<(), Error> {
        let path = self.generate_path(key);

        todo!()
    }

    /// Delete a file from storage using a key
    pub async fn remove(&self, key: &str) -> Result<(), std::io::Error> {
        let path = self.generate_path(key);
        tokio::fs::remove_file(path).await
    }

    /// Generate an internal path to a file using a key
    fn generate_path(&self, key: &str) -> PathBuf {
        match self.use_subdir {
            true => self.root.join(&key[..2]).join(&key[2..4]).join(key),
            false => self.root.join(&key),
        }
    }
}
