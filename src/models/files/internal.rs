use std::process::id;

use chrono::{DateTime, Utc};

use crate::{
    models::files::{api::FileApiModel, state::FileState, uploading::UploadedFile},
    util,
};

#[derive(sqlx::FromRow, Debug, Clone)]
/// Internal model for file entity
///
/// Stores all information about the file
/// and the logic for working with it
pub struct File {
    /// Unique 32-character file identifier
    id: String,

    /// ID of the user who uploaded the card with this file
    uploader_id: uuid::Uuid,

    // TODO: Add a custom type for mimetype
    /// File type
    mimetype: String,
    /// File size in bits
    filesize: i64,

    /// File Width
    width: u32,
    /// File Height
    height: u32,

    /// File record creation date
    created: DateTime<Utc>,

    // TODO: Add a custom type for color
    /// Accent color of the file
    color: String,

    /// Current file state
    state: FileState,
}

impl File {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn uploader_id(&self) -> uuid::Uuid {
        self.uploader_id
    }

    pub fn mimetype(&self) -> &str {
        &self.mimetype
    }

    pub fn filesize(&self) -> i64 {
        self.filesize
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn state(&self) -> &FileState {
        &self.state
    }

    pub fn new(
        uploader_id: uuid::Uuid,
        mimetype: String,
        filesize: i64,
        width: u32,
        height: u32,
        color: String,
    ) -> Self {
        let id = util::file_key::generate();
        let created = Utc::now();

        let state = FileState::Pending;

        Self::new_raw(
            id,
            uploader_id,
            mimetype,
            filesize,
            width,
            height,
            created,
            color,
            state,
        )
    }

    pub fn from_uploaded(item: UploadedFile, uploader_id: uuid::Uuid) -> Self {
        let created = Utc::now();
        let state = FileState::Pending;

        Self::new_raw(
            item.file_id,
            uploader_id,
            item.mimetype,
            item.size as i64,
            0, // temp
            0, // temp
            created,
            "ffffff".into(), // temp
            state,
        )
    }

    pub fn new_raw(
        id: String,
        uploader_id: uuid::Uuid,
        mimetype: String,
        filesize: i64,
        width: u32,
        height: u32,
        created: DateTime<Utc>,
        color: String,
        state: FileState,
    ) -> Self {
        File {
            id,
            uploader_id,
            mimetype,
            filesize,
            width,
            height,
            created,
            color,
            state,
        }
    }

    pub fn is_video(&self) -> bool {
        self.mimetype.starts_with("video")
    }

    pub fn is_image(&self) -> bool {
        self.mimetype.starts_with("image")
    }

    pub fn is_ready(&self) -> bool {
        self.state.is_ready()
    }

    // -- MAPPERS --
    pub fn to_api_model(self) -> FileApiModel {
        FileApiModel {
            id: self.id,
            uploader_id: self.uploader_id,
            mimetype: self.mimetype,
            filesize: self.filesize,
            width: self.width,
            height: self.height,
            created: self.created,
            color: self.color,
            state: self.state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MIMETYPE: &str = "image/jpeg";
    const TEST_COLOR: &str = "ffffff";
    const TEST_FILESIZE: i64 = 12872;
    const TEST_WIDTH: u32 = 262;
    const TEST_HEIGHT: u32 = 642;

    fn create_file_model_for_test() -> File {
        File::new(
            uuid::Uuid::new_v4(),
            TEST_MIMETYPE.into(),
            TEST_FILESIZE,
            TEST_WIDTH,
            TEST_HEIGHT,
            TEST_COLOR.into(),
        )
    }

    #[test]
    fn should_create_a_file_model() {
        let item = create_file_model_for_test();

        assert!(
            item.state.is_pending(),
            "After creation, the file must be in the pending state"
        );

        assert_eq!(
            item.mimetype(),
            TEST_MIMETYPE,
            "The model's mimetype must match the passed mimetype"
        );

        let time_delta = Utc::now().timestamp() - item.created.timestamp();
        assert!(
            (0..=5).contains(&time_delta),
            "The creation time should roughly correspond to the current one"
        )
    }
}
