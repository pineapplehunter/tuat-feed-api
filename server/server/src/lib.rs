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
    let inner_router = Router::new()
        .merge(Router::new().nest("/v1/", app_v1(base_path.clone(), initial_state.clone())))
        .merge(Router::new().nest("/v2/", app_v2(base_path.clone(), initial_state)))
        .fallback(redirect_path!(v2 base_path));

    if base_path.is_empty() || base_path == "/" {
        inner_router
    } else {
        Router::new()
            .nest(
                // &base_path,
                &base_path,
                inner_router,
            )
            .fallback(redirect_path!(v2 base_path))
    }
}

/// use this to generate redirect paths
///
/// ineeded this make it easy to make redirect paths
#[macro_export]
macro_rules! redirect_path {
    (v1 $base_path:ident, $sub_path:expr) => {{
        let base_path_ = $base_path.clone();
        let sub_path_ = $sub_path.to_string();
        || async {
            let base_path = base_path_;
            let sub_path = sub_path_;
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
        let base_path_ = $base_path.clone();
        let sub_path_ = $sub_path.to_string();
        || async {
            let base_path = base_path_;
            let sub_path = sub_path_;
            axum::response::Redirect::temporary(&format!("{}/v2{}", base_path, sub_path))
        }
    }};
    (v2 $base_path:ident) => {{
        let base_path_ = $base_path.clone();
        || async {
            let base_path = base_path_;
            axum::response::Redirect::temporary(&format!("{}/v2/", base_path))
        }
    }};
}

#[cfg(test)]
mod test {
    use std::{sync::Arc, time::Instant};

    use tuat_feed_scraper::post::Post;

    use crate::{app, info_bundle::InfoBundle, state::ServerState};

    async fn dummy_state() -> Arc<ServerState> {
        let academic = InfoBundle::new(vec![Post::new(0), Post::new(1)], Instant::now());
        let campus = InfoBundle::new(vec![Post::new(10), Post::new(11)], Instant::now());
        let state = ServerState::init();

        *state.technology_academic.information.write().await = academic;
        *state.technology_campus.information.write().await = campus;

        Arc::new(state)
    }

    #[tokio::test]
    async fn can_create_app() {
        let _app = app("".to_string(), dummy_state().await);
        let _app = app("/".to_string(), dummy_state().await);
        let _app = app("/base_path".to_string(), dummy_state().await);
    }
}
