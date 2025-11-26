#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}
