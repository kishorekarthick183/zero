use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use serde::Deserialize;
use std::net::TcpListener;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    if form.name.is_empty() || form.email.is_empty() {
        return HttpResponse::BadRequest().finish();
    }
    println!("New subscriber: {} <{}>", form.name, form.email);
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
