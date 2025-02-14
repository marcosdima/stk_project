use diesel::{r2d2::{self, ConnectionManager}, SqliteConnection};
use diesel_migrations::{self, embed_migrations, EmbeddedMigrations, MigrationHarness};
use stk_backend::DbPool;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn init_test_db_pool() -> DbPool {
    let conn_spec = ":memory:";
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool");

    let conn = &mut pool.get().unwrap();
    run_migrations(conn);

    pool
}

pub fn run_migrations(conn: &mut SqliteConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}
