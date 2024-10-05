use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Responder};
use std::path::PathBuf;

async fn serve_file(file_name: web::Path<String>) -> impl Responder {
    let path: PathBuf = std::env::current_dir()
        .unwrap()
        .join(file_name.into_inner());
    NamedFile::open(path)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/{file_name}", web::get().to(serve_file)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
