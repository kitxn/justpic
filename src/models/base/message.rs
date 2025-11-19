/// Base server message-only response
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct MessageResponse {
    message: String,
}

impl MessageResponse {
    /// Create new message-only response
    pub fn new(msg: impl Into<String>) -> Self {
        MessageResponse {
            message: msg.into(),
        }
    }
}
