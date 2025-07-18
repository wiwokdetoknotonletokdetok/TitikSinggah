use serde::{Serialize};

#[derive(Serialize)]
pub struct WebResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<String>,
}

impl<T> WebResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            errors: None,
        }
    }

    pub fn error(msg: String) -> Self {
        Self {
            data: None,
            errors: Some(msg),
        }
    }
}
