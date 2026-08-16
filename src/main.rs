use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, run, telemetry};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let configuration = get_configuration().expect("failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to connection pool");
    tracing::info!("Database connection established");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
