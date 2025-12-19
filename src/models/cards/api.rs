use chrono::{DateTime, Utc};

use crate::models::files::api::FileApiModel;

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct CardApiModel {
    /// Card id
    #[schema(example = "12857460700869832838")]
    pub(super) id: String,

    /// File associated with the card
    pub(super) file: FileApiModel,

    /// ID of the user who uploaded this card
    pub(super) owner_id: uuid::Uuid,

    /// Card title
    #[schema(example = "cute cat :3")]
    pub(super) title: Option<String>,

    /// Card description
    #[schema(example = "This is a really cute cat :3")]
    pub(super) description: Option<String>,

    /// Card creation date
    pub(super) created: DateTime<Utc>,

    /// The source from which the card was imported
    #[schema(example = "example.com/cool-image")]
    pub(super) source_url: Option<String>,

    /// Card availability
    #[schema(example = true)]
    pub(super) is_private: bool,
}

/// Open API multipart request schema
#[derive(Debug, utoipa::ToSchema)]
pub struct CreateCardRequestSchema {
    #[schema(value_type = String, format = Binary)]
    pub(super) file: Vec<u8>,

    #[schema(example = "Cute cats :3")]
    pub(super) title: Option<String>,

    #[schema(example = "Really cute cats!")]
    pub(super) description: Option<String>,
}
