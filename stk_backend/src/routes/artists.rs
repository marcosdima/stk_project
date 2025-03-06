use crate::{
    models::{
        artist_sticker::{
            ArtistSticker,
            NewArtistSticker
        },
        artists::{
            Artist,
            ArtistUpdate,
            NewArtist
        },
        BasicModel,
        Model
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
async fn add_artist(
    pool: web::Data<DbPool>,
    form: web::Json<NewArtist>,
) -> impl Responder {   
    match Artist::create(&pool, form.into_inner()) {
        Ok(new_artist) => HttpResponse::Created().json(new_artist),
        Err(e) => default_match_error(e),
    }
}

#[post("/sticker")]
async fn assign_sticker(
    pool: web::Data<DbPool>,
    form: web::Json<NewArtistSticker>,
) -> impl Responder {   
    match ArtistSticker::create(&pool, form.into_inner()) {
        Ok(new) => HttpResponse::Created().json(new),
        Err(e) => default_match_error(e),
    }
}

#[get("")]
async fn get_artists(
    pool: web::Data<DbPool>,
) -> impl Responder {
    match Artist::get_all(&pool) {
        Ok(artists) => HttpResponse::Ok().json(artists),
        Err(e) => default_match_error(e),
    }
}

#[get("/{arts_id}/stickers")]
async fn get_artist_stickers(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    match Artist::get_by_id(&pool, id) {
        Ok(artist) => {
            match ArtistSticker::artist_stickers(&pool, artist.id) {
                Ok(artists) => HttpResponse::Ok().json(artists),
                Err(e) => default_match_error(e),
            }
        },
        Err(e) => default_match_error(e),
    }
}

#[delete("/{id}")]
async fn delete_artist(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let artist_id = path.into_inner();

    match Artist::delete(&pool, artist_id) {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                HttpResponse::Ok().body("Artist deleted successfully")
            } else {
                HttpResponse::NotFound().body("Artist not found")
            }
        }
        Err(e) => default_match_error(e),
    }
}

#[put("")]
async fn update_artist(
    pool: web::Data<DbPool>,
    data: web::Json<ArtistUpdate>,
) -> impl Responder {
    match Artist::get_by_id(&pool, data.id.to_string()) {
        Ok(_) => {
            match Artist::update(&pool, data.into_inner()) {
                Ok(_) => HttpResponse::Ok().body("Updated successfully"),
                Err(e) => default_match_error(e),
            }
        }
        Err(e) => default_match_error(e),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/artists")
            .service(get_artists)
            .service(get_artist_stickers)
            .service(add_artist)
            .service(delete_artist)
            .service(update_artist)
            .service(assign_sticker)
    );
}
