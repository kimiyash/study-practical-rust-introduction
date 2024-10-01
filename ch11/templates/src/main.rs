use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error};
use tera::{Tera, Context};

async fn render_template(name: web::Path<String>, tera: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert("name", &(name.into_inner()));

    // テンプレートをレンダリングし、結果を返す
    let rendered = tera.render("index.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template rendering error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone())) 
            .route("/{name}", web::get().to(render_template))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
