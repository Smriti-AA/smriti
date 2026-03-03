use axum::http::Method;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::storage::Database;

use super::routes::{agent, graph, notes};

/// Shared application state passed to all handlers
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}

/// Build the complete Axum router with all routes
pub fn create_router(db: Arc<Database>) -> Router {
    let state = AppState { db };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        // Health check
        .route("/health", get(health_check))
        // Note routes
        .route("/api/v1/notes", get(notes::list_notes).post(notes::create_note))
        .route(
            "/api/v1/notes/{id}",
            get(notes::get_note)
                .put(notes::update_note)
                .delete(notes::delete_note),
        )
        .route("/api/v1/notes/search", get(notes::search_notes))
        .route("/api/v1/notes/{id}/backlinks", get(notes::get_backlinks))
        .route("/api/v1/notes/{id}/links", get(notes::get_forward_links))
        .route("/api/v1/notes/{id}/tags", post(notes::add_tags))
        // Graph routes
        .route("/api/v1/graph", get(graph::get_full_graph))
        .route("/api/v1/graph/{id}", get(graph::get_subgraph))
        .route("/api/v1/stats", get(graph::get_stats))
        // Agent routes
        .route(
            "/api/v1/agent/{agent_id}/memory",
            get(agent::list_memory).post(agent::store_memory),
        )
        .route(
            "/api/v1/agent/{agent_id}/memory/{namespace}/{key}",
            get(agent::get_memory),
        )
        .route(
            "/api/v1/agent/{agent_id}/tool-logs",
            get(agent::get_tool_logs).post(agent::log_tool_call),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "name": "smriti",
    }))
}

/// Start the API server on the given address
pub async fn start_server(db: Arc<Database>, host: &str, port: u16) -> anyhow::Result<()> {
    let app = create_router(db);
    let addr = format!("{}:{}", host, port);

    tracing::info!("Starting API server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
