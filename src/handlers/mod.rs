use serde::Serialize;

pub mod department;
pub mod home;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

