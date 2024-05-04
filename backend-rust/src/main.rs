mod assets;

use actix_web::{App, HttpServer};
use crate::assets::{index, static_files};
use env_logger::Env;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_run_port = std::env::var("APP_RUN_PORT").unwrap_or("8080".to_string());
    log::info!("starting HTTP server at http://localhost:{}", &app_run_port );
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(static_files)
    })
        .bind(format!("0.0.0.0:{}",app_run_port)).expect(format!("Can not bind to port {}", app_run_port).as_str())
        .run().await.unwrap()

}
