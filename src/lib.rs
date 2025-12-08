//! Justpic backend server

pub mod config;
pub mod docs;
pub mod error;
pub mod state;

pub mod auth;

pub mod models;
pub mod routes;

pub mod traits;
pub mod util;

pub mod repositories;

pub mod database;
pub mod storage;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const SESSION_LIFETIME: u64 = 28;
pub const SESSION_COOKIE_NAME: &str = "client_session";

/// Preparing the application
/// and initializing the basic components
pub async fn setup_app(
    cfg: &config::Configuration,
    pool: database::DatabasePool,
    storage: storage::Storage,
) -> error::Result<state::State> {
    tracing::info!("Setting up {APP_NAME}-{APP_VERSION}...");

    let state = state::State::new(cfg.clone(), pool, storage);

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
        .configure(routes::config);
}

use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

const DOC_JSON_URL: &str = "/docs/openapi.json";
const DOC_UI_URL: &str = "/docs/{_:.*}";

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
