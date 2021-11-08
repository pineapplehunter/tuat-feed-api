use rocket::response::Redirect;
use rocket::{get, uri, State};

use crate::BasePath;

/// routes for technology
pub mod technology {
    use std::sync::Arc;

    use rocket::serde::json::Json;
    use rocket::{get, State};
    use tuat_feed_parser::Info;

    use crate::state::ServerState;

    /// all data
    #[get("/T")]
    pub async fn all(state: &State<Arc<ServerState>>) -> Json<Vec<Info>> {
        let info_academic = state.technology_academic.information.read().await.clone();

        let info_campus = state.technology_campus.information.read().await.clone();
        Json(
            info_academic
                .info
                .into_iter()
                .chain(info_campus.info)
                .collect(),
        )
    }

    /// academic
    #[get("/T/academic")]
    pub async fn academic(state: &State<Arc<ServerState>>) -> Json<Vec<Info>> {
        let info = state.technology_academic.information.read().await.clone();
        Json(info.info)
    }

    /// campus
    #[get("/T/campus")]
    pub async fn campus(state: &State<Arc<ServerState>>) -> Json<Vec<Info>> {
        let info = state.technology_campus.information.read().await.clone();
        Json(info.info)
    }
}

/// routes for agriculture
pub mod agriculture {
    use std::sync::Arc;

    use rocket::serde::json::Json;
    use rocket::{get, State};
    use tuat_feed_parser::Info;

    use crate::state::ServerState;

    /// all data
    #[get("/A")]
    pub async fn all(state: &State<Arc<ServerState>>) -> Json<Vec<Info>> {
        let info_academic = state.technology_academic.information.read().await.clone();

        let info_campus = state.technology_campus.information.read().await.clone();
        Json(
            info_academic
                .info
                .into_iter()
                .chain(info_campus.info)
                .collect(),
        )
    }

    /// academic
    #[get("/A/academic")]
    pub async fn academic(state: &State<Arc<ServerState>>) -> Json<Vec<Info>> {
        let info = state.technology_academic.information.read().await.clone();
        Json(info.info)
    }

    /// campus
    #[get("/A/campus")]
    pub async fn campus(state: &State<Arc<ServerState>>) -> Json<Vec<Info>> {
        let info = state.technology_campus.information.read().await.clone();
        Json(info.info)
    }
}

/// all data
#[get("/")]
pub async fn all(base_path: &State<BasePath>) -> Redirect {
    Redirect::to(uri!(base_path.base_path.clone(), technology::all()))
}

/// academic
#[get("/academic")]
pub async fn academic(base_path: &State<BasePath>) -> Redirect {
    Redirect::to(uri!(base_path.base_path.clone(), technology::academic()))
}

/// campus
#[get("/campus")]
pub async fn campus(base_path: &State<BasePath>) -> Redirect {
    Redirect::to(uri!(base_path.base_path.clone(), technology::campus()))
}

#[cfg(test)]
mod test {
    use super::{academic, all, campus};
    use crate::handlers::{agriculture, technology};
    use crate::info_bundle::InfoBundle;
    use crate::state::ServerState;
    use crate::BasePath;
    use rocket::fairing::AdHoc;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::{routes, tokio, Build, Rocket};
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

    fn rocket() -> Rocket<Build> {
        let state = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(dummy_state());
        rocket::build()
            .manage(state)
            .attach(AdHoc::config::<BasePath>())
            .mount("/", routes![all, academic, campus])
            .mount(
                "/",
                routes![technology::all, technology::academic, technology::campus],
            )
            .mount(
                "/",
                routes![agriculture::all, agriculture::academic, agriculture::campus],
            )
    }

    #[test]
    fn check_json_formatting_index() {
        let client = Client::tracked(rocket()).expect("could not create client");
        let response = client.get("/T").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let output: Vec<Info> = response.into_json().unwrap();

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

    #[test]
    fn check_json_formatting_campus() {
        let client = Client::tracked(rocket()).expect("could not create client");
        let response = client.get("/T/campus").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let output: Vec<Info> = response.into_json().unwrap();

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
    #[test]
    fn check_json_formatting_index_panic() {
        let client = Client::tracked(rocket()).expect("could not create client");
        let response = client.get("/T/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let output: Vec<Info> = response.into_json().unwrap();

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
