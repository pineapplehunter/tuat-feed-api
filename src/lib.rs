#![warn(missing_docs)]

//! # tuat-feed-api(TUAT Feed API Server library)
//!
//! This is code for a server that formatsthe TUAT feed to json.
//! This is the library part.

use rocket::http::uri::Origin;
use serde::Deserialize;

/// handlers for endpoints
pub mod handlers;
/// a place to store data for a category
pub mod info_bundle;

/// manages state
pub mod state;

/// base path of server
#[derive(Deserialize, Debug)]
pub struct BasePath {
    #[serde(default = "default_base")]
    base_path: Origin<'static>,
}

fn default_base() -> Origin<'static> {
    Origin::parse("/").unwrap()
}
