use crate::*;

fn create_tags(pool: &DbPool, n: u16) -> Vec<Tag> {
    common::create_test_objects::<Tag>(pool, n, get_tag_default)
}

#[actix_web::test]
async fn test_change_tag_name() {
    let (app, pool) = get_app().await;

    let new_tag = create_tags(&pool, 1).pop().unwrap();

    let updated_tag_data = TagUpdate::new(new_tag.id.clone(), String::from("Updated Tag"));

    // Updates tag
    let req = test::TestRequest::default()
        .method(Method::PUT)
        .uri(&format!("/tags/{}", new_tag.name.replace(" ", "%20")))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&updated_tag_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    common::expect_n_elements(
        &app,
        "/tags",
        vec![Tag { id: new_tag.id, name: updated_tag_data.name }]
    ).await;
}

#[actix_web::test]
async fn test_update_tag_not_found() {
    let (app, pool) = get_app().await;

    let new_tag = create_tags(&pool, 1).first().unwrap().to_owned();
    let updated_tag_data = TagUpdate::new(new_tag.id, String::from("NEW"));

    // Updates tag
    let req = test::TestRequest::default()
        .method(Method::PUT)
        .uri("/tags/NOEXISTS")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&updated_tag_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_update_tag_wrong_id() {
    let (app, pool) = get_app().await;

    let new_tag = create_tags(&pool, 1).pop().unwrap();

    let updated_tag_data = serde_json::json!({
        "name": "",
    });

    // Updates tag
    let req = test::TestRequest::default()
        .method(Method::PUT)
        .uri("/tags")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&updated_tag_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());

    common::expect_n_elements(
        &app,
        "/tags",
        vec![new_tag]
    ).await;
}
