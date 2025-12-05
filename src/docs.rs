use crate::routes;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
      routes::files::get::get_file_stream,
      
      routes::users::fetch_by_username::fetch_by_username,
      routes::users::fetch_by_session::fetch_by_session,
      
      routes::auth::register::register,
      routes::auth::login::login,
      routes::auth::logout::logout
    ),
    info(
      title = "justpic-backend",
      description = "justpic backend API documentation",
      version = env!("CARGO_PKG_VERSION"),
    ),
    tags(
      (name = "auth", description = "Authentication endpoints"),
      (name = "users", description = "Users endpoints"),
      (name = "cards", description = "Cards management"),
      (name = "files", description = "Files endpoints"),
      (name = "system", description = "System endpoints")
    )
)]
pub struct ApiDoc;
