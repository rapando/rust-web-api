use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use crate::{AppState, models};
use crate::handlers::Response;

#[derive(Deserialize)]
pub struct CreateDepartmentRequest {
    name: String,
    staff_count: Option<u16>,
}

#[derive(Serialize)]
pub struct CreateDepartmentResponse {
    id: u64,
    success: bool,
    message: String,
}

pub async fn create(req: web::Json<CreateDepartmentRequest>, data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.pool.get_conn().expect("failed to get a connection pool");
    let staff_count:u16 = 0;
    match models::department::create(&req.name, staff_count, &mut conn) {
        Ok(id) => HttpResponse::Created().json(CreateDepartmentResponse {
            id,
            success: true,
            message: String::from("department created"),
        }),
        Err(_) => HttpResponse::BadRequest().json(Response {
            message: String::from("failed to create department"),
        })
    }
}


pub async fn get_all(data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.pool.get_conn().expect("failed to get a connection pool");
    match models::department::get_all(&mut conn) {
        Ok(departments) => HttpResponse::Ok().json(departments),
        Err(_) => HttpResponse::BadRequest().json(Response {
            message: String::from("unable to fetch departments"),
        })
    }
}

pub async fn get_one(data: web::Data<AppState>, path: web::Path<u64>) -> impl Responder {
    let mut conn = data.pool.get_conn().expect("failed to get a connection pool");
    let id: u64 = path.into_inner();
    match models::department::get_one(id, &mut conn) {
        Some(department) => HttpResponse::Ok().json(department),
        None => HttpResponse::NotFound().json(Response {
            message: String::from("failed to get department"),
        })
    }
}

pub async fn update(req: web::Json<CreateDepartmentRequest>, data: web::Data<AppState>, path: web::Path<u64>) -> impl Responder {
    let mut conn = data.pool.get_conn().expect("failed to get a connection from db pool");
    let id: u64 = path.into_inner();
    let staff_count = match req.staff_count {
        Some(c) => c,
        None => {
            return HttpResponse::BadRequest().json(Response{
                message:String::from( "staff_count is required"),
            });
        }
    };
    match models::department::update(id, &req.name, staff_count, &mut conn) {
        Ok(_) => HttpResponse::Accepted().json(Response {
            message: String::from("Ok"),
        }),
        Err(_) => HttpResponse::BadRequest().json(Response {
            message: String::from("Failed to update department")
        })
    }
}