#![deny(clippy::all)]
mod cli;
mod handler;
mod settings;

use crate::cli::Cli;
use anyhow::Result;
use axum::{Router, extract::Request, response::Response};
use clap::Parser;
use std::{net::SocketAddr, time::Duration};
use tokio::{net::TcpListener, signal};
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with_span_events(FmtSpan::NONE)
        .init();
    let cli = Cli::parse();

    let (server_settings, service_settings) = cli.into_settings();

    let app = Router::from(service_settings).layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                tracing::info_span!(
                    "http_request",
                    method = ?request.method(),
                    path = ?request.uri().path(),
                )
            })
            .on_response(|res: &Response<_>, latency: Duration, span: &Span| {
                tracing::info!(parent: span,status = %res.status().as_u16(), latency = ?latency);
            }),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], server_settings.port));
    let tcp_listener = TcpListener::bind(addr).await?;

    axum::serve(tcp_listener, app)
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
