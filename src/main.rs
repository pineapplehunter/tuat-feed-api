//! # tuat-feed-api(TUAT Feed API Server)
//!
//! This is code for a server that formatsthe TUAT feed to json

use actix_web::{http::header, middleware, web, App, HttpRequest, HttpResponse, HttpServer};
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
                    .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
                        HttpResponse::Found()
                            .append_header((
                                header::LOCATION,
                                req.url_for_static("technology_all").unwrap().as_str(),
                            ))
                            .finish()
                    })))
                    .service(
                        web::resource("/academic").route(web::get().to(|req: HttpRequest| {
                            HttpResponse::Found()
                                .append_header((
                                    header::LOCATION,
                                    req.url_for_static("technology_academic").unwrap().as_str(),
                                ))
                                .finish()
                        })),
                    )
                    .service(
                        web::resource("/campus").route(web::get().to(|req: HttpRequest| {
                            HttpResponse::Found()
                                .append_header((
                                    header::LOCATION,
                                    req.url_for_static("technology_campus").unwrap().as_str(),
                                ))
                                .finish()
                        })),
                    ),
            )
    })
    .bind("localhost:8081")?
    .run()
    .await
}
