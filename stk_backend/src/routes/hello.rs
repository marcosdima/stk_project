use actix_web::{
    get,
    web,
    HttpResponse,
    Responder,
    dev::{
        ServiceRequest,
        ServiceResponse,
    },
    body::MessageBody,
    Error,
    middleware::{
        from_fn,
        Next,
    },
};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn print(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    println!("Hello route: {}", req.path());
    next.call(req).await
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .wrap(from_fn(print))
        .service(hello);

    cfg.service(scope);
}