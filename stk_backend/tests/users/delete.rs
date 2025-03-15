use crate::*;

fn create_users(pool: &DbPool, n: u16) -> Vec<User> {
    common::create_test_objects::<User>(pool, n, get_user_default)
}

#[actix_web::test]
async fn test_delete_user() {
    let (app, pool) = get_app().await;

    // Gets id from a new user.
    let created = create_users(&pool, 1).pop().unwrap().id;

    // Should return a succes message.
    let headers = vec![
        get_admin_token_header(&pool),
        get_plain_text_header(),
    ];
    
    let resp = basic_request(
        &app,
        &format!("/users/{created}"),
        Method::DELETE,
        headers,
        "",
    ).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "User deleted successfully");

    // Gets user, it shouldn't exists.
    assert!(User::get_by_id(&pool, created).is_err());
}

#[actix_web::test]
async fn test_delete_user_not_found() {
    let (app, pool) = get_app().await;

    // Should return a succes message.
    let headers = vec![
        get_admin_token_header(&pool),
        get_plain_text_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/users/id-not-found",
        Method::DELETE,
        headers,
        "",
    ).await;

    expect_error(AppError::NotFound("User not found"), resp).await;
}

#[actix_web::test]
async fn test_delete_user_unauthorized() {
    let (app, pool) = get_app().await;

    // Gets id from a new user.
    let created = create_users(&pool, 1).pop().unwrap().id;

    // Should return a succes message.
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
async fn test_delete_user_forbbiden() {
    let (app, pool) = get_app().await;

    // Gets id from a new user.
    let created = create_users(&pool, 1).pop().unwrap().id;

    // Should return a succes message.
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