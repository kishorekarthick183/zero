use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use serde::Deserialize;
use std::net::TcpListener;
use validator::Validate;
pub mod configuration;
use sqlx::PgPool;
pub mod telemetry;
use tracing_actix_web::TracingLogger;

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
    if let Err(e) = form.validate() {
        tracing::warn!(
            error = ?e,
            "Invalid subscriber data"
        );
        return HttpResponse::BadRequest().finish();
    }
    let subscriber = Subscriber {
        name: form.name.clone(),
        email: form.email.clone(),
    };
    match insert_subscriber(subscriber, &connection_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!(
                error = ?e,
                "Failed to insert subscriber"
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn insert_subscriber(
    subscriber: Subscriber,
    connection_pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (email, name)
        VALUES ($1, $2)
        "#,
        subscriber.email,
        subscriber.name
    )
    .execute(connection_pool)
    .await?;

    Ok(())
}

pub fn run(listener: TcpListener, connection_pool: PgPool) -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(connection_pool.clone()))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
