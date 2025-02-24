use crate::*;

fn create_stickers(pool: &DbPool, n: u16) -> Vec<Sticker> {
    common::create_test_objects::<Sticker>(pool, n, get_sticker_default)
}

#[actix_web::test]
async fn test_get_stickers_empty() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(common::init_test_db_pool()))
            .configure(stk_backend::routes::stickers::configure)
    ).await;

    let empty: Vec<Sticker> = vec![];
    common::expect_n_elements(&app, "/stickers", empty).await;
}

#[actix_web::test]
async fn test_get_stickers() {
    let (app, pool) = get_app().await;

    let expected = create_stickers(&pool, rand::random::<u16>());
    common::expect_n_elements::<Sticker>(&app, "/stickers", expected).await;
}

#[actix_web::test]
async fn test_get_sticker_categories() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_category_data = default::get_category_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let cat = Category::create(&pool, new_category_data).unwrap();
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets new data models.
    let new_stk_cat_data = NewStickerCategory::new(stk.id.clone(), cat.id.clone()).unwrap();

    // Assigns a category.
    let _ = StickerCategory::create(&pool, new_stk_cat_data);

    // Tries again, with the same data.
    let req = test::TestRequest::default()
        .method(Method::GET)
        .uri(&format!("/stickers/{}/categories", stk.id))
        .insert_header(ContentType::json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let stickers: Vec<String> = serde_json::from_slice(&body).unwrap();
    assert_eq!(stickers, vec![cat.id])
}
