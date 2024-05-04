use std::str::FromStr;
use actix_web::{get, HttpResponse, web};
use mime::Mime;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

#[get("/")]
async fn index() -> HttpResponse {
    if let Some(file) = Asset::get("index.html") {
        let mime = Mime::from_str("text/html").unwrap();
        HttpResponse::Ok().content_type(mime).body(file.data.into_owned())
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}

#[get("/{filename:.*}")]
async fn static_files(filename: web::Path<String>) -> HttpResponse {
    let file_path = format!("{}", filename);
    if let Some(file) = Asset::get(&file_path) {
        let file_content = file.data.into_owned();
        let mime = match file_path.as_str() {
            path if path.ends_with(".js") => Mime::from_str("application/javascript").unwrap(),
            path if path.ends_with(".css") => Mime::from_str("text/css").unwrap(),
            _ => Mime::from_str("application/octet-stream").unwrap(),
        };
        HttpResponse::Ok().content_type(mime).body(file_content)
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}