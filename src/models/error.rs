#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ApiError {
    #[schema(example = 500)]
    pub code: u16,
    #[schema(example = "ERROR_MESSAGE")]
    pub message: String,
}
