use axum::{
    routing::{get, post},
    Router,
    http::StatusCode,
    response::Json,
    extract::State,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, error};
use tracing_subscriber;

mod engine;
mod vector;
mod streaming;
mod ai;
mod storage;
mod config;
mod metrics;
mod cache;
mod sql_parser;

use engine::VectraEngine;
use config::Config;

#[derive(Clone)]
struct AppState {
    engine: Arc<VectraEngine>,
    config: Config,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting VectraEdge...");
    
    // Load configuration
    let config = Config::load()?;
    info!("Configuration loaded");
    
    // Initialize the Vectra engine
    let engine = Arc::new(VectraEngine::new(&config).await?);
    info!("Vectra engine initialized");
    
    // Create application state
    let state = AppState {
        engine,
        config: config.clone(),
    };
    
    // Build router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/query", post(execute_query))
        .route("/vector/search", post(vector_search))
        .route("/stream/subscribe", post(subscribe_stream))
        .with_state(state);
    
    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await?;
    
    info!("VectraEdge server listening on {}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn root() -> &'static str {
    "VectraEdge - AI-Native OLAP Engine\n"
}

async fn health() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "healthy",
            "version": env!("CARGO_PKG_VERSION"),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    )
}

async fn execute_query(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let query = payload["query"]
        .as_str()
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    match state.engine.execute_query(query).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn vector_search(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let query = payload["query"]
        .as_str()
        .ok_or(StatusCode::BAD_REQUEST)?;
    let limit = payload["limit"].as_u64().unwrap_or(10);
    
    match state.engine.vector_search(query, limit as usize).await {
        Ok(results) => Ok(Json(serde_json::json!({
            "results": results,
            "query": query,
            "limit": limit
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn subscribe_stream(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let topic = payload["topic"]
        .as_str()
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    match state.engine.subscribe_stream(topic).await {
        Ok(subscription) => Ok(Json(serde_json::json!({
            "subscription_id": subscription.id,
            "topic": topic,
            "status": "active"
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
