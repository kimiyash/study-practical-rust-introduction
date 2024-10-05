use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: Option<String>, // クエリパラメータとして「name」がオプション
}

async fn hello(query: web::Query<Info>) -> impl Responder {
    // name が存在しない場合はデフォルトメッセージを表示
    let name = query.name.clone().unwrap_or_else(|| "World".to_string());
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
