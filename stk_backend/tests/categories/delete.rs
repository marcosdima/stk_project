use crate::*;

pub fn create_categories(pool: &DbPool, n: u16) -> Vec<Category> {
    common::create_test_objects::<Category>(pool, n, default::get_category_default)
}

#[actix_web::test]
async fn test_delete_categories() {
    let (app, pool) = get_app().await;

    // Gets id from a new category.
    let created = create_categories(&pool, 1).pop().unwrap().id;

    // Should return a succes message.
    let req = test::TestRequest::default()
        .method(Method::DELETE)
        .uri(&format!("/categories/{created}"))
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Category deleted successfully");

    // Gets categories, it should be an empty vector.
    let empty: Vec<Category> = vec![];
    common::expect_n_elements(&app, "/categories", empty).await;
}

#[actix_web::test]
async fn test_delete_categories_not_found() {
    let (app, _) = get_app().await;

    // Should return a succes message.
    let req = test::TestRequest::default()
        .method(Method::DELETE)
        .uri(&format!("/categories/id-not-found"))
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Category not found");

}

#[actix_web::test]
async fn test_delete_category_but_it_has_subcategories() {
    let (app, pool) = get_app().await;

    // Create two categories.
    let new_category_data = Category::create(&pool, get_category_default(1)).unwrap();
    let new_category_data2 = Category::create(&pool, get_category_default(2)).unwrap();
    let id = new_category_data.id.clone();
    let id_2: String = new_category_data2.id.clone();

    // Create an update to set ncd2 as ncd sub category.
    let updated_category_data = CategoryUpdate::new(
        new_category_data2.id,
        new_category_data2.name,
        Some(id.clone()),
    ).unwrap();
    let _ = Category::update(&pool, updated_category_data);

    // Delete ncd.
    let _ = Category::delete(&pool, id);

    // Now, it should retrive ncd2 with no upper category.
    let result = get_element::<Category>(&app, &format!("/categories/{}", id_2)).await;
    if let Some(res) = result.get_sub_category() {
        panic!("Received: {res}")
    }
}
