use async_trait::async_trait;
use crate::{
    error::BoxcarResult,
    tool::{ToolCall, ToolDefinition, ToolResult},
};

/// Port trait for an MCP server adapter.
/// Each adapter (e.g. boxcar-github) implements this.
#[async_trait]
pub trait McpServer: Send + Sync {
    /// Unique name for this server (used as the namespace prefix).
    fn name(&self) -> &str;

    /// List all tools this server exposes.
    async fn list_tools(&self) -> BoxcarResult<Vec<ToolDefinition>>;

    /// Execute a single tool call against this server.
    /// The `call.name` will be the full namespaced name;
    /// the adapter is responsible for stripping the prefix.
    async fn call_tool(&self, call: &ToolCall) -> BoxcarResult<ToolResult>;
}
