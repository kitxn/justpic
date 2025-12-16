use chrono::{DateTime, Utc};

use crate::models::files::state::FileState;

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct FileApiModel {
    #[schema(example = "1a5fa724d3914f2ab6a0c9a4dc5b1aa4")]
    pub(super) id: String,

    pub(super) uploader_id: uuid::Uuid,

    #[schema(example = "image/jpeg")]
    pub(super) mimetype: String,

    #[schema(example = 12_725)]
    pub(super) filesize: i64,

    #[schema(example = 720)]
    pub(super) width: u32,

    #[schema(example = 1080)]
    pub(super) height: u32,

    pub(super) created: DateTime<Utc>,

    #[schema(example = "ff00ff")]
    pub(super) color: String,

    #[schema(example = FileState::Pending)]
    pub(super) state: FileState,
}
