use actix_web::{middleware, web, App, HttpServer};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use tera::Tera;
use actix_cors::Cors;

mod models;
mod handlers;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = db::init_db()
        .await
        .expect("Database connection failed");

    let tera = Tera::new("templates/**/*")
        .expect("Template parsing failed");

    let secret_key = Key::generate();

    println!("Server running at http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
            .wrap(middleware::Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .service(handlers::auth::register)
            .service(handlers::auth::login)
            .service(handlers::user::profile)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}