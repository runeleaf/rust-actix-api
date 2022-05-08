use crate::presentation::message_controller;
use crate::server::middleware::create_context::RequestContext;
use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::http::header;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use std::env;

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").expect("8080");
    let server = env::var("SERVER").expect("127.0.0.1");
    let secret_key = Key::generate();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_origin, _req_head| true)
            //.allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(RequestContext::new()))
            .service(root)
            .service(ping)
            .service(message_controller::index)
            .service(message_controller::show)
            .service(message_controller::create)
            .service(message_controller::update)
            .service(message_controller::delete)
    })
    .bind(format!("{}:{}", server, port))?
    //.bind("127.0.0.1:3000")?
    .run()
    .await
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("root")
}

#[get("/_healthcheck")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
