use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::time::Duration,
    get,
    middleware::{self, Logger},
    post, web, App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use env_logger::Env;

mod models;
mod services;

use services::{api, pods};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
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
            .service(pods::v1::resource_handler)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
