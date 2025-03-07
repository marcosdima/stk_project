use crate::*;

fn create_users(pool: &DbPool, n: u16) -> Vec<User> {
    common::create_test_objects::<User>(pool, n, get_user_default)
}

#[actix_web::test]
async fn test_delete_users() {
    let (app, pool) = get_app().await;

    // Gets id from a new user.
    let created = create_users(&pool, 1).pop().unwrap().id;

    // Should return a succes message.
    let req = test::TestRequest::default()
        .method(Method::DELETE)
        .uri(&format!("/users/{created}"))
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "User deleted successfully");

    // Gets users, it should be an empty vector.
    let empty: Vec<User> = vec![];
    common::expect_n_elements::<User>(&app, "/users", empty).await;
}

#[actix_web::test]
async fn test_delete_users_not_found() {
    let (app, _) = get_app().await;

    // Should return a succes message.
    let req = test::TestRequest::default()
        .method(Method::DELETE)
        .uri(&format!("/users/id-not-found"))
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
    let body = test::read_body(resp).await;
    assert_eq!(body, "User not found");

}
