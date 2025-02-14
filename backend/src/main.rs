mod models;
mod schema;

use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::{r2d2, SqliteConnection};
use models::Sticker;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/create")]
async fn add_sticker(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewSticker>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");
    
    match Sticker::create(conn, form.into_inner()) {
        Ok(new_sticker) => HttpResponse::Created().json(new_sticker),
        Err(e) => {
            log::error!("Failed to insert sticker: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to insert sticker")
        }
    }

    
}

#[get("/stickers")]
async fn get_stickers(
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Couldn't get DB connection from pool");

    match Sticker::get_all(conn) {
        Ok(stickers) => HttpResponse::Ok().json(stickers),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = initialize_db_pool();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(add_sticker)
            .service(get_stickers)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}