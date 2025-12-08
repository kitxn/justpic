use crate::routes;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    servers(
      (url = "/api")
    ),
    paths(
      routes::files::get::get_file_stream,
      
      routes::users::fetch_by_username::fetch_by_username,
      routes::users::fetch_me::fetch_me,

      routes::users::change_me_password::change_me_password,
      routes::users::change_me_username::change_me_username,
      
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
      (name = "auth", description = "Authentication and Authorization endpoints"),
      (name = "search", description = "Search endpoints"),
      (name = "cards", description = "Cards endpoints"),
      (name = "files", description = "Files endpoints"),
      (name = "users", description = "Justpic Users endpoints"),
      (name = "users.me", description = "Own User Endpoints. An active session is required in client_session cookie"),
      (name = "system", description = "System endpoints")
    )
)]
pub struct ApiDoc;
