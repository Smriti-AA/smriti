use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::api::server::AppState;
use crate::errors::AppError;
use crate::models::*;

#[derive(Debug, Deserialize)]
pub struct MemoryListQuery {
    pub namespace: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ToolLogQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    50
}

/// POST /api/v1/agent/:agent_id/memory
pub async fn store_memory(
    State(state): State<AppState>,
    Path(agent_id): Path<String>,
    Json(req): Json<CreateMemoryRequest>,
) -> Result<(StatusCode, Json<AgentMemory>), AppError> {
    let memory = state.db.store_memory(&agent_id, req)?;
    Ok((StatusCode::CREATED, Json(memory)))
}

/// GET /api/v1/agent/:agent_id/memory
pub async fn list_memory(
    State(state): State<AppState>,
    Path(agent_id): Path<String>,
    Query(query): Query<MemoryListQuery>,
) -> Result<Json<Vec<AgentMemory>>, AppError> {
    let memories = state
        .db
        .list_agent_memory(&agent_id, query.namespace.as_deref())?;
    Ok(Json(memories))
}

/// GET /api/v1/agent/:agent_id/memory/:namespace/:key
pub async fn get_memory(
    State(state): State<AppState>,
    Path((agent_id, namespace, key)): Path<(String, String, String)>,
) -> Result<Json<AgentMemory>, AppError> {
    let memory = state.db.get_memory(&agent_id, &namespace, &key)?;
    Ok(Json(memory))
}

/// POST /api/v1/agent/:agent_id/tool-logs
pub async fn log_tool_call(
    State(state): State<AppState>,
    Path(agent_id): Path<String>,
    Json(req): Json<CreateToolLogRequest>,
) -> Result<(StatusCode, Json<ToolLog>), AppError> {
    let log = state.db.log_tool_call(&agent_id, req)?;
    Ok((StatusCode::CREATED, Json(log)))
}

/// GET /api/v1/agent/:agent_id/tool-logs
pub async fn get_tool_logs(
    State(state): State<AppState>,
    Path(agent_id): Path<String>,
    Query(query): Query<ToolLogQuery>,
) -> Result<Json<Vec<ToolLog>>, AppError> {
    let logs = state.db.get_tool_logs(&agent_id, query.limit)?;
    Ok(Json(logs))
}
