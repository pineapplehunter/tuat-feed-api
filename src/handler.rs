use crate::State;
use std::sync::Arc;

/// handle /academic
pub async fn handle_academic(state: Arc<State>) -> Result<impl warp::Reply, warp::Rejection> {
    let data = state.get_academic().await;
    data.map(|data| warp::reply::json(&data))
        .map_err(|_e| warp::reject::reject())
}

/// handle /campus
pub async fn handle_campus(state: Arc<State>) -> Result<impl warp::Reply, warp::Rejection> {
    let data = state.get_campus().await;
    data.map(|data| warp::reply::json(&data))
        .map_err(|_e| warp::reject::reject())
}

/// handle /
pub async fn handle_index(state: Arc<State>) -> Result<impl warp::Reply, warp::Rejection> {
    let data = state.get_all().await;
    data.map(|data| warp::reply::json(&data))
        .map_err(|_e| warp::reject::reject())
}
