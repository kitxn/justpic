use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct DbCard {
    pub(super) id: i64,
    pub(super) file_key: String,

    pub(super) mimetype: String,
    pub(super) filesize: i64,

    pub(super) width: u32,
    pub(super) height: u32,

    pub(super) title: Option<String>,
    pub(super) description: Option<String>,

    pub(super) created: DateTime<Utc>,

    pub(super) source_url: Option<String>,

    pub(super) is_private: bool,
    pub(super) state: CardState,
}

impl DbCard {
    /// Create a new [`Card`] entity
    pub fn new(
        title: Option<String>,
        description: Option<String>,
        source_url: Option<String>,
        mimetype: String,
        filesize: i64,
        is_private: bool,
    ) -> Self {
        DbCard {
            id: crate::util::unid::generate(),
            file_key: crate::util::file_key::generate(),
            mimetype,
            filesize,
            // Default values
            width: 250,
            height: 250,
            title,
            description,
            created: Utc::now(),
            source_url,
            is_private,
            state: CardState::Pending,
        }
    }
}

#[derive(Debug, serde::Serialize, sqlx::Type, derive_more::Display)]
#[serde(rename_all = "lowercase")]
pub enum CardState {
    #[sqlx(rename = "pending")]
    #[display("pending")]
    Pending,
    #[sqlx(rename = "processing")]
    #[display("processing")]
    Processing,
    #[sqlx(rename = "ready")]
    #[display("ready")]
    Ready,
    #[sqlx(rename = "failed")]
    #[display("failed")]
    Failed,
}
