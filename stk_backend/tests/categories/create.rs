use crate::*;

#[actix_web::test]
async fn test_create_category() {
    let (app, pool) = get_app().await;

    let new_category_data = get_category_default(1);

    // Creates a category.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    let resp = basic_request(
        &app,
        "/categories",
        Method::POST,
        headers,
        serde_json::to_string(&new_category_data).unwrap(),
    ).await;

    assert!(resp.status().is_success());

    // Gets the new category.
    let body = test::read_body(resp).await;
    let new_category: Category = serde_json::from_slice(&body).unwrap();

    common::expect_n_elements(&app, "/categories", vec![new_category]).await;
}

#[actix_web::test]
async fn test_create_category_wrong_sub_category_of() {
    let (app, pool) = get_app().await;

    let mut new_category_data = get_category_default(1);
    new_category_data.sub_category_of = Some(String::from("no-id"));

    // Creates a category.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    let resp = basic_request(
        &app,
        "/categories",
        Method::POST,
        headers,
        serde_json::to_string(&new_category_data).unwrap(),
    ).await;

    expect_error(AppError::NotFound("Category with id provided does not exist!"), resp).await;

    // Gets the new category.
    let empty: Vec<Category> = vec![];
    common::expect_n_elements(&app, "/categories", empty).await;
}
