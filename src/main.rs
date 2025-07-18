mod controller;
mod service;
mod dto;
mod error;

use actix_web::{App, HttpServer};
use controller::location_controller::location_from_ip;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(location_from_ip)
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
