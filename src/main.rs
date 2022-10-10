use actix_web::{App, HttpServer, web};
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
            .route("/", web::get().to(handlers::home::get))
            .route("/departments", web::post().to(handlers::department::create))
            .route("/departments", web::get().to(handlers::department::get_all))
            .route("/departments/{id:[0-9]+$}", web::get().to(handlers::department::get_one))
            .route("/departments/{id:[0-9]+$}", web::put().to(handlers::department::update))
    })
        .bind((host, port))?
        .run()
        .await
}
