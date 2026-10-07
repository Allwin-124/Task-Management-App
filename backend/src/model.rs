use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Staff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: Role,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub created_by: String,
    pub assigned_to: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub login_challenge_id: Uuid,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verify2FARequest {
    pub login_challenge_id: Uuid,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
    pub priority: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignTasksRequest {
    pub task_ids: Vec<Uuid>,
    pub user_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginChallenge {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub code: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub used: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailLog {
    pub id: Uuid,
    pub recipient_email: String,
    pub login_challenge_id: Uuid,
    pub code: String,
    pub sent_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSimpleInfo {
    pub email: String,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserTaskSummary {
    pub total_assigned_tasks: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheMetadata {
    pub hit: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewMyTasksResponse {
    pub user: UserSimpleInfo,
    pub tasks: Vec<Task>,
    pub summary: UserTaskSummary,
    pub cache: CacheMetadata,
}

#[derive(Debug, Clone)]
pub struct CachedUserTasks {
    pub tasks: Vec<Task>,
    pub user_email: String,
    pub user_role: Role,
}