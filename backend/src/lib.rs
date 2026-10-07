pub mod auth;
pub mod errors;
pub mod handlers;
pub mod model;
pub mod state;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use handlers::{
    assign_tasks, create_task, latest_email_log, list_all_tasks, login, seed_users, verify_2fa,
    view_my_tasks,
};
use state::AppState;

pub fn create_app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    Router::new()
        .route("/seed/users", post(seed_users))
        .route("/auth/login", post(login))
        .route("/dev/email-logs/latest", get(latest_email_log))
        .route("/auth/verify-2fa", post(verify_2fa))
        .route("/tasks", post(create_task).get(list_all_tasks))
        .route("/tasks/assign", post(assign_tasks))
        .route("/tasks/view-my-tasks", get(view_my_tasks))
        .layer(cors)
        .with_state(state)
}
