use validator::{Validate};
use mysql as my;
use crate::domain::user::{generate_random, send_email, SignUpWithoutPasswordDTO, DTOErrors};

pub fn run(config: &crate::Config, db_conn: &my::Pool, data: &SignUpWithoutPasswordDTO) -> Result<bool, DTOErrors> {
    match data.validate() {
        Ok(_) => {
            let mut transaction = db_conn.start_transaction(true, Some(mysql::IsolationLevel::Serializable), Some(false)).unwrap();

            transaction.prep_exec(r"INSERT INTO users
                                    (username, email)
                                        VALUES
                                    (:username, :email)", params!{
                    "username" => &data.username.clone(),
                    "email" =>  &data.email.clone(),
                }).unwrap();

            let user_id: Option<String> = transaction.first("SELECT LAST_INSERT_ID();").unwrap();

            let token: String = generate_random(32);
            transaction.prep_exec(r"INSERT INTO password_updates
                                (user_id, token)
                                    VALUES
                                (:user_id, :token)", params!{
                "user_id" => user_id,
                "token" => &token
            }).unwrap();

            match transaction.commit() {
                Ok(_) => {
                    // @TODO: use Futures not need to await
                    let url = format!("{}?token={}", config.create_password_url, "1234");
                    let message = format!("Your sign up is almost complete. You just need to <a target=\"_blank\" href=\"{}\"create a password.</a>", url);
                    send_email(config, &data.email, &data.username, &message);

                    return Ok(true);
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
    fn transaction_error() {}

    #[test]
    fn success() {}
}
