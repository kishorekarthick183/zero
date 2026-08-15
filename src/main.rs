use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to connection pool");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // TODO: try with single underscore
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
