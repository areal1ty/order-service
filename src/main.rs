use axum::{routing::{get}, Router, ServiceExt};

use std::net::SocketAddr;
use tower::ServiceBuilder;
use axum::extract::State;
use std::sync::Arc;
use crate::state::state::AppState;
use crate::routes::routes::{get_order};
use crate::logging::logging::init_logging;
use crate::scripts::load_data::load_data;
use crate::config::config::Config;
use crate::db::{util::has_orders};

use clap::Parser;
use log::{info, error};
use tokio::time::Duration;
use tokio::signal;
use tower_http::trace::TraceLayer;

mod scripts;
mod config;
mod db;
mod handlers;
mod routes;
mod state;
mod logging;
mod cache;

#[tokio::main]
async fn main() {
    init_logging();
    let config = Config::from_env();

    let shared_state = Arc::new(AppState::new(&config.database_url, &config.redis_url)
        .await);

    if !has_orders(&shared_state).await.expect("Failed to check for orders") {
        info!("No orders found, inserting mock data...");
        if let Err(e) = load_data(&shared_state).await {
            error!("Failed to insert mock data: {:?}", e);
            return;
        }
        info!("Mock data inserted successfully.");
    } else {
        info!("Orders found, skipping mock data insertion.");
    }

    let app = Router::new()
        .route("/orders/:order_uid", get(get_order))
        .with_state(Arc::clone(&shared_state))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .timeout(Duration::from_secs(30))
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::info!("Server is running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");

}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    tracing::info!("Shutting down gracefully...");
}
