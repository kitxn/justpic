use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use justpic_backend::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    justpic_backend::setup_logger();

    let config = justpic_backend::config::Configuration::default(); // temp

    let state = justpic_backend::setup_app(&config).await?;

    tracing::info!("Running swagger doc on http://{}/docs/", config.host_addr());

    HttpServer::new(move || {
        App::new()
            .configure(justpic_backend::configure_api_docs)
            .configure(|cfg| {
                let state = state.to_owned();
                justpic_backend::configure_api(cfg, state)
            })
    })
    .bind(config.host_addr())?
    .run()
    .await?;

    Ok(())
}
