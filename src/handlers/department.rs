use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct DepartmentResponse {
    status: String,
}

pub async fn get() -> impl Responder {
    let res = DepartmentResponse {
        status: String::from("ok"),
    };

    HttpResponse::Ok().json(res)
}
