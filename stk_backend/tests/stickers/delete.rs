use crate::*;

fn create_stickers(pool: &DbPool, n: u16) -> Vec<Sticker> {
    common::create_test_objects::<Sticker>(pool, n, get_sticker_default)
}

#[actix_web::test]
async fn test_delete_stickers() {
    let (app, pool) = get_app().await;

    // Gets id from a new sticker.
    let created = create_stickers(&pool, 1).pop().unwrap().id;

    // Should return a succes message.
    let headers = vec![
        get_admin_token_header(&pool),
        get_plain_text_header(),
    ];
    let resp = basic_request(
        &app,
        &format!("/stickers/{created}/delete"),
        Method::DELETE,
        headers,
        "",
    ).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Sticker deleted successfully");

    // Gets stickers, it should be an empty vector.
    let empty: Vec<Sticker> = vec![];
    common::expect_n_elements::<Sticker>(&app, "/stickers", empty).await;
}

#[actix_web::test]
async fn test_delete_stickers_not_found() {
    let (app, pool) = get_app().await;

    // Should return a succes message.
    let headers = vec![
        get_admin_token_header(&pool),
        get_plain_text_header(),
    ];
    let resp = basic_request(
        &app,
        "/stickers/id-not-found/delete",
        Method::DELETE,
        headers,
        "",
    ).await;
    
    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Sticker not found"), resp).await;
}
