use std::net::TcpListener;
use zero2prod::run;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to a random port");
    let port = listener
        .local_addr()
        .expect("failed to get local add")
        .port();
    let server = run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app();
    let response = reqwest::get(format!("{}/health_check", address))
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
}
