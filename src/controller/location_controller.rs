use actix_web::{get, web, HttpRequest, HttpResponse, Responder, ResponseError};
use crate::service::ip_service::{get_client_ip, get_ip_location};
use crate::dto::{LocationData, WebResponse};

#[get("/locations/me")]
pub async fn location_from_ip(req: HttpRequest, client: web::Data<reqwest::Client>) -> impl Responder {
    match get_client_ip(&req).await {
        Some(ip) => match get_ip_location(&client, &ip).await {
            Ok(location) => HttpResponse::Ok().json(WebResponse::success(location)),
            Err(e) => e.error_response(),
        },
        None => HttpResponse::BadRequest().json(WebResponse::<LocationData>::error("Could not determine client IP".to_string())),
    }
}
