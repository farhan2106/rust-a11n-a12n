use jwt::{decode, Validation};
use crate::domain::user::{Claims, AuthenticateDTO, DTOErrors};

pub fn run(config: &crate::Config, data: &AuthenticateDTO) -> Result<bool, DTOErrors> {
    let mut validation = Validation { iss: Some(config.domain.to_string()), sub: Some(config.subject.to_string()), ..Default::default()};
    validation.set_audience(&config.app_name.to_string());
    let token_verification = decode::<Claims>(&data.token, config.secret.as_ref(), &validation);
    match token_verification {
        Ok(_) => return Ok(true),
        Err(_) => return Err(DTOErrors::ApplicationError("Incorrect token.".to_string()))
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
    fn invalid_token() {}

    #[test]
    fn success() {}
}
