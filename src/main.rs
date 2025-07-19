mod config;
mod controller;
mod service;
mod dto;
mod error;

use actix_web::{web::Data, App, HttpServer};
use config::WebConfig;
use controller::location_controller::location_from_ip;
use service::IpService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = WebConfig::from_env();
    let client = reqwest::Client::new();
    let ip_service = IpService::new(client.clone(), config.clone());

    HttpServer::new(move || {
        let cors = config.build_cors();
        App::new()
            .wrap(cors)
            .app_data(Data::new(ip_service.clone()))
            .service(location_from_ip)
    })
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}
