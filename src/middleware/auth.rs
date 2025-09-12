use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    http::HeaderMap,
};
use uuid::Uuid;
use crate::{AppError, utils::extract_user_id_from_token};
use todo_api::AppState;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let token = extract_token_from_headers(&parts.headers)?;
        let user_id = extract_user_id_from_token(&token, &state.config.jwt_secret)?;
        
        Ok(AuthenticatedUser { user_id })
    }
}

fn extract_token_from_headers(headers: &HeaderMap) -> Result<String, AppError> {
    let auth_header = headers
        .get("authorization")
        .ok_or(AppError::Unauthorized)?
        .to_str()
        .map_err(|_| AppError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized);
    }

    Ok(auth_header[7..].to_string())
}