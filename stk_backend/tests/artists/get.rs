use crate::*;

fn create_artists(pool: &DbPool, n: u16) -> Vec<Artist> {
    common::create_test_objects::<Artist>(pool, n, get_artist_default)
}

#[actix_web::test]
async fn test_get_artists_empty() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(common::init_test_db_pool()))
            .configure(stk_backend::routes::artists::configure)
    ).await;

    let empty: Vec<Artist> = vec![];
    common::expect_n_elements(&app, "/artists", empty).await;
}

#[actix_web::test]
async fn test_get_artists() {
    let (app, pool) = get_app().await;

    let expected = create_artists(&pool, rand::random::<u16>());
    common::expect_n_elements::<Artist>(&app, "/artists", expected).await;
}
