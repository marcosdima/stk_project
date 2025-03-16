use crate::*;

fn create_users(pool: &DbPool, n: u16) -> Vec<User> {
    common::create_test_objects::<User>(pool, n, get_user_default)
}

#[actix_web::test]
async fn test_assign_role() {
    let (app, pool) = get_app().await;

    // Gets id from a new user.
    let created = create_users(&pool, 1).pop().unwrap().id;

    let assign = NewUserRole::new(created.clone(), 1);

    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/users/role",
        Method::POST,
        headers,
        serde_json::to_string(&assign).unwrap(),
    ).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Role assigned");

    // Gets user, it shouldn't exists.
    assert!(UserRole::get_user_role(&pool, created).is_ok());
}

#[actix_web::test]
async fn test_assign_but_role_not_found() {
    let (app, pool) = get_app().await;

    let created = create_users(&pool, 1).pop().unwrap().id;
    let assign = NewUserRole::new(created.clone(), 666);

    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/users/role",
        Method::POST,
        headers,
        serde_json::to_string(&assign).unwrap(),
    ).await;

    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_assign_but_user_not_found() {
    let (app, pool) = get_app().await;

    let assign = NewUserRole::new(Uuid::new_v4().to_string(), 1);

    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/users/role",
        Method::POST,
        headers,
        serde_json::to_string(&assign).unwrap(),
    ).await;

    expect_error(AppError::NotFound("User with id provided does not exist!"), resp).await;
}

#[actix_web::test]
async fn test_assign_role_unauthorized() {
    let (app, pool) = get_app().await;

    // Gets id from a new user.
    let created = create_users(&pool, 1).pop().unwrap().id;

    let headers = vec![
        get_plain_text_header(),
    ];
    
    let resp = basic_request(
        &app,
        &format!("/users/{created}"),
        Method::DELETE,
        headers,
        "",
    ).await;

    expect_error(AppError::InvalidToken, resp).await;
}

#[actix_web::test]
async fn test_assign_role_forbbiden() {
    let (app, pool) = get_app().await;

    // Gets id from a new user.
    let created = create_users(&pool, 1).pop().unwrap().id;
    
    let headers = vec![
        get_token_header(generate_token(&created).unwrap()),
        get_plain_text_header(),
    ];

    let resp = basic_request(
        &app,
        &format!("/users/{created}"),
        Method::DELETE,
        headers,
        "",
    ).await;

    expect_error(AppError::RoleNeeded, resp).await;
}
