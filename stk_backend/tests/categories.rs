mod common;

#[cfg(test)]
mod tests {
    use actix_web::{
        http::{
            header::ContentType,
            Method
        },
        test,
        web,
        App,
    };

    use stk_backend::{
        models::{
            categories::{
                Category,
                CategoryUpdate,
            },
            sticker_category::{
                NewStickerCategory,
                StickerCategory
            },
            stickers::Sticker,
            BasicModel,
            Model
        },
        routes::DbPool
    };
    use crate::common::{self, default::{self, get_category_default}};
    use uuid::Uuid;

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
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::categories::configure)
        ).await;

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

        let req = test::TestRequest::default()
            .method(Method::PUT)
            .uri("/categories")
            .insert_header(ContentType::json())
            .set_payload(serde_json::to_string(&updated_category_data).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_get_categories_empty() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(common::init_test_db_pool()))
                .configure(stk_backend::routes::categories::configure)
        ).await;

        let empty: Vec<Category> = vec![];
        common::expect_n_elements(&app, "/categories", empty).await;
    }

    #[actix_web::test]
    async fn test_get_categories() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::categories::configure)
        ).await;

        let expected = create_categories(&pool, rand::random::<u16>());
        common::expect_n_elements(&app, "/categories", expected).await;
    }

    #[actix_web::test]
    async fn test_delete_categories() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::categories::configure)
        ).await;

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
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::categories::configure)
        ).await;

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
    async fn test_create_category() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::categories::configure)
        ).await;

        let new_category_data = get_category_default(1);

        // Creates a category.
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri(&format!("/categories"))
            .insert_header(ContentType::json())
            .set_payload(serde_json::to_string(&new_category_data).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Gets the new category.
        let body = test::read_body(resp).await;
        let new_category: Category = serde_json::from_slice(&body).unwrap();

        common::expect_n_elements(&app, "/categories", vec![new_category]).await;
    }

    #[actix_web::test]
    async fn test_create_category_wrong_sub_category_of() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::categories::configure)
        ).await;

        let mut new_category_data = get_category_default(1);
        new_category_data.sub_category_of = Some(String::from("no-id"));

        // Creates a category.
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri(&format!("/categories"))
            .insert_header(ContentType::json())
            .set_payload(serde_json::to_string(&new_category_data).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;
  
        assert!(resp.status().is_client_error());

        // Gets the new category.
        let empty: Vec<Category> = vec![];
        common::expect_n_elements(&app, "/categories", empty).await;
    }

    #[actix_web::test]
    async fn test_update_category() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::categories::configure)
        ).await;

        let new_category = create_categories(&pool, 1).pop().unwrap();
        let new_name = "NEW";

        let updated_category_data = CategoryUpdate::new(
            new_category.id,
            String::from(new_name),
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
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::categories::configure)
        ).await;

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

    #[actix_web::test]
    async fn test_update_category_wrong_id() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::categories::configure)
        ).await;

        let new_category = create_categories(&pool, 1).pop().unwrap();
        let new_name = "NEW";
        let new_url = "www.updated-url.com";

        let updated_category_data = serde_json::json!({
            "id": "wrong-id",
            "name": new_name,
            "url": new_url
        });

        // Updates category
        let req = test::TestRequest::default()
            .method(Method::PUT)
            .uri("/categories")
            .insert_header(ContentType::json())
            .set_payload(serde_json::to_string(&updated_category_data).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        common::expect_n_elements::<Category>(
            &app,
            "/categories", 
            vec![new_category]
        ).await;
    }

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
    async fn test_get_category_stickers() {
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
}
