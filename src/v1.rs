use rocket::serde::json::Json;
use rocket::{get, State};
use tuat_feed_api::InformationState;
use tuat_feed_parser::Info;

#[get("/")]
pub async fn all(state: &State<InformationState>) -> Json<Vec<Info>> {
    Json(state.all().await.unwrap())
}

#[get("/academic")]
pub async fn academic(state: &State<InformationState>) -> Json<Vec<Info>> {
    Json(state.academic().await.unwrap())
}

#[get("/campus")]
pub async fn campus(state: &State<InformationState>) -> Json<Vec<Info>> {
    Json(state.campus().await.unwrap())
}

#[cfg(test)]
mod test {
    use super::{academic, all, campus};
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::{routes, Build, Rocket};
    use std::collections::HashMap;
    use std::time::{Duration, Instant};
    use tuat_feed_api::info_section::InfoBundle;
    use tuat_feed_api::InformationState;
    use tuat_feed_parser::Info;

    #[test]
    fn simple() {
        assert_eq!(1 + 1, 2);
    }

    fn dummy_info(id: u32) -> Info {
        let mut data = HashMap::new();
        data.insert("hello".into(), "world".into());
        data.insert("test".into(), "value".into());
        Info { id, data }
    }

    fn dummy_state() -> InformationState {
        let academic = InfoBundle::new(vec![dummy_info(0), dummy_info(1)], Instant::now());
        let campus = InfoBundle::new(vec![dummy_info(10), dummy_info(11)], Instant::now());
        let interval = Duration::from_secs(10000000);
        InformationState::__set_all(academic, campus, interval)
    }

    fn rocket() -> Rocket<Build> {
        rocket::build()
            .manage(dummy_state())
            .mount("/", routes![all, academic, campus])
    }

    #[test]
    fn check_json_formatting_index() {
        let client = Client::tracked(rocket()).expect("could not create client");
        let response = client.get("/").dispatch();
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
        let response = client.get("/campus").dispatch();
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
        let response = client.get("/").dispatch();
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
