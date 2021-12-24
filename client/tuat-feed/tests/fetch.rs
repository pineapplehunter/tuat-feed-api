use std::env;

pub use tuat_feed_common::Info;

#[tokio::test]
async fn main() {
    let network_test = env::var("NETWORK_TEST").is_ok();
    if !network_test {
        return;
    }

    let response = reqwest::get("https://api.ihavenojob.work/tuat/")
        .await
        .unwrap()
        .json::<Vec<Info>>()
        .await
        .unwrap();

    println!("{:?}", response);
}
