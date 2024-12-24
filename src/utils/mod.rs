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