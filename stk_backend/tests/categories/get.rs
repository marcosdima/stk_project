use crate::*;

fn create_categories(pool: &DbPool, n: u16) -> Vec<Category> {
    common::create_test_objects::<Category>(pool, n, default::get_category_default)
}

#[actix_web::test]
async fn test_get_categories_empty() {
    let (app, _) = get_app().await;

    let empty: Vec<Category> = vec![];
    common::expect_n_elements(&app, "/categories", empty).await;
}

#[actix_web::test]
async fn test_get_categories() {
    let (app, pool) = get_app().await;

    let expected = create_categories(&pool, rand::random::<u16>());
    common::expect_n_elements(&app, "/categories", expected).await;
}

#[actix_web::test]
async fn test_get_category() {
    let (app, pool) = get_app().await;

    let expected = create_categories(&pool, 1).first().unwrap().to_owned();
    let result = common::get_element::<Category>(&app, &format!("/categories/{}", expected.id.clone())).await;
    assert_eq!(expected, result)
}

#[actix_web::test]
async fn test_get_category_stickers() {
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
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/categories/assign"))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_stk_cat_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Tries again, with the same data.
    let req = test::TestRequest::default()
        .method(Method::GET)
        .uri(&format!("/categories/{}/stickers", cat.id))
        .insert_header(ContentType::json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let stickers: Vec<String> = serde_json::from_slice(&body).unwrap();
    assert_eq!(stickers, vec![stk.id])
}
