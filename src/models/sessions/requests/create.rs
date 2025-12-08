use serde::Deserialize;
use utoipa::ToSchema;

use crate::traits::validation::Validatable;

#[derive(Debug, Deserialize, ToSchema)]
pub struct SessionCreateRequest {
    #[schema(example = "john_doe")]
    pub username: String,

    #[schema(example = "hunter42!")]
    pub password: String,
}

impl Validatable for SessionCreateRequest {
    fn validate(&self) -> Result<(), crate::error::Error> {
        if self.username.len() > 42 {
            return Err(crate::error::Error::Validation {
                field: "username",
                message: "Incorrect username length (3-42 characters required)",
            });
        }

        if self.password.len() > 72 {
            return Err(crate::error::Error::Validation {
                field: "password",
                message: "Incorrect password length (8-72 characters required)",
            });
        }

        Ok(())
    }
}
