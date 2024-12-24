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
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl,
    Connection,
    PgConnection,
};
use crate::models::{SessionUser, User};


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


pub fn get_limit (
    limit: Option<i64>,
    default_limit: i64
) -> i64 {
    let _limit: i64;
    if limit.is_some() {
        let l_unwrap = limit.unwrap();
        if l_unwrap > 100 {
            _limit = default_limit;
        }
        else {
            _limit = l_unwrap;
        }
    }
    else {
        _limit = default_limit;
    }
    _limit
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

#[derive(Debug)]
struct EmailResp {
    status:  String,
}
pub async fn send_email(data: EmailF) -> bool {
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

    let body = json!({
            "personalizations": [{
                "from": {
                    "email": sender.email.clone(),
                    "name": sender.name.clone()
                },
                "to": [{
                    "email": recipient.email.clone(),
                    "name": recipient.name.clone()
                }]
            }],
            "from": {
                "email": sender.email.clone(),
                "name": sender.name.clone()
            },
            "subject": data.subject.clone(),
            "content": [
                {
                    "type": "text/plain",
                    "value": data.text.clone()
                },
            ]
        });
    let client = Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .json(&body)
        .bearer_auth(api_key)
        .header(
            header::CONTENT_TYPE, 
            header::HeaderValue::from_static("application/json")
        );

    let response = client.send();
    if response.await.is_ok() {
        println!("200");
        return true
    }
    else {
        println!("400");
        return false
    }
}

pub fn get_user(id: i32) -> User {
    let _connection = establish_connection();
    return schema::users::table
        .filter(schema::users::id.eq(id))
        .first::<User>(&_connection)
        .expect("Error.");
}

pub fn get_current_user(session: &Session) -> Result<User, AuthError> {
    let msg = "Не удалось извлечь пользователя из сеанса"; 

    let some_user = session
        .get::<String>("user")
        .map_err(|_| AuthError::AuthenticationError(String::from(msg)))
        .unwrap() 
        .map_or(
          Err(AuthError::AuthenticationError(String::from(msg))),
          |user| serde_json::from_str::<SessionUser>(&user).or_else(|_| Err(AuthError::AuthenticationError(String::from(msg))))
    );

    if some_user.is_err() {
        Err(AuthError::AuthenticationError(String::from("Error")))
    }
    else {
        let _user = some_user.expect("Error.");
        Ok(get_user(_user.id))
    }

}