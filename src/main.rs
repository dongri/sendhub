#[macro_use]
extern crate lazy_static;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

mod app;
mod env;
mod provider;
mod utils;
mod webhook;

#[derive(Serialize, Deserialize)]
struct StatusObj {
    status: String,
}

fn status() -> HttpResponse {
    HttpResponse::Ok().json(StatusObj {
        status: String::from("OK"),
    })
}

fn main() {
    // Logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let listen = format!("{}:{}", "0.0.0.0", &env::ENV_CONFIG.port);
    println!("Listen {:?} on {:?}", listen, &env::ENV_CONFIG.name);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/api/status", web::get().to(status))
            .service(
                web::resource("/api/email/settings")
                    .route(web::post().to(app::email::settings)),
            )
            .service(
                web::resource("/api/email/raw").route(web::post()
                    .to(app::email::raw)))
            .service(
                web::resource("/api/email/template")
                    .route(web::post().to(app::email::templae)),
            )
            .service(
                web::resource("/api/slack/message")
                    .route(web::post().to(app::slack::slack_message)),
            )
    })
    .bind(listen)
    .unwrap()
    .run()
    .unwrap();
}
