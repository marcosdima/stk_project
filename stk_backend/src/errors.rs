use thiserror::Error;
use diesel::{r2d2::Error as R2R2Error, result::Error as DieselError};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DieselError(#[from] DieselError),

    #[error("R2R2 error: {0}")]
    R2R2Error(#[from] R2R2Error),

    #[error("Element not found")]
    NotFound(&'static str),

    #[error("This should not happen, wierd...")]
    UnexpectedError(&'static str),

    #[error("Data provided can not be used")]
    InvalidData(&'static str),
}

impl AppError {
    pub fn message(&self) -> String {
        match self {
            Self::DieselError(err) => err.to_string(),
            Self::R2R2Error(err) => err.to_string(),
            Self::NotFound(msg) | Self::UnexpectedError(msg) | Self::InvalidData(msg) => msg.to_string(),
        }
    }
}