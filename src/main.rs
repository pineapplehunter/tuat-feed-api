use async_std::prelude::*;
use std::env::args;
use std::future::Future;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tide::{Request, Response};
use tokio::runtime::Runtime;
use tuat_feed_parser::{get_academic_feed, get_campus_feed, Info};

const INTERVAL_MIN: u64 = 15;

const INTERVAL: Duration = Duration::from_secs(INTERVAL_MIN * 60);

struct State {
    academic: Arc<RwLock<InfoSection>>,
    campus: Arc<RwLock<InfoSection>>,
}

impl State {
    fn init() -> impl Future<Output = Result<Self, String>> {
        async {
            let (academic, campus) = get_academic_info().join(get_campus_info()).await;
            Ok(Self {
                academic: Arc::new(RwLock::new(InfoSection::new(academic?))),
                campus: Arc::new(RwLock::new(InfoSection::new(campus?))),
            })
        }
    }
}

struct InfoSection {
    last_checked: Instant,
    info: Vec<Info>,
}

impl InfoSection {
    fn new(info: Vec<Info>) -> Self {
        InfoSection {
            info,
            last_checked: Instant::now(),
        }
    }

    fn set(&mut self, info: Vec<Info>) {
        self.info = info;
        self.last_checked = Instant::now();
    }
}

async fn get_academic_info() -> Result<Vec<Info>, String> {
    let rt = Runtime::new();
    if let Err(e) = rt {
        return Err(e.to_string());
    }
    let mut rt = rt.unwrap();
    rt.block_on(async { get_academic_feed().await })
}

async fn get_campus_info() -> Result<Vec<Info>, String> {
    let rt = Runtime::new();
    if let Err(e) = rt {
        return Err(e.to_string());
    }
    let mut rt = rt.unwrap();
    rt.block_on(async { get_campus_feed().await })
}

async fn handle_index(req: Request<State>) -> Response {
    if Instant::now() > req.state().academic.read().unwrap().last_checked + INTERVAL {
        println!("fetching academic");
        let data = get_academic_info().await;
        match data {
            Ok(data) => req.state().academic.write().unwrap().set(data),
            Err(e) => return Response::new(400).body_string(e),
        }
    }

    if Instant::now() > req.state().campus.read().unwrap().last_checked + INTERVAL {
        println!("fetching campus");
        let data = get_campus_info().await;
        match data {
            Ok(data) => req.state().campus.write().unwrap().set(data),
            Err(e) => return Response::new(400).body_string(e),
        }
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

async fn handle_campus(req: Request<State>) -> Response {
    if Instant::now() > req.state().campus.read().unwrap().last_checked + INTERVAL {
        println!("fetching campus");
        let data = get_campus_info().await;
        match data {
            Ok(data) => req.state().campus.write().unwrap().set(data),
            Err(e) => return Response::new(400).body_string(e),
        }
    }

    Response::new(200)
        .body_json(&req.state().campus.read().unwrap().info)
        .unwrap()
}

async fn handle_academic(req: Request<State>) -> Response {
    if Instant::now() > req.state().academic.read().unwrap().last_checked + INTERVAL {
        println!("fetching academic");
        let data = get_academic_info().await;
        match data {
            Ok(data) => req.state().academic.write().unwrap().set(data),
            Err(e) => return Response::new(400).body_string(e),
        }
    }

    Response::new(200)
        .body_json(&req.state().academic.read().unwrap().info)
        .unwrap()
}

#[async_std::main]
async fn main() -> Result<(), String> {
    let port: u16 = args()
        .nth(1)
        .unwrap_or_else(|| "8080".to_string())
        .parse()
        .unwrap();
    let mut app = tide::with_state(State::init().await?);
    app.at("/").get(handle_index);
    app.at("/campus").get(handle_campus);
    app.at("/academic").get(handle_academic);
    app.listen(("127.0.0.1", port))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
