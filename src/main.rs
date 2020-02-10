use tide::{Request, Response};
use tokio::runtime::Runtime;
use tuat_feed_parser::parser;

async fn handle(_: Request<()>) -> Response {
    let rt = Runtime::new();
    if let Err(e) = rt {
        return Response::new(400).body_string(e.to_string());
    }
    let mut rt = rt.unwrap();
    let c = rt.block_on(async { parser().await });
    let data = c;
    match data {
        Ok(data) => Response::new(200).body_json(&data).unwrap(),
        Err(e) => Response::new(400).body_string(e),
    }
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/").get(handle);
    app.listen("127.0.0.1:9001").await?;
    Ok(())
}
