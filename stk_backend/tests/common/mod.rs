pub mod default;

use actix_http::Request;
use actix_web::{
    dev::{
        Service,
        ServiceResponse
    }, http::header::ContentType, test, web::{self, Data}, App, Error
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
use serde::Deserialize;
use stk_backend::{
    models::BasicModel,
    routes::DbPool,
};
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

pub fn create_test_objects<T: BasicModel>(
    pool: &DbPool,
    n: u16,
    default_data: impl Fn(u16) -> T::NewT,
) -> Vec<T> {
    let mut res: Vec<T> = vec![];
    for id in 1..n + 1 {
        res.push(
            T::create(
                pool,
                default_data(id),
            ).unwrap()
        );
    }
    res
}

pub async fn parse_response<T: BasicModel + for<'a> Deserialize<'a>>(resp: ServiceResponse) -> Vec<T> {
    test::read_body_json(resp).await
}

pub async fn get_element<T: BasicModel>(
    app: &impl Service<Request, Response = ServiceResponse, Error = Error>, 
    route: &str,
) -> T {
    let req = test::TestRequest::default()
        .uri(route)
        .insert_header(ContentType::plaintext())
        .to_request();

    let resp = test::call_service(app, req).await;

    test::read_body_json(resp).await
}

pub async fn get_elements<T: BasicModel>(
    app: &impl Service<Request, Response = ServiceResponse, Error = Error>, 
    route: &str,
) -> Vec<T> {
    let req = test::TestRequest::default()
        .uri(route)
        .insert_header(ContentType::plaintext())
        .to_request();

    let resp = test::call_service(app, req).await;

    parse_response::<T>(resp).await
}

pub async fn expect_n_elements<T: BasicModel>(
    app: &impl Service<Request, Response = ServiceResponse, Error = Error>, 
    route: &str,
    expected: Vec<T>
) {
    let categories = get_elements(app, route).await;
    assert_eq!(expected, categories);
}

pub async fn get_app() -> (impl Service<Request, Response = ServiceResponse, Error = Error>, Data<DbPool>) {
    let pool = web::Data::new(init_test_db_pool());

    let app = test::init_service(
        App::new()
            .app_data(pool.clone())
            .configure(stk_backend::routes::stickers::configure)
            .configure(stk_backend::routes::categories::configure)
            .configure(stk_backend::routes::tags::configure)
    ).await;

    (app, pool)
}
