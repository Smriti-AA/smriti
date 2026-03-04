# Smriti Lab Trial Optimizer Example

## Overview

This example demonstrates an **autonomous lab assistant** that uses Smriti's knowledge graph to optimize experimental design for drug trials. The assistant tracks drug compounds, biomarker responses, dosage curves, trial protocols, and adverse events—then uses graph traversal to discover optimization opportunities.

## Use Case: Pharmaceutical Research

In drug development, researchers need to:
- Track multiple test compounds and their properties
- Monitor biomarker responses across different dosages
- Correlate adverse events with dosage levels
- Identify which compounds share biomarker pathways
- Optimize experimental protocols based on discovered relationships

This example shows how an AI-powered lab assistant can query interconnected trial data to make intelligent decisions about compound selection, dosing strategies, and biomarker tracking.

## Prerequisites

1. **Smriti installed**:
   ```bash
   cargo install smriti
   ```

2. **Smriti initialized**:
   ```bash
   smriti init
   ```
   (Creates a `.smriti` directory in the current working directory)

## Files in This Example

- **setup.sh** — Creates 8-10 interconnected notes representing compounds, biomarkers, dosage curves, protocols, and adverse events
- **demo.sh** — Demonstrates graph traversal and optimization queries
- **README.md** — This file

## How to Run

### 1. Setup Phase
```bash
cd /path/to/lab-trial-optimizer
bash setup.sh
```

This creates a knowledge base with realistic pharmaceutical data, including:
- **Compounds**: CB-209, CB-415 with pharmacokinetic properties
- **Biomarkers**: Troponin-T, BNP, CRP with clinical significance
- **Dosage Curves**: Efficacy and safety data at different doses
- **Protocols**: Phase I, II, III trial designs
- **Adverse Events**: AE logs linked to dosages and biomarkers

### 2. Demo Phase
```bash
bash demo.sh
```

The demo performs several queries:
1. **Compound Search** — Find efficacy data for a specific compound
2. **Graph Traversal** — From compound → dosage curves → biomarkers → adverse events
3. **Cross-compound Analysis** — Discover that CB-209 and CB-415 both affect the same biomarker pathway
4. **Optimization Recommendation** — Suggest trial design based on discovered relationships

## Expected Output

When you run `demo.sh`, you'll see:
- Search results highlighting compound efficacy data
- Traversal paths showing relationships between compounds and biomarkers
- Detection of shared biomarker pathways across compounds
- Actionable recommendations for trial optimization (e.g., "CB-209 shows favorable Troponin-T profile; recommend Phase II expansion")

## Knowledge Graph Structure

```
Compound (CB-209)
    ├── [[Dosage-Curve-CB209-100mg]]
    │   └── [[Biomarker-TroponinT]]
    │       └── [[Literature-Troponin-Cardiotoxicity]]
    ├── [[Dosage-Curve-CB209-300mg]]
    │   └── [[Biomarker-BNP]]
    └── [[AE-Log-CB209-GI-Upset]]
        └── [[Dosage-300mg]]

Protocol (Phase-II-Cardiotoxicity)
    ├── [[Compound-CB-209]]
    ├── [[Biomarker-TroponinT]]
    └── [[Biomarker-BNP]]
```

## Example Queries

After setup, try these Smriti commands:

```bash
# Find all biomarkers
smriti search "biomarker"

# Traverse a compound's relationships
smriti graph --note CB-209 --depth 3

# Find adverse events
smriti search "adverse event"

# List all notes to see the knowledge base
smriti list
```

## Key Insights for Lab Optimization

This example shows how an AI assistant can use Smriti to:
1. **Consolidate data** — All trial info in one searchable, linkable database
2. **Discover patterns** — Graph traversal reveals which compounds affect the same pathways
3. **Optimize design** — Combine knowledge about dosage, biomarkers, and AEs to suggest efficient protocols
4. **Support decisions** — Quick access to related literature and historical trial data

## Next Steps

Extend this example by:
- Adding patient demographic notes linked to trial protocols
- Creating notes for manufacturing considerations linked to compounds
- Building regulatory pathway notes linked to protocols
- Adding cost-benefit analysis notes that traverse compound and protocol nodes

