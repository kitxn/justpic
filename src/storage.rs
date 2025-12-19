use std::path::{Path, PathBuf};

use futures::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::io::ReaderStream;

use crate::error::Error;

pub type FileStream = ReaderStream<File>;

const MAX_UPLOADING_FILE_SIZE: usize = 64 * 1024 * 1024;

const DEFAULT_TEMP_DIR: &str = "tmp";

#[derive(Debug, Clone)]
pub struct Storage {
    root: std::path::PathBuf,

    temp: std::path::PathBuf,
}

// TODO: Add key validation
impl Storage {
    pub fn new(root: std::path::PathBuf) -> Self {
        let temp = root.join(DEFAULT_TEMP_DIR);
        Storage { root, temp }
    }

    pub fn init(&self) -> Result<(), Error> {
        std::fs::create_dir_all(&self.root)?;
        std::fs::create_dir_all(&self.temp)?;

        tracing::info!("File storage opened!");

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<FileStream, Error> {
        let path = generate_path_from_key(&self.root, key);
        let file = File::open(path).await?;

        let stream = FileStream::new(file);

        Ok(stream)
    }

    pub async fn set_from_stream<T, B, E>(
        &self,
        key: &str,
        stream: &mut T,
        is_temp: bool,
    ) -> Result<(), Error>
    where
        T: futures::Stream<Item = Result<B, E>> + Unpin,
        B: AsRef<[u8]>,
        Error: From<E>,
    {
        let root = if is_temp { &self.temp } else { &self.root };
        let path = generate_path_from_key(root, key);

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut file = File::create(path).await?;
        {
            let mut uploaded_bytes = 0;

            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                let chunk = chunk.as_ref();

                uploaded_bytes += chunk.len();
                if uploaded_bytes > MAX_UPLOADING_FILE_SIZE {
                    return Err(Error::PayloadTooLarge);
                }

                file.write_all(chunk).await?;
            }

            tracing::debug!("Writen {} bytes for {}", uploaded_bytes, key);
        }

        Ok(())
    }

    pub async fn copy(&self, from: &Path, to: &Path) -> Result<(), Error> {
        if !from.is_file() {
            return Err(Error::ItemNotFound);
        }

        if let Some(parent) = to.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::copy(from, to).await?;
        Ok(())
    }

    pub async fn commit_temp(&self, key: &str) -> Result<(), Error> {
        let temp_path = generate_path_from_key(&self.temp, key);
        if !temp_path.is_file() {
            return Err(Error::ItemNotFound);
        }

        let dest_path = generate_path_from_key(&self.root, key);
        if let Some(parent) = dest_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        if let Err(err) = tokio::fs::rename(&temp_path, &dest_path).await {
            match err.kind() {
                std::io::ErrorKind::CrossesDevices => {
                    tracing::warn!("Error moving file: fallback on copy");

                    self.copy(&temp_path, &dest_path).await?;
                    tokio::fs::remove_file(&temp_path).await?;
                }
                _ => {
                    return Err(err.into());
                }
            }
        }

        Ok(())
    }

    pub async fn remove(&self, key: &str) -> Result<(), Error> {
        let path = generate_path_from_key(&self.root, key);

        tokio::fs::remove_file(path).await?;

        Ok(())
    }
}

fn generate_path_from_key(root: &Path, key: &str) -> PathBuf {
    root.join(&key[..2]).join(&key[2..4]).join(key)
}
