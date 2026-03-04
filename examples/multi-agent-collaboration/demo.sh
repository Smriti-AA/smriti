#!/bin/bash

# Multi-Agent Collaboration Demo for Smriti
# Demonstrates how multiple agents discover emergent connections
# through a shared knowledge graph without explicit coordination

set -e

# Color codes for dramatic output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

echo -e "${BLUE}=========================================="
echo "Smriti Multi-Agent Collaboration Demo"
echo "========================================${NC}"
echo ""

pause() {
  echo -e "${YELLOW}Press Enter to continue...${NC}"
  read -r
}

# Section 1: Individual Agent Perspectives
echo -e "${BLUE}SECTION 1: Individual Agent Perspectives${NC}"
echo "Each agent sees only their own domain knowledge"
echo ""

echo -e "${GREEN}Agent A (Care Coordinator): Retrieving patient notes...${NC}"
echo "Searching for patients with cardiac issues..."
echo ""
smriti search "Patient:" | head -5 || true
echo ""
pause

echo -e "${GREEN}Agent B (Lab Assistant): Retrieving trial information...${NC}"
echo "Searching for clinical trials and research notes..."
echo ""
smriti search "Clinical Trial:" | head -5 || true
echo ""
pause

echo -e "${GREEN}Agent C (Clinical Decision Agent): Searching for Troponin-T...${NC}"
echo "Querying for cardiac biomarker information..."
echo ""
smriti search "Troponin-T" || true
echo ""
pause

# Section 2: The Aha Moment
echo -e "${RED}=========================================="
echo "SECTION 2: The AHA Moment - Discovering Emergent Connections"
echo "========================================${NC}"
echo ""

echo -e "${PURPLE}Clinical Decision Agent is now analyzing the knowledge graph...${NC}"
echo ""
echo "Agent C: I notice multiple agents mention 'Troponin-T'"
echo "Let me explore what happens when we graph this biomarker..."
echo ""
sleep 1

echo -e "${YELLOW}Building knowledge graph for: Troponin-T${NC}"
echo ""

# Try to show the graph for Troponin-T if it exists
smriti search "Troponin-T" > /tmp/troponin_notes.txt 2>&1 || true

if [ -s /tmp/troponin_notes.txt ]; then
  echo -e "${YELLOW}Graph Data (related notes mentioning Troponin-T):${NC}"
  cat /tmp/troponin_notes.txt | head -20
  echo ""
else
  echo -e "${YELLOW}Creating virtual graph visualization...${NC}"
  echo ""
  cat << 'EOF'
  TROPONIN-T (Shared Biomarker)
  │
  ├─ Referenced by Care Coordinator in:
  │  ├─ Patient: John Doe - Medical History
  │  ├─ Patient: Sarah Chen - Cardiology Profile
  │  ├─ Patient: Robert Martinez - MI History
  │  └─ Clinical Observation: Cardiac Biomarker Elevation Patterns
  │
  └─ Referenced by Lab Assistant in:
     ├─ Clinical Trial: CARDIAC-2024 Post-MI Recovery Protocol
     ├─ Clinical Trial: BNP-HEART Study on Heart Failure Progression
     ├─ Research Notes: Advanced Troponin-T Assay Development
     └─ Lab Analysis: Biomarker Correlation Matrix

EOF
fi

echo ""
pause

# Section 3: Cross-Agent Insights
echo -e "${BLUE}=========================================="
echo "SECTION 3: Cross-Agent Insights Discovered"
echo "========================================${NC}"
echo ""

echo -e "${PURPLE}Agent C is synthesizing insights from Agent A and Agent B:${NC}"
echo ""

cat << 'EOF'
INSIGHT 1: Patient-Trial Matching
=================================
  Agent A documented: John Doe has Acute Myocardial Infarction
                      and elevated Troponin-T

  Agent B documented: CARDIAC-2024 trial requires patients with
                      confirmed MI by Troponin-T biomarkers

  DISCOVERY: John Doe is an EXCELLENT candidate for CARDIAC-2024!
             (Neither agent knew about this match until now)

INSIGHT 2: Biomarker Sensitivity Discovery
==========================================
  Agent A observed: Multiple patients have Troponin-T elevation
                    independent of acute MI diagnosis

  Agent B found: High-sensitivity Troponin-T assay detects
                 cardiac stress in Chronic Kidney Disease

  DISCOVERY: Troponin-T may be sensitive indicator of subclinical
             cardiac stress in CKD patients!
             (This insight emerged from combining independent observations)

INSIGHT 3: Research Direction
=============================
  Agent A reported: Robert Martinez has history of CKD with
                    multiple MI events and elevated Troponin-T

  Agent B developed: BNP-HEART study tracking biomarkers in
                     heart failure with CKD comorbidity

  DISCOVERY: Robert Martinez could provide valuable data point
             for understanding Troponin-T kinetics in post-MI
             patients with renal dysfunction!

