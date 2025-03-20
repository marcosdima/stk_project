use crate::*;

fn create_artists(pool: &DbPool, n: u16) -> Vec<Artist> {
    common::create_test_objects::<Artist>(pool, n, get_artist_default)
}

#[actix_web::test]
async fn test_update_artist() {
    let (app, pool) = get_app().await;

    let new_artist = create_artists(&pool, 1).pop().unwrap();
    let new_name = String::from("NEW");
    let new_url = String::from("www.updated-url.com");
    let new_presentation = Some(String::from("Now, I am me..."));

    let updated_artist_data = ArtistUpdate::new(
        new_artist.id.clone(),
        new_name.clone(),
        new_url.clone(),
        new_presentation.clone(),
    );

    // Updates artist
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/artists/update",
        Method::PUT,
        headers,
        serde_json::to_string(&updated_artist_data).unwrap(),
    ).await;

    assert!(resp.status().is_success());

    common::expect_n_elements(
        &app,
        "/artists",
        vec![
            Artist {
                id: updated_artist_data.id.to_string(),
                name: new_name,
                logo_url: new_url,
                presentation: new_presentation,
            }
        ]
    ).await;
}

#[actix_web::test]
async fn test_update_artist_not_found() {
    let (app, pool) = get_app().await;

    let updated_artist_data = ArtistUpdate::new(
        Uuid::new_v4().to_string(),
        String::from("NEW"),
        String::from("www.updated-url.com"),
        Some(String::from("Now, I am me...")),
    );

    // Updates artist
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/artists/update",
        Method::PUT,
        headers,
        serde_json::to_string(&updated_artist_data).unwrap(),
    ).await;
    
    expect_error(AppError::NotFound("Artist with id provided does not exist!"), resp).await;
}

#[actix_web::test]
async fn test_update_artist_wrong_id() {
    let (app, pool) = get_app().await;

    let new_artist = create_artists(&pool, 1).pop().unwrap();
    let new_label = "NEW";
    let new_url = "www.updated-url.com";

    let updated_artist_data = serde_json::json!({
        "id": "wrong-id",
        "name": new_label,
        "url": new_url,
        "logo_url": "None",
    });

    // Updates artist
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/artists/update",
        Method::PUT,
        headers,
        serde_json::to_string(&updated_artist_data).unwrap(),
    ).await;

    expect_error(AppError::NotFound("Artist with id provided does not exist!"), resp).await;

    common::expect_n_elements(
        &app,
        "/artists",
        vec![new_artist]
    ).await;
}
