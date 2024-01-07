use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse};
use chrono::Utc;

use crate::models::{
    loan::{Loan, LoanQuery, LoanRequest, LoanStatus},
    user::{UserAuthentication, UserRole},
};

#[get("")]
pub async fn get_loans(query: web::Query<LoanQuery>) -> HttpResponse {
    Loan::find_many(&query).await
}
#[get("/{loan_id}")]
pub async fn get_loan(loan_id: web::Path<String>) -> HttpResponse {
    let loan_id = match loan_id.parse() {
        Ok(loan_id) => loan_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match Loan::find_by_id(&loan_id).await {
        Ok(loan) => HttpResponse::Ok().json(loan),
        Err(res) => res,
    }
}
#[put("/{loan_id}")]
pub async fn update_loan(loan_id: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let loan_id = match loan_id.parse() {
        Ok(loan_id) => loan_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    let issuer = match req.extensions().get::<UserAuthentication>() {
        Some(issuer) => issuer.clone(),
        None => return HttpResponse::Unauthorized().body("UNAUTHORIZED"),
    };

    match Loan::find_by_id(&loan_id).await {
        Ok(mut loan) => {
            if issuer.role != UserRole::Admin && issuer._id != loan.book_id {
                return HttpResponse::Unauthorized().body("UNAUTHORIZED");
            }

            if loan.status == LoanStatus::Unreturned {
                loan.status = LoanStatus::Returned;
                loan.return_date = Some(Utc::now().timestamp_millis());
                loan.update().await
            } else {
                HttpResponse::BadRequest().body("LOAN_ALREADY_RETURNED")
            }
        }
        Err(res) => res,
    }
}
#[delete("/{loan_id}")]
pub async fn delete_loan(loan_id: web::Path<String>) -> HttpResponse {
    let loan_id = match loan_id.parse() {
        Ok(loan_id) => loan_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match Loan::find_by_id(&loan_id).await {
        Ok(loan) => loan.delete().await,
        Err(res) => res,
    }
}
#[post("")]
pub async fn create_loan(payload: web::Json<LoanRequest>, req: HttpRequest) -> HttpResponse {
    let issuer = match req.extensions().get::<UserAuthentication>() {
        Some(issuer) => issuer.clone(),
        None => return HttpResponse::Unauthorized().body("UNAUTHORIZED".to_string()),
    };

    let payload = payload.into_inner();

    let loan = Loan::from(payload, issuer._id);

    loan.save().await
}
