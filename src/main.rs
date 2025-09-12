use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use todo_api::{AppState, Config, AppError, Result};

mod database;
mod handlers;
mod middleware;
mod models;
mod utils;

use crate::{
    database::{create_pool, migrate},
    handlers::{auth, todos},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "todo_api=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()
        .map_err(|e| {
            tracing::error!("Failed to load configuration: {}", e);
            AppError::InternalServerError
        })?;

    // Create database connection pool
    let pool = create_pool(&config.database_url).await
        .map_err(|e| {
            tracing::error!("Failed to create database pool: {}", e);
            AppError::InternalServerError
        })?;

    // Run migrations
    migrate(&pool).await
        .map_err(|e| {
            tracing::error!("Failed to run migrations: {}", e);
            AppError::InternalServerError
        })?;

    // Create app state
    let app_state = AppState {
        pool,
        config: config.clone(),
    };

    // Build our application with routes
    let app = create_app(app_state).await;

    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
        .await
        .map_err(|e| {
            tracing::error!("Failed to bind to address: {}", e);
            AppError::InternalServerError
        })?;

    tracing::info!("Server running on http://{}:{}", config.server_host, config.server_port);

    axum::serve(listener, app)
        .await
        .map_err(|e| {
            tracing::error!("Server error: {}", e);
            AppError::InternalServerError
        })?;

    Ok(())
}

async fn create_app(app_state: AppState) -> Router {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
        ])
        .allow_headers(Any)
        .allow_origin(Any);

    // Build the router
    Router::new()
        // Health check endpoint (no auth required)
        .route("/health", get(todos::health_check))
        
        // Authentication routes (no auth required)
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        
        // Todo routes (authentication required)
        .route("/todos", get(todos::get_todos).post(todos::create_todo))
        .route("/todos/:id", put(todos::update_todo).delete(todos::delete_todo))
        
        // Add middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
        )
        
        // Add shared state
        .with_state(app_state)
}