use sqlx::PgPool;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let db_pool = PgPool::connect("postgres://postgres:postgres@127.0.0.1:5432/newsletter_test")
        .await
        .expect("failed to connect to Postgres");
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
