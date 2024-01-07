use actix_web::{delete, get, post, put, web, HttpResponse};
use mongodb::bson::{doc, to_bson};

use crate::models::user::{
    User, UserCredential, UserQuery, UserRefreshRequest, UserRequest, UserResponse, UserRole,
};

#[get("")]
pub async fn get_users() -> HttpResponse {
    let query: UserQuery = UserQuery {
        _id: None,
        username: None,
        limit: None,
    };

    User::find_many(&query).await
}
#[get("/{user_id}")]
pub async fn get_user(user_id: web::Path<String>) -> HttpResponse {
    let user_id = match user_id.parse() {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match User::find_by_id(&user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(res) => res,
    }
}
#[put("/{user_id}")]
pub async fn update_user(
    user_id: web::Path<String>,
    payload: web::Json<UserRequest>,
) -> HttpResponse {
    let user_id = match user_id.parse() {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    let payload = payload.into_inner();

    if payload.password.len() < 8 {
        return HttpResponse::BadRequest().body("USER_MUST_HAVE_VALID_PASSWORD");
    }

    match User::find_by_id(&user_id).await {
        Ok(mut user) => {
            let mut password = None;
            if payload.role == UserRole::Admin {
                return HttpResponse::BadRequest().body("USER_ROLE_INVALID");
            }
            if payload.password.len() >= 8 {
                password = Some(payload.password);
            }
            user.role = payload.role;
            user.name = payload.name;
            user.username = payload.username;

            user.update(password).await
        }
        Err(res) => res,
    }
}
#[delete("/{user_id}")]
pub async fn delete_user(user_id: web::Path<String>) -> HttpResponse {
    let user_id = match user_id.parse() {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match User::find_by_id(&user_id).await {
        Ok(user) => {
            if user.role == UserRole::Admin {
                return HttpResponse::BadRequest().body("USER_Admin_CANNOT_BE_DELETED");
            }

            user.delete().await
        }
        Err(res) => res,
    }
}
#[post("")]
pub async fn create_user(payload: web::Json<UserRequest>) -> HttpResponse {
    let payload: UserRequest = payload.into_inner();

    if payload.password.len() < 8 {
        return HttpResponse::BadRequest().body("USER_MUST_HAVE_VALID_PASSWORD");
    }

    let mut user = User::from(payload);

    if let Ok(_) = User::find_by_username(&user.username).await {
        HttpResponse::BadRequest().body("USER_ALREADY_EXIST")
    } else {
        user.save().await
    }
}
#[post("/login")]
pub async fn login(payload: web::Json<UserCredential>) -> HttpResponse {
    let payload = payload.into_inner();

    match payload.authenticate().await {
        Ok((atk, rtk, user)) => HttpResponse::Ok().json(doc! {
            "atk": to_bson::<String>(&atk).unwrap(),
            "rtk": to_bson::<String>(&rtk).unwrap(),
            "user": to_bson::<UserResponse>(&user).unwrap()
        }),
        Err(res) => res,
    }
}
#[post("/refresh")]
pub async fn refresh(payload: web::Json<UserRefreshRequest>) -> HttpResponse {
    let payload = payload.into_inner();

    match UserCredential::refresh(&payload.rtk).await {
        Ok((atk, rtk, user)) => HttpResponse::Ok().json(doc! {
            "atk": to_bson::<String>(&atk).unwrap(),
            "rtk": to_bson::<String>(&rtk).unwrap(),
            "user": to_bson::<UserResponse>(&user).unwrap()
        }),
        Err(res) => res,
    }
}
