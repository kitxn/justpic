use crate::models::cards::api::CreateCardRequestSchema;

#[utoipa::path(
    post,
    path = "/cards/", 
    tag = "cards", 
    request_body (
      content = CreateCardRequestSchema,
      content_type = "multipart/form-data"
    ),
)]
pub fn create_new() {}
