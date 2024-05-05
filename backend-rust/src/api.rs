use actix_web::{get, HttpResponse};
use custom_marcos::running_time_derive;

#[running_time_derive]
#[get("/api/hello")]
pub async fn hello() -> HttpResponse {
   HttpResponse::Ok().body("hello")
}
