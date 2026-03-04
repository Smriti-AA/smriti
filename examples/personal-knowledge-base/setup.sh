#!/bin/bash

# Smriti Personal Knowledge Base Setup
# Creates an interconnected knowledge base demonstrating a developer's learning journey

echo "Building personal knowledge base with Smriti..."
echo ""

# Daily Notes - Entry points for daily capture
echo "Creating daily notes..."
smriti create "2026-03-01 Daily Note" \
  --content "Spent time learning about memory management in Rust. Read about ownership and borrowing. [[Rust Ownership]] seems critical for understanding the language. Also started thinking about [[Async Programming]] for a future project."

smriti create "2026-03-02 Daily Note" \
  --content "Implemented a simple function using Rust patterns. Ran into issues with lifetimes. Found [[Rust Ownership]] and [[Borrowing]] concepts clearer now. [[Knowledge Graphs]] article on HN was interesting - relates to how I'm building connections between ideas."

smriti create "2026-03-03 Daily Note" \
  --content "Code review taught me about [[Error Handling]] patterns. Also researched [[Async Programming]] for the [[Project: Real-time Chat Server]]. Need to understand [[Futures and Promises]]."

# Concept Notes - Core building blocks of knowledge
echo "Creating concept notes..."

smriti create "Rust Ownership" \
  --content "Rust's memory management system. Every value has an owner. When owner goes out of scope, memory is freed. This eliminates need for garbage collection. Key concepts: [[Ownership]], [[Borrowing]], [[Lifetimes]]. Prevents entire classes of memory bugs. #rust #memory-safety"

smriti create "Borrowing" \
  --content "Temporary access to a value without taking ownership. References are either mutable or immutable. Mutable references are exclusive (only one at a time). Related to [[Rust Ownership]]. #rust"

smriti create "Lifetimes" \
  --content "Rust's way of validating that references are valid. Every reference has a lifetime. Lifetimes ensure you don't use references after the data they point to is freed. Works with [[Rust Ownership]] and [[Borrowing]]. #rust"

smriti create "Async Programming" \
  --content "Concurrent execution pattern. Allows one thread to handle many tasks. Key models: callbacks, promises, async/await. In Rust: [[Futures and Promises]]. Enables non-blocking I/O. Essential for [[Project: Real-time Chat Server]]. #async #concurrency"

smriti create "Futures and Promises" \
  --content "A future represents a value that may not be available yet. Composable with combinators. Rust futures are lazy - they do nothing until awaited. Core to understanding [[Async Programming]]. #async #rust"

smriti create "Error Handling" \
  --content "Managing failures gracefully. Rust uses Result type instead of exceptions. Pattern matching on Results. Related to type safety. Learned from [[Reading: Programming in Rust]] and code reviews. #rust"

smriti create "Knowledge Graphs" \
  --content "Data structures representing interconnected concepts. Nodes are entities, edges are relationships. Similar to what we're building here with [[wiki-links]]. Used in semantic search and reasoning. Inspired [[Project: Real-time Chat Server]] architecture."

smriti create "Type System" \
  --content "Compile-time checking of type correctness. Rust's strong static typing prevents many runtime errors. Works together with [[Rust Ownership]]. Enables zero-cost abstractions. #rust"

# Project Notes - Applying knowledge
echo "Creating project notes..."

smriti create "Project: Real-time Chat Server" \
  --content "Building a high-performance chat server in Rust. Uses [[Async Programming]] with tokio for handling many concurrent connections. Implements [[Error Handling]] patterns for robustness. Architecture inspired by [[Knowledge Graphs]] concepts. Started 2026-03-03. Status: In Progress. #project #rust"

smriti create "Project: CLI Tool" \
  --content "A command-line tool that demonstrates [[Type System]] best practices. Uses [[Error Handling]] patterns from [[Reading: Programming in Rust]]. Small focused project to solidify Rust fundamentals. #project #rust"

# Reading Notes - Learning sources
echo "Creating reading notes..."

smriti create "Reading: Programming in Rust" \
  --content "Official Rust book. Chapter 4 on [[Rust Ownership]] was enlightening. Explains [[Borrowing]] and [[Lifetimes]] clearly. Chapter 10 covers [[Type System]] and generics. Foundational resource. #book #rust #learning"

smriti create "Reading: Async Patterns in Systems Design" \
  --content "Article on medium about [[Async Programming]] approaches. Compares callbacks, promises, and async/await. Key insight: [[Futures and Promises]] in Rust are more composable than traditional approaches. Relevant for [[Project: Real-time Chat Server]]. #article #async #learning"

smriti create "Reading: Building Scalable Systems" \
  --content "Explores [[Knowledge Graphs]] for system design. Discusses non-blocking I/O and [[Async Programming]]. Inspired thinking about architecture for [[Project: Real-time Chat Server]]. #article #systems-design"

# Emerging Ideas - Insights from connections
echo "Creating emerging ideas..."

smriti create "Idea: Memory-efficient Async Runtime" \
  --content "Insight from connecting [[Async Programming]], [[Rust Ownership]], and [[Futures and Promises]]. Could Rust's ownership system optimize async runtime memory usage? Inspired by [[Reading: Async Patterns in Systems Design]] and work on [[Project: Real-time Chat Server]]. #idea #research"

smriti create "Idea: Type-safe Error Propagation" \
  --content "Observation: [[Rust Ownership]] + [[Error Handling]] could provide compile-time guarantees about error flows. Different from typical type systems. Started thinking about this after [[Reading: Programming in Rust]] and [[Project: CLI Tool]]. #idea #rust"

smriti create "Idea: Knowledge Graph Query Language" \
  --content "Build a query interface for [[Knowledge Graphs]]. Use [[Async Programming]] for concurrent queries. Inspired by how Smriti works. Could be useful for [[Project: Real-time Chat Server]] configuration. #idea #project-idea"

echo ""
echo "Knowledge base created! Run 'smriti list' to see all notes."
echo "Run 'bash demo.sh' to explore the connections."
