use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC request sent to the GitHub MCP server.
#[derive(Debug, Serialize)]
pub struct McpRequest {
    pub jsonrpc: &'static str,
    pub id: u64,
    pub method: String,
    pub params: Value,
}

impl McpRequest {
    pub fn new(id: u64, method: impl Into<String>, params: Value) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            method: method.into(),
            params,
        }
    }
}

/// JSON-RPC response from the GitHub MCP server.
#[derive(Debug, Deserialize)]
pub struct McpResponse {
    pub id: u64,
    pub result: Option<Value>,
    pub error: Option<McpResponseError>,
}

#[derive(Debug, Deserialize)]
pub struct McpResponseError {
    pub code: i64,
    pub message: String,
}

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
