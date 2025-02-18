use actix_web::{middleware, web, App, HttpServer};
use stk_backend::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = routes::initialize_db_pool();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(
        move || {
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                .configure(routes::hello::configure)
                .configure(routes::stickers::configure)
                .configure(routes::categories::configure)
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
