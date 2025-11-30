use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use crate::database::schemas::users::DbUser;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub(crate) id: uuid::Uuid,

    #[schema(example = "john_doe")]
    pub(crate) username: String,

    pub(super) created: DateTime<Utc>,
}

impl From<DbUser> for UserResponse {
    fn from(value: DbUser) -> Self {
        UserResponse {
            id: value.id,
            username: value.username,
            created: value.created,
        }
    }
}
