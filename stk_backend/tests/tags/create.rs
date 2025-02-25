use crate::*;

#[actix_web::test]
async fn test_create_tag() {
    let (app, pool) = get_app().await;

    let new_tag_data = get_tag_default(1);

    // Creates a tag.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/tags"))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_tag_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Gets the new tag.
    let body = test::read_body(resp).await;
    let new_tag: Tag = serde_json::from_slice(&body).unwrap();

    assert_eq!(Tag::get_all(&pool).unwrap(), vec![new_tag])
}
