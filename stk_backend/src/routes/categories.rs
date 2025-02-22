use crate::{errors::AppError, models::{categories::{Category, CategoryUpdate, NewCategory}, Model}};
use crate::routes::DbPool;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[post("")]
async fn add_category(
    pool: web::Data<DbPool>,
    form: web::Json<NewCategory>,
) -> impl Responder { 
    match Category::create(&pool, form.into_inner()) {
        Ok(new_category) => HttpResponse::Created().json(new_category),
        Err(AppError::NotFound(err)) => HttpResponse::NotFound().body(format!("Error: {err}")),
        Err(_) => HttpResponse::InternalServerError().body("Failed to insert category"),
    }
}

#[get("")]
async fn get_categories(
    pool: web::Data<DbPool>,
) -> impl Responder {
    match Category::get_all(&pool) {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{id}")]
async fn delete_category(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let category_id = path.into_inner();

    match Category::delete(&pool, category_id) {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                HttpResponse::Ok().body("Category deleted successfully")
            } else {
                HttpResponse::NotFound().body("Category not found")
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("")]
async fn update_category(
    pool: web::Data<DbPool>,
    data: web::Json<CategoryUpdate>,
) -> impl Responder {
    match Category::get_by_id(&pool, data.id.to_string()) {
        Ok(_) => {
            match Category::update(&pool, data.into_inner()) {
                Ok(_) => HttpResponse::Ok().body("Updated successfully"),
                Err(AppError::InvalidData(err)) => HttpResponse::BadRequest().body(format!("Error: {err}")),
                Err(AppError::NotFound(err)) => HttpResponse::NotFound().body(format!("Error: {err}")),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::NotFound().body("Category not found"),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .service(get_categories)
            .service(add_category)
            .service(delete_category)
            .service(update_category)
    );
}