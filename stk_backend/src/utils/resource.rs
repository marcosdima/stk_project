use actix_web::{
    dev::HttpServiceFactory,
    middleware::from_fn,
    web,
    Responder,
};

use super::middleware::*;

pub fn delete<H, R>(
    path: &str,
    handler: H
) -> impl HttpServiceFactory 
where
    H: actix_web::Handler<R> + Clone + 'static,
    R: 'static + actix_web::FromRequest,
    <H as actix_web::Handler<R>>::Output: Responder,
{
    web::resource(path)
        .wrap(from_fn(restrict_delete)) 
        .route(web::delete().to(handler))
}

pub fn post<H, R>(
    path: &str,
    handler: H
) -> impl HttpServiceFactory 
where
    H: actix_web::Handler<R> + Clone + 'static,
    R: 'static + actix_web::FromRequest,
    <H as actix_web::Handler<R>>::Output: Responder,
{
    web::resource(path)
        .wrap(from_fn(restrict_create)) 
        .route(web::post().to(handler))
}

pub fn update<H, R>(
    path: &str,
    handler: H
) -> impl HttpServiceFactory 
where
    H: actix_web::Handler<R> + Clone + 'static,
    R: 'static + actix_web::FromRequest,
    <H as actix_web::Handler<R>>::Output: Responder,
{
    web::resource(path)
        .wrap(from_fn(restrict_update)) 
        .route(web::put().to(handler))
}

pub fn assign<H, R>(
    path: &str,
    handler: H
) -> impl HttpServiceFactory 
where
    H: actix_web::Handler<R> + Clone + 'static,
    R: 'static + actix_web::FromRequest,
    <H as actix_web::Handler<R>>::Output: Responder,
{
    web::resource(path)
        .wrap(from_fn(restrict_assign_role)) 
        .route(web::post().to(handler))
}
