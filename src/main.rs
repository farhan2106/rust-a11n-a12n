#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate validator_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate mysql;
extern crate jsonwebtoken as jwt;
extern crate chrono;
extern crate env_logger;

mod database;
mod api;
mod domain;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder, Result};
use mysql as my;
use dotenv::dotenv;
use std::env;

pub struct Config {
    rust_env: String,
    create_password_url: String,
    app_name: String,
    domain: String,
    secret: String,
    subject: String,
    token_exp: i32,
    sender_email: String,
    smtp_user: String,
    smtp_pass: String,
    smtp_server: String
}

struct AppState {
    config: Config,
    db_conn: my::Pool
}

fn config() -> Config {
    let rust_env = env::var("RUST_ENV").expect("RUST_ENV needs to be set.");
    let create_password_url = env::var("CREATE_PASSWORD_URL").expect("CREATE_PASSWORD_URL needs to be set.");
    let app_name = env::var("APP_NAME").expect("APP_NAME needs to be set.");
    let domain = env::var("DOMAIN").expect("DOMAIN needs to be set.");
    let secret = env::var("SECRET").expect("SECRET needs to be set.");
    let subject = env::var("SUBJECT").expect("SUBJECT needs to be set.");
    let token_exp = env::var("TOKEN_EXPIRY").expect("TOKEN_EXPIRY needs to be set.");
    let sender_email = env::var("SENDER_EMAIL").expect("SENDER_EMAIL needs to be set.");
    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER needs to be set.");
    let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS needs to be set.");
    let smtp_server = env::var("SMTP_USER").expect("SMTP_USER needs to be set.");
    Config {
        rust_env,
        create_password_url,
        app_name,
        domain,
        secret,
        subject,
        token_exp: token_exp.parse::<i32>().unwrap(),
        sender_email,
        smtp_user,
        smtp_pass,
        smtp_server
    }
}

fn database_connection() -> my::Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL needs to be set.");
    return my::Pool::new(&env::var("DATABASE_URL").expect("DATABASE_URL needs to be set.")).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}

fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.config.app_name;
    HttpResponse::Ok().body(format!("Hello {}!", app_name))
}

fn api(data: web::Data<AppState>, message: web::Json<api::user::RPCRequest>) -> Result<HttpResponse> {
    if message.method == "app.sign_up" {
        return api::user::sign_up(&data.config, &data.db_conn, &message);
    } else if message.method == "app.sign_up_without_password" {
        return api::user::sign_up_without_password(&data.config, &data.db_conn, &message);
    } else if message.method == "app.sign_in" {
        return api::user::sign_in(&data.config, &data.db_conn, &message);
    } else if message.method == "app.authenticate" {
        return api::user::authenticate(&data.config, &message);
    }  else if message.method == "app.update_password" {
        return api::user::update_password(&data.config, &data.db_conn, &message);
    }  else if message.method == "app.identity_check" {
        return api::user::identity_check(&data.db_conn, &message);
    }  else if message.method == "app.forgot_my_password" {
        return api::user::forgot_my_password(&data.config, &data.db_conn, &message);
    } else {
        Ok(HttpResponse::NotFound()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "error", "error": "Method doesn't exist." }),
                "id": message.id.to_string()
            })))
    }
}

fn main() {
    dotenv().ok();
    let rust_env = env::var("RUST_ENV").unwrap_or("development".to_string());
    let port = env::var("PORT").unwrap_or("8000".to_string());
    let allowed_origin = env::var("ALLOWED_ORIGIN").unwrap_or("http://localhost:1234".to_string());

    // https://docs.rs/env_logger/0.6.2/env_logger/
    if rust_env == "development" {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
    } else {
        std::env::set_var("RUST_LOG", "actix_web=info");
        // std::env::set_var("RUST_LOG", "my_=debug,actix_web=info");
    }
    env_logger::init();

    let context = web::Data::new(AppState {
        config: config(),
        db_conn: database_connection()
    });
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin(&allowed_origin)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .register_data(context.clone()) // <- create app with shared state
            // .data() <- create app with non-shared state
            .route("/", web::get().to(index))
            .route("/api", web::post().to(api))
    })
    .bind(format!("127.0.0.1:{}", port))
    .unwrap()
    .run()
    .unwrap();
}
