use crate::*;

#[actix_web::test]
async fn test_assign_sticker() {
    let (app, pool) = get_app().await;

    let new_artist_data = get_artist_default(1);
    let new_sticker_data = get_sticker_default(1);

    let stk_id = Sticker::create(&pool, new_sticker_data).unwrap().id;
    let arts_id = Artist::create(&pool, new_artist_data).unwrap().id;

    let data = NewArtistSticker::new(stk_id.clone(), arts_id.clone());

    // Creates a artist-sticker.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/artists/sticker/assign",
        Method::POST,
        headers,
        serde_json::to_string(&data).unwrap(),
    ).await;
    assert!(resp.status().is_success());

    // Gets the new artist-sticker.
    let body = test::read_body(resp).await;
    let new_artist_stk: ArtistSticker = serde_json::from_slice(&body).unwrap();

    assert_eq!(new_artist_stk.sticker_id, stk_id);
    assert_eq!(new_artist_stk.artist_id, arts_id);
}

#[actix_web::test]
async fn test_assign_sticker_stk_not_found() {
    let (app, pool) = get_app().await;

    let new_artist_data = get_artist_default(1);
    let arts_id = Artist::create(&pool, new_artist_data).unwrap().id;
    let data = NewArtistSticker::new(Uuid::new_v4().to_string(), arts_id);

    // Creates a artist-sticker.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/artists/sticker/assign",
        Method::POST,
        headers,
        serde_json::to_string(&data).unwrap(),
    ).await;
    
    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Sticker with id provided does not exist!"), resp).await;
}

#[actix_web::test]
async fn test_assign_sticker_arts_not_found() {
    let (app, pool) = get_app().await;

    let new_sticker_data = get_sticker_default(1);
    let stk_id = Sticker::create(&pool, new_sticker_data).unwrap().id;
    let data = NewArtistSticker::new(stk_id, Uuid::new_v4().to_string());

    // Creates a artist-sticker.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/artists/sticker/assign",
        Method::POST,
        headers,
        serde_json::to_string(&data).unwrap(),
    ).await;
    
    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Artist with id provided does not exist!"), resp).await;
}

#[actix_web::test]
async fn test_unassign_sticker() {
    let (app, pool) = get_app().await;

    let new_artist_data = get_artist_default(1);
    let new_sticker_data = get_sticker_default(1);

    let stk_id: String = Sticker::create(&pool, new_sticker_data).unwrap().id;
    let arts_id = Artist::create(&pool, new_artist_data).unwrap().id;

    let data = NewArtistSticker::new(stk_id.clone(), arts_id.clone());
    let _ = ArtistSticker::create(&pool, data);
    let target = GetArtistSticker {
        artist_id: arts_id.clone(),
        sticker_id: stk_id.clone(),
    };

    // Creates a artist-sticker.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/artists/sticker/unassign",
        Method::DELETE,
        headers,
        serde_json::to_string(&target).unwrap(),
    ).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_unassign_sticker_stk_not_found() {
    let (app, pool) = get_app().await;

    let new_artist_data = get_artist_default(1);
    let new_sticker_data = get_sticker_default(1);

    let stk_id: String = Sticker::create(&pool, new_sticker_data).unwrap().id;
    let arts_id = Artist::create(&pool, new_artist_data).unwrap().id;

    let data = NewArtistSticker::new(stk_id.clone(), arts_id.clone());
    let _ = ArtistSticker::create(&pool, data);
    let target = GetArtistSticker {
        sticker_id: stk_id.clone(),
        artist_id: Uuid::new_v4().to_string(),
    };

    // Creates a artist-sticker.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/artists/sticker/unassign",
        Method::DELETE,
        headers,
        serde_json::to_string(&target).unwrap(),
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Artist-Sticker with id provided does not exist!"), resp).await;
}

#[actix_web::test]
async fn test_unassign_sticker_arts_not_found() {
    let (app, pool) = get_app().await;

    let new_artist_data = get_artist_default(1);
    let new_sticker_data = get_sticker_default(1);

    let stk_id: String = Sticker::create(&pool, new_sticker_data).unwrap().id;
    let arts_id = Artist::create(&pool, new_artist_data).unwrap().id;

    let data = NewArtistSticker::new(stk_id.clone(), arts_id.clone());
    let _ = ArtistSticker::create(&pool, data);
    let target = GetArtistSticker {
        artist_id: arts_id.clone(),
        sticker_id: Uuid::new_v4().to_string(),
    };

    // Creates a artist-sticker.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/artists/sticker/unassign",
        Method::DELETE,
        headers,
        serde_json::to_string(&target).unwrap(),
    ).await;

    assert!(resp.status().is_client_error());
    expect_error(AppError::NotFound("Artist-Sticker with id provided does not exist!"), resp).await;
}
