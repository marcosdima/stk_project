use crate::{models::Sticker, DbPool};
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[post("")]
async fn add_sticker(
    pool: web::Data<DbPool>,
    form: web::Json<crate::models::NewSticker>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
    
    match Sticker::create(conn, form.into_inner()) {
        Ok(new_sticker) => HttpResponse::Created().json(new_sticker),
        Err(e) => {
            log::error!("Failed to insert sticker: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to insert sticker")
        }
    }
}

#[get("")]
async fn get_stickers(
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Couldn't get DB connection from pool");

    match Sticker::get_all(conn) {
        Ok(stickers) => HttpResponse::Ok().json(stickers),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{id}")]
async fn delete_sticker(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
    let sticker_id = path.into_inner();

    match Sticker::delete(conn, sticker_id) {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                HttpResponse::Ok().body("Sticker deleted successfully")
            } else {
                HttpResponse::NotFound().body("Sticker not found")
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stickers")
            .service(get_stickers)
            .service(add_sticker)
            .service(delete_sticker)
    );
}