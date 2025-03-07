use crate::*;

fn create_users(pool: &DbPool, n: u16) -> Vec<User> {
    common::create_test_objects::<User>(pool, n, get_user_default)
}

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

#[actix_web::test]
async fn test_create_user_invalid_fields() {
    let (app, pool) = get_app().await;

    let new_user_data = create_users(&pool, 1).first().unwrap().to_owned();
    let updated_user_data = serde_json::json!({
        "id": new_user_data.id,
        "name": new_user_data.name,
        "lastname": new_user_data.lastname,
        "username": "Another UserName",
        "password_hash": new_user_data.lastname,
    });
    let keys = vec!["name", "lastname", "username"];

    for key in keys {
        let mut key_updated_data = updated_user_data.clone();
        key_updated_data[key] = serde_json::json!("");

        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri(&format!("/users"))
            .insert_header(ContentType::json())
            .set_payload(serde_json::to_string(&key_updated_data).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}