// #![warn(missing_docs)]

//! # tuat-feed-api(TUAT Feed API Server library)
//!
//! This is code for a server that formatsthe TUAT feed to json.
//! This is the library part.

mod state;
pub use state::State;
pub mod handler;
pub mod info_section;
