use serde::Serialize;
use actix_web::{HttpResponse, Responder};

#[derive(Serialize)]
pub struct DepartmentResponse {
    status: String,
}

pub async fn get() -> impl Responder {
HttpResponse::Ok().json(DepartmentResponse{status: "ok"})
}