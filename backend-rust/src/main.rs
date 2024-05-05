extern crate core;

mod route;
mod api;
mod assets;


use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::http::header;
use env_logger::Env;
use crate::route::config;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_run_port = std::env::var("APP_RUN_PORT").unwrap_or("8080".to_string());
    log::info!("starting HTTP server at http://localhost:{}", &app_run_port );

    let app = ||{ App::new()
        .wrap(Cors::permissive()
            .allowed_methods(vec!["*"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
            )
        .configure(config)};

    HttpServer::new(app)
        .bind(format!("0.0.0.0:{}",app_run_port)).expect(format!("Can not bind to port {}", app_run_port).as_str())
        .run().await.unwrap()

}
