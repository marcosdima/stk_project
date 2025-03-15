use actix_web::{
    HttpResponse,
    ResponseError,
};

use diesel::{
    r2d2::{
        self,
        ConnectionManager,
        PooledConnection
    },
    RunQueryDsl,
    SqliteConnection
};

use crate::errors::AppError;

pub mod stickers;
pub mod hello;
pub mod categories;
pub mod tags;
pub mod artists;
pub mod users;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

pub fn get_connection_from_pool(pool: &DbPool) -> PooledConnection<ConnectionManager<SqliteConnection>> {
    let mut conn = pool
        .get()
        .expect("Failed to get connection from pool");

    // Activates FK.
    diesel::sql_query("PRAGMA foreign_keys = ON;")
        .execute(&mut conn)
        .expect("Failed to enable foreign key support");
    
    conn
}

pub fn default_match_error(
    err: AppError,
) -> HttpResponse {
    println!("Error: {}", err.message());
    match err {
        AppError::InvalidData(err) => HttpResponse::BadRequest().body(format!("{err}")),
        AppError::NotFound(err) => HttpResponse::NotFound().body(format!("{err}")),
        AppError::DieselError(err) => HttpResponse::BadRequest().body(format!("{err}")),
        AppError::JSONWebTokenError(err) => HttpResponse::BadRequest().body(format!("{err}")),
        _ => err.error_response(),
    }
}