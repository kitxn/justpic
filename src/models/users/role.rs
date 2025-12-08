#[derive(Debug, sqlx::Type, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    #[sqlx(rename = "regular")]
    Regular,

    #[sqlx(rename = "admin")]
    Admin,
}

impl UserRole {
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }
}
