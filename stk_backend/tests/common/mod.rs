mod setup;
pub mod admin;
pub mod default;
pub mod requests;

use actix_http::Request;

pub use setup::{
    get_app,
    get_just_app,
    get_just_pool,
};

use actix_web::{
    dev::{
        Service,
        ServiceResponse
    },
    http::header::ContentType,
    test,
    Error,
};

use serde::Deserialize;

use stk_backend::{
    errors::AppError,
    models::BasicModel,
    routes::DbPool,
};

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

pub async fn expect_error(
    err: AppError,
    resp: ServiceResponse,
) {
    assert_eq!(err.message(), test::read_body(resp).await)
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
