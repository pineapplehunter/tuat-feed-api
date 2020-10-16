#![warn(missing_docs)]

//! # tuat-feed-api(TUAT Feed API Server)
//!
//! This is code for a server that formatsthe TUAT feed to json

use anyhow::{Context, Result};
use argh::FromArgs;
use log::info;
use std::env;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::Duration;
use warp::Filter;

mod state;
use state::State;
mod handler;
pub(crate) mod info_section;
use handler::{handle_academic, handle_campus, handle_index};

/// Interval time (in minutes) for checking for new content.
#[cfg(feature = "cache")]
pub(crate) const INTERVAL_MIN: u64 = 15;
#[cfg(not(feature = "cache"))]
pub(crate) const INTERVAL_MIN: u64 = 0;

/// Interval duration computed from `INTERVAL_MIN`.
const INTERVAL: Duration = Duration::from_secs(INTERVAL_MIN * 60);

#[derive(FromArgs)]
/// tuat feed api server
struct Args {
    /// the hostname
    #[argh(option, default = "String::from(\"localhost\")")]
    hostname: String,
    /// the port
    #[argh(option, default = "8888")]
    port: u16,
}

#[tokio::main]
/// the main server function
async fn main() -> Result<()> {
    // if env is not set then default to RUST_LOG=info
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    // parse args
    let args: Args = argh::from_env();

    // crate state
    let state = Arc::new(State::init().await?);
    let state = warp::any().map(move || state.clone());

    // paths
    let index = warp::any().and(state.clone()).and_then(handle_index);
    let academic = warp::path("academic")
        .and(state.clone())
        .and_then(handle_academic);
    let campus = warp::path("campus")
        .and(state.clone())
        .and_then(handle_campus);
    let routes = warp::get().and(academic.or(campus).or(index));

    // parse address
    let address = format!("{}:{}", args.hostname, args.port)
        .to_socket_addrs()?
        .next()
        .context("could not resolve address")?;

    // start server
    info!("start server on {}", address);
    warp::serve(routes).run(address).await;

    Ok(())
}
