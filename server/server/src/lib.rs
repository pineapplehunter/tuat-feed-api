#![warn(missing_docs)]

//! # tuat-feed-api(TUAT Feed API Server library)
//!
//! This is code for a server that formatsthe TUAT feed to json.
//! This is the library part.

/// handlers for endpoints v1
pub mod handlers_v1;
/// handlers for endpoints v2
pub mod handlers_v2;
/// a place to store data for a category
pub mod info_bundle;

/// manages state
pub mod state;
