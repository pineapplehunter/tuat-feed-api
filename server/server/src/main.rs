//! # tuat-feed-api(TUAT Feed API Server)
//!
//! This is code for a server that formatsthe TUAT feed to json

use std::{env, net::SocketAddr, str::FromStr, sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tuat_feed_server::{app, state::ServerState};

/// Interval time (in minutes) for checking for new content.
const INTERVAL_MINUTES: u64 = 15;

/// Interval duration computed from `INTERVAL_MIN`.
const INTERVAL: Duration = Duration::from_secs(INTERVAL_MINUTES * 60);

#[tokio::main]
async fn main() {
    let state = Arc::new(ServerState::init());
    let state_cloned = state.clone();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "tuat_feed_server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let base_path = env::var("TUAT_FEED_API_BASEPATH").unwrap_or_else(|_| "".to_string());
    let addr = env::var("TUAT_FEED_API_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_owned());

    tokio::spawn(async move {
        loop {
            state_cloned.update().await;
            sleep(INTERVAL).await;
        }
    });
    let address = SocketAddr::from_str(&addr).unwrap();
    info!("starting server on http://{}/{}", address, base_path);

    axum::Server::bind(&address)
        .serve(app(base_path, state.clone()).into_make_service())
        .await
        .unwrap();
}
