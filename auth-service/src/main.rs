use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .nest_service("/hello", get(hello_handler));

    // app.run().await.expect("Failed to run app");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler() -> Html<&'static str> {
    // TODO: Update this to a custom message!
    // Done!
    Html("<h1>Hello, I'm GP!</h1>")
}
