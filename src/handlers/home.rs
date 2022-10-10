use actix_web::{HttpResponse, Responder};
use chrono::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct HomeResponse {
    pub status: String,
    pub time: String,
}

pub async fn get() -> impl Responder {
    let now = Utc::now();
    let res: HomeResponse = HomeResponse {
        status: String::from("OK"),
        time: now.format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    HttpResponse::Ok().json(res)
}