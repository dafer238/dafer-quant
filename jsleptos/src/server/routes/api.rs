// src/server/routes/api.rs
use axum::{
    Router,
    routing::{get, post},
};
use crate::server::handlers::api;

pub fn create_routes() -> Router {
    Router::new()
        .route("/health", get(api::health::check_health))
        //.route("/users", get(api::users::list_users))
        //.route("/users", post(api::users::create_user))
}

