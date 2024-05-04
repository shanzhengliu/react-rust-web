use actix_web::{get, HttpResponse};
#[get("/api/hello")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("hello")
}
