use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;
use todo_api::AppState;
use crate::{
    AppError,
    middleware::AuthenticatedUser,
    models::{CreateTodoRequest, UpdateTodoRequest, TodoResponse, Todo},
};

pub async fn get_todos(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT * FROM todos WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(user.user_id)
    .fetch_all(&app_state.pool)
    .await?;

    let todo_responses: Vec<TodoResponse> = todos.into_iter().map(TodoResponse::from).collect();
    Ok(Json(todo_responses))
}

pub async fn create_todo(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<TodoResponse>), AppError> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (user_id, title, description) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(user.user_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .fetch_one(&app_state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(TodoResponse::from(todo))))
}

pub async fn update_todo(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
    Path(todo_id): Path<Uuid>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, AppError> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Check if todo exists and belongs to user
    let existing_todo = sqlx::query_as::<_, Todo>(
        "SELECT * FROM todos WHERE id = $1 AND user_id = $2"
    )
    .bind(todo_id)
    .bind(user.user_id)
    .fetch_optional(&app_state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    // Update todo based on provided fields
    let todo = if let (Some(title), Some(description), Some(completed)) = (&payload.title, &payload.description, payload.completed) {
        sqlx::query_as::<_, Todo>(
            "UPDATE todos SET title = $1, description = $2, completed = $3, updated_at = NOW() WHERE id = $4 AND user_id = $5 RETURNING *"
        )
        .bind(title)
        .bind(description)
        .bind(completed)
        .bind(todo_id)
        .bind(user.user_id)
        .fetch_one(&app_state.pool)
        .await?
    } else if let (Some(title), Some(completed)) = (&payload.title, payload.completed) {
        sqlx::query_as::<_, Todo>(
            "UPDATE todos SET title = $1, completed = $2, updated_at = NOW() WHERE id = $3 AND user_id = $4 RETURNING *"
        )
        .bind(title)
        .bind(completed)
        .bind(todo_id)
        .bind(user.user_id)
        .fetch_one(&app_state.pool)
        .await?
    } else if let Some(title) = &payload.title {
        sqlx::query_as::<_, Todo>(
            "UPDATE todos SET title = $1, updated_at = NOW() WHERE id = $2 AND user_id = $3 RETURNING *"
        )
        .bind(title)
        .bind(todo_id)
        .bind(user.user_id)
        .fetch_one(&app_state.pool)
        .await?
    } else if let Some(completed) = payload.completed {
        sqlx::query_as::<_, Todo>(
            "UPDATE todos SET completed = $1, updated_at = NOW() WHERE id = $2 AND user_id = $3 RETURNING *"
        )
        .bind(completed)
        .bind(todo_id)
        .bind(user.user_id)
        .fetch_one(&app_state.pool)
        .await?
    } else if payload.description.is_some() {
        sqlx::query_as::<_, Todo>(
            "UPDATE todos SET description = $1, updated_at = NOW() WHERE id = $2 AND user_id = $3 RETURNING *"
        )
        .bind(&payload.description)
        .bind(todo_id)
        .bind(user.user_id)
        .fetch_one(&app_state.pool)
        .await?
    } else {
        // If no fields to update, return the existing todo
        existing_todo
    };

    Ok(Json(TodoResponse::from(todo)))
}

pub async fn delete_todo(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
    Path(todo_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query(
        "DELETE FROM todos WHERE id = $1 AND user_id = $2"
    )
    .bind(todo_id)
    .bind(user.user_id)
    .execute(&app_state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn health_check() -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now()
    })))
}