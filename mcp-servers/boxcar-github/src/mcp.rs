use serde::Deserialize;
use serde_json::Value;

/// A tool as described in the MCP tools/list response.
#[derive(Debug, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// Parsed result of a tools/list call.
#[derive(Debug, Deserialize)]
pub struct McpToolsListResult {
    pub tools: Vec<McpTool>,
}

/// Parsed result of a tools/call response.
#[derive(Debug, Deserialize)]
pub struct McpToolCallResult {
    pub content: Vec<McpContent>,
    #[serde(rename = "isError", default)]
    pub is_error: bool,
}

#[derive(Debug, Deserialize)]
pub struct McpContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct McpContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}
