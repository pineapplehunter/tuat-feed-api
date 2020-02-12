use std::env::args;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tide::{Request, Response};
use tokio::runtime::Runtime;
use tuat_feed_parser::{parser, Info};

const INTERVAL_MIN: u64 = 15;

const INTERVAL: Duration = Duration::from_secs(INTERVAL_MIN * 60);

struct State {
    last_checked: Instant,
    last_infos: Vec<Info>,
}

async fn get_info() -> Result<Vec<Info>, String> {
    let rt = Runtime::new();
    if let Err(e) = rt {
        return Err(e.to_string());
    }
    let mut rt = rt.unwrap();
    rt.block_on(async { parser().await })
}

async fn handle(req: Request<Arc<RwLock<State>>>) -> Response {
    if Instant::now() > req.state().read().unwrap().last_checked + INTERVAL {
        println!("fetching {:?}", Instant::now());
        req.state().write().unwrap().last_checked = Instant::now();
        let data = get_info().await;
        match data {
            Ok(data) => req.state().write().unwrap().last_infos = data,
            Err(e) => return Response::new(400).body_string(e),
        }
    } else {
        //println!("cached");
    }

    Response::new(200)
        .body_json(&req.state().read().unwrap().last_infos)
        .unwrap()
}

#[async_std::main]
async fn main() -> Result<(), String> {
    let port: u32 = args().nth(1).unwrap_or("8080".to_string()).parse().unwrap();
    let mut app = tide::with_state(Arc::new(RwLock::new(State {
        last_checked: Instant::now(),
        last_infos: get_info().await?,
    })));
    app.at("/").get(handle);
    app.listen(&format!("127.0.0.1:{}", port))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
