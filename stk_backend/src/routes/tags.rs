use crate::{
    models::{
        sticker_tag::{
            DeleteStickerTag,
            NewStickerTag,
            StickerTag
        },
        tags::{
            NewTag,
            Tag,
            TagUpdate,
        },
        BasicModel,
        Model,
    },
    routes::default_match_error,
    utils::{
        resource,
        errors::AppError,
    },
};

use crate::routes::DbPool;

use actix_web::{
    get,
    web,
    HttpResponse,
    Responder,
};

#[get("")]
async fn get_tags(
    pool: web::Data<DbPool>,
) -> impl Responder {
    match Tag::get_all(&pool) {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(e) => default_match_error(e),
    }
}

async fn create_tag(
    pool: web::Data<DbPool>,
    form: web::Json<NewTag>,
) -> impl Responder {   
    match Tag::create(&pool, form.into_inner()) {
        Ok(new_tag) => HttpResponse::Created().json(new_tag),
        Err(e) => default_match_error(e),
    }
}

async fn assign_tag(
    pool: web::Data<DbPool>,
    form: web::Json<NewStickerTag>,
) -> impl Responder {
    let data = form.into_inner();

    match Tag::get_by_id(&pool, data.tag_id.clone()) {
        Ok(_) => {
            match StickerTag::create(&pool, data) {
                Ok(assigned) => HttpResponse::Created().json(assigned),
                Err(e) => default_match_error(e),
            }
        },
        Err(e) => default_match_error(e),
    }
}

async fn delete_tag(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let tag_id = path.into_inner();

    match Tag::delete(&pool, tag_id) {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                HttpResponse::Ok().body("Tag deleted successfully")
            } else {
                default_match_error(AppError::NotFound("Tag not found"))
            }
        }
        Err(e) => default_match_error(e),
    }
}

async fn unassign_tag(
    pool: web::Data<DbPool>,
    form: web::Json<DeleteStickerTag>,
) -> impl Responder {
    let data = form.into_inner();
    match StickerTag::delete(&pool, (data.tag_id, data.sticker_id)) {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                HttpResponse::Ok().body("Tag deleted successfully")
            } else {
                HttpResponse::NotFound().body("Sticker-Tag not found")
            }
        }
        Err(e) => default_match_error(e),
    }
}

async fn update_tag(
    pool: web::Data<DbPool>,
    data: web::Json<TagUpdate>,
    id: web::Path<String>,
) -> impl Responder {
    match Tag::get_by_id(&pool, id.into_inner()) {
        Ok(_) => {
            match Tag::update(&pool, data.into_inner()) {
                Ok(_) => HttpResponse::Ok().body("Updated successfully"),
                Err(e) => default_match_error(e),
            }
        },
        Err(e) => default_match_error(e),
    }  
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let create = resource::post("", create_tag);
    let assign = resource::assign("/assign", assign_tag);
    
    let unassign = resource::unassign("/unassign", unassign_tag);

    // TODO: This did not work when both were just '/{id}'.
    let delete = resource::delete("/delete", delete_tag);
    let update = resource::update("/update", update_tag);
    let scope = web::scope("/{id}")
        .service(delete)
        .service(update);

    cfg.service(
        web::scope("/tags")
            .service(get_tags)
            .service(create)
            .service(unassign)
            .service(assign)
            .service(scope)
    );
}
