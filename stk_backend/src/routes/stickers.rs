use crate::{
    models::{
        stickers::{
            NewSticker,
            Sticker,
            StickerUpdate
        },
        Model,
        BasicModel
    },
    routes::default_match_error
};
use crate::routes::DbPool;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[post("")]
async fn add_sticker(
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

#[delete("/{id}")]
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

#[put("")]
async fn update_sticker(
    pool: web::Data<DbPool>,
    data: web::Json<StickerUpdate>,
) -> impl Responder {
    match Sticker::get_by_id(&pool, data.id.to_string()) {
        Ok(_) => {
            match Sticker::update(&pool, data.into_inner()) {
                Ok(_) => HttpResponse::Ok().body("Updated successfully"),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(e) => default_match_error(e),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stickers")
            .service(get_stickers)
            .service(add_sticker)
            .service(delete_sticker)
            .service(update_sticker)
    );
}