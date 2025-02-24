use crate::*;

#[actix_web::test]
async fn test_assign_category() {
    let pool = web::Data::new(common::init_test_db_pool());

    let app = test::init_service(
        App::new()
            .app_data(pool.clone())
            .configure(stk_backend::routes::categories::configure)
    ).await;

    // Get default data.
    let new_category_data = default::get_category_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let cat = Category::create(&pool, new_category_data).unwrap();
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets new data models.
    let new_stk_cat_data = NewStickerCategory::new(stk.id.clone(), cat.id.clone()).unwrap();

    // Assigns a category.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/categories/assign"))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_stk_cat_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
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
    let pool = web::Data::new(common::init_test_db_pool());

    let app = test::init_service(
        App::new()
            .app_data(pool.clone())
            .configure(stk_backend::routes::categories::configure)
    ).await;

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
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/categories/assign"))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_stk_cat_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());

    // Tries again.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/categories/assign"))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_stk_cat_data2).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());

    // Tries again.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/categories/assign"))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_stk_cat_data_uuid).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{:?}", resp);
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_assign_category_twice() {
    let pool = web::Data::new(common::init_test_db_pool());

    let app = test::init_service(
        App::new()
            .app_data(pool.clone())
            .configure(stk_backend::routes::categories::configure)
    ).await;

    // Get default data.
    let new_category_data = default::get_category_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let cat = Category::create(&pool, new_category_data).unwrap();
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets new data models.
    let new_stk_cat_data = NewStickerCategory::new(stk.id.clone(), cat.id.clone()).unwrap();

    // Creates a category.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/categories/assign"))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_stk_cat_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let new_stk_cat_data = NewStickerCategory::new(stk.id, cat.id).unwrap();

    // Tries again, with the same data.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/categories/assign"))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_stk_cat_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_assign_category_with_sub_category() {
    let pool = web::Data::new(common::init_test_db_pool());

    let app = test::init_service(
        App::new()
            .app_data(pool.clone())
            .configure(stk_backend::routes::categories::configure)
    ).await;

    // Get default data.
    let new_category_data = default::get_category_default(1);
    let new_category_data2 = default::get_category_default(2);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let cat = Category::create(&pool, new_category_data).unwrap();
    let cat2 = Category::create(&pool, new_category_data2).unwrap();
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets SickerCategory between 'stk' and 'cat'.
    let new_stk_cat_data = NewStickerCategory::new(stk.id.clone(), cat.id.clone()).unwrap();

    // Sets 'cat2' as subcategory of 'cat'.
    let _ = Category::update(&pool, CategoryUpdate::new(cat2.id, cat2.name, Some(cat.id.clone())).unwrap());

    // Creates a category.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/categories/assign"))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_stk_cat_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
