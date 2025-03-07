use crate::*;

fn create_users(pool: &DbPool, n: u16) -> Vec<User> {
    common::create_test_objects::<User>(pool, n, get_user_default)
}

#[actix_web::test]
async fn test_update_user() {
    let (app, pool) = get_app().await;

    let new_user = create_users(&pool, 1).pop().unwrap();
    let new_name = "NEW name";
    let new_lastname = "NEW lastname";
    let new_username = "NEW username";
    let new_pass = "NEW pass";

    let updated_user_data = UserUpdate::new(
        new_user.id.clone(),
        new_name.to_owned(),
        new_lastname.to_owned(),
        new_username.to_owned(),
        new_pass.to_owned(),
    );

    // Updates user
    let req = test::TestRequest::default()
        .method(Method::PUT)
        .uri("/users")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&updated_user_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());

    common::expect_n_elements(
        &app,
        "/users",
        vec![
            User {
                id: updated_user_data.id.to_string(),
                name: new_name.to_owned(),
                lastname: new_lastname.to_owned(),
                username: new_username.to_owned(),
                password_hash: new_pass.to_owned(),
            }
        ]
    ).await;
}

#[actix_web::test]
async fn test_update_user_not_found() {
    let (app, _) = get_app().await;

    let new_name = "NEW name";
    let new_lastname = "NEW lastname";
    let new_username = "NEW username";
    let new_pass = "NEW pass";

    let updated_user_data = UserUpdate::new(
        Uuid::new_v4().to_string(),
        new_name.to_owned(),
        new_lastname.to_owned(),
        new_username.to_owned(),
        new_pass.to_owned(),
    );

    // Updates user
    let req = test::TestRequest::default()
        .method(Method::PUT)
        .uri("/users")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&updated_user_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_update_user_wrong_id() {
    let (app, pool) = get_app().await;

    let new_user = create_users(&pool, 1).pop().unwrap();
    let new_name = "NEW name";
    let new_lastname = "NEW lastname";
    let new_username = "NEW username";
    let new_pass = "NEW pass";

    let updated_user_data = serde_json::json!({
        "id": "wrong-id",
        "name": new_name,
        "lastname": new_lastname,
        "username": new_username,
        "password_hash": new_pass,
    });

    // Updates user
    let req = test::TestRequest::default()
        .method(Method::PUT)
        .uri("/users")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&updated_user_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());

    common::expect_n_elements(
        &app,
        "/users",
        vec![new_user]
    ).await;
}
