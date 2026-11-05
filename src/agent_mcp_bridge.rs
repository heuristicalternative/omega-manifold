#![allow(dead_code)]
// Model Context Protocol (MCP) server and peer-agent sandbox node bridge.

use crate::nars_triadic_engine::{NarsTriadicEngine, NarsTruthValue, TriadicState};
use serde_json::{Value, json};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgentMcpBridge {
    pub endpoints: Vec<String>,
    pub sandbox_nodes: HashMap<String, String>, // node_id -> shared_scope
    pub grammar_buffers: Vec<String>,
    pub nars_engine: NarsTriadicEngine,
}

impl AgentMcpBridge {
    pub fn new() -> Self {
        Self {
            endpoints: vec!["/mcp/v1".to_string()],
            sandbox_nodes: HashMap::new(),
            grammar_buffers: Vec::new(),
            nars_engine: NarsTriadicEngine::new(),
        }
    }

    /// Process an external agent JSON-RPC request, validate AST hex, and emit
    /// a zero-drift response.
    pub fn handle_mcp_request(&mut self, request_json: &str) -> Result<String, String> {
        let req: Value = serde_json::from_str(request_json)
            .map_err(|e| format!("invalid JSON-RPC: {}", e))?;

        let method = req
            .get("method")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "missing method".to_string())?;

        // Validate AST hex coordinate if present.
        if let Some(params) = req.get("params") {
            if let Some(ast) = params.get("ast_hex").and_then(|v| v.as_str()) {
                if ast != "0xASTx702" {
                    return Err("AST hex coordinate obstruction: invalid 0xASTx702".to_string());
                }
            }
            // Grammar translation buffer.
            if let Some(grammar) = params.get("grammar").and_then(|v| v.as_str()) {
                self.grammar_buffers.push(grammar.to_string());
            }

            // NARS triadic cognitive evaluation gate for external patches.
            let f = params.get("frequency").and_then(|v| v.as_f64()).unwrap_or(0.9);
            let c = params.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.9);
            let truth = NarsTruthValue::new(f, c);
            if self.nars_engine.evaluate_triadic_gate(&truth) == TriadicState::DecoherentPDMinus {
                return Err("triadic gate rejected: decoherent patch".to_string());
            }
        }

        let response = match method {
            "query_state" => json!({
                "jsonrpc": "2.0",
                "result": {
                    "status": "ok",
                    "ast_hex": "0xASTx702",
                    "drift": 0.0
                },
                "id": req.get("id").cloned().unwrap_or(Value::Null)
            }),
            "spawn_sandbox" => {
                let node_id = req
                    .get("params")
                    .and_then(|p| p.get("node_id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("sandbox_node");
                let scope = req
                    .get("params")
                    .and_then(|p| p.get("shared_scope"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("shared");
                let ok = self.spawn_sandbox_node(node_id, scope).unwrap_or(false);
                json!({
                    "jsonrpc": "2.0",
                    "result": { "spawned": ok },
                    "id": req.get("id").cloned().unwrap_or(Value::Null)
                })
            }
            _ => {
                return Err(format!("unknown method: {}", method));
            }
        };

        Ok(response.to_string())
    }

    pub fn spawn_sandbox_node(&self, node_id: &str, shared_scope: &str) -> Result<bool, String> {
        if node_id.trim().is_empty() {
            return Err("node_id must not be empty".to_string());
        }
        if shared_scope.trim().is_empty() {
            return Err("shared_scope must not be empty".to_string());
        }
        // Simulated isolated sandbox container linked to Radicle P2P mesh.
        Ok(true)
    }

    pub fn spawn_sandbox_node_mut(
        &mut self,
        node_id: &str,
        shared_scope: &str,
    ) -> Result<bool, String> {
        let ok = self.spawn_sandbox_node(node_id, shared_scope)?;
        if ok {
            self.sandbox_nodes
                .insert(node_id.to_string(), shared_scope.to_string());
        }
        Ok(ok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_jsonrpc_parsing() {
        let mut bridge = AgentMcpBridge::new();
        let req = r#"{"jsonrpc":"2.0","method":"query_state","params":{"ast_hex":"0xASTx702"},"id":1}"#;
        let resp = bridge.handle_mcp_request(req).unwrap();
        assert!(resp.contains("\"status\":\"ok\"") || resp.contains("\"status\": \"ok\""));
        assert!(resp.contains("0xASTx702"));
    }

    #[test]
    fn test_ast_hex_obstruction_rejected() {
        let mut bridge = AgentMcpBridge::new();
        let req = r#"{"jsonrpc":"2.0","method":"query_state","params":{"ast_hex":"0xBADHEX"},"id":2}"#;
        let result = bridge.handle_mcp_request(req);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("obstruction"));
    }

    #[test]
    fn test_spawn_sandbox_node() {
        let bridge = AgentMcpBridge::new();
        assert!(bridge.spawn_sandbox_node("node_1", "shared").unwrap());
        assert!(bridge.spawn_sandbox_node("node_1", "").is_err());
        assert!(bridge.spawn_sandbox_node("", "shared").is_err());
    }

    #[test]
    fn test_spawn_sandbox_mut_registers() {
        let mut bridge = AgentMcpBridge::new();
        assert!(bridge.spawn_sandbox_node_mut("sb_1", "shared").unwrap());
        assert!(bridge.sandbox_nodes.contains_key("sb_1"));
    }

    #[test]
    fn test_grammar_translation_buffer() {
        let mut bridge = AgentMcpBridge::new();
        let req = r#"{"jsonrpc":"2.0","method":"query_state","params":{"ast_hex":"0xASTx702","grammar":"W=stable"},"id":3}"#;
        let _ = bridge.handle_mcp_request(req).unwrap();
        assert_eq!(bridge.grammar_buffers, vec!["W=stable"]);
    }

    #[test]
    fn test_unknown_method_rejected() {
        let mut bridge = AgentMcpBridge::new();
        let req = r#"{"jsonrpc":"2.0","method":"unknown","id":4}"#;
        assert!(bridge.handle_mcp_request(req).is_err());
    }

    #[test]
    fn test_invalid_json_rejected() {
        let mut bridge = AgentMcpBridge::new();
        assert!(bridge.handle_mcp_request("not json").is_err());
    }
}
