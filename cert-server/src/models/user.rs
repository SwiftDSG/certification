use std::{collections::BTreeMap, fs::read_to_string, rc::Rc, str::FromStr};

use crate::{
    database::get_db,
    logger::{Logger, UserErrorKind, UserMessageKind},
};
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage, HttpResponse,
};
use chrono::Utc;
use futures::{
    future::{ready, LocalBoxFuture, Ready},
    stream::StreamExt,
    FutureExt,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::{doc, from_document, oid::ObjectId, to_bson};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};

static mut KEYS: BTreeMap<String, String> = BTreeMap::new();

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    Regular,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub _id: ObjectId,
    pub role: UserRole,
    pub name: String,
    pub username: String,
    pub password: String,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct UserResponse {
    pub _id: String,
    pub role: UserRole,
    pub name: String,
    pub username: String,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct UserMinimalResponse {
    pub _id: String,
    pub name: String,
}
#[derive(Deserialize, Serialize)]
pub struct UserRequest {
    pub role: UserRole,
    pub name: String,
    pub username: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct UserRefreshRequest {
    pub rtk: String,
}
pub struct UserQuery {
    pub _id: Option<ObjectId>,
    pub username: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Clone, Serialize, Deserialize)]
struct UserClaim {
    aud: String,
    exp: i64,
    iss: String,
    sub: String,
}
#[derive(Clone)]
pub struct UserAuthenticationData {
    pub _id: ObjectId,
    pub role: UserRole,
    pub token: String,
}
#[derive(Clone, Deserialize)]
pub struct UserCredential {
    pub username: String,
    pub password: String,
}
pub struct UserAuthenticationMiddleware<S> {
    service: Rc<S>,
}
pub struct UserAuthenticationMiddlewareFactory;

pub type UserAuthentication = Rc<UserAuthenticationData>;

impl From<UserRequest> for User {
    fn from(a: UserRequest) -> Self {
        Self {
            _id: ObjectId::new(),
            role: UserRole::Regular,
            name: a.name,
            username: a.username,
            password: a.password,
        }
    }
}
impl From<User> for UserResponse {
    fn from(a: User) -> Self {
        Self {
            _id: a._id.to_string(),
            role: a.role,
            name: a.name,
            username: a.username,
        }
    }
}

impl User {
    pub async fn save(&mut self) -> HttpResponse {
        let db = get_db();
        let collection = db.collection::<Self>("users");

        if let Ok(hash) = bcrypt::hash(&self.password) {
            self.password = hash;
            if collection.insert_one(self.clone(), None).await.is_ok() {
                Logger::UserMessage(UserMessageKind::UserSaved).emit(Some(
                    &serde_json::to_string(&UserResponse::from(self.clone())).unwrap(),
                ))
            } else {
                Logger::UserError(UserErrorKind::UserSavingFailed).emit(None)
            }
        } else {
            Logger::UserError(UserErrorKind::UserHashingFailed).emit(None)
        }
    }
    pub async fn update(&mut self, password: Option<String>) -> HttpResponse {
        let db = get_db();
        let collection = db.collection::<Self>("users");

        if let Some(password) = password {
            if let Ok(hash) = bcrypt::hash(password) {
                self.password = hash;
            }
        }

        if collection
            .update_one(
                doc! { "_id": self._id },
                doc! { "$set": to_bson::<Self>(self).unwrap() },
                None,
            )
            .await
            .is_ok()
        {
            Logger::UserMessage(UserMessageKind::UserUpdated).emit(Some(
                &serde_json::to_string(&UserResponse::from(self.clone())).unwrap(),
            ))
        } else {
            Logger::UserError(UserErrorKind::UserUpdatingFailed).emit(None)
        }
    }
    pub async fn delete(&self) -> HttpResponse {
        let db = get_db();
        let collection = db.collection::<Self>("users");

        if self.role == UserRole::Admin {
            return HttpResponse::BadRequest().body("USER_IS_ADMIN");
        }

        if collection
            .delete_one(doc! { "_id": self._id }, None)
            .await
            .is_ok()
        {
            Logger::UserMessage(UserMessageKind::UserDeleted).emit(None)
        } else {
            Logger::UserError(UserErrorKind::UserDeletingFailed).emit(None)
        }
    }
    pub async fn find_many(query: &UserQuery) -> HttpResponse {
        let db = get_db();
        let collection = db.collection::<Self>("users");

        let mut pipeline: Vec<mongodb::bson::Document> = Vec::new();
        let mut users: Vec<UserResponse> = Vec::new();

        if let Some(limit) = query.limit {
            pipeline.push(doc! {
                "$limit": to_bson::<usize>(&limit).unwrap()
            })
        }

        pipeline.push(doc! {
            "$project": {
                "_id": {
                    "$toString": "$_id"
                },
                "role": "$role",
                "name": "$name",
                "username": "$username",
            }
        });

        if let Ok(mut cursor) = collection.aggregate(pipeline, None).await {
            while let Some(Ok(doc)) = cursor.next().await {
                let user = from_document::<UserResponse>(doc).unwrap();
                users.push(user);
            }
            if !users.is_empty() {
                HttpResponse::Ok().json(users)
            } else {
                Logger::UserError(UserErrorKind::UserNotFound).emit(None)
            }
        } else {
            Logger::UserError(UserErrorKind::UserNotFound).emit(None)
        }
    }
    pub async fn find_by_id(_id: &ObjectId) -> Result<Self, HttpResponse> {
        let db = get_db();
        let collection = db.collection::<Self>("users");

        if let Ok(Some(user)) = collection.find_one(doc! { "_id": _id }, None).await {
            Ok(user)
        } else {
            Err(Logger::UserError(UserErrorKind::UserNotFound).emit(None))
        }
    }
    pub async fn find_by_username(username: &String) -> Result<User, HttpResponse> {
        let db = get_db();
        let collection = db.collection::<Self>("users");

        if let Ok(Some(user)) = collection
            .find_one(doc! { "username": username }, None)
            .await
        {
            Ok(user)
        } else {
            Err(Logger::UserError(UserErrorKind::UserNotFound).emit(None))
        }
    }
}

impl UserCredential {
    pub async fn authenticate(&self) -> Result<(String, String, UserResponse), HttpResponse> {
        let user = User::find_by_username(&self.username).await?;
        if !bcrypt::verify(self.password.clone(), &user.password) {
            return Err(HttpResponse::BadRequest().body("USER_INVALID_COMBINATION"));
        }

        let claim_access = UserClaim {
            sub: ObjectId::to_string(&user._id),
            exp: Utc::now().timestamp() + 1800,
            iss: "Redian".to_string(),
            aud: std::env::var("BASE_URL").unwrap(),
        };
        let claim_refresh = UserClaim {
            sub: ObjectId::to_string(&user._id),
            exp: Utc::now().timestamp() + 259200,
            iss: "Redian".to_string(),
            aud: std::env::var("BASE_URL").unwrap(),
        };

        let header = Header::new(Algorithm::RS256);
        unsafe {
            match (
                encode(
                    &header,
                    &claim_access,
                    &EncodingKey::from_rsa_pem(KEYS.get("private_access").unwrap().as_bytes())
                        .unwrap(),
                ),
                encode(
                    &header,
                    &claim_refresh,
                    &EncodingKey::from_rsa_pem(KEYS.get("private_refresh").unwrap().as_bytes())
                        .unwrap(),
                ),
            ) {
                (Ok(atk), Ok(rtk)) => match User::find_by_id(&user._id).await {
                    Ok(user) => Ok((atk, rtk, UserResponse::from(user))),
                    Err(res) => Err(res),
                },
                _ => Err(Logger::UserError(UserErrorKind::UserTokenEncodingFailed).emit(None)),
            }
        }
    }
    pub async fn refresh(token: &str) -> Result<(String, String, UserResponse), HttpResponse> {
        let validation = Validation::new(Algorithm::RS256);
        let data;

        unsafe {
            data = decode::<UserClaim>(
                token,
                &DecodingKey::from_rsa_pem(KEYS.get("public_refresh").unwrap().as_bytes()).unwrap(),
                &validation,
            )
            .map_err(|_| Logger::UserError(UserErrorKind::UserTokenDecodingFailed).emit(None))?;
        }
        let _id = ObjectId::from_str(&data.claims.sub)
            .map_err(|_| HttpResponse::BadRequest().body("INVALID_ID"))?;

        let user = User::find_by_id(&_id).await?;

        let claim_access = UserClaim {
            sub: ObjectId::to_string(&user._id),
            exp: Utc::now().timestamp() + 1800,
            iss: "Redian".to_string(),
            aud: std::env::var("BASE_URL").unwrap(),
        };
        let claim_refresh = UserClaim {
            sub: ObjectId::to_string(&user._id),
            exp: Utc::now().timestamp() + 259200,
            iss: "Redian".to_string(),
            aud: std::env::var("BASE_URL").unwrap(),
        };

        let header = Header::new(Algorithm::RS256);
        unsafe {
            match (
                encode(
                    &header,
                    &claim_access,
                    &EncodingKey::from_rsa_pem(KEYS.get("private_access").unwrap().as_bytes())
                        .unwrap(),
                ),
                encode(
                    &header,
                    &claim_refresh,
                    &EncodingKey::from_rsa_pem(KEYS.get("private_refresh").unwrap().as_bytes())
                        .unwrap(),
                ),
            ) {
                (Ok(atk), Ok(rtk)) => match User::find_by_id(&user._id).await {
                    Ok(user) => Ok((atk, rtk, UserResponse::from(user))),
                    Err(res) => Err(res),
                },
                _ => Err(Logger::UserError(UserErrorKind::UserTokenEncodingFailed).emit(None)),
            }
        }
    }
    pub fn verify(token: &str) -> Option<ObjectId> {
        let validation: Validation = Validation::new(Algorithm::RS256);
        unsafe {
            match decode::<UserClaim>(
                token,
                &DecodingKey::from_rsa_pem(KEYS.get("public_access").unwrap().as_bytes()).unwrap(),
                &validation,
            ) {
                Ok(data) => match ObjectId::from_str(&data.claims.sub) {
                    Ok(id) => Some(id),
                    Err(_) => None,
                },
                Err(_) => None,
            }
        }
    }
}

impl<S, B> Service<ServiceRequest> for UserAuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        async move {
            let headers: &actix_web::http::header::HeaderMap = req.headers();
            if let Some(bearer_token) = headers.get("Authorization") {
                let mut bytes_token = Vec::new();
                for i in bearer_token.as_bytes() {
                    bytes_token.push(*i);
                }
                if bytes_token.len() > 7 {
                    bytes_token.drain(0..7);
                    let token = String::from_utf8(bytes_token).unwrap();
                    if let Some(_id) = UserCredential::verify(&token) {
                        if let Ok(user) = User::find_by_id(&_id).await {
                            let auth_data = UserAuthenticationData {
                                _id,
                                role: user.role,
                                token,
                            };
                            req.extensions_mut()
                                .insert::<UserAuthentication>(Rc::new(auth_data));
                        }
                    }
                }
            }
            let res = srv.call(req).await?;
            Ok(res)
        }
        .boxed_local()
    }
}
impl<S, B> Transform<S, ServiceRequest> for UserAuthenticationMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = UserAuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(UserAuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub fn load_keys() {
    let private_access_file =
        read_to_string("./keys/private_access.key").expect("LOAD_FAILED_PRIVATE_ACCESS");
    let public_access_file =
        read_to_string("./keys/public_access.pem").expect("LOAD_FAILED_PUBLIC_ACCESS");
    let private_refresh_file =
        read_to_string("./keys/private_refresh.key").expect("LOAD_FAILED_PRIVATE_ACCESS");
    let public_refresh_file =
        read_to_string("./keys/public_refresh.pem").expect("LOAD_FAILED_PUBLIC_ACCESS");
    unsafe {
        KEYS.insert("private_access".to_string(), private_access_file);
        KEYS.insert("public_access".to_string(), public_access_file);
        KEYS.insert("private_refresh".to_string(), private_refresh_file);
        KEYS.insert("public_refresh".to_string(), public_refresh_file);
    }
}
