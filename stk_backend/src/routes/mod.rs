use actix_web::HttpResponse;
use diesel::{r2d2, SqliteConnection};

use crate::errors::AppError;

pub mod stickers;
pub mod hello;
pub mod categories;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

pub fn default_match_error(
    err: AppError,
) -> HttpResponse {
    match err {
        AppError::InvalidData(err) => HttpResponse::BadRequest().body(format!("{err}")),
        AppError::NotFound(err) => HttpResponse::NotFound().body(format!("{err}")),
        _ => HttpResponse::InternalServerError().finish(),
    }
}