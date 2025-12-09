/// A public user model stripped of all private fields
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct UserPublic {
    pub id: uuid::Uuid,

    #[schema(example = "john_doe")]
    pub username: String,

    #[schema(value_type = String, example = "regular")]
    pub role: crate::models::users::role::UserRole,

    pub created: chrono::DateTime<chrono::Utc>,
}
