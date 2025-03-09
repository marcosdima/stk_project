use stk_backend::utils::{
    hash_password,
    verify_password,
};

#[actix_web::test]
async fn test_hash_password() {
    let password = "PASSWORD";
    
    let hassed = hash_password(password).unwrap();

    assert_eq!(verify_password(password, &hassed), true);
    assert_eq!(verify_password("no-password", &hassed), false);
}