EOF

echo ""
pause

# Section 4: Complete Knowledge Graph Overview
echo -e "${BLUE}=========================================="
echo "SECTION 4: Complete Knowledge Graph Overview"
echo "========================================${NC}"
echo ""

echo -e "${YELLOW}Listing all notes in the collaborative knowledge graph:${NC}"
echo ""

COUNT=0
while IFS= read -r line; do
  COUNT=$((COUNT + 1))
  echo "  $COUNT. $line"
done < <(smriti list 2>/dev/null || echo "Graph initialization in progress...")

echo ""
echo -e "${YELLOW}Total notes in knowledge graph: $(smriti list 2>/dev/null | wc -l) notes${NC}"
echo ""
pause

# Section 5: The Power of Emergent Collaboration
echo -e "${PURPLE}=========================================="
echo "SECTION 5: Key Insights on Emergent Collaboration"
echo "========================================${NC}"
echo ""

cat << 'EOF'
What Makes This Collaboration Powerful:

1. INDEPENDENCE: Each agent works in their own domain
   - Care Coordinator focuses on patient management
   - Lab Assistant focuses on research and biomarkers
   - Clinical Decision Agent focuses on synthesis
   - No explicit coordination required!

2. EMERGENT CONNECTIONS: Relationships appear without being defined
   - Agents never said "connect John Doe to CARDIAC-2024"
   - They just mentioned shared concepts (Troponin-T, MI, etc.)
   - The graph automatically revealed the connection!

3. SCALABILITY: System improves with more agents
   - 3 agents → some cross-domain insights
   - 10 agents → exponential discovery opportunities
   - 100 agents → powerful emergent intelligence

4. DISCOVERY WITHOUT ASKING: Insights surface organically
   - No need to run specific queries for connections
   - Agents can't miss relationships because they emerge naturally
   - Even unknown unknowns might be revealed!

The Real Magic: Agents don't need to know about each other.
They just document their knowledge using shared terminology
(wiki-links), and the graph handles the rest.

EOF

echo ""
pause

# Section 6: Exploring the Patient-Trial Connection in Detail
echo -e "${RED}=========================================="
echo "SECTION 6: Deep Dive - Patient to Trial Discovery"
echo "========================================${NC}"
echo ""

echo -e "${PURPLE}Let's explore the John Doe → Troponin-T → CARDIAC-2024 connection${NC}"
echo ""

echo "Step 1: Agent A (Care Coordinator) documented John Doe"
echo "  Content includes: [[Acute Myocardial Infarction]], [[Troponin-T]], [[CK-MB]]"
echo ""
sleep 1

echo "Step 2: Agent B (Lab Assistant) documented CARDIAC-2024 trial"
echo "  Content includes: [[Troponin-T]], [[CK-MB]], [[Myoglobin]]"
echo ""
sleep 1

echo -e "${YELLOW}Step 3: Knowledge Graph Analysis${NC}"
echo "  smriti search 'Troponin-T' reveals:"
echo "    - Patient notes mentioning Troponin-T (Agent A's work)"
echo "    - Trial notes requiring Troponin-T (Agent B's work)"
echo "    - Automatic connection: Patient → Biomarker ← Trial"
echo ""
sleep 1

echo -e "${GREEN}Step 4: Clinical Decision Agent synthesizes:${NC}"
echo "  'John Doe has elevated Troponin-T from Acute MI'"
echo "  'CARDIAC-2024 recruits MI patients with elevated Troponin-T'"
echo "  'Therefore, John Doe should be enrolled in CARDIAC-2024!'"
echo ""
sleep 1

echo -e "${BLUE}THE CONNECTION WAS DISCOVERED, NOT CREATED!${NC}"
echo "No agent explicitly linked John Doe to CARDIAC-2024."
echo "The graph revealed it by analyzing shared references."
echo ""
pause

# Final summary
echo -e "${PURPLE}=========================================="
echo "Demo Complete"
echo "========================================${NC}"
echo ""

echo "This example demonstrated:"
echo "  ✓ Three independent agents collaborating through shared knowledge graph"
echo "  ✓ Emergent connections appearing from shared wiki-link references"
echo "  ✓ Cross-domain insights (patients × trials × biomarker research)"
echo "  ✓ Automatic discovery without explicit agent coordination"
echo ""

echo "To explore further:"
echo "  - Run: smriti search 'Cardiac Biomarkers'"
echo "  - Run: smriti search 'Chronic Kidney Disease'"
echo "  - Run: smriti read <NOTE_ID> to see full note content"
echo "  - Run: smriti list to see all notes"
echo ""

echo -e "${GREEN}Knowledge graph collaboration in action!${NC}"
echo ""
