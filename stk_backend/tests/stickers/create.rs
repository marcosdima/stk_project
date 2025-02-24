use crate::*;

#[actix_web::test]
async fn test_create_sticker() {
    let pool = web::Data::new(common::init_test_db_pool());

    let app = test::init_service(
        App::new()
            .app_data(pool.clone())
            .configure(stk_backend::routes::stickers::configure)
    ).await;

    let new_sticker_data = get_sticker_default(1);

    // Creates a sticker.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/stickers"))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_sticker_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Gets the new sticker.
    let body = test::read_body(resp).await;
    let new_sticker: Sticker = serde_json::from_slice(&body).unwrap();

    common::expect_n_elements(&app, "/stickers", vec![new_sticker]).await;
}
