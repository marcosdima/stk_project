use crate::*;

fn create_artists(pool: &DbPool, n: u16) -> Vec<Artist> {
    common::create_test_objects::<Artist>(pool, n, get_artist_default)
}

#[actix_web::test]
async fn test_delete_artists() {
    let (app, pool) = get_app().await;

    // Gets id from a new artist.
    let created = create_artists(&pool, 1).pop().unwrap().id;

    let headers = vec![
        get_admin_token_header(&pool),
        get_plain_text_header(),
    ];
    
    let resp = basic_request(
        &app,
        &format!("/artists/{created}/delete"),
        Method::DELETE,
        headers,
        "",
    ).await;
    let body = test::read_body(resp).await;
    assert_eq!(body, "Artist deleted successfully");

    // Gets artists, it should be an empty vector.
    let empty: Vec<Artist> = vec![];
    common::expect_n_elements::<Artist>(&app, "/artists", empty).await;
}

#[actix_web::test]
async fn test_delete_artists_not_found() {
    let (app, pool) = get_app().await;

    let headers = vec![
        get_admin_token_header(&pool),
        get_plain_text_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/artists/id-not-found/delete",
        Method::DELETE,
        headers,
        "",
    ).await;
    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Artist not found"), resp).await;
}
