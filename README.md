<p align="center">
  <h1 align="center">Smriti</h1>
  <p align="center"><em>Sanskrit: स्मृति — memory, remembrance</em></p>
  <p align="center">A lightning-fast, self-hosted knowledge store and memory layer for AI agents.</p>
</p>

<p align="center">
  <a href="https://crates.io/crates/smriti"><img src="https://img.shields.io/crates/v/smriti.svg" alt="crates.io"></a>
  <a href="https://github.com/smriti-AA/smriti/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License"></a>
  <a href="https://github.com/smriti-AA/smriti"><img src="https://img.shields.io/badge/language-Rust-orange.svg" alt="Rust"></a>
</p>

---

## Why Smriti?

Every AI agent needs memory. [Mem0](https://mem0.ai) is cloud-only. [Letta](https://letta.com) is research-heavy. Neither has a knowledge graph.

**Smriti is different:** self-hosted, knowledge-graph-native, MCP-ready, and fast enough to handle millions of operations. Your data never leaves your machine.

### Key Features

- **MCP Server** — Plug into Claude, GPT, or any MCP-compatible agent instantly
- **Knowledge Graph** — Notes auto-link via `[[wiki-links]]`; agents discover connections via graph traversal
- **Agent Memory** — Key-value store with namespaces, TTL, and tool execution logs
- **Full-Text Search** — SQLite FTS5 with sub-millisecond queries
- **REST API** — Full CRUD + graph + agent endpoints on Axum
- **Self-Hosted** — SQLite database, no cloud dependency, no API costs
- **Sync** — Cross-device via Synology NAS, WebDAV, or any filesystem mount

### How It Compares

| Feature | Smriti | Mem0 | Letta | LangMem |
|---------|--------|------|-------|---------|
| Self-hosted | Yes | No (cloud) | Yes | Partial |
| Knowledge graph | Yes | No | No | No |
| MCP native | Yes | No | No | No |
| Wiki-links | Yes | No | No | No |
| Full-text search | FTS5 | Vector | Vector | Vector |
| Language | Rust | Python | Python | Python |
| TTL support | Yes | No | No | No |

---

## Quick Start

```bash
cargo install smriti
```

```bash
# Create notes with wiki-links — connections are automatic
smriti create "LLM Architecture" \
  --content "Transformers use [[Attention Mechanisms]] for [[Parallel Processing]]"

smriti create "Attention Mechanisms" \
  --content "Self-attention is the core of [[LLM Architecture]]. See also #transformers"

# Search across all notes
smriti search "attention"

# View the knowledge graph
smriti graph

# Start the MCP server (for AI agents)
smriti mcp

# Start the REST API
smriti serve --port 3000
```

### Build from source

```bash
git clone https://github.com/smriti-AA/smriti.git
cd smriti
cargo build --release
./target/release/smriti --help
```

---

## MCP Server

Start with `smriti mcp`. Agents communicate via JSON-RPC 2.0 over stdio.

**8 tools available to agents:**

| Tool | Description |
|------|-------------|
| `notes_create` | Create a note with markdown content. `[[wiki-links]]` and `#tags` are auto-detected |
| `notes_read` | Read note by ID or title |
| `notes_search` | Full-text search across all notes |
| `notes_list` | List recent notes, optionally filtered by tag |
| `notes_graph` | Get full knowledge graph or subgraph around a note |
| `memory_store` | Store key-value memory with optional namespace and TTL |
| `memory_retrieve` | Retrieve a memory by agent ID, namespace, and key |
| `memory_list` | List all memory entries for an agent |

### Claude Desktop Integration

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "smriti": {
      "command": "smriti",
      "args": ["mcp", "--db", "/path/to/smriti.db"]
    }
  }
}
```

### Example: Agent Stores and Retrieves Memory

```bash
# Agent stores a finding
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"memory_store","arguments":{"agent_id":"researcher-1","key":"finding","value":"Transformers scale logarithmically with data size"}}}' | smriti mcp

# Agent creates a linked note
echo '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"notes_create","arguments":{"title":"Scaling Laws","content":"Key insight: [[Transformer]] performance scales logarithmically. Related to [[Chinchilla]] findings."}}}' | smriti mcp
```

---

## CLI Reference

```
smriti create <title>       Create a note (--content, --file, --tags)
smriti read <id>            Read a note by ID or title (--json)
smriti list                 List notes (--limit, --tag, --json)
smriti search <query>       Full-text search (--limit)
smriti graph                Knowledge graph (--format json|dot|text, --center)
smriti stats                Database stats + smart link suggestions
smriti serve                REST API server (--host, --port)
smriti mcp                  MCP server over stdio
smriti sync                 Sync with remote (--remote, --direction push|pull|both)
smriti import <dir>         Import .md files (--recursive)
smriti export <dir>         Export to .md files (--frontmatter)
```

---

## REST API

Start with `smriti serve --port 3000`. All endpoints at `/api/v1/`:

### Notes
```
POST   /notes                  Create note { title, content, tags[] }
GET    /notes                  List notes ?limit=20&tag=rust
GET    /notes/:id              Get note by ID
PUT    /notes/:id              Update note
DELETE /notes/:id              Delete note
GET    /notes/search?q=...     Full-text search
GET    /notes/:id/backlinks    Notes linking TO this note
GET    /notes/:id/links        Notes this note links TO
```

### Knowledge Graph
```
GET    /graph                  Full graph (nodes + edges)
GET    /graph/:id?depth=2      Subgraph around a note
GET    /stats                  Database statistics
```

### Agent Memory
```
POST   /agent/:id/memory                    Store memory
GET    /agent/:id/memory                    List memory (?namespace=default)
GET    /agent/:id/memory/:namespace/:key    Get specific entry
POST   /agent/:id/tool-logs                 Log tool execution
GET    /agent/:id/tool-logs                 Get tool logs (?limit=50)
```

---

## Sync

```bash
# Filesystem sync (Synology NAS mount, shared folder, etc.)
smriti sync --remote /Volumes/nas/smriti --direction both

# WebDAV sync
SYNC_USER=admin SYNC_PASS=secret smriti sync --remote https://nas:5006/smriti

# Or just point the DB at a synced folder
smriti --db ~/SynologyDrive/smriti.db create "Note" --content "Auto-synced!"
```

---

## Architecture

```
src/
├── models/      Note, Link, AgentMemory, ToolLog structs
├── storage/     SQLite + FTS5 full-text search + WAL mode
├── parser/      [[wiki-link]] and #tag extraction (regex)
├── graph/       petgraph-based knowledge graph with BFS traversal
├── api/         Axum REST API with CORS and tracing
├── mcp/         MCP JSON-RPC 2.0 server (stdio transport)
├── cli/         clap-based CLI with 11 commands
├── sync/        WebDAV + filesystem sync engine
└── features/    Smart link suggestions, daily digest
```

**Tech stack:** Rust, Axum, SQLite (FTS5 + WAL), petgraph, clap, serde, tokio

---

## Roadmap

- [ ] Vector embeddings for semantic search
- [ ] Multi-agent collaboration (shared knowledge graphs)
- [ ] Temporal memory queries ("what changed since last session?")
- [ ] Web dashboard for graph visualization
- [ ] Official MCP registry listing
- [ ] Python and TypeScript client SDKs

---

## Contributing

Contributions welcome! Please open an issue first to discuss what you'd like to change.

```bash
git clone https://github.com/smriti-AA/smriti.git
cd smriti
cargo test
cargo build
```

## License

[MIT](LICENSE)
