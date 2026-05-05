use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A tool exposed by an MCP server, as seen by an agent.
/// The server name is namespaced in so agents can distinguish
/// tools across servers (e.g. "github/get_file_contents").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Namespaced name: "<server>/<tool>"
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// A request from an agent to call a tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Namespaced name: "<server>/<tool>"
    pub name: String,
    pub input: Value,
}

/// The result of a tool call returned to the agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub name: String,
    pub output: ToolOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolOutput {
    Success { content: Value },
    Error { message: String },
}
