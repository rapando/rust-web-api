use serde::Serialize;

pub mod department;
pub mod home;
pub mod user;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

