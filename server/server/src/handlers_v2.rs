use crate::{redirect_path, state::SharedState};
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde_derive::Deserialize;
use tuat_feed_scraper::post::Post;

#[derive(Debug, Deserialize)]
enum Gakubu {
    Technology,
    Agriculture,
}

impl Default for Gakubu {
    fn default() -> Self {
        Self::Technology
    }
}

#[derive(Debug, Deserialize)]
enum Category {
    All,
    Academic,
    Campus,
}

impl Default for Category {
    fn default() -> Self {
        Self::All
    }
}

/// http querys
#[derive(Debug, Deserialize)]
pub struct QueryType {
    #[serde(default = "Gakubu::default")]
    gakubu: Gakubu,
    #[serde(default = "Category::default")]
    category: Category,
}

/// all data
pub async fn index(
    State(state): State<SharedState>,
    Query(query): Query<QueryType>,
) -> Json<Vec<Post>> {
    match query.gakubu {
        Gakubu::Technology => {
            let info_academic = state.technology_academic.information.read().await.clone();
            let info_campus = state.technology_campus.information.read().await.clone();
            match query.category {
                Category::All => Json(
                    info_academic
                        .post
                        .into_iter()
                        .chain(info_campus.post)
                        .collect::<Vec<Post>>(),
                ),
                Category::Campus => Json(info_campus.post),
                Category::Academic => Json(info_academic.post),
            }
        }
        Gakubu::Agriculture => {
            let info_academic = state.agriculture_academic.information.read().await.clone();
            let info_campus = state.agriculture_campus.information.read().await.clone();
            match query.category {
                Category::All => Json(
                    info_academic
                        .post
                        .into_iter()
                        .chain(info_campus.post)
                        .collect::<Vec<Post>>(),
                ),
                Category::Campus => Json(info_campus.post),
                Category::Academic => Json(info_academic.post),
            }
        }
    }
}

/// routes for app v2
pub fn app_v2(base_path: String, initial_state: SharedState) -> Router {
    Router::new()
        .route("/", get(index))
        .fallback(redirect_path!(v2 base_path))
        .with_state(initial_state)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::info_bundle::InfoBundle;
    use crate::state::ServerState;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use hyper::{Request, StatusCode};
    use std::sync::Arc;
    use std::time::Instant;
    use tower::ServiceExt;

    async fn dummy_state() -> Arc<ServerState> {
        let academic = InfoBundle::new(vec![Post::new(0), Post::new(1)], Instant::now());
        let campus = InfoBundle::new(vec![Post::new(10), Post::new(11)], Instant::now());
        let state = ServerState::init();

        *state.technology_academic.information.write().await = academic;
        *state.technology_campus.information.write().await = campus;

        Arc::new(state)
    }

    #[tokio::test]
    async fn check_json_formatting_index() {
        let app = app_v2("/".to_string(), dummy_state().await);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK, "response {:?}", response);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let output: Vec<Post> = serde_json::from_slice(&body).unwrap();

        let correct_outputs = [Post::new(0), Post::new(1), Post::new(10), Post::new(11)];

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
        let app = app_v2("/".to_string(), dummy_state().await);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/?category=Campus&gakubu=Technology")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let output: Vec<Post> = serde_json::from_slice(&body).unwrap();

        let correct_outputs = [Post::new(10), Post::new(11)];

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
        let app = app_v2("/".to_string(), dummy_state().await);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let output: Vec<Post> = serde_json::from_slice(&body).unwrap();

        // not enough.
        let correct_outputs = [Post::new(10), Post::new(11)];

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
