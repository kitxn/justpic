use serde::Deserialize;
use utoipa::ToSchema;

use crate::traits::validation::Validatable;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserChangePasswordRequest {
    #[schema(example = "hunter42!")]
    pub old_password: String,

    #[schema(example = "hunter52!")]
    pub new_password: String,
}

impl Validatable for UserChangePasswordRequest {
    fn validate(&self) -> Result<(), crate::error::Error> {
        // TODO: Add password complexity checking
        if !(8..72).contains(&self.new_password.len()) {
            return Err(crate::error::Error::Validation {
                field: "new_password",
                message: "Incorrect password length (8-72 characters required)",
            });
        }

        if self.old_password == self.new_password {
            return Err(crate::error::Error::Validation {
                field: "new_password",
                message: "The new password must not be the same as the old one",
            });
        }

        Ok(())
    }
}
