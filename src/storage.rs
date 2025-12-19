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

        tracing::info!("File storage opened!");

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<FileStream, Error> {
        // todo: add base key validation

        let path = self.root.join(&key[..2]).join(&key[2..4]).join(key);
        let file = File::open(path).await?;

        let stream = FileStream::new(file);

        Ok(stream)
    }

    pub async fn set_from_stream<T, B, E>(&self, key: &str, stream: &mut T) -> Result<(), Error>
    where
        T: futures::Stream<Item = Result<B, E>> + Unpin,
        B: AsRef<[u8]>,
        Error: From<E>,
    {
        let path = self.root.join(&key[..2]).join(&key[2..4]).join(key);

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
                    // FIXME: TEMP ERROR TYPE!
                    return Err(Error::BadInput);
                }

                file.write_all(chunk).await?;
            }

            tracing::info!("Writed {} bytes for {}", uploaded_bytes, key);
        }

        Ok(())
    }

    pub async fn remove(&self, key: &str) -> Result<(), Error> {
        let path = self.root.join(&key[..2]).join(&key[2..4]).join(key);

        tokio::fs::remove_file(path).await?;

        Ok(())
    }
}
