mod config;
mod control;
mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use control::PowerController;
use eyre::Context;
use std::sync::Arc;
use tokio::signal::unix::SignalKind;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), eyre::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env()?;

    tracing::info!(config = ?config, "Configured with");

    let controller = PowerController::new(config.controller_config).map(Arc::new)?;
    let app = Router::new()
        .route("/info/version", get(handlers::version))
        .route("/info/status", get(handlers::status))
        .route("/control/shutdown", post(handlers::shutdown))
        .route("/control/reboot", post(handlers::reboot))
        .route("/control/sleep", post(handlers::sleep))
        .with_state(controller);

    let shutdown = {
        #[cfg(target_os = "linux")]
        {
            let signal =
                |sig| tokio::signal::unix::signal(sig).wrap_err("Failed installing signal handler");
            let mut int = signal(SignalKind::interrupt())?;
            let mut term = signal(SignalKind::terminate())?;
            async move {
                tokio::select! {
                    _ = term.recv() => (),
                    _ = int.recv() => (),
                }
            }
        }
    };

    let listener = tokio::net::TcpListener::bind(&config.socket_addr)
        .await
        .with_context(|| eyre::format_err!("Failed binding to {}", config.socket_addr))?;
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown)
        .await?;

    Ok(())
}
