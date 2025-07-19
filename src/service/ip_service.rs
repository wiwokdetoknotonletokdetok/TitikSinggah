use crate::config::WebConfig;
use crate::dto::location_data::{ApiResponse, LocationData};
use crate::error::IPLocationError;
use actix_web::HttpRequest;
use reqwest::Client;

#[derive(Clone)]
pub struct IpService {
    client: Client,
    config: WebConfig,
}

impl IpService {
    pub fn new(client: Client, config: WebConfig) -> Self {
        Self { client, config }
    }

    pub async fn get_client_ip(req: &HttpRequest) -> Option<String> {
        if let Some(forwarded) = req.headers().get("X-Forwarded-For") {
            if let Ok(forwarded_str) = forwarded.to_str() {
                let ip = forwarded_str.split(',').next().unwrap().trim();
                return Some(ip.to_string());
            }
        }

        req.peer_addr().map(|addr| addr.ip().to_string())
    }

    pub async fn get_ip_location(&self, ip_address: &str) -> Result<LocationData, IPLocationError> {
        let url = format!("{}/{}", self.config.ip_geolocation_url, ip_address);

        let resp = self.client.get(&url).send()
            .await
            .map_err(|e| IPLocationError::ServiceError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(IPLocationError::ServiceError(format!("Failed request with status {}", resp.status())));
        }

        let api_resp: ApiResponse = resp.json().await.map_err(|e| IPLocationError::ServiceError(e.to_string()))?;

        if api_resp.status != "success" {
            return Err(IPLocationError::NotFound);
        }

        Ok(LocationData {
            country: api_resp.country,
            region: api_resp.regionName,
            city: api_resp.city,
            latitude: api_resp.lat,
            longitude: api_resp.lon,
        })
    }
}
