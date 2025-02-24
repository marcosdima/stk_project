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
        models::{categories::Category, sticker_category::{NewStickerCategory, StickerCategory}, stickers::{
            Sticker,
            StickerUpdate,
        }, BasicModel},
        routes::DbPool
    };
    use crate::common::{self, default::{self, get_sticker_default}};
    use uuid::Uuid;

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
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::stickers::configure)
        ).await;

        let expected = create_stickers(&pool, rand::random::<u16>());
        common::expect_n_elements::<Sticker>(&app, "/stickers", expected).await;
    }

    #[actix_web::test]
    async fn test_delete_stickers() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::stickers::configure)
        ).await;

        // Gets id from a new sticker.
        let created = create_stickers(&pool, 1).pop().unwrap().id;

        // Should return a succes message.
        let req = test::TestRequest::default()
            .method(Method::DELETE)
            .uri(&format!("/stickers/{created}"))
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Sticker deleted successfully");

        // Gets stickers, it should be an empty vector.
        let empty: Vec<Sticker> = vec![];
        common::expect_n_elements::<Sticker>(&app, "/stickers", empty).await;
    }

    #[actix_web::test]
    async fn test_delete_stickers_not_found() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::stickers::configure)
        ).await;

        // Should return a succes message.
        let req = test::TestRequest::default()
            .method(Method::DELETE)
            .uri(&format!("/stickers/id-not-found"))
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Sticker not found");

    }

    #[actix_web::test]
    async fn test_create_sticker() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::stickers::configure)
        ).await;

        let new_sticker_data = get_sticker_default(1);

        // Creates a sticker.
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri(&format!("/stickers"))
            .insert_header(ContentType::json())
            .set_payload(serde_json::to_string(&new_sticker_data).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Gets the new sticker.
        let body = test::read_body(resp).await;
        let new_sticker: Sticker = serde_json::from_slice(&body).unwrap();

        common::expect_n_elements(&app, "/stickers", vec![new_sticker]).await;
    }

    #[actix_web::test]
    async fn test_update_sticker() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::stickers::configure)
        ).await;

        let new_sticker = create_stickers(&pool, 1).pop().unwrap();
        let new_label = "NEW";
        let new_url = "www.updated-url.com";

        let updated_sticker_data = StickerUpdate::new(
            new_sticker.id.clone(),
            String::from(new_label),
            String::from(new_url)
        ).unwrap();

        // Updates sticker
        let req = test::TestRequest::default()
            .method(Method::PUT)
            .uri("/stickers")
            .insert_header(ContentType::json())
            .set_payload(serde_json::to_string(&updated_sticker_data).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        common::expect_n_elements(
            &app,
            "/stickers",
            vec![
                Sticker {
                    id: updated_sticker_data.id.to_string(),
                    label: new_label.to_owned(),
                    url: new_url.to_owned(),
                }
            ]
        ).await;
    }

    #[actix_web::test]
    async fn test_update_sticker_not_found() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::stickers::configure)
        ).await;

        let updated_sticker_data = StickerUpdate::new(
            Uuid::new_v4().to_string(),
            String::from("NEW"),
            String::from("www.updated-url.com")
        ).unwrap();

        // Updates sticker
        let req = test::TestRequest::default()
            .method(Method::PUT)
            .uri("/stickers")
            .insert_header(ContentType::json())
            .set_payload(serde_json::to_string(&updated_sticker_data).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_update_sticker_wrong_id() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::stickers::configure)
        ).await;

        let new_sticker = create_stickers(&pool, 1).pop().unwrap();
        let new_label = "NEW";
        let new_url = "www.updated-url.com";

        let updated_sticker_data = serde_json::json!({
            "id": "wrong-id",
            "label": new_label,
            "url": new_url
        });

        // Updates sticker
        let req = test::TestRequest::default()
            .method(Method::PUT)
            .uri("/stickers")
            .insert_header(ContentType::json())
            .set_payload(serde_json::to_string(&updated_sticker_data).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        common::expect_n_elements(
            &app,
            "/stickers",
            vec![new_sticker]
        ).await;
    }

    #[actix_web::test]
    async fn test_get_sticker_categories() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::stickers::configure)
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
}
