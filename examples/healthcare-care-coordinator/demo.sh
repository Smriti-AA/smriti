#!/bin/bash

# Healthcare Care Coordinator Example - Demonstration Script
# This script demonstrates how the care coordinator discovers connections
# between patients, conditions, medications, and wearable alerts.

# Color codes for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to display section headers
print_header() {
  echo -e "\n${BLUE}════════════════════════════════════════════════════════${NC}"
  echo -e "${BLUE}$1${NC}"
  echo -e "${BLUE}════════════════════════════════════════════════════════${NC}\n"
}

# Function to display a command being run
run_command() {
  echo -e "${YELLOW}▶ Running: $1${NC}"
  echo ""
}

print_header "SMRITI HEALTHCARE CARE COORDINATOR DEMONSTRATION"

# Intro
echo -e "${GREEN}This demo shows how a care coordinator uses Smriti to discover"
echo -e "connections between patient records, medications, and alerts.${NC}\n"

# ============================================================================
print_header "PART 1: Listing All Patient Records"
# ============================================================================

run_command "smriti list"
echo "This command shows all notes in the knowledge base."
echo "We expect to see: 2 patients, 3 conditions, 3 medications, 1 wearable device, 1 risk assessment, and 1 care plan."
echo ""
smriti list
echo ""

# ============================================================================
print_header "PART 2: Searching for Cardiac-Related Records"
# ============================================================================

run_command "smriti search \"cardiac\""
echo "The care coordinator searches for all cardiac-related notes."
echo "This helps quickly identify high-risk cardiovascular cases."
echo ""
smriti search "cardiac"
echo ""

# ============================================================================
print_header "PART 3: Searching for Real-Time Alerts"
# ============================================================================

run_command "smriti search \"alert\""
echo "Find all active alerts from wearables and clinical assessments."
echo "This surfaces urgent findings requiring immediate attention."
echo ""
smriti search "alert"
echo ""

# ============================================================================
print_header "PART 4: Locating Atrial Fibrillation Information"
# ============================================================================

run_command "smriti search \"Atrial Fibrillation\""
echo "Search for the specific condition to see all related notes."
echo "Results should show: the condition definition, the patient, and recommended medications."
echo ""
smriti search "Atrial Fibrillation"
echo ""

# ============================================================================
print_header "PART 5: Graph Traversal - Starting from Patient James Mitchell"
# ============================================================================

echo -e "${GREEN}SCENARIO: A care coordinator needs to review James Mitchell's care status.${NC}"
echo "She starts with the patient record and traverses the graph to discover:"
echo "  1. His active conditions"
echo "  2. Current medications managing those conditions"
echo "  3. Recent wearable data"
echo "  4. Associated risk assessments"
echo ""

run_command "smriti read 1"
echo "Reading the patient note (ID 1) shows:"
echo "  - Demographics"
echo "  - Active conditions with [[wiki-links]]"
echo "  - Recent alerts"
echo ""
smriti read 1
echo ""

# ============================================================================
print_header "PART 6: Expanding the View - Condition Details"
# ============================================================================

echo -e "${GREEN}NEXT STEP: The care coordinator wants to understand Atrial Fibrillation.${NC}"
echo "She searches for the condition to understand treatment requirements."
echo ""

run_command "smriti search \"Condition: Atrial Fibrillation\""
echo "This reveals:"
echo "  - Clinical definition"
echo "  - Which patients have this condition"
echo "  - Which medications treat it"
echo "  - Monitoring parameters"
echo "  - Risk assessment tools"
echo ""
smriti search "Condition: Atrial Fibrillation"
echo ""

# ============================================================================
print_header "PART 7: Verifying Medication Appropriateness"
# ============================================================================

echo -e "${GREEN}SCENARIO: Verify that medications are appropriate for the condition.${NC}"
echo ""

run_command "smriti search \"Medication: Apixaban\""
echo "Looking up the anticoagulant to confirm it's indicated for AFib."
echo ""
smriti search "Medication: Apixaban"
echo ""

run_command "smriti search \"Medication: Metoprolol\""
echo "Checking the beta blocker for rate control."
echo ""
smriti search "Medication: Metoprolol"
echo ""

# ============================================================================
print_header "PART 8: Real-Time Wearable Data Integration"
# ============================================================================

echo -e "${GREEN}SCENARIO: The patient's smartwatch has detected irregular heartbeat.${NC}"
echo "The care coordinator reviews the wearable data to assess urgency."
echo ""

run_command "smriti search \"Wearable\""
echo "This shows the real-time vital signs linked to the patient."
echo ""
smriti search "Wearable"
echo ""

# ============================================================================
print_header "PART 9: Risk Assessment - Identifying High-Risk Patients"
# ============================================================================

echo -e "${GREEN}SCENARIO: Priority triage - which patients need urgent review?${NC}"
echo ""

run_command "smriti search \"HIGH RISK\""
echo "Searching for risk assessments highlights high-priority cases."
echo "Result: James Mitchell with CHA₂DS₂-VASc score of 3 (high stroke risk)."
echo ""
smriti search "HIGH RISK"
echo ""

# ============================================================================
print_header "PART 10: Care Plan Review and Action Items"
# ============================================================================

echo -e "${GREEN}SCENARIO: What follow-up is needed for James Mitchell?${NC}"
echo ""

