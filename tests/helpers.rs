use sqlx::PgPool;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let configuration =
        zero2prod::configuration::get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&configuration.test_database.connection_string())
        .await
        .expect("failed to connect to Postgres");
    sqlx::query("TRUNCATE TABLE subscriptions")
        .execute(&db_pool)
        .await
        .expect("Failed to clean subscriptions table");
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener
        .local_addr()
        .expect("failed to get local address")
        .port();
    let server = zero2prod::run(listener, db_pool.clone()).expect("failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool,
    }
}
