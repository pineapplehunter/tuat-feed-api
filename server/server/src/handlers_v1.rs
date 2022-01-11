/// routes for technology
pub mod technology {
    use crate::state::ServerState;
    use actix_web::{get, web};
    use std::sync::Arc;
    use tuat_feed_common::PostCompatv1;

    /// all data
    #[get("/", name = "technology_all")]
    pub async fn all(state: web::Data<Arc<ServerState>>) -> web::Json<Vec<PostCompatv1>> {
        let info_academic = state.technology_academic.information.read().await.clone();

        let info_campus = state.technology_campus.information.read().await.clone();
        web::Json(
            info_academic
                .post
                .into_iter()
                .chain(info_campus.post)
                .map(Into::into)
                .collect(),
        )
    }

    /// academic
    #[get("/academic", name = "technology_academic")]
    pub async fn academic(state: web::Data<Arc<ServerState>>) -> web::Json<Vec<PostCompatv1>> {
        let info = state.technology_academic.information.read().await.clone();
        web::Json(info.post.into_iter().map(Into::into).collect())
    }

    /// campus
    #[get("/campus", name = "technology_campus")]
    pub async fn campus(state: web::Data<Arc<ServerState>>) -> web::Json<Vec<PostCompatv1>> {
        let info = state.technology_campus.information.read().await.clone();
        web::Json(info.post.into_iter().map(Into::into).collect())
    }
}

/// routes for agriculture
pub mod agriculture {
    use crate::state::ServerState;
    use actix_web::{get, web};
    use std::sync::Arc;
    use tuat_feed_common::PostCompatv1;

    /// all data
    #[get("/", name = "agriculture_all")]
    pub async fn all(state: web::Data<Arc<ServerState>>) -> web::Json<Vec<PostCompatv1>> {
        let info_academic = state.agriculture_academic.information.read().await.clone();

        let info_campus = state.agriculture_campus.information.read().await.clone();
        web::Json(
            info_academic
                .post
                .into_iter()
                .chain(info_campus.post)
                .map(Into::into)
                .collect(),
        )
    }

    /// academic
    #[get("/academic")]
    pub async fn academic(state: web::Data<Arc<ServerState>>) -> web::Json<Vec<PostCompatv1>> {
        let info = state.agriculture_academic.information.read().await.clone();
        web::Json(info.post.into_iter().map(Into::into).collect())
    }

    /// campus
    #[get("/campus")]
    pub async fn campus(state: web::Data<Arc<ServerState>>) -> web::Json<Vec<PostCompatv1>> {
        let info = state.agriculture_campus.information.read().await.clone();
        web::Json(info.post.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod test {
    use super::technology;
    use crate::info_bundle::InfoBundle;
    use crate::state::ServerState;
    use actix_web::http::StatusCode;
    use actix_web::{test, web, App};
    use std::sync::Arc;
    use std::time::Instant;
    use tuat_feed_common::{Post, PostCompatv1};

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
                .service(technology::all),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::OK, "response {:?}", response);
        let output: Vec<PostCompatv1> = test::read_body_json(response).await;

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

    #[actix_rt::test]
    async fn check_json_formatting_campus() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(dummy_state().await))
                .service(technology::all)
                .service(technology::campus)
                .service(technology::academic),
        )
        .await;

        let req = test::TestRequest::get().uri("/campus").to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::OK);
        let output: Vec<PostCompatv1> = test::read_body_json(response).await;

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
    #[actix_rt::test]
    async fn check_json_formatting_index_panic() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(dummy_state().await))
                .service(technology::all),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::OK);
        let output: Vec<PostCompatv1> = test::read_body_json(response).await;

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
