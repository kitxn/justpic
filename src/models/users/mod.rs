pub mod change_password;
pub mod change_username;

pub mod delete;

use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use crate::database::schemas::users::DbUser;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserPublicModel {
    pub(crate) id: uuid::Uuid,

    #[schema(example = "john_doe")]
    pub(crate) username: String,

    pub(super) created: DateTime<Utc>,
}

impl From<DbUser> for UserPublicModel {
    fn from(value: DbUser) -> Self {
        UserPublicModel {
            id: value.id,
            username: value.username,
            created: value.created,
        }
    }
}
