use actix_http::{
    header::{
        HeaderName,
        HeaderValue,
        AUTHORIZATION,
        CONTENT_TYPE,
    },
    Method
};

use actix_web::{
    dev::{
        Service,
        ServiceResponse,
    },
    test,
    web::Bytes,
    Error,
};

use stk_backend::routes::DbPool;

use actix_http::Request;

use crate::get_admin_token;

pub async fn basic_request(
    app: &impl Service<Request, Response = ServiceResponse, Error = Error>,
    route: &str,
    method: Method,
    headers: Vec<(HeaderName, HeaderValue)>,
    data: impl Into<Bytes>,
) -> ServiceResponse {
    // Set method and uri.
    let mut req = test::TestRequest::default()
        .method(method)
        .uri(route);

    // Set headers.
    for header in headers {
        req = req.insert_header(header);
    }

    // Set payload.
    req = req.set_payload(data);
    
    test::call_service(app, req.to_request()).await
}

pub fn get_admin_token_header(pool: &DbPool) -> (HeaderName, HeaderValue) {
    get_token_header(get_admin_token(pool))
}

pub fn get_token_header(token: String) -> (HeaderName, HeaderValue) {
    (
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))
            .expect("Test Error: Header Value parse error"),
    )
}


pub fn get_json_header() -> (HeaderName, HeaderValue) {
    (
        CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    )
}

pub fn get_plain_text_header() -> (HeaderName, HeaderValue) {
    (
        CONTENT_TYPE,
        HeaderValue::from_static("text/plain"),
    )
}
