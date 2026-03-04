# Multi-Agent Collaboration with Smriti

## Overview

This example demonstrates how multiple AI agents with different expertise domains can collaborate through a shared Smriti knowledge graph. Each agent works independently with their own domain knowledge, but their notes automatically create cross-agent connections through wiki-links, enabling discovery of insights that emerge from the combination of their perspectives.

## The Scenario

Three healthcare AI agents collaborate to improve patient care and trial matching:

### Agent A: Care Coordinator
- **Domain**: Patient care management and clinical history
- **Role**: Documents patient conditions, symptoms, and medical history
- **Focus**: Creating detailed patient profiles with relevant medical markers

### Agent B: Lab Assistant
- **Domain**: Biomedical research and clinical trials
- **Role**: Documents research findings, biomarkers, and trial information
- **Focus**: Creating trial information with biomarker requirements and outcomes

### Agent C: Clinical Decision Agent
- **Domain**: Data synthesis and clinical decision support
- **Role**: Queries the knowledge graph to discover connections
- **Focus**: Finding patient-to-trial matches through shared biomarkers and conditions

## How It Works

### The Magic: Wiki-Links Create Automatic Connections

When multiple agents independently mention the same concept (e.g., `[[Cardiac Biomarkers]]` or `[[Troponin-T]]`), the knowledge graph automatically creates backlinks and relationships:

1. **Care Coordinator** creates a patient note mentioning `[[Troponin-T]]`
2. **Lab Assistant** creates a trial note mentioning `[[Troponin-T]]`
3. **Clinical Decision Agent** queries `smriti graph --note "Troponin-T"` and discovers both the patient AND the trial—without ever needing to explicitly connect them

This emergent capability—discovering relationships that no agent individually created—is the power of collaborative knowledge graphs.

## Prerequisites

- Smriti CLI installed and accessible via `smriti` command
- Bash shell (sh or bash)
- Write permissions to the working directory

## File Structure

- **README.md** - This file
- **setup.sh** - Simulates three agents creating notes in the knowledge graph
- **demo.sh** - Demonstrates cross-agent discovery and insights

## Running the Example

### Step 1: Initialize the Knowledge Graph

```bash
bash setup.sh
```

This script runs through three phases:
- **Phase 1**: Care Coordinator adds patient notes with condition and biomarker references
- **Phase 2**: Lab Assistant adds trial and research notes with biomarker requirements
- **Phase 3**: Graph is populated and ready for discovery

### Step 2: Demonstrate Cross-Agent Discovery

```bash
bash demo.sh
```

This script demonstrates:
- Each agent's isolated view of the knowledge graph
- The combined graph revealing cross-agent connections
- Graph traversal that discovers: Patient → Biomarkers → Trial
- The moment when Agent C synthesizes insights from Agents A and B

## Key Concepts

### Emergent Connections
Connections emerge naturally through shared wiki-links. Agents don't need to know about each other or explicitly coordinate—just mention related concepts, and the graph handles discovery.

### Domain Isolation with Global Discovery
Each agent maintains domain expertise (cardiology, biochemistry, clinical decisions) while benefiting from insights across domains through the shared knowledge graph.

### Scalability
As more agents add notes, more connections emerge. The system scales beyond pairwise agent relationships to support many agents and domains.

## Example Connections Discovered

After running the scripts, you'll see the graph discover connections like:

```
Patient: John Doe
  └─ Has: Acute Myocardial Infarction
      └─ Requires monitoring of: Troponin-T
          └─ Trial CARDIAC-2024 measures: Troponin-T
              └─ Focus: Post-MI Recovery
```

This connection was never explicitly created by any single agent—it emerged from their independent documentation of related concepts.

## Customization

You can modify the agents and domains by editing `setup.sh` and `demo.sh`:
- Add more agents by creating additional phases
- Change biomarkers and conditions to match your domain
- Expand the demo queries to explore different connection pathways

## Troubleshooting

- **`smriti: command not found`**: Ensure Smriti is installed and in your PATH
- **Empty graph**: Run `setup.sh` first to populate the knowledge graph
- **Permission errors**: Ensure you have write permissions in the current directory

## Further Reading

- Smriti CLI documentation for advanced graph queries
- Wiki-link syntax for creating cross-references
- Graph traversal techniques for knowledge discovery
