use crate::*;

fn create_stickers(pool: &DbPool, n: u16) -> Vec<Sticker> {
    common::create_test_objects::<Sticker>(pool, n, get_sticker_default)
}

#[actix_web::test]
async fn test_delete_stickers() {
    let pool = web::Data::new(common::init_test_db_pool());

    let app = test::init_service(
        App::new()
            .app_data(pool.clone())
            .configure(stk_backend::routes::stickers::configure)
    ).await;

    // Gets id from a new sticker.
    let created = create_stickers(&pool, 1).pop().unwrap().id;

    // Should return a succes message.
    let req = test::TestRequest::default()
        .method(Method::DELETE)
        .uri(&format!("/stickers/{created}"))
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Sticker deleted successfully");

    // Gets stickers, it should be an empty vector.
    let empty: Vec<Sticker> = vec![];
    common::expect_n_elements::<Sticker>(&app, "/stickers", empty).await;
}

#[actix_web::test]
async fn test_delete_stickers_not_found() {
    let pool = web::Data::new(common::init_test_db_pool());

    let app = test::init_service(
        App::new()
            .app_data(pool.clone())
            .configure(stk_backend::routes::stickers::configure)
    ).await;

    // Should return a succes message.
    let req = test::TestRequest::default()
        .method(Method::DELETE)
        .uri(&format!("/stickers/id-not-found"))
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Sticker not found");

}
