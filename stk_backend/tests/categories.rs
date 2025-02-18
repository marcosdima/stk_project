mod common;

#[cfg(test)]
mod tests {
    use actix_web::{
        dev::{self, Service, ServiceResponse},
        http::{header::ContentType, Method},
        test, web, App, Error
    };
    use actix_http::Request;
    use diesel::SqliteConnection;
    use stk_backend::models::{categories::{NewCategory, Category, CategoryUpdate}, Model};
    use crate::common;
    use uuid::Uuid;

    async fn parse_response(resp: dev::ServiceResponse) -> Vec<Category> {
        test::read_body_json(resp).await
    }

    fn get_stk_default_data(id: u16) -> NewCategory {
        NewCategory {
            name: format!("Test Category - {id}"),
            sub_category_of: None,
        }
    }

    fn create_test_categories(conn: &mut SqliteConnection, n: u16) -> Vec<Category> {
        let mut res: Vec<Category> = vec![];
        for id in 1..n + 1 {
            res.push(
                Category::create(
                    conn,
                    get_stk_default_data(id),
                ).unwrap()
            );
        }
        res
    }

    async fn expect_n_stk(
        app: &impl Service<Request, Response = ServiceResponse, Error = Error>, 
        expected: Vec<Category>
    ) {
        let req = test::TestRequest::default()
            .uri("/categories")
            .insert_header(ContentType::plaintext())
            .to_request();

        let resp = test::call_service(app, req).await;
        
        let categories = parse_response(resp).await;

        assert_eq!(expected, categories);
    }

    #[actix_web::test]
    async fn test_get_categories_empty() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(common::init_test_db_pool()))
                .configure(stk_backend::routes::categories::configure)
        ).await;

        expect_n_stk(&app, vec![]).await;
    }

    #[actix_web::test]
    async fn test_get_categories() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::categories::configure)
        ).await;

        let expected = create_test_categories(&mut pool.get().unwrap(), rand::random::<u16>());
        expect_n_stk(&app, expected).await;
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
        let created = create_test_categories(&mut pool.get().unwrap(), 1).pop().unwrap().id;

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
        expect_n_stk(&app, vec![]).await;
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

        let new_category_data = get_stk_default_data(1);

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

        expect_n_stk(&app, vec![new_category]).await;
    }

    #[actix_web::test]
    async fn test_update_category() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::categories::configure)
        ).await;

        let new_category = create_test_categories(&mut pool.get().unwrap(), 1).pop().unwrap();
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

        expect_n_stk(
            &app,
            vec![
                Category {
                    id: updated_category_data.id.to_string(),
                    name: new_name.to_owned(),
                    sub_category_of: None,
                }
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

        let new_category = create_test_categories(&mut pool.get().unwrap(), 1).pop().unwrap();
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

        expect_n_stk(
            &app,
            vec![new_category]
        ).await;
    }


}
