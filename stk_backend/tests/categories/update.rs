use crate::*;

fn create_categories(pool: &DbPool, n: u16) -> Vec<Category> {
    common::create_test_objects::<Category>(pool, n, default::get_category_default)
}

fn create_n_categories_join(pool: &DbPool) -> Vec<Category> {
    let mut new_categories = create_categories(pool, 3);

    for c in 0..2 {
        let target = new_categories[c].clone();
        let upper_category_id = new_categories[c + 1].id.clone();
        let update_category_data = CategoryUpdate::new(
            target.id.clone(),
            target.name.clone(),
            Some(upper_category_id),
        ).unwrap();
        

        if Category::update(pool, update_category_data).is_ok() {
            if let Ok(updated) = Category::get_by_id(pool, target.id.clone()) {
                new_categories[c] = updated;
            }
        }
    }

    new_categories
}

#[actix_web::test]
async fn test_update_category_circular_error() {
    let (app, pool) = get_app().await;

    // Make the last connection, creating a circular relation...
    let categories = create_n_categories_join(&pool);

    // Curr relation: first -> second -> last (Last is not a subcategory)
    let first = categories.first().unwrap();
    let last = categories.last().unwrap();

    // Tries to set last as subcategory of fist.
    let updated_category_data = CategoryUpdate::new(
        last.id.clone(),
        last.name.clone(),
        Some(first.id.to_string()),
    ).unwrap();

    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    let resp = basic_request(
        &app,
        "/categories/update",
        Method::PUT,
        headers,
        serde_json::to_string(&updated_category_data).unwrap(),
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::InvalidData("Circular relation is prohibited."), resp).await;
}

#[actix_web::test]
async fn test_update_category() {
    let (app, pool) = get_app().await;

    let new_category = create_categories(&pool, 1).pop().unwrap();
    let new_name = "NEW";

    let updated_category_data = CategoryUpdate::new(
        new_category.id,
        String::from(new_name),
        None
    ).unwrap();

    // Updates category
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    let resp = basic_request(
        &app,
        "/categories/update",
        Method::PUT,
        headers,
        serde_json::to_string(&updated_category_data).unwrap(),
    ).await;

    assert!(resp.status().is_success());

    common::expect_n_elements(
        &app,
        "/categories", 
        vec![
            Category::test_new(
                updated_category_data.id.to_string(),
                new_name.to_owned(),
                None,
            )
        ]
    ).await;
}

#[actix_web::test]
async fn test_update_category_not_found() {
    let (app, _) = get_app().await;

    let updated_category_data = CategoryUpdate::new(
        Uuid::new_v4().to_string(),
        String::from("NEW"),
        None
    ).unwrap();

    // Updates category
    let req = test::TestRequest::default()
        .method(Method::PUT)
        .uri("/categories")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&updated_category_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
