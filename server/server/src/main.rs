//! # tuat-feed-api(TUAT Feed API Server)
//!
//! This is code for a server that formatsthe TUAT feed to json

use actix_web::{
    http::header, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Resource, Route,
};
use log::info;
use std::{env, net::SocketAddr, str::FromStr, sync::Arc, time::Duration};
use tokio::time::sleep;
use tuat_feed_server::{
    handlers_v1::{agriculture, technology},
    handlers_v2,
    state::ServerState,
};

/// Interval time (in minutes) for checking for new content.
const INTERVAL_MINUTES: u64 = 15;

/// Interval duration computed from `INTERVAL_MIN`.
const INTERVAL: Duration = Duration::from_secs(INTERVAL_MINUTES * 60);

fn redirect_path_to_name(path: &'static str, name: &'static str) -> Resource {
    web::resource(path).route(web::get().to(move |req: HttpRequest| async move {
        let url = req.url_for_static(name).unwrap();
        HttpResponse::Found()
            .append_header((header::LOCATION, url.as_str()))
            .body(format!("redirect to {:?}", url.path()))
    }))
}

fn redirect_to_name(name: &'static str) -> Route {
    web::route().to(move |req: HttpRequest| async move {
        let url = req.url_for_static(name).unwrap();
        HttpResponse::Found()
            .append_header((header::LOCATION, url.as_str()))
            .body(format!("redirect to {:?}", url.path()))
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = Arc::new(ServerState::init());
    let state_cloned = state.clone();

    env::set_var(
        "RUST_LOG",
        "actix_web=debug,actix_server=info,tuat_feed_scraper=info,tuat_feed_server=info",
    );
    env_logger::init();

    let base_path = env::var("TUAT_FEED_API_BASEPATH").unwrap_or_else(|_| String::new());
    let addr = env::var("TUAT_FEED_API_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_owned());

    tokio::spawn(async move {
        loop {
            state_cloned.update().await;
            sleep(INTERVAL).await;
        }
    });
    let address = SocketAddr::from_str(&addr).unwrap();
    info!("starting server on http://{}/{}", address, base_path);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope(&base_path)
                    .service(
                        web::scope("/T")
                            .service(technology::all)
                            .service(technology::academic)
                            .service(technology::campus)
                            .default_service(redirect_to_name("technology_all")),
                    )
                    .service(
                        web::scope("/A")
                            .service(agriculture::all)
                            .service(agriculture::academic)
                            .service(agriculture::campus)
                            .default_service(redirect_to_name("agriculture_all")),
                    )
                    .service(redirect_path_to_name("/academic", "technology_academic"))
                    .service(redirect_path_to_name("/campus", "technology_campus"))
                    .service(web::scope("v2").service(handlers_v2::index))
                    .default_service(redirect_to_name("index_v2")),
            )
            .default_service(
                web::route().to(|| async { HttpResponse::NotFound().body("404 Not Found") }),
            )
    })
    .bind(address)?
    .run()
    .await
}
