use actix_web::{App, HttpServer};

use justpic_backend::{database, error::Result, storage};

// TODO: ADD INTEGR. TESTS FOR DB MODELS

#[tokio::main]
async fn main() -> Result<()> {
    justpic_backend::setup_logger();

    // Todo: add loading config from file
    let config = justpic_backend::config::Configuration::default();

    tracing::info!("Opening database file...");
    let db_path = config.data_dir().join(config.db_path());
    let pool = database::sqlite::open_db(&db_path).await?;
    database::sqlite::apply_migrations(&pool).await?;

    tracing::info!("Opening files storage...");
    let storage = storage::Storage::new(config.media_dir().to_path_buf());
    storage.init()?;

    let state = justpic_backend::setup_app(&config, pool, storage).await?;

    tracing::info!("Running swagger doc on http://{}/docs/", config.host_addr());
    HttpServer::new(move || {
        App::new()
            .configure(justpic_backend::configure_api_docs)
            .configure(|cfg| {
                let state = state.clone();
                justpic_backend::configure_api(cfg, state)
            })
    })
    .bind(config.host_addr())?
    .run()
    .await?;

    Ok(())
}
