use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    #[schema(example = "Something happened!")]
    pub message: String,
}
