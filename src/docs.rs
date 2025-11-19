use crate::routes;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
      routes::ping::ping,
    ),
    info(
      title = "justpic-backend",
      description = "justpic backend API documentation",
      version = env!("CARGO_PKG_VERSION"),
    ),
    tags(
      (name = "auth", description = "Authentication endpoints"),
      (name = "cards", description = "Cards management"),
      (name = "system", description = "System endpoints")
    )
)]
pub struct ApiDoc;
