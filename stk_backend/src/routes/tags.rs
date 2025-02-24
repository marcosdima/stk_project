use crate::{
    models::{
        tags::{
            NewTag,
            Tag,
            TagUpdate
        },
        BasicModel,
    },
    routes::default_match_error
};
use crate::routes::DbPool;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[post("")]
async fn add_tag(
    pool: web::Data<DbPool>,
    form: web::Json<NewTag>,
) -> impl Responder {   
    match Tag::create(&pool, form.into_inner()) {
        Ok(new_tag) => HttpResponse::Created().json(new_tag),
        Err(e) => default_match_error(e),
    }
}

#[get("")]
async fn get_tags(
    pool: web::Data<DbPool>,
) -> impl Responder {
    match Tag::get_all(&pool) {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(e) => default_match_error(e),
    }
}

#[delete("/{id}")]
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
                HttpResponse::NotFound().body("Tag not found")
            }
        }
        Err(e) => default_match_error(e),
    }
}

#[put("/{name}")]
async fn update_tag(
    pool: web::Data<DbPool>,
    data: web::Json<TagUpdate>,
    curr_name: web::Path<String>,
) -> impl Responder {
    match Tag::change_name(&pool, &curr_name.into_inner(), data.into_inner().name) {
        Ok(_) => HttpResponse::Ok().body("Updated successfully"),
        Err(e) => default_match_error(e),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tags")
            .service(get_tags)
            .service(add_tag)
            .service(delete_tag)
            .service(update_tag)
    );
}