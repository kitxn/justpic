use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
    ),
    info(
      title = "justpic-backend",
      description = "justpic backend API documentation",
      version = env!("CARGO_PKG_VERSION"),
    ),
    tags(
      (name = "auth", description = "Authentication endpoints"),
      (name = "cards", description = "Cards management")
    )
)]
pub struct ApiDoc;
