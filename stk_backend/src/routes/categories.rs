use crate::{
    models::{
        categories::{
            Category,
            CategoryUpdate,
            NewCategory,
        }, 
        sticker_category::{
            NewStickerCategory,
            StickerCategory,
        },
        BasicModel,
        Model
    },
    routes::default_match_error,
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
async fn add_category(
    pool: web::Data<DbPool>,
    form: web::Json<NewCategory>,
) -> impl Responder { 
    match Category::create(&pool, form.into_inner()) {
        Ok(new_category) => HttpResponse::Created().json(new_category),
        Err(e) => default_match_error(e),
    }
}

#[post("/assign")]
async fn assign_category(
    pool: web::Data<DbPool>,
    form: web::Json<NewStickerCategory>,
) -> impl Responder {
    match StickerCategory::create(&pool, form.into_inner()) {
        Ok(new_obj) => HttpResponse::Created().json(new_obj),
        Err(e) => default_match_error(e),
    }
}

#[get("")]
async fn get_categories(
    pool: web::Data<DbPool>,
) -> impl Responder {
    match Category::get_all(&pool) {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => default_match_error(e),
    }
}

#[get("/{id}")]
async fn get_category(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let category_id = path.into_inner();

    match Category::get_by_id(&pool, category_id) {
        Ok(ids) => HttpResponse::Ok().json(ids),
        Err(e) => default_match_error(e),
    }
}


#[get("/{id}/stickers")]
async fn get_stickers(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let category_id = path.into_inner();

    match StickerCategory::category_stickers(&pool, category_id) {
        Ok(ids) => HttpResponse::Ok().json(ids),
        Err(e) => default_match_error(e),
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
        Err(e) => default_match_error(e),
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
                Err(e) => default_match_error(e),
            }
        }
        Err(_) => HttpResponse::NotFound().body("Category not found"),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .service(get_categories)
            .service(get_category)
            .service(add_category)
            .service(delete_category)
            .service(update_category)
            .service(assign_category)
            .service(get_stickers)
    );
}