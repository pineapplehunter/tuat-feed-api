//! # tuat-feed-api(TUAT Feed API Server)
//!
//! This is code for a server that formatsthe TUAT feed to json

use rocket::{
    fairing::AdHoc,
    launch, routes,
    tokio::{self, time::sleep},
};
use std::{sync::Arc, time::Duration};
use tuat_feed_api::{
    handlers::{academic, agriculture, all, campus, technology},
    state::ServerState,
    BasePath,
};

/// Interval time (in minutes) for checking for new content.
const INTERVAL_MIN: u64 = 15;

/// Interval duration computed from `INTERVAL_MIN`.
const INTERVAL: Duration = Duration::from_secs(INTERVAL_MIN * 60);

#[launch]
fn rocket() -> _ {
    let state = Arc::new(ServerState::init());
    let state_cloned = state.clone();
    tokio::spawn(async move {
        loop {
            state_cloned.update().await;
            sleep(INTERVAL).await;
        }
    });
    rocket::build()
        .manage(state)
        .attach(AdHoc::config::<BasePath>())
        .mount("/", routes![all, academic, campus])
        .mount(
            "/",
            routes![technology::all, technology::academic, technology::campus],
        )
        .mount(
            "/",
            routes![agriculture::all, agriculture::academic, agriculture::campus],
        )
}
