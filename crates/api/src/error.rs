use actix_web::HttpResponse;
use actix_web::ResponseError;
use store::StoreError;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Forbidden(&'static str),
    Internal(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "not found"),
            Self::Forbidden(msg) => write!(f, "{msg}"),
            Self::Internal(msg) => write!(f, "{msg}"),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::NotFound => HttpResponse::NotFound().finish(),
            Self::Forbidden(msg) => HttpResponse::Forbidden().body(*msg),
            Self::Internal(msg) => HttpResponse::InternalServerError().body(msg.clone()),
        }
    }
}

impl From<StoreError> for ApiError {
    fn from(e: StoreError) -> Self {
        match e {
            StoreError::NotFound => Self::NotFound,
            e => Self::Internal(e.to_string()),
        }
    }
}
