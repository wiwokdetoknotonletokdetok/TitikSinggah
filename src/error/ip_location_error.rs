use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum IPLocationError {
    #[display("IP Location Service Error: {}", _0)]
    ServiceError(String),

    #[display("IP not found")]
    NotFound,
}

impl ResponseError for IPLocationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            IPLocationError::ServiceError(msg) => {
                HttpResponse::InternalServerError().json(crate::dto::WebResponse::<()>::error(msg.clone()))
            }
            IPLocationError::NotFound => {
                HttpResponse::NotFound().json(crate::dto::WebResponse::<()>::error("IP not found".to_string()))
            }
        }
    }
}
