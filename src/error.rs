use actix_web::error::ResponseError;
use thiserror::Error;

/// Wrapper type around anyhow errors so that we can use them through actic_web as response errors.
/// Magic error conversion happens when using the ? operator on anyhow errors when the Result type is of
/// WebError. This means we don't have to to the translation manually on every call site.
#[derive(Error, Debug)]
pub enum WebError {
    #[error("{0}")]
    AnyhowWrapper(#[from] anyhow::Error),
}

impl ResponseError for WebError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let message = self.to_string();
        actix_web::HttpResponse::build(self.status_code()).body(message)
    }
}
