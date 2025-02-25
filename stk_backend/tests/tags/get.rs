use crate::*;

fn create_tags(pool: &DbPool, n: u16) -> Vec<Tag> {
    common::create_test_objects::<Tag>(pool, n, get_tag_default)
}

#[actix_web::test]
async fn test_get_tags_empty() {
    let (app, _) = get_app().await;

    let empty: Vec<Tag> = vec![];
    common::expect_n_elements(&app, "/tags", empty).await;
}

#[actix_web::test]
async fn test_get_tags() {
    let (app, pool) = get_app().await;

    let expected = create_tags(&pool, rand::random::<u16>());
    common::expect_n_elements::<Tag>(&app, "/tags", expected).await;
}
