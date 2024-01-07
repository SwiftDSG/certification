use std::fs;

use actix_web::HttpResponse;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookErrorKind {
    BookNotFound,
    BookUpdatingFailed,
    BookSavingFailed,
    BookDeletingFailed,
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookMessageKind {
    BookSaved,
    BookUpdated,
    BookDeleted,
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoanErrorKind {
    LoanNotFound,
    LoanUpdatingFailed,
    LoanSavingFailed,
    LoanDeletingFailed,
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoanMessageKind {
    LoanSaved,
    LoanUpdated,
    LoanDeleted,
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserErrorKind {
    UserNotFound,
    UserUpdatingFailed,
    UserSavingFailed,
    UserDeletingFailed,
    UserHashingFailed,
    UserTokenDecodingFailed,
    UserTokenEncodingFailed,
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserMessageKind {
    UserSaved,
    UserUpdated,
    UserDeleted,
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaErrorKind {
    MediaExtensionNotFound,
    MediaUpdatingFailed,
    MediaSavingFailed,
    MediaDeletingFailed,
}

pub enum Logger {
    BookError(BookErrorKind),
    BookMessage(BookMessageKind),
    LoanError(LoanErrorKind),
    LoanMessage(LoanMessageKind),
    UserError(UserErrorKind),
    UserMessage(UserMessageKind),
    MediaError(MediaErrorKind),
}

impl Logger {
    pub fn emit(&self, message: Option<&str>) -> HttpResponse {
        let mut builder;
        let response;
        let kind = match self {
            Logger::BookError(kind) => {
                builder = match kind {
                    BookErrorKind::BookNotFound => HttpResponse::NotFound(),
                    _ => HttpResponse::InternalServerError(),
                };
                serde_json::to_string(&kind)
            }
            Logger::BookMessage(kind) => {
                builder = match kind {
                    BookMessageKind::BookSaved => HttpResponse::Created(),
                    _ => HttpResponse::Ok(),
                };
                serde_json::to_string(&kind)
            }
            Logger::LoanError(kind) => {
                builder = match kind {
                    LoanErrorKind::LoanNotFound => HttpResponse::NotFound(),
                    _ => HttpResponse::InternalServerError(),
                };
                serde_json::to_string(&kind)
            }
            Logger::LoanMessage(kind) => {
                builder = match kind {
                    LoanMessageKind::LoanSaved => HttpResponse::Created(),
                    _ => HttpResponse::Ok(),
                };
                serde_json::to_string(&kind)
            }
            Logger::UserError(kind) => {
                builder = match kind {
                    UserErrorKind::UserNotFound => HttpResponse::NotFound(),
                    _ => HttpResponse::InternalServerError(),
                };
                serde_json::to_string(&kind)
            }
            Logger::UserMessage(kind) => {
                builder = match kind {
                    UserMessageKind::UserSaved => HttpResponse::Created(),
                    _ => HttpResponse::Ok(),
                };
                serde_json::to_string(&kind)
            }
            Logger::MediaError(kind) => {
                builder = match kind {
                    MediaErrorKind::MediaExtensionNotFound => HttpResponse::NotFound(),
                    _ => HttpResponse::InternalServerError(),
                };
                serde_json::to_string(&kind)
            }
        };

        let date = Utc::now().format("%d/%m/%Y %H:%M");
        let msg;
        let log = match kind {
            Ok(kind) => {
                if let Some(message) = message {
                    response = builder.body(message.to_owned());
                    format!("{}, {} - {}\n", kind, message, date)
                } else {
                    response = builder.body(kind.clone());
                    format!("{} - {}\n", kind, date)
                }
            }
            Err(err) => {
                msg = "ERROR_SERIALIZATION_FAILED".to_string();
                response = builder.body(msg.clone());
                format!("{}, {} - {}\n", msg, err.to_string(), date)
            }
        };

        match fs::read_to_string("./logs") {
            Ok(mut val) => {
                val.push_str(&log);
                if fs::write("./logs", &val).is_err() {
                    println!("{}", log);
                };
            }
            Err(_) => {
                if fs::write("./logs", &log).is_err() {
                    println!("{}", log);
                };
            }
        };

        response
    }
}
