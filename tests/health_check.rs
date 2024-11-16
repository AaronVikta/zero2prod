
// tokio::test is the testing equivalent of tokio::main
//  It also spares us from having to specify the 
// #[test] attribute

// We can inspect what code gets generated usinng 
// cargo expand --test health_check

use actix_web::Result;

#[tokio::test]

async fn health_check_works(){
    // Arrange 
    spawn_app().await.expect("Failed to spawn our app");
    // We need to bring in reqwest to perform HTTP 
    // requests against our application

    let client = reqwest::Client::new();
    let response = client.get("http://127.0.0.1:8000/health_check")
    .send()
    .await.
    expect("Failed to send HTTP request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


async fn spawn_app()-> Result<(), std::io::Error>{
    todo!()
}