use crate::State;
use std::sync::Arc;
use tuat_feed_parser::Info;

/// handle /academic
pub async fn handle_academic(state: Arc<State>) -> Result<Vec<Info>, warp::Rejection> {
    let data = state.get_academic().await;
    data.map_err(|_e| warp::reject::reject())
}

/// handle /campus
pub async fn handle_campus(state: Arc<State>) -> Result<Vec<Info>, warp::Rejection> {
    let data = state.get_campus().await;
    data.map_err(|_e| warp::reject::reject())
}

/// handle /
pub async fn handle_index(state: Arc<State>) -> Result<Vec<Info>, warp::Rejection> {
    let data = state.get_all().await;
    data.map_err(|_e| warp::reject::reject())
}
