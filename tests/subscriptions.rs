mod helpers;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = helpers::spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .form(&[("name", "John Doe"), ("email", "john@example.com")])
        .send()
        .await
        .expect("failed to execute request");
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = helpers::spawn_app();
    let client = reqwest::Client::new();
    let test_cases = [
        [("name", ""), ("email", "john@example.com")],
        [("name", "John Doe"), ("email", "")],
    ];
    for test_case in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app.address))
            .form(&test_case)
            .send()
            .await
            .expect("Failed to execute request");
        assert_eq!(400, response.status().as_u16());
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_email_is_invalid() {
    let app = helpers::spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .form(&[("name", "John Doe"), ("email", "not-an-email")])
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(400, response.status().as_u16());
}
