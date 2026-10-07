use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use backend::{
    create_app,
    model::{
        AssignTasksRequest, CreateTaskRequest, EmailLog, LoginRequest, LoginResponse, Task,
        TokenResponse, Verify2FARequest, ViewMyTasksResponse,
    },
    state::AppState,
};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_full_application_workflow() {
    let state = AppState::new();
    let app = create_app(state);

    // 1. Seed Users
    let req = Request::builder()
        .method("POST")
        .uri("/seed/users")
        .header("content-type", "application/json")
        .body(Body::empty())
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // 2. Login as Admin
    let login_admin_body = serde_json::to_vec(&LoginRequest {
        email: "admin@example.com".to_string(),
        password: "Password123!".to_string(),
    })
    .unwrap();

    let req = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(login_admin_body))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let admin_login_res: LoginResponse = serde_json::from_slice(&body_bytes).unwrap();
    let admin_challenge_id = admin_login_res.login_challenge_id;

    // 3. Get Admin 2FA Code
    let req = Request::builder()
        .method("GET")
        .uri("/dev/email-logs/latest")
        .body(Body::empty())
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let admin_email_log: EmailLog = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(admin_email_log.login_challenge_id, admin_challenge_id);

    // 4. Verify Admin 2FA
    let verify_admin_body = serde_json::to_vec(&Verify2FARequest {
        login_challenge_id: admin_challenge_id,
        code: admin_email_log.code,
    })
    .unwrap();

    let req = Request::builder()
        .method("POST")
        .uri("/auth/verify-2fa")
        .header("content-type", "application/json")
        .body(Body::from(verify_admin_body))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let admin_token_res: TokenResponse = serde_json::from_slice(&body_bytes).unwrap();
    let admin_jwt = admin_token_res.access_token;

    // 5. Create 5 Tasks as Admin
    let priorities = vec!["high", "high", "medium", "medium", "low"];
    let mut task_ids = Vec::new();

    for i in 1..=5 {
        let task_req_body = serde_json::to_vec(&CreateTaskRequest {
            title: format!("Task {}", i),
            description: format!("Description for task {}", i),
            priority: Some(priorities[i - 1].to_string()),
            status: Some("todo".to_string()),
        })
        .unwrap();

        let req = Request::builder()
            .method("POST")
            .uri("/tasks")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", admin_jwt))
            .body(Body::from(task_req_body))
            .unwrap();

        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
        let created_task: Task = serde_json::from_slice(&body_bytes).unwrap();
        task_ids.push(created_task.id);
    }
    assert_eq!(task_ids.len(), 5);

    // 6. Login as James Bond
    let login_bond_body = serde_json::to_vec(&LoginRequest {
        email: "jamesbond@example.com".to_string(),
        password: "Password123!".to_string(),
    })
    .unwrap();

    let req = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(login_bond_body))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let bond_login_res: LoginResponse = serde_json::from_slice(&body_bytes).unwrap();
    let bond_challenge_id = bond_login_res.login_challenge_id;

    // 7. Get James Bond 2FA Code
    let req = Request::builder()
        .method("GET")
        .uri("/dev/email-logs/latest")
        .body(Body::empty())
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let bond_email_log: EmailLog = serde_json::from_slice(&body_bytes).unwrap();

    // 8. Verify James Bond 2FA
    let verify_bond_body = serde_json::to_vec(&Verify2FARequest {
        login_challenge_id: bond_challenge_id,
        code: bond_email_log.code,
    })
    .unwrap();

    let req = Request::builder()
        .method("POST")
        .uri("/auth/verify-2fa")
        .header("content-type", "application/json")
        .body(Body::from(verify_bond_body))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let bond_token_res: TokenResponse = serde_json::from_slice(&body_bytes).unwrap();
    let bond_jwt = bond_token_res.access_token;

    // 9. James Bond attempts to create task -> MUST return 403 Forbidden!
    let illegal_task_body = serde_json::to_vec(&CreateTaskRequest {
        title: "Unauthorized Task".to_string(),
        description: "Should fail".to_string(),
        priority: None,
        status: None,
    })
    .unwrap();

    let req = Request::builder()
        .method("POST")
        .uri("/tasks")
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", bond_jwt))
        .body(Body::from(illegal_task_body))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // 10. Admin assigns 3 tasks to James Bond
    let assigned_task_ids = vec![task_ids[0], task_ids[1], task_ids[2]];
    let assign_body = serde_json::to_vec(&AssignTasksRequest {
        task_ids: assigned_task_ids.clone(),
        user_email: "jamesbond@example.com".to_string(),
    })
    .unwrap();

    let req = Request::builder()
        .method("POST")
        .uri("/tasks/assign")
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", admin_jwt))
        .body(Body::from(assign_body))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // 11. View James Bond's tasks (1st call: cache.hit = false)
    let req = Request::builder()
        .method("GET")
        .uri("/tasks/view-my-tasks")
        .header("authorization", format!("Bearer {}", bond_jwt))
        .body(Body::empty())
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let view_res_1: ViewMyTasksResponse = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(view_res_1.user.email, "jamesbond@example.com");
    assert_eq!(view_res_1.summary.total_assigned_tasks, 3);
    assert_eq!(view_res_1.tasks.len(), 3);
    assert_eq!(view_res_1.cache.hit, false);

    // 12. View James Bond's tasks (2nd call: cache.hit = true)
    let req = Request::builder()
        .method("GET")
        .uri("/tasks/view-my-tasks")
        .header("authorization", format!("Bearer {}", bond_jwt))
        .body(Body::empty())
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let view_res_2: ViewMyTasksResponse = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(view_res_2.summary.total_assigned_tasks, 3);
    assert_eq!(view_res_2.cache.hit, true);
}

