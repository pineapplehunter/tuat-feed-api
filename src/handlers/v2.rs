use crate::State;
use color_eyre::eyre::Result;
use serde::Serialize;
use std::sync::Arc;
use tuat_feed_parser::Info;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Ok,
    Err(String),
}

/// Type for repersenting the data that would be returned on request
#[derive(Debug, PartialEq, Serialize)]
pub struct ResponseType {
    status: Status,
    data: Option<Vec<Info>>,
}

impl ResponseType {
    fn from_data_result(result: Result<Vec<Info>>) -> Self {
        match result {
            Ok(data) => Self {
                status: Status::Ok,
                data: Some(data),
            },
            Err(err) => Self {
                status: Status::Err(err.to_string()),
                data: None,
            },
        }
    }
}

/// routes to v2 api
pub fn v2_paths(state: BoxedFilter<(Arc<State>,)>) -> BoxedFilter<(impl Reply,)> {
    // paths
    let index = warp::any()
        .and(state.clone())
        .and_then(handle_index)
        .map(|data| warp::reply::json(&data))
        .boxed();
    let academic = warp::path("academic")
        .and(state.clone())
        .and_then(handle_academic)
        .map(|data| warp::reply::json(&data))
        .boxed();
    let campus = warp::path("campus")
        .and(state)
        .and_then(handle_campus)
        .map(|data| warp::reply::json(&data))
        .boxed();

    warp::path("v2").and(academic.or(campus).or(index)).boxed()
}

/// handle /academic
pub async fn handle_academic(state: Arc<State>) -> Result<ResponseType, Rejection> {
    Ok(ResponseType::from_data_result(state.get_academic().await))
        .map_err(|_: ()| warp::reject::not_found())
}

/// handle /campus
pub async fn handle_campus(state: Arc<State>) -> Result<ResponseType, Rejection> {
    Ok(ResponseType::from_data_result(state.get_campus().await))
        .map_err(|_: ()| warp::reject::not_found())
}

/// handle /
pub async fn handle_index(state: Arc<State>) -> Result<ResponseType, Rejection> {
    Ok(ResponseType::from_data_result(state.get_all().await))
        .map_err(|_: ()| warp::reject::not_found())
}
