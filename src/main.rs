use actix_web::{web, App, HttpServer};

use rusttest::config::read_config;
use rusttest::infra;
use rusttest::modules::music;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = read_config();

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .configure(infra::endpoints::config)
                .configure(music::infra::endpoints::config),
        )
    })
    .bind((cfg.host, cfg.port))?
    .run()
    .await
}
