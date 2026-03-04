# Smriti Examples

Practical examples showing how to use Smriti as a knowledge store and memory layer for AI agents.

## Examples

| Example | Description | Key Features |
|---------|-------------|--------------|
| [Healthcare Care Coordinator](./healthcare-care-coordinator/) | Digital care coordinator monitoring patient records and wearable data | Wiki-links, graph traversal, agent memory with TTL |
| [Lab Trial Optimizer](./lab-trial-optimizer/) | Autonomous lab assistant optimizing experimental design | Full-text search, knowledge graph, structured notes |
| [Multi-Agent Collaboration](./multi-agent-collaboration/) | Multiple agents sharing knowledge through a common graph | Cross-agent discovery, backlinks, graph BFS |
| [MCP + Claude Integration](./mcp-claude-integration/) | Connect Smriti to Claude via Model Context Protocol | MCP server, JSON-RPC, tool usage |
| [Personal Knowledge Base](./personal-knowledge-base/) | Build a Zettelkasten-style knowledge base | Wiki-links, tags, daily notes, search |

## Prerequisites

```bash
# Install Smriti
cargo install smriti

# Verify installation
smriti --version
```

## Quick Start

Each example includes:
- `README.md` — What it does and how to run it
- `setup.sh` — Shell script to set up the example data
- `demo.sh` — Interactive demo you can run

Pick any example folder and follow its README.

## Running Examples

```bash
# Clone the repo
git clone https://github.com/smriti-AA/smriti.git
cd smriti/examples

# Run any example
cd healthcare-care-coordinator
chmod +x setup.sh demo.sh
./setup.sh
./demo.sh
```

## Architecture

All examples use Smriti's three interfaces:

```
AI Agents ──► MCP Server (stdio)  ──► SQLite
Developers ──► REST API (HTTP)    ──► Knowledge Graph
Terminal   ──► CLI                ──► Full-Text Search
```
