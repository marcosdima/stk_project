use crate::{
    models::{
        categories::{
            Category,
            CategoryUpdate,
            NewCategory,
        }, sticker_category::{
            GetStickerCategory,
            NewStickerCategory,
            StickerCategory,
        }, stickers::Sticker, BasicModel, Model
    },
    routes::default_match_error,
    utils::resource,
};
use crate::routes::DbPool;

use actix_web::{
    get,
    web,
    HttpResponse,
    Responder,
};

async fn create_category(
    pool: web::Data<DbPool>,
    form: web::Json<NewCategory>,
) -> impl Responder { 
    match Category::create(&pool, form.into_inner()) {
        Ok(new_category) => HttpResponse::Created().json(new_category),
        Err(e) => default_match_error(e),
    }
}

async fn assign_category(
    pool: web::Data<DbPool>,
    form: web::Json<NewStickerCategory>,
) -> impl Responder {
    let data = form.into_inner();
    match Category::get_by_id(&pool, data.category_id.clone()) {
        Ok(_)=> {
            match Sticker::get_by_id(&pool, data.sticker_id.clone()) {
                Ok(_) => {
                    match StickerCategory::create(&pool, data) {
                        Ok(new_obj) => HttpResponse::Created().json(new_obj),
                        Err(e) => default_match_error(e),
                    }
                },
                Err(e) => default_match_error(e),
            }
        },
        Err(e) => default_match_error(e),
    }
}

async fn unassign_sticker(
    pool: web::Data<DbPool>,
    form: web::Json<GetStickerCategory>,
) -> impl Responder {   
    match StickerCategory::get(&pool, form.into_inner()) {
        Ok(found) => {
            match StickerCategory::delete(&pool, (found.sticker_id, found.category_id)) {
                Ok(_) => HttpResponse::Ok().body("Deleted successfully"),
                Err(e) => default_match_error(e),
            }
        },
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
    let create = resource::post("", create_category);
    let assign = resource::post("/sticker/assign", assign_category);
    
    let unassign = resource::delete("/sticker/unassign", unassign_sticker);

    let delete = resource::delete("/{id}", delete_category);
    let update = resource::update("/update", update_category);

    cfg.service(
        web::scope("/categories")
            .service(get_categories)
            .service(get_category)
            .service(get_stickers)
            .service(create)
            .service(update)
            .service(assign)
            .service(unassign)
            .service(delete)
    );
}