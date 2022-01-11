use crate::state::ServerState;
use actix_web::{get, web};
use serde::Deserialize;
use std::sync::Arc;
use tuat_feed_common::Post;

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

#[derive(Debug, Deserialize)]
struct QueryType {
    #[serde(default = "Gakubu::default")]
    gakubu: Gakubu,
    #[serde(default = "Category::default")]
    category: Category,
}

/// all data
#[get("/", name = "index_v2")]
async fn index(
    state: web::Data<Arc<ServerState>>,
    query: web::Query<QueryType>,
) -> web::Json<Vec<Post>> {
    match query.gakubu {
        Gakubu::Technology => {
            let info_academic = state.technology_academic.information.read().await.clone();
            let info_campus = state.technology_campus.information.read().await.clone();
            match query.category {
                Category::All => web::Json(
                    info_academic
                        .post
                        .into_iter()
                        .chain(info_campus.post)
                        .collect::<Vec<Post>>(),
                ),
                Category::Campus => web::Json(info_campus.post),
                Category::Academic => web::Json(info_academic.post),
            }
        }
        Gakubu::Agriculture => {
            let info_academic = state.agriculture_academic.information.read().await.clone();
            let info_campus = state.agriculture_campus.information.read().await.clone();
            match query.category {
                Category::All => web::Json(
                    info_academic
                        .post
                        .into_iter()
                        .chain(info_campus.post)
                        .collect::<Vec<Post>>(),
                ),
                Category::Campus => web::Json(info_campus.post),
                Category::Academic => web::Json(info_academic.post),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::handlers_v2::index;
    use crate::info_bundle::InfoBundle;
    use crate::state::ServerState;
    use actix_web::http::StatusCode;
    use actix_web::{test, web, App};
    use std::sync::Arc;
    use std::time::Instant;
    use tuat_feed_common::Post;

    async fn dummy_state() -> Arc<ServerState> {
        let academic = InfoBundle::new(vec![Post::new(0), Post::new(1)], Instant::now());
        let campus = InfoBundle::new(vec![Post::new(10), Post::new(11)], Instant::now());
        let state = ServerState::init();

        *state.technology_academic.information.write().await = academic;
        *state.technology_campus.information.write().await = campus;

        Arc::new(state)
    }

    #[actix_rt::test]
    async fn check_json_formatting_index() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(dummy_state().await))
                .service(index),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::OK, "response {:?}", response);
        let output: Vec<Post> = test::read_body_json(response).await;

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

    #[actix_rt::test]
    async fn check_json_formatting_campus() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(dummy_state().await))
                .service(index),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/?category=Campus&gakubu=Technology")
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::OK);
        let output: Vec<Post> = test::read_body_json(response).await;

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
    #[actix_rt::test]
    async fn check_json_formatting_index_panic() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(dummy_state().await))
                .service(index),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::OK);
        let output: Vec<Post> = test::read_body_json(response).await;

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
