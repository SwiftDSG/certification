use crate::{
    database::get_db,
    logger::{LoanErrorKind, LoanMessageKind, Logger},
};

use actix_web::HttpResponse;
use chrono::Utc;
use futures::StreamExt;
use mongodb::bson::{doc, from_document, oid::ObjectId, to_bson};
use serde::{Deserialize, Serialize};

use super::{
    book::{Book, BookMinimalResponse},
    user::{User, UserMinimalResponse},
};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LoanStatus {
    Returned,
    Unreturned,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct Loan {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub book_id: ObjectId,
    pub borrow_date: i64,
    pub return_date: Option<i64>,
    pub status: LoanStatus,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct LoanResponse {
    pub _id: String,
    pub user: UserMinimalResponse,
    pub book: BookMinimalResponse,
    pub borrow_date: i64,
    pub return_date: Option<i64>,
    pub status: LoanStatus,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct LoanRequest {
    pub book_id: ObjectId,
}
#[derive(Deserialize)]
pub struct LoanQuery {
    pub text: Option<String>,
}

impl LoanResponse {
    pub async fn from(a: Loan) -> Self {
        let mut loan = Self {
            _id: a._id.to_string(),
            book: BookMinimalResponse {
                _id: a.book_id.to_string(),
                name: a.book_id.to_string(),
            },
            user: UserMinimalResponse {
                _id: a.user_id.to_string(),
                name: a.user_id.to_string(),
            },
            borrow_date: a.borrow_date,
            return_date: a.return_date,
            status: a.status,
        };

        if let Ok(book) = Book::find_by_id(&a.book_id).await {
            loan.book.name = book.name;
        }
        if let Ok(user) = User::find_by_id(&a.user_id).await {
            loan.user.name = user.name;
        }

        loan
    }
}
impl Loan {
    pub fn from(a: LoanRequest, user_id: ObjectId) -> Self {
        Self {
            _id: ObjectId::new(),
            book_id: a.book_id,
            user_id,
            borrow_date: Utc::now().timestamp_millis(),
            return_date: None,
            status: LoanStatus::Unreturned,
        }
    }
    pub async fn save(&self) -> HttpResponse {
        let db = get_db();
        let collection = db.collection::<Self>("loans");

        if collection.insert_one(self, None).await.is_ok() {
            Logger::LoanMessage(LoanMessageKind::LoanSaved).emit(Some(
                &serde_json::to_string(&LoanResponse::from(self.clone()).await).unwrap(),
            ))
        } else {
            Logger::LoanError(LoanErrorKind::LoanSavingFailed).emit(None)
        }
    }
    pub async fn update(&self) -> HttpResponse {
        let db = get_db();
        let collection = db.collection::<Self>("loans");

        if collection
            .update_one(
                doc! { "_id": self._id },
                doc! { "$set": to_bson::<Self>(self).unwrap() },
                None,
            )
            .await
            .is_ok()
        {
            Logger::LoanMessage(LoanMessageKind::LoanUpdated).emit(Some(
                &serde_json::to_string(&LoanResponse::from(self.clone()).await).unwrap(),
            ))
        } else {
            Logger::LoanError(LoanErrorKind::LoanUpdatingFailed).emit(None)
        }
    }
    pub async fn delete(&self) -> HttpResponse {
        let db = get_db();
        let collection = db.collection::<Self>("loans");

        if collection
            .delete_one(doc! { "_id": self._id }, None)
            .await
            .is_ok()
        {
            Logger::LoanMessage(LoanMessageKind::LoanDeleted).emit(None)
        } else {
            Logger::LoanError(LoanErrorKind::LoanDeletingFailed).emit(None)
        }
    }
    pub async fn find_by_id(_id: &ObjectId) -> Result<Self, HttpResponse> {
        let db = get_db();
        let collection = db.collection::<Self>("loans");

        if let Ok(Some(loan)) = collection.find_one(doc! { "_id": _id }, None).await {
            Ok(loan)
        } else {
            Err(Logger::LoanError(LoanErrorKind::LoanNotFound).emit(None))
        }
    }
    pub async fn find_many(query: &LoanQuery) -> HttpResponse {
        let db = get_db();
        let collection = db.collection::<Self>("loans");

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
            "$lookup": {
                "from": "users",
                "as": "user",
                "let": {
                    "user_id": "$user_id"
                },
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$eq": ["$_id", "$$user_id"]
                            }
                        }
                    },
                    {
                        "$project": {
                            "_id": {
                                "$toString": "$_id"
                            },
                            "name": "$name"
                        }
                    }
                ],
            }
        });
        pipeline.push(doc! {
            "$lookup": {
                "from": "books",
                "as": "book",
                "let": {
                    "book_id": "$book_id"
                },
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$eq": ["$_id", "$$book_id"]
                            }
                        }
                    },
                    {
                        "$project": {
                            "_id": {
                                "$toString": "$_id"
                            },
                            "name": "$name"
                        }
                    }
                ],
            }
        });
        pipeline.push(doc! {
            "$project": {
                "_id": {
                    "$toString": "$_id"
                },
                "user": {
                    "$first": "$user"
                },
                "book": {
                    "$first": "$book"
                },
                "borrow_date": "$borrow_date",
                "return_date": "$return_date",
                "status": "$status",
            }
        });

        if let Ok(mut cursor) = collection.aggregate(pipeline, None).await {
            let mut loans = Vec::<LoanResponse>::new();
            while let Some(Ok(doc)) = cursor.next().await {
                let loan = from_document::<LoanResponse>(doc).unwrap();
                loans.push(loan);
            }
            if !loans.is_empty() {
                HttpResponse::Ok().json(loans)
            } else {
                Logger::LoanError(LoanErrorKind::LoanNotFound).emit(None)
            }
        } else {
            Logger::LoanError(LoanErrorKind::LoanNotFound).emit(None)
        }
    }
}
