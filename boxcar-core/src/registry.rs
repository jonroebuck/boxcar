use std::collections::HashMap;
use std::sync::Arc;
use crate::{
    error::{BoxcarError, BoxcarResult},
    server::McpServer,
    tool::{ToolCall, ToolDefinition, ToolResult},
};

/// The central registry of MCP server adapters.
/// Owns the routing logic: given a namespaced tool name,
/// find the right server and dispatch the call.
pub struct ServerRegistry {
    servers: HashMap<String, Arc<dyn McpServer>>,
}

impl ServerRegistry {
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
        }
    }

    /// Register an MCP server adapter.
    pub fn register(&mut self, server: Arc<dyn McpServer>) {
        self.servers.insert(server.name().to_string(), server);
    }

    /// Aggregate tool definitions from all registered servers.
    pub async fn list_all_tools(&self) -> BoxcarResult<Vec<ToolDefinition>> {
        let mut tools = Vec::new();
        for server in self.servers.values() {
            let server_tools = server.list_tools().await?;
            tools.extend(server_tools);
        }
        Ok(tools)
    }

    /// Route a tool call to the correct server based on namespace prefix.
    pub async fn call_tool(&self, call: &ToolCall) -> BoxcarResult<ToolResult> {
        let server_name = call
            .name
            .split('/')
            .next()
            .ok_or_else(|| BoxcarError::ToolNotFound(call.name.clone()))?;

        let server = self
            .servers
            .get(server_name)
            .ok_or_else(|| BoxcarError::ToolNotFound(call.name.clone()))?;

        server.call_tool(call).await
    }
}

impl Default for ServerRegistry {
    fn default() -> Self {
        Self::new()
    }
}
