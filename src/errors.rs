use actix_web::{
    error::{BlockingError, ResponseError},
    Error as ActixError, HttpResponse,
};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Display)]
pub enum AuthError {
    #[display(fmt = "DuplicateValue: {}", _0)]
    DuplicateValue(String),

    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),

    #[display(fmt = "AuthenticationError: {}", _0)]
    AuthenticationError(String),

    #[display(fmt = "GenericError: {}", _0)]
    GenericError(String),
}


impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {

            AuthError::NotFound(ref message) => HttpResponse::NotFound().json(message),

            AuthError::AuthenticationError(ref message) => HttpResponse::Unauthorized().json(message),

            AuthError::DuplicateValue(ref message) => HttpResponse::BadRequest().json(message),

            AuthError::GenericError(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}

impl From<DBError> for AuthError {
    fn from(error: DBError) -> AuthError {
        match error {
            DBError::DatabaseError(kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();

                match kind {
                    DatabaseErrorKind::UniqueViolation => AuthError::DuplicateValue(message),
                    _ => AuthError::GenericError(message)
                }
            }
            _ => AuthError::GenericError(String::from("Some database error occured")),
        }
    }
}


#[derive(Debug, Display, PartialEq)]
pub enum Error {
    BadRequest(String),
    InternalServerError(String),
    Unauthorized,
    Forbidden,
    NotFound(String),
    BlockingError(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::BadRequest(message) => {
                let error: ErrorResponse = message.into();
                HttpResponse::BadRequest().json(error)
            }
            Error::NotFound(message) => {
                let error: ErrorResponse = message.into();
                HttpResponse::NotFound().json(error)
            }
            Error::Forbidden => {
                let error: ErrorResponse = "Forbidden".into();
                HttpResponse::Forbidden().json(error)
            }
            _ => {
                let error: ErrorResponse = "Internal Server Error".into();
                HttpResponse::InternalServerError().json(error)
            }
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
}

impl From<&str> for ErrorResponse {
    fn from(error: &str) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

impl From<Vec<String>> for ErrorResponse {
    fn from(error: Vec<String>) -> Self {
        ErrorResponse { errors: error }
    }
}

impl From<DBError> for Error {
    fn from(error: DBError) -> Error {
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::BadRequest(message);
                }
                Error::InternalServerError("Unknown database error".into())
            }
            DBError::NotFound => Error::NotFound("Record not found".into()),
            _ => Error::InternalServerError("Unknown database error".into()),
        }
    }
}

impl From<BlockingError> for Error {
    fn from(error: BlockingError) -> Error {
        Error::BlockingError("Thread blocking error".into())
    }
}

impl From<ActixError> for Error {
    fn from(error: ActixError) -> Error {
        Error::InternalServerError(error.to_string())
    }
}
