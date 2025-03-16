use crate::*;

#[actix_web::test]
async fn test_login() {
    let (app, _) = get_app().await;

    let new_user_data = get_user_default(1);

    // Creates a user.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/users")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_user_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Get user.
    let body = test::read_body(resp).await;
    let user: User = serde_json::from_slice(&body).unwrap();

    // Login.
    let credentials = serde_json::json!({
        "password": new_user_data.password_hash,
        "username": new_user_data.username,
    });

    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/users/login")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&credentials).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Get token.
    let body = test::read_body(resp).await;
    let token = String::from_utf8_lossy(&body).to_string();

    // Compare token claim with user id.
    let claim = stk_backend::utils::validate_token(&token).unwrap();
    assert_eq!(claim.sub, user.id);
}

#[actix_web::test]
async fn test_login_wrong_credentials() {
    let (app, _) = get_app().await;

    let new_user_data = get_user_default(1);
    let another_user = get_user_default(2);

    // Creates a user 1.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/users")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_user_data).unwrap())
        .to_request();
    let _ = test::call_service(&app, req).await;

    // Creates a user 2.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/users")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&another_user).unwrap())
        .to_request();
    let _ = test::call_service(&app, req).await;

    // Wrong password.
    let credentials = serde_json::json!({
        "password": "NOT_THE_PASSWORD",
        "username": new_user_data.username,
    });

    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/users/login")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&credentials).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());

    // Wrong user-password.
    let credentials = serde_json::json!({
        "password": new_user_data.password_hash,
        "username": another_user,
    });

    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/users/login")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&credentials).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_login_user_not_found() {
    let (app, _) = get_app().await;


    // Login.
    let credentials = serde_json::json!({
        "password": "123",
        "username": "does_not_exists",
    });

    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/users/login")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&credentials).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
