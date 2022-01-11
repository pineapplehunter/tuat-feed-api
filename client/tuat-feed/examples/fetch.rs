pub use tuat_feed_common::Post;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.ihavenojob.work/tuat/")
        .await?
        .json::<Vec<Post>>()
        .await?;

    println!("{:?}", response);

    Ok(())
}
