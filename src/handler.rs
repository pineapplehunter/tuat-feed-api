use crate::State;
use std::sync::Arc;
use tide::Body;

/// handle /academic
pub async fn handle_academic(req: tide::Request<Arc<State>>) -> tide::Result<Body> {
    let state = req.state();
    let data = state.get_academic().await;
    Ok(Body::from_json(&data?)?)
}

/// handle /campus
pub async fn handle_campus(req: tide::Request<Arc<State>>) -> tide::Result<Body> {
    let state = req.state();
    let data = state.get_campus().await;
    Ok(Body::from_json(&data?)?)
}

/// handle /
pub async fn handle_index(req: tide::Request<Arc<State>>) -> tide::Result<Body> {
    let state = req.state();
    let data = state.get_all().await;
    Ok(Body::from_json(&data?)?)
}
