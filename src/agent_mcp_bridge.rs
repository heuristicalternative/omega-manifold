#![allow(dead_code)]

pub struct AgentMcpBridge;

impl AgentMcpBridge {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_mcp_request(&self, _req: &str) -> Result<String, String> {
        Ok("McpResponse".to_string())
    }

    pub fn spawn_sandbox_node(&self, _node_id: &str, _channel: &str) -> Result<bool, String> {
        Ok(true)
    }
}
