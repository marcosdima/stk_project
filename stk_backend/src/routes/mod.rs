use diesel::{r2d2, SqliteConnection};

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