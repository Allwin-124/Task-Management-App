use axum::http::HeaderMap;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError,
    model::{Role, User},
    state::AppState,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub role: String,
    pub exp: usize,
}

pub fn create_jwt(
    user: &User,
    secret: &str,
) -> Result<String, AppError> {
    let expiration = Utc::now() + Duration::hours(24);

    let role_str = match user.role {
        Role::Admin => "admin",
        Role::Staff => "staff",
    };

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        role: role_str.to_string(),
        exp: expiration.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Failed to create token: {}", e)))
}

pub fn authenticate(
    headers: &HeaderMap,
    state: &AppState,
) -> Result<Claims, AppError> {
    let authorization = headers
        .get("Authorization")
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;

    let authorization = authorization
        .to_str()
        .map_err(|_| AppError::Unauthorized("Invalid Authorization header".to_string()))?;

    let token = authorization
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Bearer token required".to_string()))?;

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("Invalid or expired JWT token".to_string()))?;

    Ok(decoded.claims)
}

pub fn require_admin(
    claims: &Claims,
) -> Result<(), AppError> {
    if claims.role != "admin" {
        return Err(AppError::Forbidden(
            "Forbidden: Only Admin users can perform this action".to_string(),
        ));
    }

    Ok(())
}