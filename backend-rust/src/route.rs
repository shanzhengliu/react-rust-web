use actix_web::web;
use crate::api::hello;
use crate::assets::{index, static_files};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
       .service(index)
       .service(static_files);
}