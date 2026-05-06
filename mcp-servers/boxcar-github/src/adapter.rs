use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};
use tracing::{debug, instrument};

use boxcar_core::{
    error::{BoxcarError, BoxcarResult},
    server::McpServer,
    tool::{ToolCall, ToolDefinition, ToolOutput, ToolResult},
};

use crate::{
    error::GitHubError,
    mcp::{McpToolCallResult, McpToolsListResult},
};

/// Adapter that connects Boxcar to GitHub's remote MCP server.
///
/// Reads GITHUB_TOKEN from the environment on construction.
/// Uses JSON-RPC 2.0 over HTTP POST to https://api.githubcopilot.com/mcp/
/// Responses are in SSE (Server-Sent Events) format — we extract the data line.
pub struct GitHubMcpServer {
    client: Client,
    base_url: String,
    token: String,
}

impl GitHubMcpServer {
    /// Construct from environment. Reads GITHUB_TOKEN.
    pub fn from_env() -> Result<Self, GitHubError> {
        let token = std::env::var("GITHUB_TOKEN")
            .map_err(|_| GitHubError::MissingToken)?;

        Ok(Self::new(
            "https://api.githubcopilot.com/mcp/".to_string(),
            token,
        ))
    }

    /// Construct with explicit URL and token (useful for testing).
    pub fn new(base_url: String, token: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            token,
        }
    }

    /// Send a JSON-RPC 2.0 request and parse the SSE response.
    async fn send(&self, method: &str, params: Value) -> Result<Value, GitHubError> {
        debug!(method = %method, "Sending MCP request to GitHub");

        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        });

        let text = self
            .client
            .post(&self.base_url)
            .bearer_auth(&self.token)
            .header("Accept", "application/json, text/event-stream")
            .json(&request)
            .send()
            .await?
            .text()
            .await?;

        // GitHub returns SSE format: extract JSON from the "data: {...}" line
        let json_str = text
            .lines()
            .find(|line| line.starts_with("data: "))
            .map(|line| &line["data: ".len()..])
            .ok_or_else(|| GitHubError::McpError("No data line in SSE response".to_string()))?;

        let response: Value = serde_json::from_str(json_str)?;

        if let Some(err) = response.get("error") {
            return Err(GitHubError::McpError(err.to_string()));
        }

        Ok(response["result"].clone())
    }

    /// Strip the "github/" namespace prefix from a tool name.
    fn strip_prefix<'a>(&self, name: &'a str) -> &'a str {
        name.strip_prefix("github/").unwrap_or(name)
    }
}

#[async_trait]
impl McpServer for GitHubMcpServer {
    fn name(&self) -> &str {
        "github"
    }

    #[instrument(skip(self), name = "github.list_tools")]
    async fn list_tools(&self) -> BoxcarResult<Vec<ToolDefinition>> {
        let result = self
            .send("tools/list", json!({}))
            .await
            .map_err(|e| BoxcarError::TransportError(e.to_string()))?;

        let parsed: McpToolsListResult = serde_json::from_value(result)
            .map_err(|e| BoxcarError::TransportError(e.to_string()))?;

        let tools = parsed
            .tools
            .into_iter()
            .map(|t| ToolDefinition {
                name: format!("github/{}", t.name),
                description: t.description,
                input_schema: t.input_schema,
            })
            .collect();

        Ok(tools)
    }

    #[instrument(skip(self), name = "github.call_tool", fields(tool = %call.name))]
    async fn call_tool(&self, call: &ToolCall) -> BoxcarResult<ToolResult> {
        let tool_name = self.strip_prefix(&call.name).to_string();

        let result = self
            .send("tools/call", json!({
                "name": tool_name,
                "arguments": call.input,
            }))
            .await
            .map_err(|e| BoxcarError::ToolCallFailed(e.to_string()))?;

        let parsed: McpToolCallResult = serde_json::from_value(result)
            .map_err(|e| BoxcarError::ToolCallFailed(e.to_string()))?;

        let output = if parsed.is_error {
            let message = parsed
                .content
                .into_iter()
                .filter_map(|c| c.text)
                .collect::<Vec<_>>()
                .join("\n");
            ToolOutput::Error { message }
        } else {
            let content: Vec<Value> = parsed
                .content
                .into_iter()
                .filter_map(|c| c.text.map(Value::String))
                .collect();
            ToolOutput::Success {
                content: Value::Array(content),
            }
        };

        Ok(ToolResult {
            name: call.name.clone(),
            output,
        })
    }
}
