use actix_web::{get, web, HttpRequest, HttpResponse, Responder, ResponseError};
use crate::service::IpService;
use crate::dto::{LocationData, WebResponse};

#[get("/locations/me")]
pub async fn location_from_ip(req: HttpRequest, ip_service: web::Data<IpService>) -> impl Responder {
    match IpService::get_client_ip(&req).await {
        Some(ip) => match ip_service.get_ip_location(&ip).await {
            Ok(location) => HttpResponse::Ok().json(WebResponse::success(location)),
            Err(e) => e.error_response(),
        },
        None => HttpResponse::BadRequest().json(WebResponse::<LocationData>::error("Could not determine client IP".to_string())),
    }
}
