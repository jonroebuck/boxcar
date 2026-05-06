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
/// Uses GitHub's Streamable HTTP transport: POST to path-based endpoints
/// under https://api.githubcopilot.com/mcp/
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

    /// POST a body to a path under the base URL and return the parsed response.
    async fn send(&self, path: &str, body: Value) -> Result<Value, GitHubError> {
        let url = format!("{}{}", self.base_url, path);

        debug!(path = %path, "Sending MCP request to GitHub");

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response)
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

        let body = json!({
            "name": tool_name,
            "arguments": call.input,
        });

        let result = self
            .send("tools/call", body)
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
