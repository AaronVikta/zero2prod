
// tokio::test is the testing equivalent of tokio::main
//  It also spares us from having to specify the 
// #[test] attribute

// We can inspect what code gets generated usinng 
// cargo expand --test health_check

use std::net::TcpListener;






fn spawn_app()-> String{
    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");
let port = listener.local_addr().unwrap().port();


    let server= zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]

async fn health_check_works(){
    // Arrange 
    let address = spawn_app();
    // We need to bring in reqwest to perform HTTP 
    // requests against our application

    let client = reqwest::Client::new();

    let response = client
    .get(&format!("{}/health_check", &address))
    .send()
    .await
    .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]

async fn subscribe_returns_a_200_for_valid_form_data(){
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=Aryan&email=aryan@hotmail.com";

    let response = client
    .post(&format!("{}/subscriptions", &app_address))
    .body(body)
    .send()
    .await
    .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_form_returns_400_when_data_is_missing(){
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=Aryan", "Mising the email"),
        ("email=Aryan@hotmail.com", "Missing the name"),
        ("", "Missing both name and email"),
    ];

    for(invalid_body, error_message) in test_cases{
        // Act
        let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute resquest");

        // Assert 
        assert_eq!(
            400,
            response.status().as_u16(),

            // Additional Customised error on test failure

            "The API did not fail with a 400 Bad Request {}",
            error_message
        );
    }
}