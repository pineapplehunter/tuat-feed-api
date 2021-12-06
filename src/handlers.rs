/// routes for technology
pub mod technology {
    use crate::state::ServerState;
    use actix_web::{get, web, Responder};
    use std::sync::Arc;
    use tuat_feed_parser::Info;

    /// all data
    #[get("/", name = "technology_all")]
    pub async fn all(state: web::Data<Arc<ServerState>>) -> impl Responder {
        let info_academic = state.technology_academic.information.read().await.clone();

        let info_campus = state.technology_campus.information.read().await.clone();
        web::Json(
            info_academic
                .info
                .into_iter()
                .chain(info_campus.info)
                .collect::<Vec<Info>>(),
        )
    }

    /// academic
    #[get("/academic", name = "technology_academic")]
    pub async fn academic(state: web::Data<Arc<ServerState>>) -> impl Responder {
        let info = state.technology_academic.information.read().await.clone();
        web::Json(info.info)
    }

    /// campus
    #[get("/campus", name = "technology_campus")]
    pub async fn campus(state: web::Data<Arc<ServerState>>) -> impl Responder {
        let info = state.technology_campus.information.read().await.clone();
        web::Json(info.info)
    }
}

/// routes for agriculture
pub mod agriculture {
    use crate::state::ServerState;
    use actix_web::{get, web, Responder};
    use std::sync::Arc;
    use tuat_feed_parser::Info;

    /// all data
    #[get("/")]
    pub async fn all(state: web::Data<Arc<ServerState>>) -> impl Responder {
        let info_academic = state.agriculture_academic.information.read().await.clone();

        let info_campus = state.agriculture_campus.information.read().await.clone();
        web::Json(
            info_academic
                .info
                .into_iter()
                .chain(info_campus.info)
                .collect::<Vec<Info>>(),
        )
    }

    /// academic
    #[get("/academic")]
    pub async fn academic(state: web::Data<Arc<ServerState>>) -> impl Responder {
        let info = state.agriculture_academic.information.read().await.clone();
        web::Json(info.info)
    }

    /// campus
    #[get("/campus")]
    pub async fn campus(state: web::Data<Arc<ServerState>>) -> impl Responder {
        let info = state.agriculture_campus.information.read().await.clone();
        web::Json(info.info)
    }
}

#[cfg(test)]
mod test {
    use super::technology;
    use crate::info_bundle::InfoBundle;
    use crate::state::ServerState;
    use actix_web::http::StatusCode;
    use actix_web::{test, web, App};
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::Instant;
    use tuat_feed_parser::Info;

    fn dummy_info(id: u32) -> Info {
        let mut data = HashMap::new();
        data.insert("hello".into(), "world".into());
        data.insert("test".into(), "value".into());
        Info { id, data }
    }

    async fn dummy_state() -> Arc<ServerState> {
        let academic = InfoBundle::new(vec![dummy_info(0), dummy_info(1)], Instant::now());
        let campus = InfoBundle::new(vec![dummy_info(10), dummy_info(11)], Instant::now());
        let state = ServerState::init();

        *state.technology_academic.information.write().await = academic;
        *state.technology_campus.information.write().await = campus;

        Arc::new(state)
    }

    #[actix_rt::test]
    async fn check_json_formatting_index() {
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(dummy_state().await))
                .service(technology::all),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let response = test::call_service(&mut app, req).await;
        assert_eq!(response.status(), StatusCode::OK, "response {:?}", response);
        let output: Vec<Info> = test::read_body_json(response).await;

        let correct_outputs = [dummy_info(0), dummy_info(1), dummy_info(10), dummy_info(11)];

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
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(dummy_state().await))
                .service(technology::all)
                .service(technology::campus)
                .service(technology::academic),
        )
        .await;

        let req = test::TestRequest::get().uri("/campus").to_request();
        let response = test::call_service(&mut app, req).await;
        assert_eq!(response.status(), StatusCode::OK);
        let output: Vec<Info> = test::read_body_json(response).await;

        let correct_outputs = [dummy_info(10), dummy_info(11)];

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
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(dummy_state().await))
                .service(technology::all),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let response = test::call_service(&mut app, req).await;
        assert_eq!(response.status(), StatusCode::OK);
        let output: Vec<Info> = test::read_body_json(response).await;

        // not enough.
        let correct_outputs = [dummy_info(10), dummy_info(11)];

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
