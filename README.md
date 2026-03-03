# Smriti

*Sanskrit: स्मृति — memory, remembrance*

A lightning-fast, self-hosted knowledge store and memory layer for AI agents. Built in Rust.

## Why Smriti?

Every AI agent needs memory. Mem0 is cloud-only. Letta is research-heavy. Neither has a knowledge graph.

**Smriti is different:** self-hosted, knowledge-graph-native, MCP-ready, and fast enough to handle millions of operations. Your data never leaves your machine.

- **MCP Server** — Plug into Claude, GPT, or any MCP-compatible agent
- **Agent Memory** — Key-value store with namespaces, TTL, and tool execution logs
- **Knowledge Graph** — Notes auto-link via wiki-links; agents discover connections via graph traversal
- **Full-Text Search** — SQLite FTS5 with sub-millisecond queries
- **REST API** — Full CRUD + graph + agent endpoints
- **Sync** — Cross-device via Synology NAS, WebDAV, or any filesystem mount

## Quick Start

```bash
cargo install smriti

# Create a note
smriti create "Research on LLM Memory" --content "Key findings about [[Agent Architecture]] and #memory-systems"

# Search
smriti search "memory"

# Start API server
smriti serve --port 3000

# Start MCP server (for AI agents)
smriti mcp
```

### Build from source

```bash
git clone https://github.com/kishorereddyanekalla/smriti.git
cd smriti
cargo build --release
./target/release/smriti --help
```

## CLI

```
smriti create <title>     Create a note (--content, --file, --tags)
smriti read <id>          Read a note by ID or title
smriti list               List notes (--limit, --tag, --json)
smriti search <query>     Full-text search
smriti graph              Knowledge graph (--format json|dot|text)
smriti stats              Database stats + smart link suggestions
smriti serve              Start REST API (--port 3000)
smriti mcp                Start MCP server (stdio)
smriti sync               Sync with remote (--remote <path-or-url>)
smriti import <dir>       Import .md files (--recursive)
smriti export <dir>       Export to .md files (--frontmatter)
```

## MCP Server

Start with `smriti mcp`. Communicates via JSON-RPC 2.0 over stdio.

**Tools for agents:**

| Tool | Description |
|------|-------------|
| `notes_create` | Create a note with markdown content |
| `notes_read` | Read note by ID or title |
| `notes_search` | Full-text search across all notes |
| `notes_list` | List recent notes |
| `notes_graph` | Get knowledge graph or subgraph |
| `memory_store` | Store key-value memory with optional TTL |
| `memory_retrieve` | Retrieve a memory by key |
| `memory_list` | List all memory for an agent |

**Claude Desktop config** (`claude_desktop_config.json`):
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

## REST API

All endpoints at `http://localhost:3000/api/v1/`:

**Notes:** `POST /notes`, `GET /notes`, `GET /notes/:id`, `PUT /notes/:id`, `DELETE /notes/:id`, `GET /notes/search?q=...`, `GET /notes/:id/backlinks`, `GET /notes/:id/links`

**Graph:** `GET /graph`, `GET /graph/:id?depth=2`, `GET /stats`

**Agent Memory:** `POST /agent/:id/memory`, `GET /agent/:id/memory`, `GET /agent/:id/memory/:ns/:key`, `POST /agent/:id/tool-logs`, `GET /agent/:id/tool-logs`

## Sync

```bash
# Filesystem (Synology NAS mount, shared folder, etc.)
smriti sync --remote /Volumes/nas/smriti --direction both

# WebDAV
SYNC_USER=admin SYNC_PASS=secret smriti sync --remote https://nas:5006/smriti --direction both

# Or just point the DB at a Synology Drive synced folder
smriti --db ~/SynologyDrive/smriti.db create "Synced Note" --content "Hello"
```

## Architecture

```
src/
├── models/      Note, Link, AgentMemory, ToolLog
├── storage/     SQLite + FTS5 full-text search
├── parser/      [[wiki-link]] and #tag extraction
├── graph/       petgraph knowledge graph
├── api/         Axum REST API
├── mcp/         MCP JSON-RPC server (stdio)
├── cli/         clap CLI
├── sync/        WebDAV + filesystem sync
└── features/    Smart link suggestions, daily digest
```

**Stack:** Rust, Axum, SQLite (FTS5), petgraph, clap, serde, tokio

## License

MIT
