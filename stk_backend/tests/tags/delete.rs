use crate::*;

fn create_tags(pool: &DbPool, n: u16) -> Vec<Tag> {
    common::create_test_objects::<Tag>(pool, n, get_tag_default)
}

#[actix_web::test]
async fn test_delete_tag() {
    let (app, pool) = get_app().await;

    // Gets id from a new tag. (Default data has spaces, so it has to be replaced to %20)
    let created = create_tags(&pool, 1).pop().unwrap().id.replace(" ", "%20");

    // Should return a succes message.
    let req = test::TestRequest::default()
        .method(Method::DELETE)
        .uri(&format!("/tags/{created}"))
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Tag deleted successfully");

    // Gets tags, it should be an empty vector.
    let empty: Vec<Tag> = vec![];
    common::expect_n_elements::<Tag>(&app, "/tags", empty).await;
}

#[actix_web::test]
async fn test_delete_tags_not_found() {
    let (app, _) = get_app().await;

    // Should return a succes message.
    let req = test::TestRequest::default()
        .method(Method::DELETE)
        .uri(&format!("/tags/id-not-found"))
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Tag not found");
}
