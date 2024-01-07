use std::{
    ffi::OsStr,
    fs::{create_dir_all, rename},
    path::{Path, PathBuf},
};

use actix_multipart::form::MultipartForm;
use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse};

use crate::{
    logger::{Logger, MediaErrorKind},
    models::{
        book::{Book, BookQuery, BookRequest},
        media::{Media, MediaExtension, MediaMultipartRequest},
        user::{UserAuthentication, UserRole},
    },
};

#[get("")]
pub async fn get_books(query: web::Query<BookQuery>) -> HttpResponse {
    Book::find_many(&query).await
}
#[get("/{book_id}")]
pub async fn get_book(book_id: web::Path<String>) -> HttpResponse {
    let book_id = match book_id.parse() {
        Ok(book_id) => book_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match Book::find_by_id(&book_id).await {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(res) => res,
    }
}
#[put("/{book_id}")]
pub async fn update_book(
    book_id: web::Path<String>,
    payload: web::Json<BookRequest>,
) -> HttpResponse {
    let book_id = match book_id.parse() {
        Ok(book_id) => book_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    let payload = payload.into_inner();

    match Book::find_by_id(&book_id).await {
        Ok(mut book) => {
            book.name = payload.name;
            book.author = payload.author;
            book.description = payload.description;
            book.year = payload.year;

            book.update().await
        }
        Err(res) => res,
    }
}
#[put("/{book_id}/image")]
pub async fn update_book_image(
    book_id: web::Path<String>,
    form: MultipartForm<MediaMultipartRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let book_id = match book_id.parse() {
        Ok(book_id) => book_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    let issuer = match req.extensions().get::<UserAuthentication>() {
        Some(issuer) => issuer.clone(),
        None => return HttpResponse::Unauthorized().body("UNAUTHORIZED".to_string()),
    };
    if issuer.role != UserRole::Admin {
        return HttpResponse::Unauthorized().body("UNAUTHORIZED".to_string());
    }

    match Book::find_by_id(&book_id).await {
        Ok(mut book) => {
            if let Some(media) = book.media {
                media.delete();
            }

            let mut book_media = None;
            let save_dir = "./medias/";

            if create_dir_all(&save_dir).is_err() {
                return HttpResponse::InternalServerError()
                    .body("DIRECTORY_CREATION_FAILED".to_string());
            }

            if let Some(file) = form.file.first() {
                if let Some(file_name) = &file.file_name {
                    if let Some(ext_str) = Path::new(file_name).extension().and_then(OsStr::to_str)
                    {
                        let ext = MediaExtension::from(ext_str);
                        let media = Media::new(ext);
                        let file_path_temp = file.file.path();
                        let file_path = PathBuf::from(
                            save_dir.to_owned() + &media._id.to_string() + "." + ext_str,
                        );
                        if rename(file_path_temp, &file_path).is_err() {
                            return Logger::MediaError(MediaErrorKind::MediaExtensionNotFound)
                                .emit(None);
                        }
                        book_media = Some(media);
                    } else {
                        return Logger::MediaError(MediaErrorKind::MediaExtensionNotFound)
                            .emit(None);
                    }
                }
            }

            book.media = book_media;

            book.update().await
        }
        Err(res) => res,
    }
}
#[delete("/{book_id}")]
pub async fn delete_book(book_id: web::Path<String>) -> HttpResponse {
    let book_id = match book_id.parse() {
        Ok(book_id) => book_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match Book::find_by_id(&book_id).await {
        Ok(book) => book.delete().await,
        Err(res) => res,
    }
}
#[post("")]
pub async fn create_book(payload: web::Json<BookRequest>) -> HttpResponse {
    let payload: BookRequest = payload.into_inner();

    let book = Book::from(payload);

    book.save().await
}
