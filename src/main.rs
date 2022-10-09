use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;

mod handlers;
mod helpers;
mod models;

pub struct AppState {
    pub pool: mysql::Pool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    helpers::init_logger();
    let db_pool = helpers::connect_to_db();

    let host = String::from("0.0.0.0");
    let port: u16 = helpers::read_env("PORT").parse().unwrap();

    let app_data = web::Data::new(AppState { pool: db_pool });

    info!("starting api on port {}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/departments", web::get().to(handlers::department::get))
    })
    .bind((host, port))?
    .run()
    .await
}
