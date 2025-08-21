mod helpers;
mod login;
mod logout;
mod signup;
mod verify_2fa;
mod verify_token;
mod root;

use auth_service::Application;

#[tokio::main]
async fn main() {
    let app = Application::build("0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}