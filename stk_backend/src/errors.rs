use thiserror::Error;
use diesel::{r2d2::Error as R2R2Error, result::Error as DieselError};

#[derive(Debug, Error)]
pub enum AppError<'a> {
    #[error("Database error: {0}")]
    DieselError(#[from] DieselError),

    #[error("R2R2 error: {0}")]
    R2R2Error(#[from] R2R2Error),

    #[error("Element not found")]
    NotFound(&'a str),

    #[error("This should not happen, wierd...")]
    UnexpectedError(&'a str),

    #[error("Data provided can not be used")]
    InvalidData(&'a str),
}