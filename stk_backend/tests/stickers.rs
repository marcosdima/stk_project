mod common;

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App, web, dev};
    use diesel::SqliteConnection;
    use stk_backend::models::{NewSticker, Sticker};
    use crate::common;

    async fn parse_response(resp: dev::ServiceResponse) -> Vec<Sticker> {
        test::read_body_json(resp).await
    }

    fn create_test_stickers(conn: &mut SqliteConnection, n: u16) -> Vec<Sticker> {
        let mut res: Vec<Sticker> = vec![];
        for i in 1..n {
            let test_label = format!("Test Sticker - {i}");
            let test_url = format!("www.some-url-{i}.com.ar");

            res.push(
                Sticker::create(
                    conn,
                    NewSticker::new(test_label, test_url),
                ).unwrap()
            );
        }
        res
    }

    #[actix_web::test]
    async fn test_get_stickers_empty() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(common::init_test_db_pool()))
                .configure(stk_backend::routes::stickers::configure)
        ).await;

        let req = test::TestRequest::default()
            .uri("/stickers")
            .insert_header(ContentType::plaintext())
            .to_request();

        let resp = test::call_service(&app, req).await;
        let stickers = parse_response(resp).await;
        let expected: Vec<Sticker> = vec![];

        assert_eq!(expected, stickers);
    }

    #[actix_web::test]
    async fn test_get_stickers() {
        let pool = web::Data::new(common::init_test_db_pool());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .configure(stk_backend::routes::stickers::configure)
        ).await;

        let req = test::TestRequest::default()
            .uri("/stickers")
            .insert_header(ContentType::plaintext())
            .to_request();

        let expected = create_test_stickers(&mut pool.get().unwrap(), rand::random::<u16>());
        let resp = test::call_service(&app, req).await;
        let stickers = parse_response(resp).await;

        assert_eq!(expected, stickers);
    }
}
