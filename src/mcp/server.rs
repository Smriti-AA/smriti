use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::sync::Arc;

use crate::storage::Database;

use super::handlers;

/// MCP Server implementation using JSON-RPC 2.0 over stdio
///
/// Implements the Model Context Protocol for AI agent integration.
/// Agents can create/read/search notes, manage memory, and traverse the knowledge graph.
pub struct McpServer {
    db: Arc<Database>,
}

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl McpServer {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Run the MCP server, reading JSON-RPC messages from stdin and writing responses to stdout
    pub fn run(&self) -> anyhow::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        eprintln!("MCP server started. Listening for JSON-RPC messages on stdin...");

        for line in stdin.lock().lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            let response = match serde_json::from_str::<JsonRpcRequest>(&line) {
                Ok(req) => self.handle_request(req),
                Err(e) => JsonRpcResponse {
                    jsonrpc: "2.0".into(),
                    id: Value::Null,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: format!("Parse error: {}", e),
                        data: None,
                    }),
                },
            };

            let output = serde_json::to_string(&response)?;
            writeln!(stdout, "{}", output)?;
            stdout.flush()?;
        }

        Ok(())
    }

    fn handle_request(&self, req: JsonRpcRequest) -> JsonRpcResponse {
        let id = req.id.unwrap_or(Value::Null);

        let result = match req.method.as_str() {
            "initialize" => self.handle_initialize(&req.params),
            "tools/list" => self.handle_tools_list(),
            "tools/call" => self.handle_tools_call(&req.params),
            "resources/list" => self.handle_resources_list(),
            "resources/read" => self.handle_resources_read(&req.params),
            "notifications/initialized" => {
                // Client notification — no response needed but we return OK
                Ok(json!({}))
            }
            _ => Err((-32601, format!("Method not found: {}", req.method))),
        };

        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id,
                result: Some(value),
                error: None,
            },
            Err((code, message)) => JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code,
                    message,
                    data: None,
                }),
            },
        }
    }

    fn handle_initialize(&self, _params: &Value) -> Result<Value, (i32, String)> {
        Ok(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {},
                "resources": { "listChanged": false }
            },
            "serverInfo": {
                "name": "smriti",
                "version": env!("CARGO_PKG_VERSION")
            }
        }))
    }

    fn handle_tools_list(&self) -> Result<Value, (i32, String)> {
        Ok(json!({
            "tools": [
                {
                    "name": "notes_create",
                    "description": "Create a new note with markdown content. Wiki-links [[like this]] and #tags are auto-detected.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "title": { "type": "string", "description": "Note title" },
                            "content": { "type": "string", "description": "Markdown content" },
                            "tags": { "type": "array", "items": { "type": "string" }, "description": "Tags" }
                        },
                        "required": ["title", "content"]
                    }
                },
                {
                    "name": "notes_read",
                    "description": "Read a note by ID or title",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "id": { "type": "string", "description": "Note ID or title" }
                        },
                        "required": ["id"]
                    }
                },
                {
                    "name": "notes_search",
                    "description": "Full-text search across all notes",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "query": { "type": "string", "description": "Search query" },
                            "limit": { "type": "integer", "description": "Max results (default: 10)" }
                        },
                        "required": ["query"]
                    }
                },
                {
                    "name": "notes_list",
                    "description": "List recent notes",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "limit": { "type": "integer", "description": "Max notes (default: 20)" },
                            "tag": { "type": "string", "description": "Filter by tag" }
                        }
                    }
                },
                {
                    "name": "notes_graph",
                    "description": "Get the knowledge graph or a subgraph around a note",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "center_id": { "type": "string", "description": "Center note ID (optional, returns full graph if omitted)" },
                            "depth": { "type": "integer", "description": "Depth for subgraph (default: 2)" }
                        }
                    }
                },
                {
                    "name": "memory_store",
                    "description": "Store a key-value pair in agent memory",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "agent_id": { "type": "string", "description": "Agent identifier" },
                            "key": { "type": "string", "description": "Memory key" },
                            "value": { "description": "Value to store (any JSON)" },
                            "namespace": { "type": "string", "description": "Namespace (default: 'default')" },
                            "ttl_seconds": { "type": "integer", "description": "Time-to-live in seconds" }
                        },
                        "required": ["agent_id", "key", "value"]
                    }
                },
                {
                    "name": "memory_retrieve",
                    "description": "Retrieve a value from agent memory",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "agent_id": { "type": "string", "description": "Agent identifier" },
                            "key": { "type": "string", "description": "Memory key" },
                            "namespace": { "type": "string", "description": "Namespace (default: 'default')" }
                        },
                        "required": ["agent_id", "key"]
                    }
                },
                {
                    "name": "memory_list",
                    "description": "List all memory entries for an agent",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "agent_id": { "type": "string", "description": "Agent identifier" },
                            "namespace": { "type": "string", "description": "Filter by namespace" }
                        },
                        "required": ["agent_id"]
                    }
                }
            ]
        }))
    }

    fn handle_tools_call(&self, params: &Value) -> Result<Value, (i32, String)> {
        let tool_name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or((-32602, "Missing tool name".to_string()))?;

        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        let result = match tool_name {
            "notes_create" => handlers::handle_notes_create(&self.db, &arguments),
            "notes_read" => handlers::handle_notes_read(&self.db, &arguments),
            "notes_search" => handlers::handle_notes_search(&self.db, &arguments),
            "notes_list" => handlers::handle_notes_list(&self.db, &arguments),
            "notes_graph" => handlers::handle_notes_graph(&self.db, &arguments),
            "memory_store" => handlers::handle_memory_store(&self.db, &arguments),
            "memory_retrieve" => handlers::handle_memory_retrieve(&self.db, &arguments),
            "memory_list" => handlers::handle_memory_list(&self.db, &arguments),
            _ => Err(format!("Unknown tool: {}", tool_name)),
        };

        match result {
            Ok(value) => Ok(json!({
                "content": [{
                    "type": "text",
                    "text": serde_json::to_string_pretty(&value).unwrap_or_default()
                }]
            })),
            Err(e) => Ok(json!({
                "content": [{
                    "type": "text",
                    "text": format!("Error: {}", e)
                }],
                "isError": true
            })),
        }
    }

    fn handle_resources_list(&self) -> Result<Value, (i32, String)> {
        // List notes as resources
        let notes = self
            .db
            .list_notes(&crate::models::NoteListQuery {
                limit: 100,
                offset: 0,
                sort: crate::models::SortOrder::UpdatedDesc,
                tag: None,
            })
            .map_err(|e| (-32000, e.to_string()))?;

        let resources: Vec<Value> = notes
            .iter()
            .map(|n| {
                json!({
                    "uri": format!("note://{}", n.id),
                    "name": n.title,
                    "description": n.preview,
                    "mimeType": "text/markdown"
                })
            })
            .collect();

        Ok(json!({ "resources": resources }))
    }

    fn handle_resources_read(&self, params: &Value) -> Result<Value, (i32, String)> {
        let uri = params
            .get("uri")
            .and_then(|v| v.as_str())
            .ok_or((-32602, "Missing uri".to_string()))?;

        let note_id = uri
            .strip_prefix("note://")
            .ok_or((-32602, "Invalid URI format".to_string()))?;

        let note = self
            .db
            .get_note(note_id)
            .map_err(|e| (-32000, e.to_string()))?;

        Ok(json!({
            "contents": [{
                "uri": uri,
                "mimeType": "text/markdown",
                "text": note.content
            }]
        }))
    }
}
