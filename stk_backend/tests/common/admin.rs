use stk_backend::{
    models::{
        user_role::{
            NewUserRole,
            UserRole,
        },
        users::User,
        BasicModel,
    },
    routes::DbPool,
    utils::generate_token,
};

use crate::get_user_default;

fn create_admin_user(pool: &DbPool) -> String {
    // Create admin user and set their role.
    match User::get_by_username(pool, "Administrator".to_owned()) {
        Ok(founded) => founded.id,
        Err(_) => {
            let mut user_data = get_user_default(666);
            user_data.username = "Administrator".to_owned();
            let user = User::create(&pool, user_data).expect("Test Error: admin user creation");
            let _ = UserRole::create(
                &pool,
                NewUserRole::new(user.id.clone(), 1)
            ).expect("Test Error: UserRole creation");
            user.id
        }
    }
}

pub fn get_admin_token(pool: &DbPool) -> String {
    let user_id = &create_admin_user(pool);
    generate_token(user_id).expect("Test Error: token generation")
}
