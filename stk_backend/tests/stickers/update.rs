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
    );

    // Updates sticker
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    let resp = basic_request(
        &app,
        "/stickers/update",
        Method::PUT,
        headers,
        serde_json::to_string(&updated_sticker_data).unwrap(),
    ).await;
    
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
    let (app, pool) = get_app().await;

    let updated_sticker_data = StickerUpdate::new(
        Uuid::new_v4().to_string(),
        String::from("NEW"),
        String::from("www.updated-url.com")
    );

    // Updates sticker
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    let resp = basic_request(
        &app,
        "/stickers/update",
        Method::PUT,
        headers,
        serde_json::to_string(&updated_sticker_data).unwrap(),
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Sticker with id provided does not exist!"), resp).await;
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
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    let resp = basic_request(
        &app,
        "/stickers/update",
        Method::PUT,
        headers,
        serde_json::to_string(&updated_sticker_data).unwrap(),
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Sticker with id provided does not exist!"), resp).await;

    common::expect_n_elements(
        &app,
        "/stickers",
        vec![new_sticker]
    ).await;
}
