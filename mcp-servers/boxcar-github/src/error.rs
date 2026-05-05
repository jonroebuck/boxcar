use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitHubError {
    #[error("GitHub MCP server returned an error: {0}")]
    McpError(String),

    #[error("HTTP error communicating with GitHub MCP server: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Failed to deserialize GitHub MCP response: {0}")]
    DeserializationError(#[from] serde_json::Error),

    #[error("GITHUB_TOKEN environment variable not set")]
    MissingToken,
}
