use validator::{Validate};
use mysql as my;
use crate::domain::user::{generate_random, send_email, ForgotMyPasswordDTO, DTOErrors};

pub fn run(config: &crate::Config, db_conn: &my::Pool, data: &ForgotMyPasswordDTO) -> Result<bool, DTOErrors> {
    match data.validate() {
        Ok(_) => {
            let result: Vec<(usize, String, String)> = db_conn.prep_exec(r"
                SELECT id, username, email
                FROM users
                WHERE username = :username OR email = :email", params!{
                    "username" => &data.username_or_email,
                    "email" => &data.username_or_email
                }).map(|result| {
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow your schema
                        let (user_id, username, email) = my::from_row(row);
                        (user_id, username, email)
                    }).collect()
                }).unwrap();

            if result.is_empty() {
                return Err(DTOErrors::ApplicationError("Username or email not found.".to_string()));
            }

            let (user_id, username, email) = result[0].clone();

            let mut transaction = db_conn.start_transaction(true, Some(mysql::IsolationLevel::Serializable), Some(false)).unwrap();

            transaction.prep_exec(r"
                UPDATE users SET enabled = 0 WHERE id = :id;", params!{
                    "id" => &user_id
                }).unwrap();

            let token: String = generate_random(32);
            transaction.prep_exec(r"INSERT INTO password_updates
                                (user_id, token)
                                    VALUES
                                (:user_id, :token)", params!{
                "user_id" => &user_id,
                "token" => &token
            }).unwrap();

            match transaction.commit() {
                Ok(_) => {
                    // @TODO: use Futures not need to await
                    let message = "A password reset token has been sent to your email.";
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
    // use super::*;
    // use std::collections::HashMap;
    // use serde_json::Value as JsonValue;
    // use serde_json::Number as Number;
    use dotenv::dotenv;

    #[test]
    fn invalid_dto() {
        dotenv().ok();
    }

    #[test]
    fn token_not_found() {}

    #[test]
    fn database_error() {}

    #[test]
    fn success() {}
}
