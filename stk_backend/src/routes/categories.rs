use crate::models::{Model, categories::{Category, CategoryUpdate, NewCategory}};
use crate::routes::DbPool;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[post("")]
async fn add_category(
    pool: web::Data<DbPool>,
    form: web::Json<NewCategory>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
    
    match Category::create(conn, form.into_inner()) {
        Ok(new_category) => HttpResponse::Created().json(new_category),
        Err(e) => {
            log::error!("Failed to insert category: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to insert category")
        }
    }
}

#[get("")]
async fn get_categories(
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Couldn't get DB connection from pool");

    match Category::get_all(conn) {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{id}")]
async fn delete_category(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
    let category_id = path.into_inner();

    match Category::delete(conn, &category_id) {
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
    let conn = &mut pool.get().expect("Couldn't get DB connection from pool");

    match Category::get_by_id(conn, &data.id.to_string()) {
        Ok(_) => {
            match Category::update(conn, data.into_inner()) {
                Ok(_) => HttpResponse::Ok().body("Updated successfully"),
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