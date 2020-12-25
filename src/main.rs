#![warn(missing_docs)]

//! # tuat-feed-api(TUAT Feed API Server)
//!
//! This is code for a server that formatsthe TUAT feed to json

use anyhow::{anyhow, Context, Result};
use argh::FromArgs;
use log::info;
use std::env;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::Duration;

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
    #[argh(option, short = 'h', default = "String::from(\"localhost\")")]
    hostname: String,
    /// the port
    #[argh(option, short = 'p', default = "8888")]
    port: u16,
}

#[async_std::main]
/// the main server function
async fn main() -> Result<()> {
    // if env is not set then default to RUST_LOG=info
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "tuat_feed_api=info,tuat_feed_parser=info");
    }
    env_logger::init();

    // parse args
    let args: Args = argh::from_env();

    let mut app = tide::with_state(Arc::new(State::init().await?));
    app.at("/").get(handle_index);
    app.at("/academic").get(handle_academic);
    app.at("/campus").get(handle_campus);

    // parse address
    let address = format!("{}:{}", args.hostname, args.port)
        .to_socket_addrs()?
        .next()
        .context("could not resolve address")?;

    // start server
    info!("start server on {}", address);
    // warp::serve(routes).run(address).await;
    app.listen(address).await.map_err(|e| anyhow!(e))
}
