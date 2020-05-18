#![warn(missing_docs)]

//! # tuat-feed-api(TUAT Feed API Server)
//!
//! This is code for a server that formatsthe TUAT feed to json

use anyhow::{anyhow, Result};
use async_std::prelude::*;
use std::future::Future;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use structopt::StructOpt;
use tide::{Request, Response};
use tokio::runtime::Runtime;
use tuat_feed_parser::{get_academic_feed, get_campus_feed, Info};

/// Interval time (in minutes) for checking for new content.
const INTERVAL_MIN: u64 = 15;

/// Interval duration computed from `INTERVAL_MIN`.
const INTERVAL: Duration = Duration::from_secs(INTERVAL_MIN * 60);

/// State of the server.
/// contains data for both academic and campus information.
struct State {
    /// academic information.
    academic: Arc<RwLock<InfoSection>>,
    /// campus information.
    campus: Arc<RwLock<InfoSection>>,
}

impl State {
    /// initializes the state.
    /// fetches the data from tuat feed and stores it.
    fn init() -> impl Future<Output = Result<Self>> {
        async {
            let (academic, campus) = get_academic_info().join(get_campus_info()).await;
            Ok(Self {
                academic: Arc::new(RwLock::new(InfoSection::new(
                    academic.map_err(|e| anyhow!("could not get academic info: {}", e))?,
                ))),
                campus: Arc::new(RwLock::new(InfoSection::new(
                    campus.map_err(|e| anyhow!("could not get campus info: {}", e))?,
                ))),
            })
        }
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

/// this function get's the academic information from `tuat-feed-parser`
async fn get_academic_info() -> Result<Vec<Info>> {
    let mut rt = Runtime::new()?;
    Ok(rt.block_on(async { get_academic_feed().await })?)
}

/// this function get's the campus information from `tuat-feed-parser`
async fn get_campus_info() -> Result<Vec<Info>> {
    let mut rt = Runtime::new()?;
    Ok(rt.block_on(async { get_campus_feed().await })?)
}

/// handler for /
async fn handle_index(req: Request<State>) -> Response {
    let update_academic =
        Instant::now() > req.state().academic.read().unwrap().last_checked + INTERVAL;
    let update_campus = Instant::now() > req.state().campus.read().unwrap().last_checked + INTERVAL;

    match (update_academic, update_campus) {
        (true, true) => {
            println!("fetching both");
            let (data_academic, data_campus) = get_academic_info().join(get_campus_info()).await;
            match data_academic {
                Ok(data) => req.state().academic.write().unwrap().set(data),
                Err(e) => return Response::new(400).body_string(format!("error:{}", e)),
            }

            match data_campus {
                Ok(data) => req.state().campus.write().unwrap().set(data),
                Err(e) => return Response::new(400).body_string(format!("error:{}", e)),
            }
        }
        (true, false) => {
            println!("fetching academic");
            let data = get_academic_info().await;
            match data {
                Ok(data) => req.state().academic.write().unwrap().set(data),
                Err(e) => return Response::new(400).body_string(format!("error:{}", e)),
            }
        }
        (false, true) => {
            println!("fetching campus");
            let data = get_campus_info().await;
            match data {
                Ok(data) => req.state().campus.write().unwrap().set(data),
                Err(e) => return Response::new(400).body_string(format!("error:{}", e)),
            }
        }
        _ => {}
    }

    let res = req
        .state()
        .academic
        .read()
        .unwrap()
        .info
        .iter()
        .chain(req.state().campus.read().unwrap().info.iter())
        .cloned()
        .collect::<Vec<Info>>();

    Response::new(200).body_json(&res).unwrap()
}

/// handler for /campus
async fn handle_campus(req: Request<State>) -> Response {
    if Instant::now() > req.state().campus.read().unwrap().last_checked + INTERVAL {
        println!("fetching campus");
        let data = get_campus_info().await;
        match data {
            Ok(data) => req.state().campus.write().unwrap().set(data),
            Err(e) => return Response::new(400).body_string(format!("error:{}", e)),
        }
    }

    Response::new(200)
        .body_json(&req.state().campus.read().unwrap().info)
        .unwrap()
}

/// handler for /academic
async fn handle_academic(req: Request<State>) -> Response {
    if Instant::now() > req.state().academic.read().unwrap().last_checked + INTERVAL {
        println!("fetching academic");
        let data = get_academic_info().await;
        match data {
            Ok(data) => req.state().academic.write().unwrap().set(data),
            Err(e) => return Response::new(400).body_string(format!("error:{}", e)),
        }
    }

    Response::new(200)
        .body_json(&req.state().academic.read().unwrap().info)
        .unwrap()
}

#[derive(StructOpt, Debug)]
#[structopt(name = "tuat feed api")]
struct Opt {
    #[structopt(short, long)]
    port: u16,
    #[structopt(short, long, default_value = "localhost")]
    hostname: String,
}

#[async_std::main]
/// the main server function
async fn main() -> Result<()> {
    let Opt { port, hostname, .. } = Opt::from_args();
    let mut app = tide::with_state(State::init().await?);
    app.at("/").get(handle_index);
    app.at("/campus").get(handle_campus);
    app.at("/academic").get(handle_academic);
    println!("server start!");

    app.listen((hostname.as_str(), port))
        .await
        .map_err(|e| anyhow!("could not start server: {}", e))?;
    Ok(())
}
