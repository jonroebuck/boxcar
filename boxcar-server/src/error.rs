use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use boxcar_core::error::BoxcarError;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    Core(#[from] BoxcarError),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ServerError::Core(BoxcarError::ToolNotFound(name)) => (
                StatusCode::NOT_FOUND,
                format!("Tool not found: {name}"),
            ),
            ServerError::Core(BoxcarError::ToolCallFailed(msg)) => (
                StatusCode::BAD_GATEWAY,
                format!("Tool call failed: {msg}"),
            ),
            ServerError::Core(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
            ),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
