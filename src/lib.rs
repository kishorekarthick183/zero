use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use serde::Deserialize;
use std::net::TcpListener;
use validator::Validate;
pub mod configuration;
use sqlx::PgPool;

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

pub struct AppState {
    // We'll add the database connection here later.
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(form: web::Form<FormData>, connection_pool: web::Data<PgPool>) -> HttpResponse {
    if form.validate().is_err() {
        return HttpResponse::BadRequest().finish();
    }
    let subscriber = Subscriber {
        name: form.name.clone(),
        email: form.email.clone(),
    };
    insert_subscriber(subscriber, &connection_pool).await;
    HttpResponse::Ok().finish()
}

async fn insert_subscriber(subscriber: Subscriber, connection_pool: &PgPool) {
    println!("Saving subscriber: {:?}", subscriber);
    // Database INSERT will come here.
}

pub fn run(listener: TcpListener, connection_pool: PgPool) -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(connection_pool.clone()))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
