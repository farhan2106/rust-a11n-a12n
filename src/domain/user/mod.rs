pub mod update_password;
pub mod authenticate;
pub mod sign_up;
pub mod sign_in;
pub mod sign_up_without_password;
pub mod identity_check;
pub mod forgot_my_password;

use validator::{Validate, ValidationErrors};
use serde::ser::{Serialize, Serializer};
use bcrypt::{verify};
use rand::Rng;
use rand::thread_rng;
use rand::distributions::Alphanumeric;
use std::env::temp_dir;
use lettre::file::FileTransport;
use lettre_email::Email;
use lettre::{Transport, SmtpClient};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
/// https://tools.ietf.org/html/rfc7519#section-4.1
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String, // domain name
    aud: String, // this service name i.e. user-service OR the application name that will be using this JWT. THe client must verify this string, if not the same then reject token
    sub: String, // subject
    exp: i64,
    username: String,
    email : String
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct SignUpWithoutPasswordDTO {
    #[validate(length(min = 2))]
    pub username: String,

    #[validate(email)]
    pub email: String
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct SignUpDTO {
    #[validate(length(min = 2))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6))]
    pub password: String
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct SignInDTO {  
    #[validate(length(min = 1))]
    pub username_or_email: String,

    #[validate(length(min = 6))]
    pub password: String
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct AuthenticateDTO {  
    #[validate(length(min = 1))]
    pub token: String,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct UpdatePasswordDTO {  
    #[validate(length(min = 1))]
    pub token: String,

    #[validate(length(min = 6))]
    pub password: String
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct IdentityCheckDTO {  
    #[validate(length(min = 1))]
    pub identity: String
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct ForgotMyPasswordDTO {
    #[validate(length(min = 1))]
    pub username_or_email: String
}

#[derive(PartialEq, Debug)]
pub enum DTOErrors {
    ValidationError(ValidationErrors),
    ApplicationError(String),
    DatabaseError(String)
}

impl Serialize for DTOErrors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DTOErrors::ValidationError(ref s) => serializer.serialize_newtype_variant("E", 0, "validation", s),
            DTOErrors::ApplicationError(ref s) => serializer.serialize_newtype_variant("E", 0, "application", s),
            DTOErrors::DatabaseError(ref s) => serializer.serialize_newtype_variant("E", 0, "database", s)
        }
    }
}

fn generate_random(count: usize) -> String {
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .take(count)
        .collect();
}

fn send_email(config: &crate::Config, to: &String, username: &String, message: &str) {
    let subject = format!("Hi, {}. {}", username, message);
    let subject_html = format!("<h2>Hi, {}.</h2>", username);
    let email = Email::builder()
        .to((to, username))
        .from(config.sender_email.to_owned())
        .subject(subject)
        .alternative(subject_html, message)
        .build()
        .unwrap();

    // Write to the local temp directory
    if config.rust_env == "development" {
        let mut sender = FileTransport::new(temp_dir());
        match sender.send(email.into()) {
            Ok(_) => println!("Successfully sent email to {}.", to),
            Err(_) => println!("Unable to send email to {}.", to),
        }
    } else {
        // Connect to a remote server on a custom port
        let mut sender = SmtpClient::new_simple(&config.smtp_server).unwrap()
            // Set the name sent during EHLO/HELO, default is `localhost`
            .hello_name(ClientId::Domain(config.domain.to_owned()))
            // Add credentials for authentication
            .credentials(Credentials::new(config.smtp_user.to_owned(), config.smtp_pass.to_owned()))
            // Enable SMTPUTF8 if the server supports it
            .smtp_utf8(true)
            // Configure expected authentication mechanism
            .authentication_mechanism(Mechanism::Plain)
            // Enable connection reuse
            .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).transport();
        match sender.send(email.into()) {
            Ok(_) => println!("Successfully sent email to {}.", to),
            Err(_) => println!("Unable to send email to {}.", to),
        }
    }

}
