use crate::State;
use std::sync::Arc;
use tuat_feed_parser::Info;

/// handle /academic
pub async fn handle_academic(state: Arc<State>) -> Result<Vec<Info>, warp::Rejection> {
    state
        .get_academic()
        .await
        .map_err(|_e| warp::reject::not_found())
}

/// handle /campus
pub async fn handle_campus(state: Arc<State>) -> Result<Vec<Info>, warp::Rejection> {
    state
        .get_campus()
        .await
        .map_err(|_e| warp::reject::not_found())
}

/// handle /
pub async fn handle_index(state: Arc<State>) -> Result<Vec<Info>, warp::Rejection> {
    state
        .get_all()
        .await
        .map_err(|_e| warp::reject::not_found())
}
