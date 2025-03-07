use crate::*;

fn create_users(pool: &DbPool, n: u16) -> Vec<User> {
    common::create_test_objects::<User>(pool, n, get_user_default)
}

#[actix_web::test]
async fn test_get_users_empty() {
    let (app, _) = get_app().await;

    let empty: Vec<User> = vec![];
    common::expect_n_elements(&app, "/users", empty).await;
}

#[actix_web::test]
async fn test_get_users() {
    let (app, pool) = get_app().await;

    let expected = create_users(&pool, rand::random::<u16>());
    common::expect_n_elements::<User>(&app, "/users", expected).await;
}
