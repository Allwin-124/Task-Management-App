use axum::{
    extract::State,
    http::HeaderMap,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use rand::Rng;
use serde_json::json;
use uuid::Uuid;

use crate::{
    auth::{authenticate, create_jwt, require_admin},
    errors::AppError,
    model::{
        AssignTasksRequest, CacheMetadata, CachedUserTasks, CreateTaskRequest, EmailLog,
        LoginChallenge, LoginRequest, LoginResponse, Role, Task, TokenResponse, User,
        UserSimpleInfo, UserTaskSummary, Verify2FARequest, ViewMyTasksResponse,
    },
    state::AppState,
};

pub async fn seed_users(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut users = state.users.write().await;
    let mut cache = state.cache.write().await;
    let mut tasks = state.tasks.write().await;
    let mut challenges = state.challenges.write().await;

    cache.clear();
    tasks.clear();
    challenges.clear();

    let admin = User {
        id: Uuid::new_v4(),
        full_name: "Admin".to_string(),
        email: "admin@example.com".to_string(),
        password_hash: hash("Password123!", DEFAULT_COST)
            .map_err(|e| AppError::Internal(e.to_string()))?,
        role: Role::Admin,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    let james = User {
        id: Uuid::new_v4(),
        full_name: "James Bond".to_string(),
        email: "jamesbond@example.com".to_string(),
        password_hash: hash("Password123!", DEFAULT_COST)
            .map_err(|e| AppError::Internal(e.to_string()))?,
        role: Role::Staff,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    users.insert(admin.email.clone(), admin);
    users.insert(james.email.clone(), james);

    Ok(Json(json!({
        "message": "Users seeded successfully",
        "admin_email": "admin@example.com",
        "jamesbond_email": "jamesbond@example.com"
    })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let users = state.users.read().await;

    let user = users
        .get(&request.email)
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    let password_valid = verify(&request.password, &user.password_hash)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if !password_valid {
        return Err(AppError::Unauthorized("Invalid email or password".to_string()));
    }

    let user_id = user.id;
    let user_email = user.email.clone();
    drop(users);

    let challenge_id = Uuid::new_v4();
    let code: u32 = rand::rng().random_range(100000..999999);
    let code_str = code.to_string();
    let expires_at = Utc::now() + chrono::Duration::minutes(5);

    let challenge = LoginChallenge {
        id: challenge_id,
        user_id,
        email: user_email.clone(),
        code: code_str.clone(),
        expires_at,
        used: false,
    };

    {
        let mut challenges = state.challenges.write().await;
        challenges.insert(challenge_id, challenge);
    }

    let email_log = EmailLog {
        id: Uuid::new_v4(),
        recipient_email: user_email,
        login_challenge_id: challenge_id,
        code: code_str.clone(),
        sent_at: Utc::now().to_rfc3339(),
    };

    {
        let mut logs = state.email_logs.write().await;
        logs.push(email_log);
    }

    println!("[DEV EMAIL LOG] 2FA code generated for challenge {}: {}", challenge_id, code_str);

    Ok(Json(LoginResponse {
        login_challenge_id: challenge_id,
        message: "2FA code generated and sent to email log".to_string(),
    }))
}

pub async fn latest_email_log(
    State(state): State<AppState>,
) -> Result<Json<EmailLog>, AppError> {
    let logs = state.email_logs.read().await;

    let latest = logs
        .last()
        .cloned()
        .ok_or_else(|| AppError::NotFound("No email logs available".to_string()))?;

    Ok(Json(latest))
}

pub async fn verify_2fa(
    State(state): State<AppState>,
    Json(request): Json<Verify2FARequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let mut challenges = state.challenges.write().await;

    let challenge = challenges
        .get_mut(&request.login_challenge_id)
        .ok_or_else(|| AppError::Unauthorized("Invalid or missing 2FA challenge ID".to_string()))?;

    if challenge.used {
        return Err(AppError::Unauthorized("2FA code has already been used".to_string()));
    }

    if challenge.expires_at < Utc::now() {
        return Err(AppError::Unauthorized("2FA code has expired (valid for 5 minutes)".to_string()));
    }

    if challenge.code != request.code {
        return Err(AppError::Unauthorized("Incorrect 2FA verification code".to_string()));
    }

    challenge.used = true;
    let user_id = challenge.user_id;
    let challenge_id = request.login_challenge_id;
    
    // Remove used challenge
    challenges.remove(&challenge_id);
    drop(challenges);

    let users = state.users.read().await;
    let user = users
        .values()
        .find(|u| u.id == user_id)
        .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

    let access_token = create_jwt(user, &state.jwt_secret)?;

    Ok(Json(TokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
    }))
}

pub async fn create_task(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CreateTaskRequest>,
) -> Result<Json<Task>, AppError> {
    let claims = authenticate(&headers, &state)?;
    require_admin(&claims)?;

    let now = Utc::now().to_rfc3339();
    let task = Task {
        id: Uuid::new_v4(),
        title: request.title,
        description: request.description,
        status: request.status.unwrap_or_else(|| "todo".to_string()),
        priority: request.priority.unwrap_or_else(|| "medium".to_string()),
        created_by: claims.email,
        assigned_to: None,
        created_at: now.clone(),
        updated_at: now,
    };

    let task_id = task.id;
    {
        let mut tasks = state.tasks.write().await;
        tasks.insert(task_id, task.clone());
    }

    Ok(Json(task))
}

pub async fn list_all_tasks(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<Task>>, AppError> {
    let _claims = authenticate(&headers, &state)?;
    let tasks = state.tasks.read().await;
    let task_list: Vec<Task> = tasks.values().cloned().collect();
    Ok(Json(task_list))
}

pub async fn assign_tasks(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<AssignTasksRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let claims = authenticate(&headers, &state)?;
    require_admin(&claims)?;

    let users = state.users.read().await;
    if !users.contains_key(&request.user_email) {
        return Err(AppError::NotFound(format!("User '{}' not found", request.user_email)));
    }
    drop(users);

    let mut tasks = state.tasks.write().await;
    for task_id in &request.task_ids {
        if !tasks.contains_key(task_id) {
            return Err(AppError::NotFound(format!("Task with ID '{}' not found", task_id)));
        }
    }

    let now = Utc::now().to_rfc3339();
    for task_id in &request.task_ids {
        if let Some(task) = tasks.get_mut(task_id) {
            task.assigned_to = Some(request.user_email.clone());
            task.updated_at = now.clone();
        }
    }
    drop(tasks);

    // Invalidate task cache for the target assigned user
    {
        let mut cache = state.cache.write().await;
        cache.remove(&request.user_email);
    }

    Ok(Json(json!({
        "message": "Tasks assigned successfully",
        "assigned_count": request.task_ids.len(),
        "assigned_to": request.user_email
    })))
}

pub async fn view_my_tasks(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ViewMyTasksResponse>, AppError> {
    let claims = authenticate(&headers, &state)?;

    let user_role = match claims.role.as_str() {
        "admin" => Role::Admin,
        _ => Role::Staff,
    };

    // Check Cache
    {
        let cache = state.cache.read().await;
        if let Some(cached_data) = cache.get(&claims.email) {
            return Ok(Json(ViewMyTasksResponse {
                user: UserSimpleInfo {
                    email: claims.email.clone(),
                    role: user_role,
                },
                tasks: cached_data.tasks.clone(),
                summary: UserTaskSummary {
                    total_assigned_tasks: cached_data.tasks.len(),
                },
                cache: CacheMetadata { hit: true },
            }));
        }
    }

    // Cache Miss - Query Tasks assigned to this user
    let tasks = state.tasks.read().await;
    let my_tasks: Vec<Task> = tasks
        .values()
        .filter(|t| t.assigned_to.as_deref() == Some(&claims.email))
        .cloned()
        .collect();
    drop(tasks);

    // Populate Cache
    {
        let mut cache = state.cache.write().await;
        cache.insert(
            claims.email.clone(),
            CachedUserTasks {
                tasks: my_tasks.clone(),
                user_email: claims.email.clone(),
                user_role: user_role.clone(),
            },
        );
    }

    Ok(Json(ViewMyTasksResponse {
        user: UserSimpleInfo {
            email: claims.email.clone(),
            role: user_role,
        },
        tasks: my_tasks.clone(),
        summary: UserTaskSummary {
            total_assigned_tasks: my_tasks.len(),
        },
        cache: CacheMetadata { hit: false },
    }))
}
