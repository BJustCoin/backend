mod auth;

pub use self::{
    auth::*,
};
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    dev::ConnectionInfo,
};
use crate::schema;
use serde::{Deserialize, Serialize};
use actix_session::Session;
use crate::errors::AuthError;
use diesel::{PgConnection, Connection};


#[derive(Deserialize, Serialize)]
pub struct NewUserForm {
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub password:   String,
}

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Deserialize, Serialize)]
pub struct EmailF {
    pub recipient_name:  String,
    pub recipient_email: String,
    pub subject:         String,
    pub text:            String,
}
struct EmailUser {
    name: String,
    email: String,
}

use reqwest::Client;
use reqwest::header;
use serde_json::json;


pub fn send_email(data: EmailF) -> i16 {
    dotenv::dotenv().ok();
    let api_key = std::env::var("EMAIL_KEY")
        .expect("EMAIL_KEY must be set");
    let sender = EmailUser {
        name: "BJustCoin Team".to_string(),
        email: "no-reply@bjustcoin.com".to_string(),
    };

    let recipient = EmailUser {
        name: data.recipient_name.clone(),
        email: data.recipient_email.clone(),
    };

    let body = json!(
        {
            "personalizations": [{
                "from": {
                    "email": sender.email,
                    "name": sender.name
                },
                "to": [{
                    "email": recipient.email,
                    "name": recipient.name
                }]
            }],
            "from": {
                "email": sender.email,
                "name": sender.name
            },
            "subject": data.subject.clone(),
            "content": [
                {
                    "type": "text/plain",
                    "value": data.text.clone()
                },
            ]
        }
    );
    let client = Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .json(&body)
        .bearer_auth(api_key)
        .header(
            header::CONTENT_TYPE, 
            header::HeaderValue::from_static("application/json")
        );

    let response = client.send();
    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
            //println!("Email sent!");
            return 1;
        },
        _ => {
            //println!(
            //    "Unable to send your email. Status code was: {}. Body content was: {:?}",
            //    response.status(),
            //    response.text()
            //),
            return 0;
        },
    }
}