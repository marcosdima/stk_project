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
    use stk_backend::models::{NewSticker, Sticker, StickerUpdate};
    use crate::common;

    async fn parse_response(resp: dev::ServiceResponse) -> Vec<Sticker> {
        test::read_body_json(resp).await
    }

    fn get_stk_default_data(id: u16) -> NewSticker {
        NewSticker {
            label: format!("Test Sticker - {id}"),
            url: format!("www.some-url-{id}.com.ar"),
        }
    }

    fn create_test_stickers(conn: &mut SqliteConnection, n: u16) -> Vec<Sticker> {
        let mut res: Vec<Sticker> = vec![];
        for id in 1..n + 1 {
            res.push(
                Sticker::create(
                    conn,
                    get_stk_default_data(id),
                ).unwrap()
            );
        }
        res
    }

    async fn expect_n_stk(
        app: &impl Service<Request, Response = ServiceResponse, Error = Error>, 
        expected: Vec<Sticker>
    ) {
        let req = test::TestRequest::default()
            .uri("/stickers")
            .insert_header(ContentType::plaintext())
            .to_request();

        let resp = test::call_service(app, req).await;
        
        let stickers = parse_response(resp).await;

        assert_eq!(expected, stickers);
    }

    #[actix_web::test]
    async fn test_get_stickers_empty() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(common::init_test_db_pool()))
                .configure(stk_backend::routes::stickers::configure)
        ).await;

        expect_n_stk(&app, vec![]).await;
    }

    #[actix_web::test]
    async fn test_get_stickers() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::stickers::configure)
        ).await;

        let expected = create_test_stickers(&mut pool.get().unwrap(), rand::random::<u16>());
        expect_n_stk(&app, expected).await;
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
        let created = create_test_stickers(&mut pool.get().unwrap(), 1).pop().unwrap().id;

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
        expect_n_stk(&app, vec![]).await;
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

        let new_sticker_data = get_stk_default_data(1);

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

        expect_n_stk(&app, vec![new_sticker]).await;
    }

    #[actix_web::test]
    async fn test_update_sticker() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::stickers::configure)
        ).await;

        let new_sticker = create_test_stickers(&mut pool.get().unwrap(), 1).pop().unwrap();
        let new_label = "NEW";
        let new_url = "www.updated-url.com";

        let updated_sticker_data = StickerUpdate::new(new_sticker.id, String::from(new_label), String::from(new_url));

        // Updates sticker
        let req = test::TestRequest::default()
            .method(Method::PUT)
            .uri("/stickers")
            .insert_header(ContentType::json())
            .set_payload(serde_json::to_string(&updated_sticker_data).unwrap())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        expect_n_stk(
            &app,
            vec![
                Sticker {
                    id: updated_sticker_data.id,
                    label: new_label.to_owned(),
                    url: new_url.to_owned(),
                }
            ]
        ).await;
    }
}
