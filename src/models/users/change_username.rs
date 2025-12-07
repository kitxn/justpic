use serde::Deserialize;
use utoipa::ToSchema;

use crate::traits::validation::Validatable;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangeUsernameRequestData {
    #[schema(example = "new_john_doe")]
    pub username: String,
}

impl Validatable for ChangeUsernameRequestData {
    fn validate(&self) -> Result<(), crate::error::Error> {
        if !(3..42).contains(&self.username.len()) {
            return Err(crate::error::Error::Validation {
                field: "username",
                message: "Incorrect username length (3-42 characters required)",
            });
        }

        Ok(())
    }
}
