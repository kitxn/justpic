use utoipa::OpenApi;

use crate::routes::docs::{users, auth, files};

#[derive(OpenApi)]
#[openapi(
    servers(
      (url = "/api")
    ),
    paths(
      files::get_file_stream,
    
      users::get_me,
      users::get_all,
      users::get_by_username,
      users::update_me_username,
      users::update_me_password,
      
      auth::register,
      auth::login,
      auth::logout
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
