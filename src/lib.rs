//! Justpic backend server

pub mod config;
pub mod docs;
pub mod error;
pub mod state;

pub mod auth;

pub mod models;
pub mod routes;

pub mod util;

pub mod database;
pub mod storage;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

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
use utoipa_rapidoc::RapiDoc;

const DOC_JSON_URL: &str = "/api-docs/openapi.json";
const DOC_UI_URL: &str = "/docs/{_:.*}";

/// Configuring endpoints for API documentation
pub fn configure_api_docs(cfg: &mut actix_web::web::ServiceConfig) {
    let open_api = docs::ApiDoc::openapi();
    cfg.service(RapiDoc::with_openapi(DOC_JSON_URL, open_api).path(DOC_UI_URL));
}
