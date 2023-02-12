#![warn(missing_docs)]

//! # tuat-feed-api(TUAT Feed API Server library)
//!
//! This is code for a server that formatsthe TUAT feed to json.
//! This is the library part.

use axum::Router;
use handlers_v1::app_v1;
use handlers_v2::app_v2;
use state::SharedState;

/// handlers for endpoints v1
pub mod handlers_v1;
/// handlers for endpoints v2
pub mod handlers_v2;
/// a place to store data for a category
pub mod info_bundle;

/// manages state
pub mod state;

/// router for the whole app
pub fn app(base_path: String, initial_state: SharedState) -> Router {
    Router::new()
        .nest(
            &base_path,
            Router::new()
                .nest("/v1", app_v1(base_path.clone(), initial_state.clone()))
                .nest("/v2", app_v2(base_path.clone(), initial_state))
                .fallback(redirect_path!(v2 base_path)),
        )
        .fallback(redirect_path!(v2 base_path))
}

/// use this to generate redirect paths
///
/// ineeded this make it easy to make redirect paths
#[macro_export]
macro_rules! redirect_path {
    (v1 $base_path:ident, $sub_path:expr) => {{
        let base_path = $base_path.clone();
        let sub_path = $sub_path.to_string();
        || async {
            let base_path = base_path;
            let sub_path = sub_path;
            axum::response::Redirect::temporary(&format!("{}/v1{}", base_path, sub_path))
        }
    }};
    (v1 $base_path:ident) => {{
        let base_path = $base_path.clone();
        || async {
            let base_path = base_path;
            axum::response::Redirect::temporary(&format!("{}/v1/", base_path))
        }
    }};
    (v2 $base_path:ident, $sub_path:expr) => {{
        let base_path = $base_path.clone();
        let sub_path = $sub_path.to_string();
        || async {
            let base_path = base_path;
            let sub_path = sub_path;
            axum::response::Redirect::temporary(&format!("{}/v2{}", base_path, sub_path))
        }
    }};
    (v2 $base_path:ident) => {{
        let base_path = $base_path.clone();
        || async {
            let base_path = base_path;
            axum::response::Redirect::temporary(&format!("{}/v2/", base_path))
        }
    }};
}
