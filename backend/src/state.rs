use std::{
    collections::HashMap,
    sync::Arc,
};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::model::{
    CachedUserTasks, EmailLog, LoginChallenge, Task, User,
};

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<String, User>>>,
    pub tasks: Arc<RwLock<HashMap<Uuid, Task>>>,
    pub challenges: Arc<RwLock<HashMap<Uuid, LoginChallenge>>>,
    pub cache: Arc<RwLock<HashMap<String, CachedUserTasks>>>,
    pub email_logs: Arc<RwLock<Vec<EmailLog>>>,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            tasks: Arc::new(RwLock::new(HashMap::new())),
            challenges: Arc::new(RwLock::new(HashMap::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
            email_logs: Arc::new(RwLock::new(Vec::new())),
            jwt_secret: "development-secret-key-change-in-production".to_string(),
        }
    }
}