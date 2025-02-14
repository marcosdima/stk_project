#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(
            App::new().configure(stk_backend::routes::hello::configure)
        ).await;

        let req = test::TestRequest::default()
            .uri("/hello")
            .insert_header(ContentType::plaintext())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello world!");
    }
}
