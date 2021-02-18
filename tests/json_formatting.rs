use std::collections::HashMap;
use std::{sync::Arc, time::Duration};

use tuat_feed_api::State;
use tuat_feed_api::{
    handler::{handle_academic, handle_campus, handle_index},
    info_section::InfoSection,
};
use tuat_feed_parser::Info;
use warp::Filter;

fn dummy_info(id: u32) -> Info {
    let mut data = HashMap::new();
    data.insert("hello".into(), "world".into());
    data.insert("test".into(), "value".into());
    Info { id, data }
}

fn dummy_state() -> Arc<State> {
    let academic = InfoSection::new(vec![dummy_info(0), dummy_info(1)]);
    let campus = InfoSection::new(vec![dummy_info(10), dummy_info(11)]);
    let interval = Duration::from_secs(10000000);
    Arc::new(State::set_all(academic, campus, interval))
}

#[tokio::test]
async fn check_json_formatting_index() {
    let state = dummy_state();
    let state = warp::any().map(move || state.clone());
    let filter = warp::get().and(state.clone()).and_then(handle_index);

    let output: Vec<Info> = warp::test::request()
        .path("/")
        .filter(&filter)
        .await
        .unwrap();

    let correct_outputs = [dummy_info(0), dummy_info(1), dummy_info(10), dummy_info(11)];

    for out in output {
        let mut flg = true;
        for correct in correct_outputs.iter() {
            if &out == correct {
                flg = false;
                break;
            }
        }
        if flg {
            panic!("output {:?} did not match any correct outputs", out);
        }
    }
}

#[tokio::test]
async fn check_json_formatting_academic() {
    let state = dummy_state();
    let state = warp::any().map(move || state.clone());
    let filter = warp::get().and(state.clone()).and_then(handle_academic);

    let output: Vec<Info> = warp::test::request()
        .path("/")
        .filter(&filter)
        .await
        .unwrap();

    let correct_outputs = [dummy_info(0), dummy_info(1)];

    for out in output {
        let mut flg = true;
        for correct in correct_outputs.iter() {
            if &out == correct {
                flg = false;
                break;
            }
        }
        if flg {
            panic!("output {:?} did not match any correct outputs", out);
        }
    }
}

#[tokio::test]
async fn check_json_formatting_campus() {
    let state = dummy_state();
    let state = warp::any().map(move || state.clone());
    let filter = warp::get().and(state.clone()).and_then(handle_campus);

    let output: Vec<Info> = warp::test::request()
        .path("/")
        .filter(&filter)
        .await
        .unwrap();

    let correct_outputs = [dummy_info(10), dummy_info(11)];

    for out in output {
        let mut flg = true;
        for correct in correct_outputs.iter() {
            if &out == correct {
                flg = false;
                break;
            }
        }
        if flg {
            panic!("output {:?} did not match any correct outputs", out);
        }
    }
}

#[should_panic]
#[tokio::test]
async fn check_json_formatting_index_panic() {
    let state = dummy_state();
    let state = warp::any().map(move || state.clone());
    let filter = warp::get().and(state.clone()).and_then(handle_index);

    let output: Vec<Info> = warp::test::request()
        .path("/")
        .filter(&filter)
        .await
        .unwrap();

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
        if flg {
            panic!("output {:?} did not match any correct outputs", out);
        }
    }
}
