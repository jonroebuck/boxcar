# boxcar

A lightweight MCP gateway written in Rust.

Boxcar sits in front of your MCP servers and gives your agents a single, stable endpoint for tool discovery and execution — regardless of which MCP servers are running behind it. Add new servers or swap them out without changing your agent code.

## How it works
Boxcar exposes a simple REST API that any agent can call over plain HTTP. Internally it routes tool calls to the correct MCP server adapter based on the namespace prefix in the tool name.

```
GET  /tools
→ returns the aggregated list of tools from all registered MCP servers

POST /tools/call
{ "name": "github/get_file_contents", "input": { ... } }  → GitHub MCP server
{ "name": "linear/create_issue",       "input": { ... } }  → Linear MCP server
{ "name": "slack/post_message",        "input": { ... } }  → Slack MCP server
```

Tools are namespaced by server name so agents always know which server a tool belongs to, and Boxcar always knows where to route the call.

## MCP servers

| Server         | Namespace  | Configured via  |
|----------------|------------|-----------------|
| GitHub         | `github/`  | `GITHUB_TOKEN`  |

Boxcar starts with whatever servers are available. If `GITHUB_TOKEN` is not set, Boxcar starts fine and simply won't route `github/*` tool calls.

## Running

With Docker:
```bash
docker run -e GITHUB_TOKEN=ghp_... -p 3000:3000 ghcr.io/<your-username>/boxcar:latest
```

Natively:
```bash
GITHUB_TOKEN=ghp_... cargo run -p boxcar-server
```

## Calling Boxcar from an agent

List all available tools:
```bash
curl http://localhost:3000/tools
```

Call a tool:
```bash
curl -X POST http://localhost:3000/tools/call \
  -H "Content-Type: application/json" \
  -d '{
    "name": "github/get_file_contents",
    "input": {
      "owner": "my-org",
      "repo":  "my-repo",
      "path":  "README.md"
    }
  }'
```

Because Boxcar is plain HTTP, any agent in any language can call it with no special library.

## Adding a private MCP server

Implement the `McpServer` trait from `boxcar-core` in your own crate, then register it at startup:

```rust
let mut registry = ServerRegistry::new();
registry.register(Arc::new(MyPrivateMcpServer::new()));
```

The adapter is compiled in and never needs to be published.

## Crates

| Crate                        | Description                                                  |
|------------------------------|--------------------------------------------------------------|
| `boxcar-core`                | `McpServer` trait, `ToolDefinition`, `ToolCall`, `ToolResult` — the stable port interface |
| `boxcar-server`              | Axum HTTP server exposing `/tools` and `/tools/call`         |
| `mcp-servers/boxcar-github`  | GitHub MCP server adapter                                    |
