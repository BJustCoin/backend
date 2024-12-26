use argonautica::{Hasher, Verifier};
use actix_web::{
  http::header::CONTENT_TYPE,
  HttpRequest,
};
use crate::{errors::AuthError, vars};
use crate::models::User;


pub fn hash_password(password: &str) -> String {
  Hasher::default() 
      .with_password(password)
      .with_secret_key(vars::secret_key().as_str())
      .hash()
      .expect("E.")
      //.map_err(|_| AuthError::AuthenticationError(String::from("Не удалось хэшировать пароль")))
}

pub fn verify(hash: &str, password: &str) -> Result<bool, AuthError> {
  Verifier::default()
      .with_hash(hash)
      .with_password(password)
      .with_secret_key(vars::secret_key().as_str())
      .verify()
      .map_err(|_| AuthError::AuthenticationError(String::from("Не удалось подтвердить пароль")))
}

pub fn is_json_request(req: &HttpRequest) -> bool {
    req
      .headers()
      .get(CONTENT_TYPE)
      .map_or(
        false,
        |header| header.to_str().map_or(false, |content_type| "application/json" == content_type)
      )
}
fn get_secret<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("secret")?.to_str().ok();
}

pub fn is_signed_in(req: &HttpRequest) -> bool {
  get_secret(&req).is_some()
}

pub fn get_current_user(req: &HttpRequest) -> User {
    let uuid = get_secret(&req).unwrap();
    let _connection = establish_connection();
    return schema::users::table
        .filter(schema::users::uuid.eq(uuid))
        .first::<User>(&_connection)
        .expect("Error.");
}
