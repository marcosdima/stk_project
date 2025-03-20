use crate::*;

#[actix_web::test]
async fn test_create_artist() {
    let (app, pool) = get_app().await;

    let new_artist_data = get_artist_default(1);

    // Creates a artist.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/artists",
        Method::POST,
        headers,
        serde_json::to_string(&new_artist_data).unwrap(),
    ).await;

    assert!(resp.status().is_success());

    // Gets the new artist.
    let body = test::read_body(resp).await;
    let new_artist: Artist = serde_json::from_slice(&body).unwrap();

    common::expect_n_elements(&app, "/artists", vec![new_artist]).await;
}
