use actix_web::{
    dev::{
        Service,
        ServiceResponse
    },
    test,
    web::{
        self,
        Data
    },
    App,
    Error
};

use diesel::{
    r2d2::{
        self,
        ConnectionManager,
    },
    SqliteConnection,
};

use diesel_migrations::{
    self,
    embed_migrations,
    EmbeddedMigrations,
    MigrationHarness,
};

use stk_backend::routes::DbPool;

use actix_http::Request;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

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

pub async fn get_app() -> (impl Service<Request, Response = ServiceResponse, Error = Error>, Data<DbPool>) {
    let pool = web::Data::new(init_test_db_pool());

    let app = test::init_service(
        App::new()
            .app_data(pool.clone())
            .configure(stk_backend::routes::stickers::configure)
            .configure(stk_backend::routes::categories::configure)
            .configure(stk_backend::routes::tags::configure)
            .configure(stk_backend::routes::artists::configure)
            .configure(stk_backend::routes::users::configure)
    ).await;

    (app, pool)
}
