use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
}

pub fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener
        .local_addr()
        .expect("failed to get local address")
        .port();
    let server = zero2prod::run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
    }
}
