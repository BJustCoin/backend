use argonautica::{Hasher, Verifier};
use actix_web::{
  http::header::CONTENT_TYPE,
  HttpRequest,
};
use crate::{errors::AuthError, vars};


pub fn hash_password(password: &str) -> String {
  Hasher::default() 
      .with_password(password)
      .with_secret_key(vars::secret_key().as_str())
      .hash()
      .expect("E.")
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
