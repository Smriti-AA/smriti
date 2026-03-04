# Smriti MCP + Claude Integration

This guide shows how to connect Smriti (an AI-native notes and memory system) to Claude Desktop or any other MCP client, enabling Claude to create, search, and manage notes and agent memory directly.

## What is Smriti?

Smriti is a CLI tool that provides an MCP (Model Context Protocol) server for managing notes and agent memory. It allows Claude to:
- Create and organize notes with metadata
- Search through your knowledge base
- Explore connections between topics via graph relationships
- Store and retrieve agent memory with configurable TTL (time-to-live)
- Build healthcare workflows with structured data

## Setting Up Smriti with Claude Desktop

### 1. Install Smriti

First, ensure you have Smriti installed on your system. Follow the installation instructions in the main Smriti repository.

### 2. Configure Claude Desktop

Claude Desktop reads MCP server configurations from a config file:

**Linux/Mac:**
```
~/.config/Claude/claude_desktop_config.json
```

**Windows:**
```
%APPDATA%\Claude\claude_desktop_config.json
```

### 3. Add Smriti to Your Config

Edit your `claude_desktop_config.json` file and add the Smriti MCP server:

```json
{
  "mcpServers": {
    "smriti": {
      "command": "smriti",
      "args": ["mcp"]
    }
  }
}
```

If you already have other MCP servers configured, add Smriti to the `mcpServers` object:

```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/home/user/projects"]
    },
    "smriti": {
      "command": "smriti",
      "args": ["mcp"]
    }
  }
}
```

### 4. Restart Claude Desktop

After updating the config, restart Claude Desktop. The Smriti server will start automatically and its tools will become available to Claude.

### 5. Verify Connection

In a new Claude conversation, Claude should be able to use Smriti tools. You can verify this by asking Claude something like: "What Smriti tools do you have available?" Claude will show you the list of connected tools.

## Available Tools

Once Smriti is connected, Claude has access to 8 tools:

### Notes Management
- **`notes_create`** — Create a new note with title, content, and optional metadata (tags, links, references)
- **`notes_read`** — Retrieve a specific note by ID or name
- **`notes_search`** — Full-text search across all notes, with optional filtering by tags
- **`notes_list`** — List all notes, optionally filtered by metadata criteria
- **`notes_graph`** — Explore the knowledge graph: find related notes, discover connection paths between topics

### Agent Memory
- **`memory_store`** — Store a fact, preference, or context in agent memory with optional namespace and TTL (time-to-live in seconds)
- **`memory_retrieve`** — Retrieve stored memories by key or namespace
- **`memory_list`** — List all memories, optionally filtered by namespace or creation time

## How Agent Memory Works

Agent memory allows Claude to store context that persists across conversations or operations. Key concepts:

- **Namespace:** Organize memories by topic (e.g., "user_preferences", "healthcare:patient_123", "project_alpha")
- **TTL (Time-to-Live):** Memories can expire automatically. Set TTL in seconds, or omit for permanent storage
- **Key-Value Storage:** Store any structured or unstructured data (strings, JSON objects, etc.)

Example: Store user preferences with a 30-day TTL:
```json
{
  "key": "favorite_note_format",
  "value": "markdown_with_timestamps",
  "namespace": "user_preferences",
  "ttl_seconds": 2592000
}
```

Later, Claude can retrieve this and use it to format notes according to your preferences.

## Typical Workflow

1. **Initialize your workspace** — Ask Claude to create a namespace for your project and set up initial memory
2. **Create and organize notes** — As you work with Claude, it creates notes of key findings, decisions, and context
3. **Search and explore** — Ask Claude to search notes or explore the knowledge graph to find related information
4. **Maintain context** — Claude stores relevant preferences and project context in agent memory for future sessions
5. **Discover insights** — Use the graph tool to uncover unexpected connections between topics

## Example Conversation Starter

Try opening Claude Desktop and pasting this:

> "I'd like to set up a personal knowledge management system using Smriti. Can you:
> 1. Create a namespace called 'my_knowledge' in agent memory
> 2. Create a note about my learning goals
> 3. Show me how I can use this system to build a knowledge graph"

Claude will use the Smriti tools to set up your system and explain what it's doing.

## Troubleshooting

### Smriti tools not appearing
- Ensure `smriti` command is in your PATH
- Check that the config file syntax is valid JSON
- Restart Claude Desktop after editing the config
- Check Claude Desktop's settings for MCP server status

### Connection errors
- Verify Smriti is installed and `smriti mcp` runs without errors in your terminal
- Check for firewall or permission issues
- Review Claude Desktop logs for error messages

### Memory not persisting
- Verify the backend database is writable at Smriti's data directory
- Check the TTL settings if memories are expiring unexpectedly
- Use `memory_list` to verify memories were actually stored

## For Developers

If you're building an MCP client other than Claude Desktop, connect to Smriti's JSON-RPC 2.0 server:

```bash
smriti mcp
```

This starts the server on stdio. Send JSON-RPC 2.0 requests to call any of the 8 tools. See the Smriti documentation for JSON-RPC protocol details.

## Learn More

- **Smriti Documentation:** See the main Smriti repository for detailed tool parameters and response formats
- **Model Context Protocol:** Learn about MCP at https://modelcontextprotocol.io
- **Claude Desktop:** Documentation at https://support.anthropic.com/en/articles/8784995-claude-desktop

---

Happy note-taking! Claude + Smriti enables you to build intelligent, persistent knowledge systems.
