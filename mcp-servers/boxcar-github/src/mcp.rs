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
///
/// `content` preserves each raw MCP content item as a JSON [`Value`] so that
/// all fields (type, text, uri, mimeType, blob, …) are available to callers.
#[derive(Debug, Deserialize)]
pub struct McpToolCallResult {
    pub content: Vec<Value>,
    #[serde(rename = "isError", default)]
    pub is_error: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_text_content_item() {
        let raw = json!({
            "content": [{"type": "text", "text": "hello"}],
            "isError": false
        });
        let result: McpToolCallResult = serde_json::from_value(raw).unwrap();
        assert!(!result.is_error);
        assert_eq!(result.content.len(), 1);
        assert_eq!(result.content[0]["type"], "text");
        assert_eq!(result.content[0]["text"], "hello");
    }

    #[test]
    fn deserializes_resource_content_item_preserving_all_fields() {
        let raw = json!({
            "content": [{
                "type": "resource",
                "uri": "file:///repo/src/main.rs",
                "mimeType": "text/plain",
                "text": "fn main() {}"
            }],
            "isError": false
        });
        let result: McpToolCallResult = serde_json::from_value(raw).unwrap();
        assert!(!result.is_error);
        let item = &result.content[0];
        assert_eq!(item["type"], "resource");
        assert_eq!(item["uri"], "file:///repo/src/main.rs");
        assert_eq!(item["mimeType"], "text/plain");
        assert_eq!(item["text"], "fn main() {}");
    }

    #[test]
    fn deserializes_error_result() {
        let raw = json!({
            "content": [{"type": "text", "text": "tool not found"}],
            "isError": true
        });
        let result: McpToolCallResult = serde_json::from_value(raw).unwrap();
        assert!(result.is_error);
        assert_eq!(result.content[0]["text"], "tool not found");
    }

    #[test]
    fn is_error_defaults_to_false() {
        let raw = json!({"content": []});
        let result: McpToolCallResult = serde_json::from_value(raw).unwrap();
        assert!(!result.is_error);
    }
}
