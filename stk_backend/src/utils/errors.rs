use thiserror::Error;

use diesel::{
    r2d2::Error as R2R2Error,
    result::Error as DieselError
};

use actix_web::{
    Error as ActixWebError,
    ResponseError,
    HttpResponse,
    http::StatusCode
};

use argon2::password_hash::Error as PasswordHashError;

use jsonwebtoken::errors::Error as JSONWebTokenError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DieselError(#[from] DieselError),

    #[error("R2R2 error: {0}")]
    R2R2Error(#[from] R2R2Error),

    #[error("PasswordHass error: {0}")]
    PasswordHashError(String),

    #[error("JSONWebTokenError error: {0}")]
    JSONWebTokenError(#[from] JSONWebTokenError),

    #[error("ActixWebError error: {0}")]
    ActixWebError(#[from] ActixWebError),

    #[error("Element not found")]
    NotFound(&'static str),

    #[error("This should not happen, wierd...")]
    UnexpectedError(&'static str),

    #[error("Data provided can not be used")]
    InvalidData(&'static str),

    #[error("Token provided is not valid")]
    InvalidToken,

    #[error("User does not have permissions needed")]
    Forbidden,
}

impl AppError {
    pub fn message(&self) -> String {
        match self {
            Self::NotFound(msg) |
            Self::UnexpectedError(msg) |
            Self::InvalidData(msg)
                => msg.to_string(),
            _ => format!("{}", self),
        }
    }
}

impl From<PasswordHashError> for AppError {
    fn from(err: PasswordHashError) -> Self {
        AppError::PasswordHashError(err.to_string())
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.message())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InvalidData(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidToken => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
