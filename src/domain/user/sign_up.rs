use validator::{Validate};
use mysql as my;
use bcrypt::{hash};
use crate::domain::user::{generate_random, send_email, SignUpDTO, DTOErrors};

pub fn run(config: &crate::Config, db_conn: &my::Pool, data: &SignUpDTO) -> Result<bool, DTOErrors> {
    match data.validate() {
        Ok(_) => {
            let salt: String = generate_random(4);
            let user_pass = data.password.to_owned() + &salt;
            let hashed = hash(user_pass, 4).expect("Unable to hash password.");

            let result = db_conn.prep_exec(r"INSERT INTO users
                                    (username, email, password, salt)
                                        VALUES
                                    (:username, :email, :password, :salt)", params!{
                    "username" => &data.username.clone(),
                    "email" =>  &data.email.clone(),
                    "password" => &hashed,
                    "salt" => &salt
                });

            match result {
                Ok(_) => {
                    // @TODO: use Futures not need to await
                    let message = "Your sign up was successful.";
                    send_email(config, &data.email, &data.username, message);

                    return Ok(true)
                },
                Err(e) => return Err(DTOErrors::DatabaseError(e.to_string()))
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
    fn database_error() {}

    #[test]
    fn success() {}
}