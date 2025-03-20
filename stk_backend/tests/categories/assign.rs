use crate::*;

#[actix_web::test]
async fn test_assign_category() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_category_data = default::get_category_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let cat = Category::create(&pool, new_category_data).unwrap();
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets new data models.
    let new_stk_cat_data = NewStickerCategory::new(stk.id.clone(), cat.id.clone());

    // Assigns a category.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/categories/sticker/assign",
        Method::POST,
        headers,
        serde_json::to_string(&new_stk_cat_data).unwrap(),
    ).await;

    assert!(resp.status().is_success());

    // Prepare comparation sets.
    let curr_stk = StickerCategory::category_stickers(&pool, cat.id.clone()).unwrap();
    let curr_cat = StickerCategory::sticker_categories(&pool, stk.id.clone()).unwrap();
    let expected_stk = vec![stk.id];
    let expected_cat = vec![cat.id];
    
    assert_eq!(expected_stk, curr_stk);
    assert_eq!(expected_cat, curr_cat);
}

#[actix_web::test]
async fn test_assign_category_wrong_id() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_category_data = default::get_category_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let cat = Category::create(&pool, new_category_data).unwrap();
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets new data models.
    let new_stk_cat_data = serde_json::json!({
        "sticker_id": "wrong-id",
        "category_id": cat.id.clone(),
    });
    let new_stk_cat_data2 = serde_json::json!({
        "sticker_id": stk.id.clone(),
        "category_id": "wrong-id",
    });
    let new_stk_cat_data_uuid = serde_json::json!({
        "sticker_id": Uuid::new_v4().to_string(),
        "category_id": Uuid::new_v4().to_string(),
    });

    // Tries to assing.
    let admin_token = get_admin_token_header(&pool);
    let json_header = get_json_header();

    let headers = vec![admin_token.clone(), json_header.clone()];
    let resp = basic_request(
        &app,
        "/categories/sticker/assign",
        Method::POST,
        headers,
        serde_json::to_string(&new_stk_cat_data).unwrap(),
    ).await;
    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Sticker with id provided does not exist!"), resp).await;

    // Tries again.
    let headers = vec![admin_token.clone(), json_header.clone()];
    let resp = basic_request(
        &app,
        "/categories/sticker/assign",
        Method::POST,
        headers,
        serde_json::to_string(&new_stk_cat_data2).unwrap(),
    ).await;
    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Category with id provided does not exist!"), resp).await;

    // Tries again.
    let headers = vec![admin_token, json_header];
    let resp = basic_request(
        &app,
        "/categories/sticker/assign",
        Method::POST,
        headers,
        serde_json::to_string(&new_stk_cat_data_uuid).unwrap(),
    ).await;
    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Category with id provided does not exist!"), resp).await;
}

#[actix_web::test]
async fn test_assign_category_twice() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_category_data = default::get_category_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let cat = Category::create(&pool, new_category_data).unwrap();
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets new data models.
    let new_stk_cat_data = NewStickerCategory::new(stk.id.clone(), cat.id.clone());

    let admin_token = get_admin_token_header(&pool);
    let json_header = get_json_header();

    let headers = vec![admin_token.clone(), json_header.clone()];
    let resp = basic_request(
        &app,
        "/categories/sticker/assign",
        Method::POST,
        headers,
        serde_json::to_string(&new_stk_cat_data).unwrap(),
    ).await;
    assert!(resp.status().is_success());

    let new_stk_cat_data = NewStickerCategory::new(stk.id, cat.id);

    // Tries again, with the same data.
    let headers = vec![admin_token, json_header];
    let resp = basic_request(
        &app,
        "/categories/sticker/assign",
        Method::POST,
        headers,
        serde_json::to_string(&new_stk_cat_data).unwrap(),
    ).await;
    assert!(resp.status().is_client_error()); // TODO: UNIQUE constraint failed.
}

#[actix_web::test]
async fn test_assign_category_with_sub_category() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_category_data = default::get_category_default(1);
    let new_category_data2 = default::get_category_default(2);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let cat = Category::create(&pool, new_category_data).unwrap();
    let cat2 = Category::create(&pool, new_category_data2).unwrap();
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets SickerCategory between 'stk' and 'cat'.
    let new_stk_cat_data = NewStickerCategory::new(stk.id.clone(), cat.id.clone());

    // Sets 'cat2' as subcategory of 'cat'.
    let _ = Category::update(&pool, CategoryUpdate::new(cat2.id, cat2.name, Some(cat.id.clone())).unwrap());

    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    let resp = basic_request(
        &app,
        "/categories/sticker/assign",
        Method::POST,
        headers,
        serde_json::to_string(&new_stk_cat_data).unwrap(),
    ).await;
    expect_error(AppError::InvalidData("Category must have no sub-categories"), resp).await;
}

#[actix_web::test]
async fn test_unassign_sticker() {
    let (app, pool) = get_app().await;

    let new_category_data = get_category_default(1);
    let new_sticker_data = get_sticker_default(1);

    let stk_id: String = Sticker::create(&pool, new_sticker_data).unwrap().id;
    let arts_id = Category::create(&pool, new_category_data).unwrap().id;

    let data = NewStickerCategory::new(stk_id.clone(), arts_id.clone());
    let _ = StickerCategory::create(&pool, data);
    let target = GetStickerCategory {
        category_id: arts_id.clone(),
        sticker_id: stk_id.clone(),
    };

    // Creates a category-sticker.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/categories/sticker/unassign",
        Method::DELETE,
        headers,
        serde_json::to_string(&target).unwrap(),
    ).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_unassign_sticker_stk_not_found() {
    let (app, pool) = get_app().await;

    let new_category_data = get_category_default(1);
    let new_sticker_data = get_sticker_default(1);

    let stk_id: String = Sticker::create(&pool, new_sticker_data).unwrap().id;
    let arts_id = Category::create(&pool, new_category_data).unwrap().id;

    let data = NewStickerCategory::new(stk_id.clone(), arts_id.clone());
    let _ = StickerCategory::create(&pool, data);
    let target = GetStickerCategory {
        sticker_id: stk_id.clone(),
        category_id: Uuid::new_v4().to_string(),
    };

    // Creates a category-sticker.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/categories/sticker/unassign",
        Method::DELETE,
        headers,
        serde_json::to_string(&target).unwrap(),
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Sticker-Category with id provided does not exist!"), resp).await;
}

#[actix_web::test]
async fn test_unassign_sticker_arts_not_found() {
    let (app, pool) = get_app().await;

    let new_category_data = get_category_default(1);
    let new_sticker_data = get_sticker_default(1);

    let stk_id: String = Sticker::create(&pool, new_sticker_data).unwrap().id;
    let arts_id = Category::create(&pool, new_category_data).unwrap().id;

    let data = NewStickerCategory::new(stk_id.clone(), arts_id.clone());
    let _ = StickerCategory::create(&pool, data);
    let target = GetStickerCategory {
        category_id: arts_id.clone(),
        sticker_id: Uuid::new_v4().to_string(),
    };

    // Creates a category-sticker.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/categories/sticker/unassign",
        Method::DELETE,
        headers,
        serde_json::to_string(&target).unwrap(),
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Sticker-Category with id provided does not exist!"), resp).await;
}
