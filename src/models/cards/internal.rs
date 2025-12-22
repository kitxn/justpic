use chrono::{DateTime, Utc};
use sqlx::{FromRow, Row, sqlite::SqliteRow};

use crate::{
    models::{cards::api::CardApiModel, files::internal::File},
    util,
};

/// Internal model for card entity
#[derive(Debug)]
pub struct Card {
    /// Unique numeric ID of the card
    id: String,

    /// Model of the file associated with this card
    file: File,

    /// ID of the user who uploaded this card
    owner_id: uuid::Uuid,

    /// Card title
    title: Option<String>,

    /// Card description
    description: Option<String>,

    /// Card creation date
    created: DateTime<Utc>,

    /// The source from which the card was imported
    source_url: Option<String>,

    /// Card availability
    is_private: bool,
}

impl Card {
    // -- GETTERS --
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn file(&self) -> &File {
        &self.file
    }

    pub fn owner_id(&self) -> uuid::Uuid {
        self.owner_id
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }

    pub fn source_url(&self) -> Option<&String> {
        self.source_url.as_ref()
    }

    pub fn is_private(&self) -> bool {
        self.is_private
    }

    // -- FEATURES --
    pub fn new(
        file: File,
        owner_id: uuid::Uuid,
        title: Option<String>,
        description: Option<String>,
        source_url: Option<String>,
        is_private: bool,
    ) -> Self {
        let id = util::unid::generate().to_string();
        let created = Utc::now();

        Card {
            id,
            file,
            owner_id,
            title,
            description,
            created,
            source_url,
            is_private,
        }
    }

    pub fn is_owner(&self, user_id: &uuid::Uuid) -> bool {
        &self.owner_id == user_id
    }

    /// The user's ability to see the card
    pub fn can_view(&self, user_id: &uuid::Uuid) -> bool {
        if self.is_private() {
            return self.is_owner(user_id);
        }
        true
    }

    pub fn to_api_model(self) -> CardApiModel {
        CardApiModel {
            id: self.id,
            file: self.file.to_api_model(),
            owner_id: self.owner_id,
            title: self.title,
            description: self.description,
            created: self.created,
            source_url: self.source_url,
            is_private: self.is_private,
        }
    }
}

impl<'r> FromRow<'r, SqliteRow> for Card {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        // Manual mapping with nested structure
        Ok(Card {
            // TODO: REFACTOR PARSE ERROR MAPPING
            id: row.try_get::<String, _>("card_id")?,
            file: File::new_raw(
                row.try_get("file_id")?,
                row.try_get("file_uploader_id")?,
                row.try_get("file_mimetype")?,
                row.try_get("file_size")?,
                row.try_get("file_width")?,
                row.try_get("file_height")?,
                row.try_get("file_created")?,
                row.try_get("file_color")?,
                row.try_get("file_state")?,
            ),
            owner_id: row.try_get("card_owner_id")?,
            title: row.try_get("card_title")?,
            description: row.try_get("card_description")?,
            created: row.try_get("card_created")?,
            source_url: row.try_get("card_source_url")?,
            is_private: row.try_get("card_is_private")?,
        })
    }
}
