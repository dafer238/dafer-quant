// src/server/mod.rs
use crate::config::app_config::AppConfig;
use anyhow::Result;
use axum::{Extension, Router};
use std::sync::Arc;

mod errors;
mod routes;
mod handlers;

pub async fn create_app(config: AppConfig) -> Result<Router> {
    let app_state = Arc::new(AppState { config });

    let api_routes = routes::api::create_routes();
    //let web_routes = routes::web::create_routes();

    let app = Router::new()
        .nest("/api", api_routes)
        //.nest("/", web_routes)
        .layer(Extension(app_state));

    Ok(app)
}

pub struct AppState {
    pub config: AppConfig,
}
