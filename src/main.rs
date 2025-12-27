use std::path::Path;

use actix_web::{App, HttpServer};

use justpic_backend::{
    DATA_DIR, DATABASE_FILE, HOST_ADDR, MEDIA_DIR, database, error::Result, storage,
};

#[tokio::main]
async fn main() -> Result<()> {
    justpic_backend::setup_logger();

    tracing::info!("Opening database file...");
    let data_dir = Path::new(DATA_DIR);

    let db_path = data_dir.join(DATABASE_FILE);
    let pool = database::open(&db_path).await?;
    database::migrate(&pool).await?;

    tracing::info!("Opening files storage...");
    let media_dir = data_dir.join(MEDIA_DIR);
    let storage = storage::Storage::new(media_dir, true);
    storage.init()?;

    let state = justpic_backend::setup_app(pool, storage).await?;

    tracing::info!("Running swagger doc on http://{}/docs/", HOST_ADDR);
    HttpServer::new(move || {
        App::new()
            .configure(|cfg| {
                let state = state.clone();
                justpic_backend::configure_api(cfg, state)
            })
            .configure(justpic_backend::configure_api_docs)
    })
    .bind(HOST_ADDR)?
    .run()
    .await?;

    Ok(())
}
