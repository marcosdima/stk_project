use crate::{
    errors::AppError, models::{
        artist_sticker::{
            ArtistSticker,
            GetArtistSticker,
            NewArtistSticker,
        }, artists::{
            Artist,
            ArtistUpdate,
            NewArtist,
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

async fn create_artist(
    pool: web::Data<DbPool>,
    form: web::Json<NewArtist>,
) -> impl Responder {   
    match Artist::create(&pool, form.into_inner()) {
        Ok(new_artist) => HttpResponse::Created().json(new_artist),
        Err(e) => default_match_error(e),
    }
}

async fn assign_sticker(
    pool: web::Data<DbPool>,
    form: web::Json<NewArtistSticker>,
) -> impl Responder {
    let data = form.into_inner();
    match Artist::get_by_id(&pool, data.artist_id.clone()) {
        Ok(_)=> {
            match Sticker::get_by_id(&pool, data.sticker_id.clone()) {
                Ok(_) => {
                    match ArtistSticker::create(&pool, data) {
                        Ok(new) => HttpResponse::Created().json(new),
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
    form: web::Json<GetArtistSticker>,
) -> impl Responder {   
    match ArtistSticker::get(&pool, form.into_inner()) {
        Ok(found) => {
            match ArtistSticker::delete(&pool, (found.sticker_id, found.artist_id)) {
                Ok(_) => HttpResponse::Ok().body("Deleted successfully"),
                Err(e) => default_match_error(e),
            }
        },
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
                default_match_error(AppError::NotFound("Artist not found"))
            }
        }
        Err(e) => default_match_error(e),
    }
}

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
    let create = resource::post("", create_artist);
    let assign = resource::post("/sticker/assign", assign_sticker);
    
    let unassign = resource::delete("/sticker/unassign", unassign_sticker);

    let delete = resource::delete("/{id}/delete", delete_artist);
    let update = resource::update("/update", update_artist);
    
    cfg.service(
        web::scope("/artists")
            .service(get_artists)
            .service(get_artist_stickers)
            .service(create)
            .service(assign)
            .service(delete)
            .service(update)
            .service(unassign)
    );
}
