use serde::Deserialize;
use utoipa::ToSchema;

use crate::traits::validation::Validatable;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDeleteAccountRequest {
    #[schema(example = "hunter42!")]
    pub password: String,
}

impl Validatable for UserDeleteAccountRequest {
    fn validate(&self) -> Result<(), crate::error::Error> {
        if !(8..72).contains(&self.password.len()) {
            return Err(crate::error::Error::Validation {
                field: "password",
                message: "Incorrect password length (8-72 characters required)",
            });
        }

        Ok(())
    }
}
