mod helpers;
use helpers::spawn_app;

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app();
    let response = reqwest::get(format!("{}/health_check", address))
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
}
