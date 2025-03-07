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
    routes::default_match_error
};

use crate::routes::DbPool;

use actix_web::{
    delete,
    get,
    post,
    put,
    web,
    HttpResponse,
    Responder
};

#[post("")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<NewUser>,
) -> impl Responder {   
    match User::create(&pool, form.into_inner()) {
        Ok(new_user) => HttpResponse::Created().json(new_user),
        Err(e) => default_match_error(e),
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
    );
}