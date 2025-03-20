use crate::*;

#[actix_web::test]
async fn test_create_sticker() {
    let (app, pool) = get_app().await;

    let new_sticker_data = get_sticker_default(1);

    // Creates a sticker.    
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    let resp = basic_request(
        &app,
        "/stickers",
        Method::POST,
        headers,
        serde_json::to_string(&new_sticker_data).unwrap(),
    ).await;
    assert!(resp.status().is_success());

    // Gets the new sticker.
    let body = test::read_body(resp).await;
    let new_sticker: Sticker = serde_json::from_slice(&body).unwrap();

    common::expect_n_elements(&app, "/stickers", vec![new_sticker]).await;
}
