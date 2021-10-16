//! # tuat-feed-api(TUAT Feed API Server)
//!
//! This is code for a server that formatsthe TUAT feed to json

use rocket::{launch, routes};
use std::time::Duration;
use tuat_feed_api::InformationState;

/// Interval time (in minutes) for checking for new content.
#[cfg(feature = "cache")]
pub(crate) const INTERVAL_MIN: u64 = 15;
#[cfg(not(feature = "cache"))]
pub(crate) const INTERVAL_MIN: u64 = 0;

/// Interval duration computed from `INTERVAL_MIN`.
const INTERVAL: Duration = Duration::from_secs(INTERVAL_MIN * 60);

mod v1;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(InformationState::init(INTERVAL))
        .mount("/", routes![v1::all, v1::academic, v1::campus])
}
