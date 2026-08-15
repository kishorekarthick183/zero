use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn spawn_app() -> (Server, String) {
    // why use TcpListener?
    // 1. binds to available port.
    // 2. listens continuously.
    // 3. handshakes automatically.
    // 4. converts requests into data streams.

    // 0 to decide by Operating system
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to a random port");
    let port = listener
        .local_addr()
        .expect("failed to get local add")
        .port();
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)
        .expect("failed to bind server")
        .run();
    let address = format!("http://127.0.0.1:{}", port);
    (server, address)
}

#[actix_web::test]
async fn health_check_works() {
    let (server, address) = spawn_app();
    let _server_handle = actix_web::rt::spawn(server);
    let response = reqwest::get(format!("{}/health_check", address))
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
}
