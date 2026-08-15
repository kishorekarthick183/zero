mod helpers;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = helpers::spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/subscriptions", app_address))
        .form(&[("name", "John Doe"), ("email", "john@example.com")])
        .send()
        .await
        .expect("failed to execute request");
    assert_eq!(200, response.status().as_u16());
}
