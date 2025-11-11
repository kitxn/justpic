//! Justpic backend server
// TODO: document the main components

pub mod config;
pub mod error;
pub mod state;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Preparing the application
/// and initializing the basic components
pub async fn setup_app() -> error::Result<state::State> {
    tracing::info!("Running {APP_NAME}-{APP_VERSION} setup");

    todo!()
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
pub fn configure_api(cfg: &mut actix_web::web::ServiceConfig) {
    todo!()
}

/// Configuring endpoints for API documentation
pub fn configure_api_docs(cfg: &mut actix_web::web::ServiceConfig) {
    todo!()
}
