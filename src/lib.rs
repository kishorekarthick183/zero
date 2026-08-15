use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use serde::Deserialize;
use std::net::TcpListener;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct FormData {
    #[validate(length(min = 1))]
    name: String,
    #[validate(email)]
    email: String,
}

#[derive(Debug)]
struct Subscriber {
    name: String,
    email: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    if form.validate().is_err() {
        return HttpResponse::BadRequest().finish();
    }
    let subscriber = Subscriber {
        name: form.name.clone(),
        email: form.email.clone(),
    };
    println!("New subscriber: {:?}", subscriber);
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
