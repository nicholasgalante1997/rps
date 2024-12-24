use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::time::Duration,
    middleware::{self, Logger},
    web, App, HttpServer,
};
use dotenv::dotenv;
use env_logger::Env;

mod models;
mod services;

use services::{api, openid_configuration, pods};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let secret_key = actix_web::cookie::Key::generate();
    let redis_store = RedisSessionStore::new("redis://0.0.0.0:6379")
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-API-RPS-Version", "0.1")))
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(2)))
                    .build(),
            )
            .service(web::scope("/api").service(api::v1::auth::login))
            .service(
                web::scope("/.well-known")
                    .service(openid_configuration::v1::openid_configuration_post_handler),
            )
            .service(pods::v1::resource_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
