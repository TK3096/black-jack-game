use std::net::SocketAddr;

use anyhow::{Ok, Result};
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tracing::info;

use crate::infrastructure::axum_http::default_router;

pub async fn start() -> Result<()> {
    let app = Router::new()
        .fallback(default_router::not_found)
        .route("/helth-check", get(default_router::healt_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let listener = TcpListener::bind(addr).await?;

    info!("Server is running on port {}", 3001);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down gracefully...");
        }
        _ = terminate => {
            info!("Received termination signal, shutting down...");
        }
    }
}
