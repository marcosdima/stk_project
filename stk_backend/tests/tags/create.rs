use crate::*;

#[actix_web::test]
async fn test_create_tag() {
    let (app, pool) = get_app().await;

    let new_tag_data = get_tag_default(1);

    // Creates a tag.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];

    let resp = basic_request(
        &app,
        "/tags",
        Method::POST,
        headers,
        serde_json::to_string(&new_tag_data).unwrap(),
    ).await;

    assert!(resp.status().is_success());

    // Gets the new tag.
    let body = test::read_body(resp).await;
    let new_tag: Tag = serde_json::from_slice(&body).unwrap();

    assert_eq!(Tag::get_all(&pool).unwrap(), vec![new_tag])
}

#[actix_web::test]
async fn test_create_tag_empty_name() {
    let (app, pool) = get_app().await;

    let new_tag_data = NewTag::new("".to_owned());

    // Creates a tag.
    let headers = vec![
        get_random_token_header(&pool),
        get_json_header(),
    ];

    let resp = basic_request(
        &app,
        "/tags",
        Method::POST,
        headers,
        serde_json::to_string(&new_tag_data).unwrap(),
    ).await;

    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_create_tag_but_no_role() {
    let (app, pool) = get_app().await;

    let new_tag_data = get_tag_default(1);

    let headers = vec![
        get_random_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/tags",
        Method::POST,
        headers,
        serde_json::to_string(&new_tag_data).unwrap(),
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::RoleNeeded, resp).await;
}

#[actix_web::test]
async fn test_create_tag_but_no_token() {
    let (app, _) = get_app().await;

    let new_tag_data = get_tag_default(1);

    let headers = vec![
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/tags",
        Method::POST,
        headers,
        serde_json::to_string(&new_tag_data).unwrap(),
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::InvalidToken, resp).await;
}
