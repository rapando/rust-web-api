use actix_web::{HttpResponse, Responder, web};
use log::warn;
use serde::Deserialize;

use crate::{AppState, models};
use crate::handlers::Response;
use crate::helpers;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    first_name: String,
    last_name: String,
    department_id: u64,
    password: String,
    repeat_password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    first_name: String,
    last_name: String,
    department_id: u64,
}

pub async fn create(req: web::Json<CreateUserRequest>, data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.pool.get_conn().expect("failed to get a connection from pool");
    if !req.password.eq(&req.repeat_password) {
        warn!("passwords don't match");
        return HttpResponse::BadRequest().json(Response {
            message: String::from("Passwords don't match"),
        });
    }

    let (salt, hashed_password) = helpers::hash_password(&req.password);
    match models::user::create(&req.first_name, &req.last_name, req.department_id, &salt, &hashed_password, &mut conn) {
        Ok(_) => HttpResponse::Created().json(Response {
            message: String::from("user created"),
        }),
        Err(_) => HttpResponse::BadRequest().json(Response {
            message: String::from("failed to create user"),
        })
    }
}

pub async fn get_all(data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.pool.get_conn().expect("failed to get a connection from pool");

    match models::user::get_all(&mut conn) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::BadRequest().json(Response {
            message: String::from("unable to fetch users"),
        })
    }
}

pub async fn get_one(data: web::Data<AppState>, path: web::Path<u64>) -> impl Responder {
    let mut conn = data.pool.get_conn().expect("failed to get a connection from pool");

    let id: u64 = path.into_inner();
    match models::user::get_one(id, &mut conn) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::BadRequest().json(Response {
            message: String::from("failed to get user"),
        })
    }
}

pub async fn update(req: web::Json<UpdateUserRequest>, data: web::Data<AppState>, path: web::Path<u64>) -> impl Responder {
    let mut conn = data.pool.get_conn().expect("failed to get a connection from pool");

    let id: u64 = path.into_inner();
    match models::user::update(id, &req.first_name, &req.last_name, req.department_id, &mut conn) {
        Ok(_)=> HttpResponse::Accepted().json(Response{
            message: String::from("updated user"),
        }),
        Err(_) => HttpResponse::BadRequest().json(Response{
            message: String::from("failed to update user")
        })
    }
}