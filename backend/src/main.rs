use backend::{create_app, state::AppState};

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind server");

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}