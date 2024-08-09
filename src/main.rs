use std::env;

use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::time::Duration, get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod models;
mod services;

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

    // Generate a secret key for the session,
    // this is used to encrypt the session cookie
    // In production, you should use a more secure method to generate this key, pulled from the environment
    let secret_key = actix_web::cookie::Key::generate();
    // let secret = env::var("RPS_SECRET_SESSION_KEY").expect("Secret Key must be set to use session storage.");

    // Create a new RedisSessionStore, this is used to store the session data in Redis
    // We may need to change this to 0.0.0.0:6379 to connect to the redis service in docker
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(
                // Configure the session middleware
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(2)))
                    .build(),
            )
            .service(web::scope("/api").service())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
