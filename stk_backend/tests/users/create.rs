use crate::*;

#[actix_web::test]
async fn test_create_user() {
    let (app, _) = get_app().await;

    let new_user_data = get_user_default(1);

    // Creates a user.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/users"))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_user_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Gets the new user.
    let body = test::read_body(resp).await;
    let new_user: User = serde_json::from_slice(&body).unwrap();

    common::expect_n_elements(&app, "/users", vec![new_user]).await;
}
