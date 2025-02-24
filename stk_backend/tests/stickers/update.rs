use crate::*;

fn create_stickers(pool: &DbPool, n: u16) -> Vec<Sticker> {
    common::create_test_objects::<Sticker>(pool, n, get_sticker_default)
}

#[actix_web::test]
async fn test_update_sticker() {
    let (app, pool) = get_app().await;

    let new_sticker = create_stickers(&pool, 1).pop().unwrap();
    let new_label = "NEW";
    let new_url = "www.updated-url.com";

    let updated_sticker_data = StickerUpdate::new(
        new_sticker.id.clone(),
        String::from(new_label),
        String::from(new_url)
    ).unwrap();

    // Updates sticker
    let req = test::TestRequest::default()
        .method(Method::PUT)
        .uri("/stickers")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&updated_sticker_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    common::expect_n_elements(
        &app,
        "/stickers",
        vec![
            Sticker {
                id: updated_sticker_data.id.to_string(),
                label: new_label.to_owned(),
                url: new_url.to_owned(),
            }
        ]
    ).await;
}

#[actix_web::test]
async fn test_update_sticker_not_found() {
    let (app, _) = get_app().await;

    let updated_sticker_data = StickerUpdate::new(
        Uuid::new_v4().to_string(),
        String::from("NEW"),
        String::from("www.updated-url.com")
    ).unwrap();

    // Updates sticker
    let req = test::TestRequest::default()
        .method(Method::PUT)
        .uri("/stickers")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&updated_sticker_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_update_sticker_wrong_id() {
    let (app, pool) = get_app().await;

    let new_sticker = create_stickers(&pool, 1).pop().unwrap();
    let new_label = "NEW";
    let new_url = "www.updated-url.com";

    let updated_sticker_data = serde_json::json!({
        "id": "wrong-id",
        "label": new_label,
        "url": new_url
    });

    // Updates sticker
    let req = test::TestRequest::default()
        .method(Method::PUT)
        .uri("/stickers")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&updated_sticker_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());

    common::expect_n_elements(
        &app,
        "/stickers",
        vec![new_sticker]
    ).await;
}
