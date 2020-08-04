#![warn(missing_docs)]

//! # tuat-feed-api(TUAT Feed API Server)
//!
//! This is code for a server that formatsthe TUAT feed to json

use anyhow::{Context, Result};
use log::info;
use std::env;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::try_join;
use tuat_feed_parser::{get_academic_feed, get_campus_feed, Info};
use warp::Filter;

/// Interval time (in minutes) for checking for new content.
#[cfg(feature = "cache")]
const INTERVAL_MIN: u64 = 15;
#[cfg(not(feature = "cache"))]
const INTERVAL_MIN: u64 = 0;

/// Interval duration computed from `INTERVAL_MIN`.
const INTERVAL: Duration = Duration::from_secs(INTERVAL_MIN * 60);

/// State of the server.
/// contains data for both academic and campus information.
struct State {
    /// academic information.
    academic: RwLock<InfoSection>,
    /// campus information.
    campus: RwLock<InfoSection>,
}

impl State {
    /// initializes the state.
    /// fetches the data from tuat feed and stores it.
    async fn init() -> Result<Self> {
        info!("initializing state");
        let (academic, campus) = try_join!(get_academic_feed(), get_campus_feed())?;
        // let academic = get_academic_feed().await.context("academic")?;
        // let campus = get_campus_feed().await.context("campus")?;

        Ok(Self {
            academic: RwLock::new(InfoSection::new(academic)),
            campus: RwLock::new(InfoSection::new(campus)),
        })
    }

    async fn get_academic(&self) -> Result<Vec<Info>> {
        let update_academic = Instant::now() > self.academic.read().await.last_checked + INTERVAL;

        if update_academic {
            self.academic.write().await.set(get_academic_feed().await?);
        }

        let info = self.academic.read().await.info.clone();

        Ok(info)
    }

    async fn get_campus(&self) -> Result<Vec<Info>> {
        let update_campus = Instant::now() > self.campus.read().await.last_checked + INTERVAL;

        if update_campus {
            self.campus.write().await.set(get_academic_feed().await?);
        }

        let info = self.campus.read().await.info.clone();

        Ok(info)
    }

    async fn get_all(&self) -> Result<Vec<Info>> {
        let (mut academic, campus) = try_join!(self.get_academic(), self.get_campus())?;

        academic.extend(campus);

        Ok(academic)
    }
}

/// InfoSection.
/// This struct holds the information and when it was last checked.
struct InfoSection {
    /// the time the information was last checked.
    last_checked: Instant,
    /// actual information.
    info: Vec<Info>,
}

impl InfoSection {
    /// creates a new InfoSection from a `Vec<Info>`.
    fn new(info: Vec<Info>) -> Self {
        InfoSection {
            info,
            last_checked: Instant::now(),
        }
    }

    /// set a new state.
    /// (used for updating the information)
    fn set(&mut self, info: Vec<Info>) {
        self.info = info;
        self.last_checked = Instant::now();
    }
}

async fn handle_academic(state: Arc<State>) -> Result<impl warp::Reply, warp::Rejection> {
    let data = state.get_academic().await;
    data.map(|data| warp::reply::json(&data))
        .map_err(|_e| warp::reject::reject())
}

async fn handle_campus(state: Arc<State>) -> Result<impl warp::Reply, warp::Rejection> {
    let data = state.get_campus().await;
    data.map(|data| warp::reply::json(&data))
        .map_err(|_e| warp::reject::reject())
}

async fn handle_index(state: Arc<State>) -> Result<impl warp::Reply, warp::Rejection> {
    let data = state.get_all().await;
    data.map(|data| warp::reply::json(&data))
        .map_err(|_e| warp::reject::reject())
}

#[tokio::main]
/// the main server function
async fn main() -> Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let mut args = pico_args::Arguments::from_env();
    let hostname = args
        .opt_value_from_str("--hostname")?
        .unwrap_or("localhost".to_string());
    let port = args.opt_value_from_str("--port")?.unwrap_or(8888);

    let state = Arc::new(State::init().await?);
    let state = warp::any().map(move || state.clone());

    let index = warp::any().and(state.clone()).and_then(handle_index);
    let academic = warp::path("academic")
        .and(state.clone())
        .and_then(handle_academic);
    let campus = warp::path("campus")
        .and(state.clone())
        .and_then(handle_campus);

    let routes = warp::get().and(academic.or(campus).or(index));

    let address = format!("{}:{}", hostname, port)
        .to_socket_addrs()?
        .next()
        .context("could not resolve address")?;
    info!("start server on {}", address);
    warp::serve(routes).run(address).await;

    Ok(())
}
