use std::path::PathBuf;

use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

pub mod sessions;
pub mod users;

pub mod docs;
pub mod error;
pub mod state;

pub mod handlers;
pub mod models;

pub mod util;

pub mod repositories;

pub mod database;
pub mod storage;

pub mod types;

pub const APP_NAME: &str = "justpic";
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const SESSION_LIFETIME: u64 = 7;
pub const SESSION_COOKIE_NAME: &str = "client_session";

pub const HOST_ADDR: &str = "0.0.0.0:8080";

pub const DATA_DIR: &str = "./data";
pub const MEDIA_DIR: &str = "./media";
pub const TEMP_DIR: &str = "./tmp";
pub const DATABASE_FILE: &str = "./data.db";

const DOC_JSON_URL: &str = "/docs/openapi.json";
const DOC_UI_URL: &str = "/docs/{_:.*}";

/// Preparing the application
/// and initializing the basic components
pub async fn setup_app(
    pool: database::DatabasePool,
    storage: storage::Storage,
) -> error::Result<state::State> {
    tracing::info!("Setting up {APP_NAME}-{APP_VERSION}...");

    let temp_storage = storage::Storage::new(PathBuf::from(TEMP_DIR), false);
    temp_storage.init()?;

    let state = state::State::new(pool, storage, temp_storage);

    Ok(state)
}

/// Initializing the application logger
pub fn setup_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .with_target(false)
        .init();
}

/// Configuring API endpoints and DI
pub fn configure_api(cfg: &mut actix_web::web::ServiceConfig, state: state::State) {
    cfg.app_data(web::Data::new(state))
        .configure(users::handler::config)
        .configure(sessions::handler::config);
}

/// Configuring endpoints for API documentation
pub fn configure_api_docs(cfg: &mut actix_web::web::ServiceConfig) {
    let openapi = docs::ApiDoc::openapi();
    let config = Config::default().use_base_layout();
    cfg.service(
        SwaggerUi::new(DOC_UI_URL)
            .url(DOC_JSON_URL, openapi)
            .config(config),
    );
}