run_command "smriti search \"Care Plan\""
echo "The care plan contains:"
echo "  - Summary of all active conditions"
echo "  - Current vital signs status"
echo "  - Risk assessment link"
echo "  - Specific action items with checkboxes"
echo "  - Scheduled follow-ups"
echo ""
smriti search "Care Plan"
echo ""

# ============================================================================
print_header "PART 11: Discovering All Connected Records via Graph"
# ============================================================================

echo -e "${GREEN}SCENARIO: Map out all connections from a single patient record.${NC}"
echo "This shows the care coordinator the complete picture of care."
echo ""

run_command "smriti graph --note 1 --depth 2"
echo "Graph visualization starting from Patient James Mitchell (Note 1)."
echo "Depth=2 means we traverse 2 levels of [[wiki-link]] connections."
echo ""
echo "Expected connections:"
echo "  Level 1: Patient → Conditions, Medications, Wearable"
echo "  Level 2: Condition → Recommended medications, Risk assessment"
echo "           Medication → Indications, Monitoring parameters"
echo "           Wearable → Current vital signs, Linked medications"
echo ""
smriti graph --note 1 --depth 2
echo ""

# ============================================================================
print_header "PART 12: Medication Cross-Checking"
# ============================================================================

echo -e "${GREEN}SCENARIO: Ensure no drug interactions or contraindications.${NC}"
echo ""

run_command "smriti search \"Medication\""
echo "List all medications for this patient and verify interactions."
echo "James Mitchell is on:"
echo "  - Apixaban (anticoagulant) - check for bleeding risk"
echo "  - Metoprolol (beta blocker) - check for bradycardia"
echo "  - Metformin (for diabetes) - check renal function"
echo ""
smriti search "Medication"
echo ""

# ============================================================================
print_header "PART 13: Hypertension Management Across Multiple Patients"
# ============================================================================

echo -e "${GREEN}SCENARIO: How many patients are being managed for hypertension?${NC}"
echo "Compare treatment approaches across the patient population."
echo ""

run_command "smriti search \"Hypertension\""
echo "This shows the condition is treated in multiple patients:"
echo "  - James Mitchell (elevated readings, on Metoprolol)"
echo "  - Maria Gonzalez (stable control, on Lisinopril + Amlodipine)"
echo ""
smriti search "Hypertension"
echo ""

# ============================================================================
print_header "PART 14: Sleep Quality and Cardiac Health Connection"
# ============================================================================

echo -e "${GREEN}SCENARIO: Poor sleep can worsen AFib. Is James sleeping enough?${NC}"
echo ""

run_command "smriti search \"sleep\""
echo "The wearable data shows James is only sleeping 6.2 hours (target: 7-9)."
echo "This is a modifiable risk factor that should be addressed in care planning."
echo ""
smriti search "sleep"
echo ""

# ============================================================================
print_header "CARE COORDINATOR INSIGHTS SUMMARY"
# ============================================================================

cat << 'EOF'

Based on the knowledge graph, the care coordinator has discovered:

1. PRIORITY CASE: James Mitchell (Age 67)
   Status: HIGH RISK - Multiple chronic conditions, new AFib alert
   Conditions: Atrial Fibrillation, Hypertension, Type 2 Diabetes
   Medications: Apixaban, Metoprolol, Metformin
   Recent Alert: Irregular heartbeat on smartwatch (Apple Watch)

   Action Items:
   ✓ Verify anticoagulation compliance (prevent stroke)
   ✓ Urgent cardiology referral (AFib management)
   ✓ Sleep optimization (currently 6.2 hrs vs target 7-9)
   ✓ BP monitoring (morning readings elevated)

2. STABLE CASE: Maria Gonzalez (Age 54)
   Status: STABLE - Well-controlled hypertension, stable kidney function
   Conditions: Hypertension, Stage 2 CKD
   Medications: Lisinopril, Amlodipine
   Status: All parameters within target range

KEY INSIGHTS FROM GRAPH ANALYSIS:
- Wiki-links create rapid navigation between related records
- Wearable alerts automatically surface urgent findings
- Risk assessments flag high-priority patients for triage
- Medication tracking prevents duplications and interactions
- Care plans centralize action items and follow-ups

CARE COORDINATOR EFFICIENCY GAIN:
Instead of manually checking multiple systems, the care coordinator
used Smriti to discover all connections in seconds, enabling:
  • Faster decision-making
  • Comprehensive patient view
  • Alert detection
  • Risk prioritization
  • Medication verification

EOF

echo ""
print_header "Demo Complete!"

echo -e "${GREEN}The healthcare care coordinator has successfully:${NC}"
echo "  ✓ Identified high-risk patients"
echo "  ✓ Verified medication appropriateness"
echo "  ✓ Reviewed real-time wearable alerts"
echo "  ✓ Discovered all connected records via the knowledge graph"
echo "  ✓ Prioritized follow-up actions"
echo ""

echo -e "${YELLOW}Next Steps:${NC}"
echo "  1. Explore individual notes: smriti read <ID>"
echo "  2. Search specific topics: smriti search \"<query>\""
echo "  3. Visualize connections: smriti graph --note <ID> --depth <N>"
echo "  4. Start the REST API: smriti serve (then curl http://localhost:3000)"
echo "  5. Try the MCP server: smriti mcp (for AI assistant integration)"
echo ""
