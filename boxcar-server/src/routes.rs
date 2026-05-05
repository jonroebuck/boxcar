use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use boxcar_core::tool::ToolCall;
use crate::{error::ServerError, state::AppState};

/// GET /tools
///
/// Returns the aggregated list of tool definitions from all
/// registered MCP server adapters, namespaced by server name.
pub async fn list_tools(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ServerError> {
    let registry = state.registry.read().await;
    let tools = registry.list_all_tools().await?;
    Ok((StatusCode::OK, Json(tools)))
}

/// POST /tools/call
///
/// Accepts a ToolCall (namespaced tool name + input), routes it
/// to the correct MCP server adapter, and returns the ToolResult.
pub async fn call_tool(
    State(state): State<AppState>,
    Json(call): Json<ToolCall>,
) -> Result<impl IntoResponse, ServerError> {
    let registry = state.registry.read().await;
    let result = registry.call_tool(&call).await?;
    Ok((StatusCode::OK, Json(result)))
}

/// GET /health
///
/// Simple liveness probe.
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok" })))
}
