use crate::{
    models::{
        users::{
            NewUser,
            User,
            UserUpdate,
        },
        BasicModel,
        Model,
    },
    routes::default_match_error, utils::{self, verify_password}
};

use crate::{
    routes::DbPool,
    utils::hash_password,
};

use actix_web::{
    delete,
    get,
    post,
    put,
    web,
    HttpResponse,
    Responder
};

use serde::{
    Deserialize,
    Serialize,
};
#[derive(Deserialize, Serialize)]
struct LogIn {
    username: String,
    password: String,
}
#[post("/login")]
async fn login(
    pool: web::Data<DbPool>,
    form: web::Json<LogIn>,
) -> impl Responder {
    let data = form.into_inner();

    // Finds user by its username.
    match User::get_by_username(&pool, data.username) {
        Ok(user) => {
            // Compare password from data with user password_hash.
            if verify_password(&data.password, &user.password_hash) {
                // Try to generate a token.
                match utils::generate_token(&user.id) {
                    Ok(token) => HttpResponse::Ok().body(token),
                    Err(e) => default_match_error(e),
                }
            } else {
                HttpResponse::Unauthorized().body("Wrong username-password")
            }
        },
        Err(e) => default_match_error(e),
    }  
}

#[post("")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<NewUser>,
) -> impl Responder {
    let mut data = form.into_inner();

    match hash_password(&data.password_hash) {
        Ok(hash) => {
            data.password_hash = hash;
            match User::create(&pool, data) {
                Ok(new_user) => HttpResponse::Created().json(new_user),
                Err(e) => default_match_error(e),
            }
        },
        Err(e) => default_match_error(e.into()),
    }
}

#[get("")]
async fn get_users(
    pool: web::Data<DbPool>,
) -> impl Responder {
    match User::get_all(&pool) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => default_match_error(e),
    }
}

#[delete("/{id}")]
async fn delete_user(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = path.into_inner();

    match User::delete(&pool, user_id) {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                HttpResponse::Ok().body("User deleted successfully")
            } else {
                HttpResponse::NotFound().body("User not found")
            }
        }
        Err(e) => default_match_error(e),
    }
}

#[put("")]
async fn update_user(
    pool: web::Data<DbPool>,
    data: web::Json<UserUpdate>,
) -> impl Responder {
    match User::get_by_id(&pool, data.id.to_string()) {
        Ok(_) => {
            match User::update(&pool, data.into_inner()) {
                Ok(_) => HttpResponse::Ok().body("Updated successfully"),
                Err(e) => default_match_error(e),
            }
        },
        Err(e) => default_match_error(e),
    }  
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_users)
            .service(add_user)
            .service(delete_user)
            .service(update_user)
            .service(login)
    );
}