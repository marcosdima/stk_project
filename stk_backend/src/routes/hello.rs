use actix_web::{get, web, HttpResponse, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}