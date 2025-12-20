use actix_multipart::{Field, Multipart};
use futures::StreamExt;

use crate::{error::Error, models::files::uploading::UploadedFile, state::State, util};

const MAX_TEXT_FIELD_SIZE: usize = 5;

#[derive(Debug)]
pub struct MultipartSegment {
    key: String,

    value: MultipartSegmentItem,
}

#[derive(Debug)]
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
    pub async fn from_field(
        state: &State,
        mut field: Field,
    ) -> Result<Option<MultipartSegment>, Error> {
        let Some(key) = field.name().map(str::to_string) else {
            return Ok(None);
        };
        let mimetype = field.content_type().map(|t| t.to_string());

        let disposition = field
            .content_disposition()
            .ok_or(Error::BadInput)?
            .to_owned();

        let seg = match (disposition.get_filename(), mimetype) {
            (Some(..), Some(mimetype)) => {
                // File with mimetype
                let temp_store = state.temp_store();
                let store_key = util::file_key::generate();

                temp_store.set_from_stream(&store_key, &mut field).await?;

                MultipartSegment {
                    key,
                    value: MultipartSegmentItem::File(UploadedFile {
                        store_key,
                        mimetype,
                    }),
                }
            }
            (Some(..), None) => {
                // File without mimetype
                // TODO: Add bad mimetype error
                return Err(Error::BadInput);
            }
            (None, ..) => {
                // Not a file => text field
                let text = parse_field_to_string(&mut field).await?;

                MultipartSegment {
                    key,
                    value: MultipartSegmentItem::Text(text),
                }
            }
        };

        Ok(Some(seg))
    }
}

async fn parse_field_to_string(field: &mut Field) -> Result<String, Error> {
    let bytes = util::stream::write_buff_from_stream(field, MAX_TEXT_FIELD_SIZE).await?;

    String::from_utf8(bytes).map_err(|e| {
        tracing::warn!("Failed to parse the multipart field into a string: {e}");

        // FIXME: Add normal error handling here!
        Error::BadInput
    })
}

pub async fn parse_multipart(
    state: &State,
    mut mp: Multipart,
) -> Result<Vec<MultipartSegment>, Error> {
    let mut segments: Vec<MultipartSegment> = Vec::new();

    while let Some(field) = mp.next().await {
        let field = field?;
        let seg = MultipartSegment::from_field(state, field).await?;

        if let Some(seg) = seg {
            segments.push(seg);
        }
    }

    Ok(segments)
}
