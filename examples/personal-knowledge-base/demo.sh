#!/bin/bash

# Smriti Personal Knowledge Base Demo
# Shows how to explore an interconnected knowledge base

echo "======================================"
echo "Smriti Personal Knowledge Base Demo"
echo "======================================"
echo ""

# Demo 1: List all notes to see what we have
echo "--- DEMO 1: What's in your knowledge base? ---"
echo "Running: smriti list"
echo ""
smriti list
echo ""

# Demo 2: Database statistics
echo "--- DEMO 2: Knowledge base growth ---"
echo "Running: smriti stats"
echo ""
smriti stats
echo ""

# Demo 3: Search for a concept
echo "--- DEMO 3: Finding related notes ---"
echo "Query: 'async' — What have you captured about async programming?"
echo "Running: smriti search 'async'"
echo ""
smriti search "async"
echo ""

# Demo 4: Read a daily note to see entry points
echo "--- DEMO 4: Looking at a daily note ---"
echo "Running: smriti read (ID for '2026-03-03 Daily Note')"
echo ""
echo "This shows how daily notes link to concepts and projects."
echo "Each day, you capture what you learned and what you're working on."
echo ""
DAILY_NOTE_ID=$(smriti list | grep "2026-03-03 Daily Note" | awk '{print $1}')
if [ -n "$DAILY_NOTE_ID" ]; then
  smriti read "$DAILY_NOTE_ID"
else
  echo "Daily note not found - note IDs may vary in your setup"
fi
echo ""

# Demo 5: Explore connections with graph
echo "--- DEMO 5: How ideas connect (knowledge graph) ---"
echo "Let's see what connects to 'Async Programming'"
echo "Running: smriti graph --note 'Async Programming' --depth 2"
echo ""
smriti graph --note "Async Programming" --depth 2
echo ""

# Demo 6: Follow the connection path
echo "--- DEMO 6: From daily notes to projects through concepts ---"
echo "This is how a Zettelkasten creates insights:"
echo ""
echo "1. You capture in daily notes: 'Learned about async patterns'"
echo "2. You create a concept note: 'Async Programming - patterns and Rust futures'"
echo "3. You read articles and link them: 'Reading: Async Patterns in Systems Design -> [[Async Programming]]'"
echo "4. You start projects using it: 'Project: Real-time Chat Server -> [[Async Programming]]'"
echo "5. You get insights: Search 'async' and see all connections"
echo ""
echo "Running: smriti search 'async'"
echo ""
smriti search "async"
echo ""

# Demo 7: Explore an emerging idea
echo "--- DEMO 7: Emerging ideas from connections ---"
echo "Looking at 'Idea: Memory-efficient Async Runtime'"
echo "This idea emerged from connecting three concepts:"
echo "  - [[Async Programming]]"
echo "  - [[Rust Ownership]]"
echo "  - [[Futures and Promises]]"
echo ""
IDEA_ID=$(smriti list | grep "Memory-efficient Async Runtime" | awk '{print $1}')
if [ -n "$IDEA_ID" ]; then
  smriti read "$IDEA_ID"
else
  echo "Idea not found - note IDs may vary"
fi
echo ""

# Demo 8: See how a reading connects to your work
echo "--- DEMO 8: Tracing influence from reading to project ---"
echo "Follow this path:"
echo "  Reading: 'Reading: Programming in Rust'"
echo "    links to: 'Rust Ownership', 'Borrowing', 'Type System'"
echo "  All three link to: 'Project: CLI Tool'"
echo ""
echo "Running: smriti search 'rust'"
echo ""
smriti search "rust"
echo ""

# Demo 9: One-off search for project ideas
echo "--- DEMO 9: Finding your projects and ideas ---"
echo "Running: smriti search 'project'"
echo ""
smriti search "project"
echo ""

echo "--- DEMO 10: Searching by tag ---"
echo "Running: smriti search '#async'"
echo ""
smriti search "#async"
echo ""

echo "======================================"
echo "Demo Complete!"
echo "======================================"
echo ""
echo "Next Steps:"
echo "  1. Explore more with: smriti search 'ownership'"
echo "  2. Try: smriti graph --note 'Rust Ownership' --depth 3"
echo "  3. Create your own daily note with: smriti create 'Today's Date' --content 'What you learned [[Link to Concept]]'"
echo "  4. See connections grow as you add notes!"
echo ""
echo "Key insight: Your knowledge base grows through daily capture,"
echo "and insights emerge naturally from the connections you make."
