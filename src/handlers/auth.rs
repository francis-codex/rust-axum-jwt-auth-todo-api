use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use validator::Validate;
use todo_api::AppState;
use crate::{
    AppError,
    models::{CreateUserRequest, LoginRequest, AuthResponse, UserResponse, User},
    utils::{hash_password, verify_password, create_token},
};

pub async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Check if user already exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1 OR username = $2"
    )
    .bind(&payload.email)
    .bind(&payload.username)
    .fetch_optional(&app_state.pool)
    .await?;

    if existing_user.is_some() {
        return Err(AppError::UserAlreadyExists);
    }

    // Hash the password
    let password_hash = hash_password(&payload.password)?;

    // Insert new user
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .fetch_one(&app_state.pool)
    .await?;

    // Generate JWT token
    let token = create_token(user.id, &app_state.config.jwt_secret, app_state.config.jwt_expiration_hours)?;

    let response = AuthResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Find user by email
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&app_state.pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    // Verify password
    let is_valid = verify_password(&payload.password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::Unauthorized);
    }

    // Generate JWT token
    let token = create_token(user.id, &app_state.config.jwt_secret, app_state.config.jwt_expiration_hours)?;

    let response = AuthResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok(Json(response))
}