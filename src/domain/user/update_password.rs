use validator::{Validate};
use mysql as my;
use bcrypt::{hash};
use crate::domain::user::{generate_random, send_email, UpdatePasswordDTO, DTOErrors};

pub fn run(config: &crate::Config, db_conn: &my::Pool, data: &UpdatePasswordDTO) -> Result<bool, DTOErrors> {
    match data.validate() {
        Ok(_) => {
            let result: Vec<(usize, String, String)> = db_conn.prep_exec(r"
                SELECT u.id, username, email
                FROM users u
                INNER JOIN password_updates pu ON u.id = pu.user_id 
                WHERE pu.token = :token", params!{
                    "token" => &data.token
                }).map(|result| {
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow your schema
                        let (user_id, username, email) = my::from_row(row);
                        (user_id, username, email)
                    }).collect()
                }).unwrap();

            if result.is_empty() {
                return Err(DTOErrors::ApplicationError("Incorrect password update token.".to_string()));
            }

            let (user_id, username, email) = result[0].clone();

            let mut transaction = db_conn.start_transaction(true, Some(mysql::IsolationLevel::Serializable), Some(false)).unwrap();

            let salt: String = generate_random(4);
            let user_pass = data.password.to_owned() + &salt;
            let hashed = hash(user_pass, 4).expect("Unable to hash password.");

            transaction.prep_exec(r"
                UPDATE users SET enabled = 1, password = :password, salt = :salt WHERE id = :user_id", params!{
                    "password" => &hashed,
                    "salt" =>  &salt,
                    "token" => &data.token,
                    "user_id" => &user_id
                }).unwrap();

            // @TODO: Delete password updates by user_id
            transaction.prep_exec(r"DELETE FROM password_updates WHERE user_id = :user_id", params!{
                "user_id" => &user_id
            }).unwrap();

            match transaction.commit() {
                Ok(_) => {
                    // @TODO: use Futures not need to await
                    let message = "Your password has been updated.";
                    send_email(config, &email, &username, message);

                    return Ok(true)
                },
                Err(e) => return Err(DTOErrors::DatabaseError(e.to_string()))
            }
        },
        Err(e) => return Err(DTOErrors::ValidationError(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use serde_json::Value as JsonValue;
    use serde_json::Number as Number;
    use dotenv::dotenv;

    #[test]
    fn invalid_dto() {
        dotenv().ok();

        let data = UpdatePasswordDTO {
            token: "".to_string(),
            password: "12312".to_string()
        };

        let result = run(&crate::config(), &crate::database_connection(), &data);

        let mut error_value = HashMap::new();
        error_value.insert(
            std::borrow::Cow::Borrowed("value"),
            JsonValue::String("".to_string()),
        );
        error_value.insert(
            std::borrow::Cow::Borrowed("min"),
            JsonValue::Number(Number::from(1)),
        );
        let mut validation_error = validator::ValidationErrors::new();
        validation_error.add("token", validator::ValidationError {
            code: std::borrow::Cow::Borrowed("length"),
            message: None,
            params: error_value
        });

        let mut error_value = HashMap::new();
        error_value.insert(
            std::borrow::Cow::Borrowed("value"),
            JsonValue::String("12312".to_string()),
        );
        error_value.insert(
            std::borrow::Cow::Borrowed("min"),
            JsonValue::Number(Number::from(6)),
        );
        validation_error.add("password", validator::ValidationError {
            code: std::borrow::Cow::Borrowed("length"),
            message: None,
            params: error_value
        });

        assert_eq!(result.unwrap_err(), DTOErrors::ValidationError(
            validation_error
        ));
    }

    #[test]
    fn token_not_found() {}

    #[test]
    fn database_error() {}

    #[test]
    fn success() {}
}
