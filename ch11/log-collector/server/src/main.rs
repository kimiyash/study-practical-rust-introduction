use actix_web::{web, web::Data, App, HttpServer}; // Dataを正しくインポート
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;

mod db;
mod handlers;
mod model;
mod schema;

// アプリケーションで共有する状態
#[derive(Clone)]
pub struct Server {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Server {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Server { pool }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::handlers::*;

    // 環境変数でログレベルを設定
    env_logger::init();

    let server = Server::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone())) // 状態を共有するためにapp_dataを使用
            .route("/logs", web::post().to(handle_post_logs))
            .route("/csv", web::post().to(handle_post_csv))
            .route("/csv", web::get().to(handle_get_csv))
            .route("/logs", web::get().to(handle_get_logs))
    })
    .bind("localhost:3000")?
    .run()
    .await
}
