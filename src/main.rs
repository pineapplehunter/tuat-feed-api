//! # tuat-feed-api(TUAT Feed API Server)
//!
//! This is code for a server that formatsthe TUAT feed to json

use color_eyre::eyre::{ContextCompat, Result};
use log::info;
use std::env;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::Duration;
use structopt::StructOpt;
use tuat_feed_api::{
    handlers::{
        v1::{handle_academic, handle_campus, handle_index},
        v2,
    },
    State,
};
use warp::Filter;

/// Interval time (in minutes) for checking for new content.
#[cfg(feature = "cache")]
pub(crate) const INTERVAL_MIN: u64 = 15;
#[cfg(not(feature = "cache"))]
pub(crate) const INTERVAL_MIN: u64 = 0;

/// Interval duration computed from `INTERVAL_MIN`.
const INTERVAL: Duration = Duration::from_secs(INTERVAL_MIN * 60);

#[derive(StructOpt)]
#[structopt(name = "tuat-feed-api")]
struct Args {
    /// hostname
    #[structopt(short, long, default_value = "localhost")]
    hostname: String,
    /// port
    #[structopt(short, long, default_value = "8888")]
    port: u16,
}

/// the main server function
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    // if env is not set then default to RUST_LOG=info
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    // parse args
    let args = Args::from_args();

    // crate state
    let state = Arc::new(State::init(INTERVAL)?);
    let state = warp::any().map(move || state.clone());

    // paths
    let index = warp::any()
        .and(state.clone())
        .and_then(handle_index)
        .map(|data| warp::reply::json(&data));
    let academic = warp::path("academic")
        .and(state.clone())
        .and_then(handle_academic) 
        .map(|data| warp::reply::json(&data));
    let campus = warp::path("campus")
        .and(state.clone())
        .and_then(handle_campus)
        .map(|data| warp::reply::json(&data));

    let routes = warp::get().and(
        v2::v2_paths(state.clone().boxed())
            .or(academic)
            .or(campus)
            .or(index),
    );

    // parse address
    let address = format!("{}:{}", args.hostname, args.port)
        .to_socket_addrs()?
        .next()
        .wrap_err("could not resolve address")?;

    // start server
    info!("start server on {}", address);
    warp::serve(routes).run(address).await;

    Ok(())
}
