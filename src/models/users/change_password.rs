use serde::Deserialize;
use utoipa::ToSchema;

use crate::traits::validation::Validatable;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequestData {
    #[schema(example = "hunter52!")]
    pub new_password: String,
}

impl Validatable for ChangePasswordRequestData {
    fn validate(&self) -> Result<(), crate::error::Error> {
        // TODO: Add password complexity checking
        if !(8..72).contains(&self.new_password.len()) {
            return Err(crate::error::Error::Validation {
                field: "new_password",
                message: "Incorrect password length (8-72 characters required)",
            });
        }

        Ok(())
    }
}
