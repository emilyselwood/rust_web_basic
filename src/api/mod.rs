use actix_web::{Scope, web};

pub fn build_api() -> Scope {
    let result = web::scope("/api");

    result
}
