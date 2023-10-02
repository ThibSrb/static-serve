mod cli;
mod handler;
mod settings;

use std::net::SocketAddr;

use crate::cli::Cli;
use anyhow::Result;
use axum::Router;
use clap::Parser;

use hyper::Server;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let (server_settings, service_settings) = cli.into_settings();

    let app = Router::from(service_settings);

    let addr = SocketAddr::from(([127, 0, 0, 1], server_settings.port));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
