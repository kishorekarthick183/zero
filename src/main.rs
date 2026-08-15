use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration");
    let connect_pool = PgPoolOptions::new()
        .connect_lazy(&configuration.database.connection_string())
        .expect("failed to connect to connection pool");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // TODO: try with single underscore
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
