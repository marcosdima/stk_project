use crate::*;

#[actix_web::test]
async fn test_create_artist() {
    let (app, _) = get_app().await;

    let new_artist_data = get_artist_default(1);

    // Creates a artist.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/artists"))
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_artist_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Gets the new artist.
    let body = test::read_body(resp).await;
    let new_artist: Artist = serde_json::from_slice(&body).unwrap();

    common::expect_n_elements(&app, "/artists", vec![new_artist]).await;
}
