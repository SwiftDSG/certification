use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::{fs::read_to_string, io};

mod database;
mod logger;
mod models;
mod routes;

fn load_env() {
    if let Ok(env) = read_to_string(".env") {
        let lines: Vec<(&str, &str)> = env
            .lines()
            .map(|a| {
                let b: Vec<&str> = a.split('=').collect();
                (
                    <&str>::clone(b.first().expect("INVALID_ENVIRONMENT_VARIABLES")),
                    <&str>::clone(b.last().expect("INVALID_ENVIRONMENT_VARIABLES")),
                )
            })
            .collect();

        for (key, value) in lines {
            std::env::set_var(key, value);
        }
    }

    if std::env::var("DATABASE_URI").is_err() {
        std::env::set_var("DATABASE_URI", "mongodb://localhost:27017");
    }
    if std::env::var("BASE_URL").is_err() {
        std::env::set_var("BASE_URL", "http://localhost:8000");
    }
    if std::env::var("BASE_PATH").is_err() {
        std::env::set_var("BASE_PATH", "");
    }
    if std::env::var("HOST").is_err() {
        std::env::set_var("HOST", "127.0.0.1");
    }
    if std::env::var("PORT").is_err() {
        std::env::set_var("PORT", "8000");
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    load_env();

    let host = std::env::var("HOST").expect("INVALID_HOST");
    let port = std::env::var("PORT")
        .unwrap()
        .parse::<u16>()
        .expect("INVALID_PORT");

    database::connect(std::env::var("DATABASE_URI").unwrap()).await;
    models::user::load_keys();

    println!("Running on: http://{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(models::user::UserAuthenticationMiddlewareFactory)
            .wrap(cors)
            .service(
                web::scope(&std::env::var("BASE_PATH").unwrap())
                    .service(routes::get_media)
                    .service(
                        web::scope("/users")
                            .service(routes::user::get_user)
                            .service(routes::user::get_users)
                            .service(routes::user::create_user)
                            .service(routes::user::update_user)
                            .service(routes::user::delete_user)
                            .service(routes::user::login)
                            .service(routes::user::refresh),
                    )
                    .service(
                        web::scope("/books")
                            .service(routes::book::get_book)
                            .service(routes::book::get_books)
                            .service(routes::book::create_book)
                            .service(routes::book::update_book)
                            .service(routes::book::update_book_image)
                            .service(routes::book::delete_book),
                    )
                    .service(
                        web::scope("/loans")
                            .service(routes::loan::get_loan)
                            .service(routes::loan::get_loans)
                            .service(routes::loan::create_loan)
                            .service(routes::loan::update_loan)
                            .service(routes::loan::delete_loan),
                    ),
            )
    })
    .bind((host, port))?
    .run()
    .await
}
