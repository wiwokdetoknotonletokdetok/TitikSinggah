mod controller;
mod service;
mod dto;
mod error;

use actix_web::{web::Data, App, HttpServer};
use controller::location_controller::location_from_ip;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let client = reqwest::Client::new();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .service(location_from_ip)
    })
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}
