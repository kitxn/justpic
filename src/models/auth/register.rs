use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::traits::validation::Validatable;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequestData {
    #[schema(example = "john_doe")]
    pub username: String,

    #[schema(example = "hunter42!")]
    pub password: String,

    #[schema(example = "hunter42!")]
    pub password_confirmation: String,
}

// TODO: Cover validation with tests
impl Validatable for RegisterRequestData {
    fn validate(&self) -> Result<(), crate::error::Error> {
        if self.password != self.password_confirmation {
            return Err(crate::error::Error::Validation {
                field: "password_confirmation",
                message: "The passwords do not match",
            });
        }

        if !(3..42).contains(&self.username.len()) {
            return Err(crate::error::Error::Validation {
                field: "username",
                message: "Incorrect username length (3-42 characters required)",
            });
        }

        // TODO: Add password complexity checking
        if !(8..72).contains(&self.password.len()) {
            return Err(crate::error::Error::Validation {
                field: "password",
                message: "Incorrect password length (8-72 characters required)",
            });
        }

        // TODO: Add check for prohibited characters
        Ok(())
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponseData {
    #[schema(example = "Registered")]
    pub message: &'static str,

    #[schema(example = "john_doe")]
    pub username: String,
}
