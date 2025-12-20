use std::path::Path;

use crate::error::Error;
use futures::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};

const MB_BYTES_COUNT: usize = 1024 * 1024;

/// Record the transmitted stream to the buffer
pub async fn write_buff_from_stream<T, B, E>(
    stream: &mut T,
    mb_limit: usize,
) -> Result<Vec<u8>, Error>
where
    T: futures::Stream<Item = Result<B, E>> + Unpin,
    B: AsRef<[u8]>,
    Error: From<E>,
{
    let limit = mb_limit * MB_BYTES_COUNT;
    let mut buff = Vec::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let chunk = chunk.as_ref();

        let bytes_count = buff.len() + chunk.len();
        if bytes_count > limit {
            tracing::warn!(
                "The buffer recorded from the stream turned out to be larger than the maximum size: {} > {}",
                bytes_count,
                limit
            );
            return Err(Error::PayloadTooLarge);
        }

        buff.extend_from_slice(chunk);
    }

    Ok(buff)
}

/// Save the transferred stream to the specified file
pub async fn write_file_from_stream<T, B, E, P>(
    path: P,
    stream: &mut T,
    limit: usize,
) -> Result<(), Error>
where
    T: futures::Stream<Item = Result<B, E>> + Unpin,
    B: AsRef<[u8]>,
    Error: From<E>,
    P: AsRef<Path>,
{
    let limit = limit * MB_BYTES_COUNT;

    let path = path.as_ref();
    let mut file = File::create(path).await?;
    {
        let mut uploaded_bytes = 0;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let chunk = chunk.as_ref();

            uploaded_bytes += chunk.len();
            if uploaded_bytes > limit {
                return Err(Error::PayloadTooLarge);
            }

            file.write_all(chunk).await?;
        }

        tracing::debug!("Writen {} bytes for {:?}", uploaded_bytes, path);
    }

    Ok(())
}
