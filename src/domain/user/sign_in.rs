use validator::{Validate};
use mysql as my;
use jwt::{encode, Header};
use chrono::{Utc};
use crate::domain::user::{verify, Claims, SignInDTO, DTOErrors};

pub fn run(config: &crate::Config, db_conn: &my::Pool, data: &SignInDTO) -> Result<String, DTOErrors> {
    match data.validate() {
        Ok(_) => {
            let result: Vec<(String, String, String, String)> = db_conn.prep_exec(r"
                SELECT username, email, salt, password FROM `users` 
                WHERE (username = :username OR email = :email) AND enabled = 1 LIMIT 1", params!{
                    "username" => &data.username_or_email.clone(),
                    "email" =>  &data.username_or_email.clone()
                }).map(|result| {
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow your schema
                        let (username, email, salt, password) = my::from_row(row);
                        (username, email, salt, password)
                    }).collect()
                }).unwrap();

            if result.is_empty() {
                return Err(DTOErrors::ApplicationError("Incorrect username.".to_string()));
            }

            let (username, email, salt, hashed_pass) = result[0].clone();
            let user_pass = data.password.to_owned() + &salt;
            let verify_pass_result = verify(user_pass, &hashed_pass);

            match verify_pass_result {
                Ok(_) => {},
                Err(_) => return Err(DTOErrors::ApplicationError("Incorrect password.".to_string()))
            };

            let dt = Utc::now();
            let day_in_sec: i64 = (86400 * config.token_exp).into();
            let my_claims = Claims {
                iss: config.domain.to_string(),
                aud: config.app_name.to_string(),
                sub: config.subject.to_string(),
                exp: dt.timestamp() + day_in_sec,
                username,
                email
            };
            let token_encoding = encode(&Header::default(), &my_claims, config.secret.as_ref());

            return match token_encoding {
                Ok(token) => Ok(token),
                Err(e) => Err(DTOErrors::ApplicationError(e.to_string()))
            }
        },
        Err(e) => return Err(DTOErrors::ValidationError(e))
    };
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::collections::HashMap;
    // use serde_json::Value as JsonValue;
    // use serde_json::Number as Number;
    // use dotenv::dotenv;

    #[test]
    fn invalid_dto() {}

    #[test]
    fn incorrect_username() {}

    #[test]
    fn incorrect_password() {}

    #[test]
    fn jwt_encoding_error() {}

    #[test]
    fn success() {}
}
