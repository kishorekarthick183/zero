mod helpers;

#[actix_web::test]
async fn health_check_works() {
    let app = helpers::spawn_app();
    let response = reqwest::get(format!("{}/health_check", app.address))
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
}
