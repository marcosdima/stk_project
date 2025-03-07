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

    // Assigns a tag.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/tags/assign")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_stk_tag_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
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
    let new_stk_tag_data = NewStickerTag::new(stk.id.clone(), String::from("Imaginary")).unwrap();

    // Assigns a tag.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/tags/assign")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_stk_tag_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
   
    // Check creation.
    assert_eq!(StickerTag::sticker_tags(&pool, stk.id).unwrap(), vec![] as Vec<String>);
}

#[actix_web::test]
async fn test_unassign_imaginary_sticker() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_tag_data = default::get_tag_default(1);

    // Creates each model instance.
    let tag = Tag::create(&pool, new_tag_data).unwrap();

    // Sets new data models.
    let new_stk_tag_data = NewStickerTag::new(Uuid::new_v4().to_string(), tag.id.clone()).unwrap();

    // Assigns a tag.
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri("/tags/assign")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&new_stk_tag_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
   
    // Check creation.
    assert_eq!(StickerTag::tag_stickers(&pool, tag.id).unwrap(), vec![] as Vec<String>);
}

#[actix_web::test]
async fn test_unassign_tag() {
    let (app, pool) = get_app().await;

    // Get default data.
    let new_tag_data = default::get_tag_default(1);
    let new_sticker_data = default::get_sticker_default(1);

    // Creates each model instance.
    let tag = Tag::create(&pool, new_tag_data).unwrap();
    let stk = Sticker::create(&pool, new_sticker_data).unwrap();

    // Sets new data models.
    let new_stk_tag_data = NewStickerTag::new(stk.id.clone(), tag.id.clone()).unwrap();
    let _ = StickerTag::create(&pool, new_stk_tag_data).unwrap();

    // Unassgin
    let unassign_data = serde_json::json!({
        "tag_id": tag.id,
        "sticker_id": stk.id,
    });

    let req = test::TestRequest::default()
        .method(Method::DELETE)
        .uri("/tags/unassign")
        .insert_header(ContentType::json())
        .set_payload(serde_json::to_string(&unassign_data).unwrap())
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body = test::read_body(resp).await;
    println!("{:?}", body);
    //assert!(resp.status().is_success());
}

