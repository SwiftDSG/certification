use crate::{
    database::get_db,
    logger::{BookErrorKind, BookMessageKind, Logger},
};

use actix_web::HttpResponse;
use futures::StreamExt;
use mongodb::bson::{doc, from_document, oid::ObjectId, to_bson};
use serde::{Deserialize, Serialize};

use super::media::{Media, MediaResponse};

#[derive(Clone, Deserialize, Serialize)]
pub struct Book {
    pub _id: ObjectId,
    pub name: String,
    pub author: String,
    pub year: u16,
    pub description: Option<String>,
    pub media: Option<Media>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BookMinimalResponse {
    pub _id: String,
    pub name: String,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct BookResponse {
    pub _id: String,
    pub name: String,
    pub author: String,
    pub year: u16,
    pub description: Option<String>,
    pub media: Option<MediaResponse>,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct BookRequest {
    pub name: String,
    pub author: String,
    pub year: u16,
    pub description: Option<String>,
}
#[derive(Deserialize)]
pub struct BookQuery {
    pub text: Option<String>,
}

impl From<BookRequest> for Book {
    fn from(a: BookRequest) -> Self {
        Self {
            _id: ObjectId::new(),
            name: a.name,
            author: a.author,
            year: a.year,
            description: a.description,
            media: None,
        }
    }
}

impl From<Book> for BookResponse {
    fn from(a: Book) -> Self {
        if let Some(media) = a.media {
            Self {
                _id: a._id.to_string(),
                name: a.name,
                author: a.author,
                year: a.year,
                description: a.description,
                media: Some(MediaResponse::from(media)),
            }
        } else {
            Self {
                _id: a._id.to_string(),
                name: a.name,
                author: a.author,
                year: a.year,
                description: a.description,
                media: None,
            }
        }
    }
}
impl From<Book> for BookMinimalResponse {
    fn from(a: Book) -> Self {
        Self {
            _id: a._id.to_string(),
            name: a.name,
        }
    }
}

impl Book {
    pub async fn save(&self) -> HttpResponse {
        let db = get_db();
        let collection = db.collection::<Self>("books");

        if collection.insert_one(self, None).await.is_ok() {
            Logger::BookMessage(BookMessageKind::BookSaved).emit(Some(
                &serde_json::to_string(&BookResponse::from(self.clone())).unwrap(),
            ))
        } else {
            Logger::BookError(BookErrorKind::BookSavingFailed).emit(None)
        }
    }
    pub async fn update(&self) -> HttpResponse {
        let db = get_db();
        let collection = db.collection::<Self>("books");

        if collection
            .update_one(
                doc! { "_id": self._id },
                doc! { "$set": to_bson::<Self>(self).unwrap() },
                None,
            )
            .await
            .is_ok()
        {
            Logger::BookMessage(BookMessageKind::BookUpdated).emit(Some(
                &serde_json::to_string(&BookResponse::from(self.clone())).unwrap(),
            ))
        } else {
            Logger::BookError(BookErrorKind::BookUpdatingFailed).emit(None)
        }
    }
    pub async fn delete(&self) -> HttpResponse {
        let db = get_db();
        let collection = db.collection::<Self>("books");

        if collection
            .delete_one(doc! { "_id": self._id }, None)
            .await
            .is_ok()
        {
            Logger::BookMessage(BookMessageKind::BookDeleted).emit(None)
        } else {
            Logger::BookError(BookErrorKind::BookDeletingFailed).emit(None)
        }
    }
    pub async fn find_by_id(_id: &ObjectId) -> Result<Self, HttpResponse> {
        let db = get_db();
        let collection = db.collection::<Self>("books");

        if let Ok(Some(book)) = collection.find_one(doc! { "_id": _id }, None).await {
            Ok(book)
        } else {
            Err(Logger::BookError(BookErrorKind::BookNotFound).emit(None))
        }
    }
    pub async fn find_many(query: &BookQuery) -> HttpResponse {
        let db = get_db();
        let collection = db.collection::<Self>("books");

        let mut pipeline = Vec::new();
        let mut queries = Vec::new();

        if let Some(text) = &query.text {
            queries.push(doc! {
                "$or": [
                    {
                        "$regexMatch": {
                            "input": "$name",
                            "options": "i",
                            "regex": to_bson::<String>(text).unwrap()
                        }
                    },
                    {
                        "$regexMatch": {
                            "input": "$description",
                            "options": "i",
                            "regex": to_bson::<String>(text).unwrap()
                        }
                    },
                    {
                        "$regexMatch": {
                            "input": "$author",
                            "options": "i",
                            "regex": to_bson::<String>(text).unwrap()
                        }
                    },
                ]
            });
        }

        pipeline.push(doc! {
            "$match": {
                "$expr": {
                    "$and": queries
                }
            }
        });
        pipeline.push(doc! {
            "$project": {
                "_id": {
                    "$toString": "$_id"
                },
                "name": "$name",
                "author": "$author",
                "year": "$year",
                "description": "$description",
                "media": {
                    "$cond": [
                        "$media",
                        {
                            "id": {
                                "$toString": "$media._id"
                            },
                            "ext": "$media.extension"
                        },
                        to_bson::<Option<MediaResponse>>(&None).unwrap()
                    ]
                },
            }
        });

        if let Ok(mut cursor) = collection.aggregate(pipeline, None).await {
            let mut books = Vec::<BookResponse>::new();
            while let Some(Ok(doc)) = cursor.next().await {
                let book = from_document::<BookResponse>(doc).unwrap();
                books.push(book);
            }
            if !books.is_empty() {
                HttpResponse::Ok().json(books)
            } else {
                Logger::BookError(BookErrorKind::BookNotFound).emit(None)
            }
        } else {
            Logger::BookError(BookErrorKind::BookNotFound).emit(None)
        }
    }
}
