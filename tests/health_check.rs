use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let fail_text = "Failed to bind random port";
    let listener = TcpListener::bind("127.0.0.1:0").expect(fail_text);
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = actix_web::rt::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
