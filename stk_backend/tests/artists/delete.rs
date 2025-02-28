use crate::*;

fn create_artists(pool: &DbPool, n: u16) -> Vec<Artist> {
    common::create_test_objects::<Artist>(pool, n, get_artist_default)
}

#[actix_web::test]
async fn test_delete_artists() {
    let (app, pool) = get_app().await;

    // Gets id from a new artist.
    let created = create_artists(&pool, 1).pop().unwrap().id;

    // Should return a succes message.
    let req = test::TestRequest::default()
        .method(Method::DELETE)
        .uri(&format!("/artists/{created}"))
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Artist deleted successfully");

    // Gets artists, it should be an empty vector.
    let empty: Vec<Artist> = vec![];
    common::expect_n_elements::<Artist>(&app, "/artists", empty).await;
}

#[actix_web::test]
async fn test_delete_artists_not_found() {
    let (app, _) = get_app().await;

    // Should return a succes message.
    let req = test::TestRequest::default()
        .method(Method::DELETE)
        .uri(&format!("/artists/id-not-found"))
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Artist not found");

}
