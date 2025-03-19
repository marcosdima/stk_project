use crate::*;

fn create_tags(pool: &DbPool, n: u16) -> Vec<Tag> {
    common::create_test_objects::<Tag>(pool, n, get_tag_default)
}

#[actix_web::test]
async fn test_delete_tag() {
    let (app, pool) = get_app().await;

    let created = create_tags(&pool, 1).pop().unwrap().id;

    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        &format!("/tags/{created}/delete"),
        Method::DELETE,
        headers,
        "",
    ).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Tag deleted successfully");

    // Gets tags, it should be an empty vector.
    let empty: Vec<Tag> = vec![];
    common::expect_n_elements::<Tag>(&app, "/tags", empty).await;
}

#[actix_web::test]
async fn test_delete_tags_not_found() {
    let (app, pool) = get_app().await;

    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];

    let resp = basic_request(
        &app,
        "/tags/id-not-found/delete",
        Method::DELETE,
        headers,
        "",
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Tag not found"), resp).await;

    // With UUID...
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];

    let resp = basic_request(
        &app,
        &format!("/tags/{}/delete", Uuid::new_v4()),
        Method::DELETE,
        headers,
        "",
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Tag not found"), resp).await;
}

#[actix_web::test]
async fn test_delete_tag_but_no_role() {
    let (app, pool) = get_app().await;

    let created = create_tags(&pool, 1).pop().unwrap().id;

    let headers = vec![
        get_random_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        &format!("/tags/{created}/delete"),
        Method::DELETE,
        headers,
        "",
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::RoleNeeded, resp).await;
}

#[actix_web::test]
async fn test_delete_tag_but_no_token() {
    let (app, pool) = get_app().await;

    let created = create_tags(&pool, 1).pop().unwrap().id;

    let headers = vec![
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        &format!("/tags/{created}/delete"),
        Method::DELETE,
        headers,
        "",
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::InvalidToken, resp).await;
}
