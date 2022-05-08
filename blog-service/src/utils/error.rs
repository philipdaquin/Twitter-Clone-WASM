use derive_more::{Display, Error};

use actix_web::{
    error, get, http::{header::ContentType, StatusCode},
    App, HttpResponse
};

#[derive(Display, Error, Debug)]
pub enum ServiceError { 
    #[display(fmt = "Internal Error. Please Try Again Later")]
    InternalError,

    #[display(fmt = "Bad Request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout
}

impl error::ResponseError for ServiceError { 
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self { 
            ServiceError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadClientData => StatusCode::BAD_REQUEST,
            ServiceError::Timeout => StatusCode::GATEWAY_TIMEOUT
        }
    }
}