#[tokio::test]
async fn test_invalid_and_reused_2fa_codes() {
    let state = AppState::new();
    let app = create_app(state);

    // Seed
    let req = Request::builder()
        .method("POST")
        .uri("/seed/users")
        .body(Body::empty())
        .unwrap();
    let _ = app.clone().oneshot(req).await.unwrap();

    // Login
    let login_body = serde_json::to_vec(&LoginRequest {
        email: "admin@example.com".to_string(),
        password: "Password123!".to_string(),
    })
    .unwrap();

    let req = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(login_body))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let login_res: LoginResponse = serde_json::from_slice(&body_bytes).unwrap();

    // 1. Try wrong code -> 401 Unauthorized
    let wrong_verify_body = serde_json::to_vec(&Verify2FARequest {
        login_challenge_id: login_res.login_challenge_id,
        code: "000000".to_string(),
    })
    .unwrap();

    let req = Request::builder()
        .method("POST")
        .uri("/auth/verify-2fa")
        .header("content-type", "application/json")
        .body(Body::from(wrong_verify_body))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // 2. Fetch correct code
    let req = Request::builder()
        .method("GET")
        .uri("/dev/email-logs/latest")
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let email_log: EmailLog = serde_json::from_slice(&body_bytes).unwrap();

    // 3. Verify correct code -> 200 OK
    let correct_verify_body = serde_json::to_vec(&Verify2FARequest {
        login_challenge_id: login_res.login_challenge_id,
        code: email_log.code.clone(),
    })
    .unwrap();

    let req = Request::builder()
        .method("POST")
        .uri("/auth/verify-2fa")
        .header("content-type", "application/json")
        .body(Body::from(correct_verify_body))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // 4. Try reusing same challenge ID & code -> 401 Unauthorized
    let reuse_verify_body = serde_json::to_vec(&Verify2FARequest {
        login_challenge_id: login_res.login_challenge_id,
        code: email_log.code,
    })
    .unwrap();

    let req = Request::builder()
        .method("POST")
        .uri("/auth/verify-2fa")
        .header("content-type", "application/json")
        .body(Body::from(reuse_verify_body))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
