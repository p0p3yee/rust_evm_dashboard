
use actix_web::{ HttpResponse, ResponseError, http::{header::ContentType, StatusCode} };
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ApiError {
    AccountNotFound,
    AccountCreationFailure,
    AccountUpdateBadRequest,

    EndpointNotFound,
    EndpointCreationFailure,
    EndpointUpdateBadRequest,

    DatabaseInternalError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match &self {
            ApiError::AccountNotFound | ApiError::EndpointNotFound => StatusCode::NOT_FOUND,
            ApiError::AccountUpdateBadRequest | ApiError::EndpointUpdateBadRequest => StatusCode::BAD_REQUEST,
            ApiError::AccountCreationFailure | ApiError::EndpointCreationFailure => StatusCode::FAILED_DEPENDENCY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}