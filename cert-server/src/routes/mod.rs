use actix_web::{get, web, HttpResponse};
use serde::Deserialize;
use std::fs;

use crate::models::media::MediaExtension;

pub mod book;
pub mod loan;
pub mod user;

#[derive(Deserialize)]
pub struct FileQueryParams {
    pub _id: String,
    pub extension: String,
}

#[get("/medias")]
pub async fn get_media(query: web::Query<FileQueryParams>) -> HttpResponse {
    let path = format!("./medias/{}.{}", query._id, query.extension.clone());
    if let Ok(file) = fs::read(path.clone()) {
        let mime = MediaExtension::from(query.extension.as_str()).to_string_mime();
        HttpResponse::Ok().content_type(mime).body(file)
    } else {
        HttpResponse::NotFound().body("CONTENT_NOT_FOUND")
    }
}
