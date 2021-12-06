//! # tuat-feed-api(TUAT Feed API Server)
//!
//! This is code for a server that formatsthe TUAT feed to json

use actix_web::{
    dev::HttpServiceFactory, http::header, middleware, web, App, HttpRequest, HttpResponse,
    HttpServer,
};
use std::{env, sync::Arc, time::Duration};
use tokio::time::sleep;
use tuat_feed_api::{
    handlers::{agriculture, technology},
    state::ServerState,
};

/// Interval time (in minutes) for checking for new content.
const INTERVAL_MINUTES: u64 = 15;

/// Interval duration computed from `INTERVAL_MIN`.
const INTERVAL: Duration = Duration::from_secs(INTERVAL_MINUTES * 60);

fn redirect_to_name(path: &'static str, name: &'static str) -> impl HttpServiceFactory {
    web::resource(path).route(web::get().to(|req: HttpRequest| {
        HttpResponse::Found()
            .append_header((header::LOCATION, req.url_for_static(name).unwrap().as_str()))
            .finish()
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = Arc::new(ServerState::init());
    let state_cloned = state.clone();

    env::set_var(
        "RUST_LOG",
        "actix_web=debug,actix_server=info,tuat_feed_parser=info,tuat_feed_api=info",
    );
    env_logger::init();

    let base_path = env::var("TUAT_FEED_API_BASEPATH").unwrap_or_else(|_| String::new());

    tokio::spawn(async move {
        loop {
            state_cloned.update().await;
            sleep(INTERVAL).await;
        }
    });
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope(&base_path)
                    .service(
                        web::scope("/technology")
                            .service(technology::all)
                            .service(technology::academic)
                            .service(technology::campus),
                    )
                    .service(
                        web::scope("/agriculture")
                            .service(agriculture::all)
                            .service(agriculture::academic)
                            .service(agriculture::campus),
                    )
                    .service(redirect_to_name("/", "technology_all"))
                    .service(redirect_to_name("/academic", "technology_academic"))
                    .service(redirect_to_name("/campus", "technology_campus")),
            )
            .default_service(web::route().to(|| HttpResponse::NotFound().body("404 Not Found")))
    })
    .bind("localhost:8081")?
    .run()
    .await
}
