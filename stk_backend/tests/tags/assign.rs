use crate::*;

#[actix_web::test]
async fn test_assign_tag() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_tag_data = default::get_tag_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let tag = Tag::create(&pool, new_tag_data).unwrap();
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets new data models.
    let new_stk_tag_data = NewStickerTag::new(stk.id.clone(), tag.id.clone()).unwrap();

    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/tags/assign",
        Method::POST,
        headers,
        serde_json::to_string(&new_stk_tag_data).unwrap(),
    ).await;

    assert!(resp.status().is_success());
   
    // Check creation.
    assert_eq!(StickerTag::sticker_tags(&pool, stk.id).unwrap(), vec![tag.id]);
}

#[actix_web::test]
async fn test_assign_imaginary_tag() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets new data models.
    let new_stk_tag_data = NewStickerTag::new(stk.id.clone(), Uuid::new_v4().to_string()).unwrap();

    // Assigns a tag.
    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/tags/assign",
        Method::POST,
        headers,
        serde_json::to_string(&new_stk_tag_data).unwrap(),
    ).await;

    expect_error(AppError::NotFound("Tag with id provided does not exist!"), resp).await;
   
    // Check creation.
    assert_eq!(StickerTag::sticker_tags(&pool, stk.id).unwrap(), vec![] as Vec<String>);
}

#[actix_web::test]
async fn test_assign_tag_with_no_token() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_tag_data = default::get_tag_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let tag = Tag::create(&pool, new_tag_data).unwrap();
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets new data models.
    let new_stk_tag_data = NewStickerTag::new(stk.id.clone(), tag.id.clone()).unwrap();

    let headers = vec![
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/tags/assign",
        Method::POST,
        headers,
        serde_json::to_string(&new_stk_tag_data).unwrap(),
    ).await;

    expect_error(AppError::InvalidToken, resp).await;

    // Check creation.
    assert_eq!(StickerTag::sticker_tags(&pool, stk.id).unwrap(), vec![] as Vec<String>);
}

#[actix_web::test]
async fn test_assign_tag_with_no_role() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_tag_data = default::get_tag_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let tag = Tag::create(&pool, new_tag_data).unwrap();
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets new data models.
    let new_stk_tag_data = NewStickerTag::new(stk.id.clone(), tag.id.clone()).unwrap();

    let headers = vec![
        get_random_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/tags/assign",
        Method::POST,
        headers,
        serde_json::to_string(&new_stk_tag_data).unwrap(),
    ).await;

    expect_error(AppError::RoleNeeded, resp).await;

    // Check creation.
    assert_eq!(StickerTag::sticker_tags(&pool, stk.id).unwrap(), vec![] as Vec<String>);
}

#[actix_web::test]
async fn test_unassign_imaginary_sticker_tag() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_tag_data = default::get_tag_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let tag_id = Tag::create(&pool, new_tag_data).unwrap().id;
    let sticker_id = Sticker::create(&pool, new_sticker_data).unwrap().id;

    // Unassgin
    let unassign_data = DeleteStickerTag { tag_id, sticker_id };

    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/tags/unassign",
        Method::DELETE,
        headers,
        serde_json::to_string(&unassign_data).unwrap(),
    ).await;

    expect_error(AppError::NotFound("Sticker-Tag not found"), resp).await;
}

#[actix_web::test]
async fn test_unassign_tag() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_tag_data = default::get_tag_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let tag_id = Tag::create(&pool, new_tag_data).unwrap().id;
    let sticker_id = Sticker::create(&pool, new_sticker_data).unwrap().id;

    // Sets new data models.
    let new_stk_tag_data = NewStickerTag::new(sticker_id.clone(), tag_id.clone()).unwrap();
    let _ = StickerTag::create(&pool, new_stk_tag_data).unwrap();

    // Unassgin
    let unassign_data = DeleteStickerTag { tag_id: tag_id.clone(), sticker_id };

    let headers = vec![
        get_admin_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/tags/unassign",
        Method::DELETE,
        headers,
        serde_json::to_string(&unassign_data).unwrap(),
    ).await;

    assert!(resp.status().is_success());

    // Check delete.
    assert_eq!(StickerTag::tag_stickers(&pool, tag_id).unwrap(), vec![] as Vec<String>);
}

#[actix_web::test]
async fn test_unassign_tag_no_role() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_tag_data = default::get_tag_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let tag_id = Tag::create(&pool, new_tag_data).unwrap().id;
    let sticker_id = Sticker::create(&pool, new_sticker_data).unwrap().id;

    // Sets new data models.
    let new_stk_tag_data = NewStickerTag::new(sticker_id.clone(), tag_id.clone()).unwrap();
    let _ = StickerTag::create(&pool, new_stk_tag_data).unwrap();

    // Unassgin
    let unassign_data = DeleteStickerTag { tag_id: tag_id, sticker_id };

    let headers = vec![
        get_random_token_header(&pool),
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/tags/unassign",
        Method::DELETE,
        headers,
        serde_json::to_string(&unassign_data).unwrap(),
    ).await;

    expect_error(AppError::RoleNeeded, resp).await;
}

#[actix_web::test]
async fn test_unassign_tag_no_token() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_tag_data = default::get_tag_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let tag_id = Tag::create(&pool, new_tag_data).unwrap().id;
    let sticker_id = Sticker::create(&pool, new_sticker_data).unwrap().id;

    // Sets new data models.
    let new_stk_tag_data = NewStickerTag::new(sticker_id.clone(), tag_id.clone()).unwrap();
    let _ = StickerTag::create(&pool, new_stk_tag_data).unwrap();

    // Unassgin
    let unassign_data = DeleteStickerTag { tag_id, sticker_id };

    let headers = vec![
        get_json_header(),
    ];
    
    let resp = basic_request(
        &app,
        "/tags/unassign",
        Method::DELETE,
        headers,
        serde_json::to_string(&unassign_data).unwrap(),
    ).await;

    expect_error(AppError::InvalidToken, resp).await;
}
