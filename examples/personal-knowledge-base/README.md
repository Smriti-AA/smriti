# Personal Knowledge Base with Smriti

Build a Zettelkasten-style personal knowledge base using Smriti's wiki-link system.

## What is a Zettelkasten?

A Zettelkasten (German for "slip box") is a personal knowledge management system where:

- **Each note is atomic** — focused on a single idea or concept
- **Notes are interconnected** — wiki-links create a web of relationships
- **Connections emerge over time** — insights come from linking existing notes
- **The structure grows organically** — no rigid hierarchy, just natural connections

Instead of filing ideas in folders, you let the connections between ideas reveal the structure.

## How Smriti Supports Zettelkasten

Smriti uses **wiki-links** (`[[note-title]]`) to create a knowledge graph:

```bash
smriti create "Rust Ownership" --content "A system that manages memory safety without garbage collection. [[Memory Management]] [[Type System]]"
```

This creates bidirectional connections — Rust Ownership now links to (and is linked from) Memory Management and Type System.

## Daily Workflow

### 1. Capture Daily Notes
Each day, create a quick capture note:

```bash
smriti create "2026-03-04 Daily Note" --content "Read about async patterns. [[Async Programming]] was interesting. Starting [[Project: API Server]]."
```

### 2. Create Concept Notes
When you learn something, extract the concept:

```bash
smriti create "Async Programming" --content "Allows concurrent execution. Key patterns: futures, promises, async/await. [[Rust Ownership]] matters here."
```

### 3. Link Projects to Concepts
Your projects become connections between concepts:

```bash
smriti create "Project: API Server" --content "Building a REST API in Rust. Uses [[Async Programming]] and [[Error Handling]]."
```

### 4. Capture Reading Notes
Take notes from books/articles and link them:

```bash
smriti create "Reading: Async Rust by Example" --content "Chapter 3 explains futures. Key insight: futures are lazy. [[Async Programming]] #rust #learning"
```

## Exploring Your Knowledge Base

**Search for connections:**
```bash
smriti search "async"
```

**Visualize how ideas connect:**
```bash
smriti graph --note "Async Programming" --depth 2
```

**See what you've captured:**
```bash
smriti stats
```

## Example: From Daily Note to Insight

1. **Day 1**: Daily note mentions "async patterns"
2. **Day 2**: Create concept note on "Async Programming"
3. **Day 3**: Reading note links to that concept
4. **Day 4**: Project note links to the concept AND the reading
5. **Result**: `smriti graph --note "Async Programming"` shows how your daily work, learning, and projects all connect

This is the power of Zettelkasten — insights emerge from the connections you create naturally.

## Tags for Organization

Use `#tags` to add lightweight categorization without creating hierarchies:

```bash
smriti create "Note Title" --content "Content here #rust #async #learning"
```

Tags supplement wiki-links — they help you find related notes when you search.

## Getting Started

1. Run `bash setup.sh` to create an example knowledge base
2. Run `bash demo.sh` to see it in action
3. Adapt the examples to your own interests and workflow

---

For more on Zettelkasten, see Niklas Luhmann's original system or "How to Take Smart Notes" by Sönke Ahrens.
