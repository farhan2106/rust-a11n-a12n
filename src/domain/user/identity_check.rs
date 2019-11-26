use mysql as my;
use crate::domain::user::{IdentityCheckDTO, DTOErrors};

pub fn run(db_conn: &my::Pool, data: &IdentityCheckDTO) -> Result<Vec<(String, String)>, DTOErrors> {
    let result: Vec<(String, String)> = db_conn.prep_exec(r"
      SELECT username, email FROM `users` 
      WHERE (username = :username OR email = :email) LIMIT 1", params!{
          "username" => &data.identity.clone(),
          "email" =>  &data.identity.clone()
        }).map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
              // ⚠️ Note that from_row will panic if you don't follow your schema
              let (username, email) = my::from_row(row);
              (username, email)
            }).collect()
        }).unwrap();

    if result.is_empty() {
        return Err(DTOErrors::ApplicationError("Identity not found.".to_string()));
    }

    return Ok(result);
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::collections::HashMap;
    // use serde_json::Value as JsonValue;
    // use serde_json::Number as Number;
    // use dotenv::dotenv;

    #[test]
    fn identity_not_found() {}

    #[test]
    fn success() {}
}
