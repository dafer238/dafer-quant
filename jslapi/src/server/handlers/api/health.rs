// src/server/handlers/api/health.rs
use axum::Json;
use serde_json::json;
use chrono;

pub async fn check_health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
