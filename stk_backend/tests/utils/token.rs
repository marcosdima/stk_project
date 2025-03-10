use stk_backend::utils::{
    generate_token,
    validate_token
};

#[actix_web::test]
async fn test_generate_token() {
    dotenvy::dotenv().ok();
    
    // Target id.
    let user_id = "ID 1";

    // Generate id token.
    let token = generate_token(user_id).unwrap();

    // Get claim from token.
    let claim = validate_token(&token).unwrap();    

    println!("{token}");

    assert_eq!(user_id, claim.sub);
}

#[actix_web::test]
async fn test_generate_token_invalid_token() {
    dotenvy::dotenv().ok();
    
    // Generate id token.
    let token = "NO-TOKEN";

    // Expects an error when tries to get a claim from an invalid token.
    if let Ok(_) = validate_token(&token) {
        panic!("claim must be an error, but it does not...");
    }
}
