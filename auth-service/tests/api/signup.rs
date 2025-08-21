use crate::helpers::{get_random_email, TestApp};

// #[tokio::test]
// async fn signup_returns_200() {
//     let app = TestApp::new().await;
//     let response = app.post_signup().await;
//     assert_eq!(response.status().as_u16(), 200);
// }

#[tokio::test]
async fn should_return_422_if_malformed_input() {
let app = TestApp::new().await;

let random_email = get_random_email();

    // TODO: add more malformed input test cases

let test_cases = [
      // Email eksik
      serde_json::json!({
          "password": "password123",
          "requires2FA": true,
      }),
      // Password eksik
      serde_json::json!({
          "email": random_email.clone(),
          "requires2FA": true,
      }),
      // requires2FA eksik
      serde_json::json!({
          "email": random_email.clone(),
          "password": "password123",
      }),
      // Geçersiz email format
      serde_json::json!({
          "email": "invalid-email",
          "password": "password123",
          "requires2FA": true,
      }),
      // Boş email
      serde_json::json!({
          "email": "",
          "password": "password123",
          "requires2FA": true,
      }),
      // Boş password
      serde_json::json!({
          "email": random_email.clone(),
          "password": "",
          "requires2FA": true,
      }),
  ];

for test_case in test_cases.iter() {
    let response = app.post_signup(test_case).await;

    assert_eq!(
        response.status().as_u16(),
        422,
        "Failed for input: {:?}",
        test_case
    );
}
}