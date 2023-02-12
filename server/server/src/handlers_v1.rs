use axum::{routing::get, Router};

use crate::{redirect_path, state::SharedState};

/// routes for technology
pub mod technology {
    use crate::state::SharedState;
    use axum::extract::State;
    use axum::response::Json;
    use tuat_feed_common::PostCompatv1;

    /// all data
    pub async fn all(State(state): State<SharedState>) -> Json<Vec<PostCompatv1>> {
        let info_academic = state.technology_academic.information.read().await.clone();

        let info_campus = state.technology_campus.information.read().await.clone();
        Json(
            info_academic
                .post
                .into_iter()
                .chain(info_campus.post)
                .map(Into::into)
                .collect(),
        )
    }

    /// academic
    pub async fn academic(State(state): State<SharedState>) -> Json<Vec<PostCompatv1>> {
        let info = state.technology_academic.information.read().await.clone();
        Json(info.post.into_iter().map(Into::into).collect())
    }

    /// campus
    pub async fn campus(State(state): State<SharedState>) -> Json<Vec<PostCompatv1>> {
        let info = state.technology_campus.information.read().await.clone();
        Json(info.post.into_iter().map(Into::into).collect())
    }
}

/// routes for agriculture
pub mod agriculture {
    use crate::state::SharedState;
    use axum::extract::State;
    use axum::response::Json;
    use tuat_feed_common::PostCompatv1;

    /// all data
    pub async fn all(State(state): State<SharedState>) -> Json<Vec<PostCompatv1>> {
        let info_academic = state.agriculture_academic.information.read().await.clone();

        let info_campus = state.agriculture_campus.information.read().await.clone();
        Json(
            info_academic
                .post
                .into_iter()
                .chain(info_campus.post)
                .map(Into::into)
                .collect(),
        )
    }

    /// academic
    pub async fn academic(State(state): State<SharedState>) -> Json<Vec<PostCompatv1>> {
        let info = state.agriculture_academic.information.read().await.clone();
        Json(info.post.into_iter().map(Into::into).collect())
    }

    /// campus
    pub async fn campus(State(state): State<SharedState>) -> Json<Vec<PostCompatv1>> {
        let info = state.agriculture_campus.information.read().await.clone();
        Json(info.post.into_iter().map(Into::into).collect())
    }
}

/// routes for app v1
pub fn app_v1(base_path: String, initial_state: SharedState) -> Router {
    Router::new()
        .route("/T", get(technology::all))
        .route("/T/academic", get(technology::academic))
        .route("/T/campus", get(technology::campus))
        .route("/A", get(agriculture::all))
        .route("/A/academic", get(agriculture::academic))
        .route("/A/campus", get(agriculture::campus))
        .route(
            "/academic",
            get(redirect_path!(v1 base_path, "/T/academic")),
        )
        .route("/campus", get(redirect_path!(v1 base_path, "/T/campus")))
        .fallback(redirect_path!(v1 base_path, "/T"))
        .with_state(initial_state)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::info_bundle::InfoBundle;
    use crate::state::ServerState;
    use axum::http::Request;
    use axum::{body::Body, http::StatusCode};
    use std::sync::Arc;
    use std::time::Instant;
    use tower::ServiceExt;
    use tuat_feed_common::{Post, PostCompatv1};

    async fn dummy_state() -> SharedState {
        let academic = InfoBundle::new(vec![Post::new(0), Post::new(1)], Instant::now());
        let campus = InfoBundle::new(vec![Post::new(10), Post::new(11)], Instant::now());
        let state = ServerState::init();

        *state.technology_academic.information.write().await = academic;
        *state.technology_campus.information.write().await = campus;

        Arc::new(state)
    }

    #[tokio::test]
    async fn check_json_formatting_index() {
        let app = app_v1("/".to_string(), dummy_state().await);

        let response = app
            .oneshot(Request::builder().uri("/T").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK, "response {:?}", response);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let output: Vec<PostCompatv1> = serde_json::from_slice(&body).unwrap();

        let correct_outputs = [Post::new(0), Post::new(1), Post::new(10), Post::new(11)]
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();

        for out in output {
            let mut flg = true;
            for correct in correct_outputs.iter() {
                if &out == correct {
                    flg = false;
                    break;
                }
            }
            assert!(!flg, "output {:?} did not match any correct outputs", out);
        }
    }

    #[tokio::test]
    async fn check_json_formatting_campus() {
        let app = app_v1("/".to_string(), dummy_state().await);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/T/campus")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let output: Vec<PostCompatv1> = serde_json::from_slice(&body).unwrap();

        let correct_outputs = [Post::new(10), Post::new(11)]
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();

        for out in output {
            let mut flg = true;
            for correct in correct_outputs.iter() {
                if &out == correct {
                    flg = false;
                    break;
                }
            }
            assert!(!flg, "output {:?} did not match any correct outputs", out);
        }
    }

    #[should_panic]
    #[tokio::test]
    async fn check_json_formatting_index_panic() {
        let app = app_v1("/".to_string(), dummy_state().await);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/campus")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let output: Vec<PostCompatv1> = serde_json::from_slice(&body).unwrap();

        // not enough.
        let correct_outputs = [Post::new(10), Post::new(11)]
            .into_iter()
            .map(Into::into)
            .collect::<Vec<PostCompatv1>>();

        for out in output {
            let mut flg = true;
            for correct in correct_outputs.iter() {
                if &out == correct {
                    flg = false;
                    break;
                }
            }
            assert!(!flg, "output {:?} did not match any correct outputs", out);
        }
    }
}
