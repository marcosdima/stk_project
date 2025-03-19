use crate::*;

fn create_tags(pool: &DbPool, n: u16) -> Vec<Tag> {
    common::create_test_objects::<Tag>(pool, n, get_tag_default)
}

#[actix_web::test]
async fn test_change_tag_name() {
    let (app, pool) = get_app().await;

    let new_tag_id = create_tags(&pool, 1).pop().unwrap().id;

    let updated_tag_data = TagUpdate::new(new_tag_id.clone(), String::from("Updated Tag"));

    // Updates tag
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];

    let resp = basic_request(
        &app,
        &format!("/tags/{new_tag_id}/update"),
        Method::PUT,
        headers,
        serde_json::to_string(&updated_tag_data).unwrap(),
    ).await;

    assert!(resp.status().is_success());

    common::expect_n_elements(
        &app,
        "/tags",
        vec![Tag { id: new_tag_id, name: updated_tag_data.name }]
    ).await;
}

#[actix_web::test]
async fn test_update_tag_not_found() {
    let (app, pool) = get_app().await;

    let new_tag = create_tags(&pool, 1).first().unwrap().to_owned();
    let updated_tag_data = TagUpdate::new(new_tag.id, String::from("NEW"));

    // Updates tag
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];

    let resp = basic_request(
        &app,
        &format!("/tags/{}/update", Uuid::new_v4()),
        Method::PUT,
        headers,
        serde_json::to_string(&updated_tag_data).unwrap(),
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Tag with id provided does not exist!"), resp).await;
}

#[actix_web::test]
async fn test_update_tag_but_no_role() {
    let (app, pool) = get_app().await;

    let created = create_tags(&pool, 1).pop().unwrap().id;

    let headers = vec![
        get_random_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        &format!("/tags/{created}/update"),
        Method::DELETE,
        headers,
        "",
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::RoleNeeded, resp).await;
}

#[actix_web::test]
async fn test_update_tag_but_no_token() {
    let (app, pool) = get_app().await;

    let created = create_tags(&pool, 1).pop().unwrap().id;

    let headers = vec![
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        &format!("/tags/{created}/update"),
        Method::DELETE,
        headers,
        "",
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::InvalidToken, resp).await;
}
