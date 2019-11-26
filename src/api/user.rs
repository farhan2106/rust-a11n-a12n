use crate::domain::user;
use mysql as my;
use serde_json::Value as JsonValue;
use actix_web::{web, HttpResponse, Error};

#[derive(Serialize, Deserialize)]
pub struct RPCRequest {
  pub id: String,
  pub jsonrpc: String,
  pub method: String,
  pub params: JsonValue
}

pub fn sign_up(config: &crate::Config, db_conn: &my::Pool, message: &web::Json<RPCRequest>) -> Result<HttpResponse, Error> {
    let api_param = user::SignUpDTO {
        username: message.params["username"].as_str().unwrap().to_string(),
        email: message.params["email"].as_str().unwrap().to_string(),
        password: message.params["password"].as_str().unwrap().to_string()
    };

    return match user::sign_up::run(config, &db_conn, &api_param) {
        Ok(_) => Ok(HttpResponse::Ok()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "success" }),
                "id": message.id.to_string()
            }))),
        Err(e) => Ok(HttpResponse::BadRequest()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "error", "errors": e }),
                "id": message.id.to_string()
            })))
    };
}

pub fn sign_up_without_password(config: &crate::Config, db_conn: &my::Pool, message: &web::Json<RPCRequest>) -> Result<HttpResponse, Error> {
    let api_param = user::SignUpWithoutPasswordDTO {
        username: message.params["username"].as_str().unwrap().to_string(),
        email: message.params["email"].as_str().unwrap().to_string()
    };

    return match user::sign_up_without_password::run(config, &db_conn, &api_param) {
        Ok(_) => Ok(HttpResponse::Ok()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "success" }),
                "id": message.id.to_string()
            }))),
        Err(e) => Ok(HttpResponse::BadRequest()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "error", "errors": e }),
                "id": message.id.to_string()
            })))
    };
}

pub fn sign_in(config: &crate::Config, db_conn: &my::Pool, message: &web::Json<RPCRequest>) -> Result<HttpResponse, Error> {
    let api_param = user::SignInDTO {
        username_or_email: message.params["username_or_email"].as_str().unwrap().to_string(),
        password: message.params["password"].as_str().unwrap().to_string()
    };

    return match user::sign_in::run(config, &db_conn, &api_param) {
        Ok(token) => Ok(HttpResponse::Ok()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "success", "token": token }),
                "id": message.id.to_string()
            }))),
        Err(e) => {
            return match e {
                user::DTOErrors::ApplicationError(e) => Ok(HttpResponse::NotFound()
                        .json(json!({
                            "jsonrpc": message.jsonrpc.to_string(),
                            "result": json!({ "status": "error", "errors": e }),
                            "id": message.id.to_string()
                        }))),
                 _ => Ok(HttpResponse::BadRequest()
                        .json(json!({
                            "jsonrpc": message.jsonrpc.to_string(),
                            "result": json!({ "status": "error", "errors": e }),
                            "id": message.id.to_string()
                        })))
            }
        }
    };
}

pub fn authenticate(config: &crate::Config, message: &web::Json<RPCRequest>) -> Result<HttpResponse, Error> {
    let api_param = user::AuthenticateDTO {
        token: message.params["token"].as_str().unwrap().to_string()
    };
    return match user::authenticate::run(config, &api_param) {
        Ok(_) => Ok(HttpResponse::Ok()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "success" }),
                "id": message.id.to_string()
            }))),
        Err(e) => Ok(HttpResponse::BadRequest()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "error", "errors": e }),
                "id": message.id.to_string()
            })))
    };
}

pub fn update_password(config: &crate::Config, db_conn: &my::Pool, message: &web::Json<RPCRequest>) -> Result<HttpResponse, Error> {
    let api_param = user::UpdatePasswordDTO {
        token: message.params["token"].as_str().unwrap().to_string(),
        password: message.params["password"].as_str().unwrap().to_string()
    };
    return match user::update_password::run(config, &db_conn, &api_param) {
        Ok(_) => Ok(HttpResponse::Ok()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "success" }),
                "id": message.id.to_string()
            }))),
        Err(e) => Ok(HttpResponse::BadRequest()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "error", "errors": e }),
                "id": message.id.to_string()
            })))
    };
}

pub fn identity_check(db_conn: &my::Pool, message: &web::Json<RPCRequest>) -> Result<HttpResponse, Error> {
    let api_param = user::IdentityCheckDTO {
        identity: message.params["identity"].as_str().unwrap().to_string(),
    };
    return match user::identity_check::run(&db_conn, &api_param) {
        Ok(_) => Ok(HttpResponse::Ok()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "success" }),
                "id": message.id.to_string()
            }))),
        Err(e) => {
            return match e {
                user::DTOErrors::ApplicationError(e) => Ok(HttpResponse::NotFound()
                        .json(json!({
                            "jsonrpc": message.jsonrpc.to_string(),
                            "result": json!({ "status": "error", "errors": e }),
                            "id": message.id.to_string()
                        }))),
                 _ => Ok(HttpResponse::BadRequest()
                        .json(json!({
                            "jsonrpc": message.jsonrpc.to_string(),
                            "result": json!({ "status": "error", "errors": e }),
                            "id": message.id.to_string()
                        })))
            }
        }
    };
}

pub fn forgot_my_password(config: &crate::Config, db_conn: &my::Pool, message: &web::Json<RPCRequest>) -> Result<HttpResponse, Error> {
    let api_param = user::ForgotMyPasswordDTO {
        username_or_email: message.params["username_or_email"].as_str().unwrap().to_string()
    };

    return match user::forgot_my_password::run(config, &db_conn, &api_param) {
        Ok(_) => Ok(HttpResponse::Ok()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "success" }),
                "id": message.id.to_string()
            }))),
        Err(e) => Ok(HttpResponse::BadRequest()
            .json(json!({
                "jsonrpc": message.jsonrpc.to_string(),
                "result": json!({ "status": "error", "errors": e }),
                "id": message.id.to_string()
            })))
    };
}
