use crate::{models::Sticker, DbPool};
use actix_web::{get, post, web, HttpResponse, Responder};

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

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stickers")
            .service(get_stickers)
            .service(add_sticker)
    );
}