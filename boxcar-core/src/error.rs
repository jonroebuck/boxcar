use thiserror::Error;

#[derive(Debug, Error)]
pub enum BoxcarError {
    #[error("MCP server error: {0}")]
    ServerError(String),

    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Tool call failed: {0}")]
    ToolCallFailed(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Transport error: {0}")]
    TransportError(String),
}

pub type BoxcarResult<T> = Result<T, BoxcarError>;
