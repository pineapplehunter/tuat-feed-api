pub use tuat_feed_common::Info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.ihavenojob.work/tuat/")
        .await?
        .json::<Vec<Info>>()
        .await?;

    println!("{:?}", response);

    Ok(())
}
