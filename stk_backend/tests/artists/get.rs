use crate::*;

fn create_artists(pool: &DbPool, n: u16) -> Vec<Artist> {
    common::create_test_objects::<Artist>(pool, n, get_artist_default)
}

fn create_stickers(pool: &DbPool, n: u16) -> Vec<Sticker> {
    common::create_test_objects::<Sticker>(pool, n, get_sticker_default)
}

#[actix_web::test]
async fn test_get_artists_empty() {
    let (app, _) = get_app().await;

    let empty: Vec<Artist> = vec![];
    common::expect_n_elements(&app, "/artists", empty).await;
}

#[actix_web::test]
async fn test_get_artists() {
    let (app, pool) = get_app().await;

    let expected = create_artists(&pool, rand::random::<u16>());
    common::expect_n_elements::<Artist>(&app, "/artists", expected).await;
}

#[actix_web::test]
async fn test_get_artist_stickers() {
    let (app, pool) = get_app().await;

    // Create stickers.
    let mut stks: Vec<String> = create_stickers(&pool, 5)
        .into_iter()
        .map(|stk| stk.id)
        .collect();

    // Create artist.
    let artist_id = create_artists(&pool, 1).first().unwrap().to_owned().id;

    // Set stickers as artist's stickers.
    for stk in &stks {
        let data = NewArtistSticker::new(stk.clone(), artist_id.clone());
        let _ = ArtistSticker::create(&pool, data);
    }

    let req = test::TestRequest::default()
        .method(Method::GET)
        .uri(&format!("/artists/{artist_id}/stickers"))
        .insert_header(ContentType::json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let mut stickers: Vec<String> = serde_json::from_slice(&body).unwrap();

    assert_eq!(stickers.sort(), stks.sort());
}

#[actix_web::test]
async fn test_get_artist_stickers_but_artist_not_found() {
    let (app, _) = get_app().await;

    let artist_id = Uuid::new_v4().to_string();

    let req = test::TestRequest::default()
        .method(Method::GET)
        .uri(&format!("/artists/{artist_id}/stickers"))
        .insert_header(ContentType::json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
