use actix_multipart::{Field, Multipart};
use futures::StreamExt;

use crate::{error::Error, models::files::uploading::UploadedFile, storage::Storage};

pub struct MultipartSegment {
    key: String,

    value: MultipartSegmentItem,
}

pub enum MultipartSegmentItem {
    Text(String),

    File(UploadedFile),
}

impl MultipartSegmentItem {
    pub fn is_text(&self) -> bool {
        matches!(self, MultipartSegmentItem::Text(..))
    }

    pub fn is_file(&self) -> bool {
        matches!(self, MultipartSegmentItem::File(..))
    }
}

impl MultipartSegment {
    pub async fn from_field_with_fs(
        fs: &Storage,
        mut field: Field,
    ) -> Result<Option<MultipartSegment>, Error> {
        let Some(key) = field.name() else {
            return Ok(None);
        };

        Ok(None)
    }
}

pub async fn parse_multipart_with_fs(
    fs: &Storage,
    mut mp: Multipart,
) -> Result<Vec<MultipartSegment>, Error> {
    let mut segments: Vec<MultipartSegment> = Vec::new();

    while let Some(field) = mp.next().await {
        let field = field?;
        let seg = MultipartSegment::from_field_with_fs(fs, field).await?;

        if let Some(seg) = seg {
            segments.push(seg);
        }
    }

    Ok(segments)
}
