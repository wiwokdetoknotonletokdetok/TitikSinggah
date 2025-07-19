use std::env;
use actix_cors::Cors;
use actix_web::http::header;

#[derive(Clone)]
pub struct WebConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_headers: Vec<header::HeaderName>,
    pub ip_geolocation_url: String,
}

impl WebConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let allowed_headers = env::var("CORS_ALLOWED_HEADERS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|s| header::HeaderName::try_from(s.trim()).ok())
            .collect();

        let ip_geolocation_url = env::var("IP_GEOLOCATION_URL")
            .expect("IP_GEOLOCATION_URL must be set in .env");

        WebConfig {
            allowed_origins,
            allowed_headers,
            ip_geolocation_url,
        }
    }

    pub fn build_cors(&self) -> Cors {
        let mut cors = Cors::default();

        for origin in &self.allowed_origins {
            cors = cors.allowed_origin(origin);
        }

        cors.allowed_methods(vec!["GET"])
            .allowed_headers(self.allowed_headers.clone())
    }
}
