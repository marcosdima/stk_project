use crate::{
    models::{
        sticker_category::StickerCategory,
        stickers::{
            NewSticker,
            Sticker,
            StickerUpdate
        },
        BasicModel,
        Model
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

async fn create_sticker(
    pool: web::Data<DbPool>,
    form: web::Json<NewSticker>,
) -> impl Responder {   
    match Sticker::create(&pool, form.into_inner()) {
        Ok(new_sticker) => HttpResponse::Created().json(new_sticker),
        Err(e) => default_match_error(e),
    }
}

#[get("")]
async fn get_stickers(
    pool: web::Data<DbPool>,
) -> impl Responder {
    match Sticker::get_all(&pool) {
        Ok(stickers) => HttpResponse::Ok().json(stickers),
        Err(e) => default_match_error(e),
    }
}

#[get("/{id}/categories")]
async fn get_sticker_categories(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let sticker_id = path.into_inner();

    match StickerCategory::sticker_categories(&pool, sticker_id) {
        Ok(ids) => HttpResponse::Ok().json(ids),
        Err(e) => default_match_error(e),
    }
}

async fn delete_sticker(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let sticker_id = path.into_inner();

    match Sticker::delete(&pool, sticker_id) {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                HttpResponse::Ok().body("Sticker deleted successfully")
            } else {
                HttpResponse::NotFound().body("Sticker not found")
            }
        }
        Err(e) => default_match_error(e),
    }
}

async fn update_sticker(
    pool: web::Data<DbPool>,
    data: web::Json<StickerUpdate>,
) -> impl Responder {
    match Sticker::get_by_id(&pool, data.id.to_string()) {
        Ok(_) => {
            match Sticker::update(&pool, data.into_inner()) {
                Ok(_) => HttpResponse::Ok().body("Updated successfully"),
                Err(e) => default_match_error(e),
            }
        }
        Err(e) => default_match_error(e),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let create = resource::post("", create_sticker);
    let delete = resource::delete("/{id}/delete", delete_sticker);
    let update = resource::update("/update", update_sticker);

    cfg.service(
        web::scope("/stickers")
            .service(get_stickers)
            .service(get_sticker_categories)
            .service(delete)
            .service(update)
            .service(create)
    );
}