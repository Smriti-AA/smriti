#!/bin/bash

# Smriti Lab Trial Optimizer - Demo Script
# Demonstrates AI-driven optimization of pharmaceutical trial design

echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║     SMRITI LAB TRIAL OPTIMIZER - OPTIMIZATION DEMO             ║"
echo "║  Using Knowledge Graph Traversal to Optimize Drug Trials      ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Check if smriti is installed
if ! command -v smriti &> /dev/null; then
    echo "ERROR: smriti not found. Install with: cargo install smriti"
    exit 1
fi

# Check if knowledge base was initialized
if [ ! -d ".smriti" ]; then
    echo "ERROR: Knowledge base not initialized. Run setup.sh first."
    exit 1
fi

# ============================================================================
# SCENARIO 1: Search for Compound Efficacy Data
# ============================================================================

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "SCENARIO 1: Lab Assistant searches for compound efficacy profiles"
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "Query: 'Find all compounds and their dosage information'"
echo ""

smriti search "compound" | head -20

echo ""
echo "INSIGHT:"
echo "  The lab found 2 compounds with detailed dosage curves."
echo "  Next step: Analyze efficacy vs. safety at each dose level."
echo ""

# ============================================================================
# SCENARIO 2: Graph Traversal - CB-209 Analysis
# ============================================================================

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "SCENARIO 2: Graph traversal - analyzing CB-209 compound"
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "Query: 'Map CB-209 relationships to understand compound profile'"
echo "       (Following edges: compound → dosage curves → biomarkers)"
echo ""

smriti graph --note Compound-CB-209 --depth 3

echo ""
echo "DISCOVERY CHAIN:"
echo "  CB-209"
echo "    ├→ Dosage-Curve-CB209-100mg (safe, minimal side effects)"
echo "    ├→ Dosage-Curve-CB209-300mg (efficacy gain, but cardiac risk)"
echo "    ├→ Biomarker-TroponinT (cardiac injury marker)"
echo "    └→ AE-Log-CB209-GI-Upset (dose-related GI effects)"
echo ""
echo "OPTIMIZATION INSIGHT:"
echo "  CB-209 shows dose-dependent efficacy BUT dose-limiting"
echo "  cardiac toxicity at 300mg. Recommendation: Optimize at 200mg"
echo "  with intensive Troponin-T monitoring."
echo ""

# ============================================================================
# SCENARIO 3: Biomarker-Centric Analysis
# ============================================================================

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "SCENARIO 3: Biomarker-centric search for cardiac safety"
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "Query: 'Which compounds affect cardiac biomarkers?'"
echo ""

smriti search "troponin\|bnp" | head -15

echo ""
echo "CRITICAL FINDING:"
echo "  Multiple compounds affect cardiac biomarkers:"
echo "    - CB-209 → Troponin-T elevation at high doses"
echo "    - CB-209 → BNP slight elevation at 300mg"
echo "    - CB-415 → BNP stable or improved (protective effect?)"
echo ""
echo "OPTIMIZATION STRATEGY:"
echo "  CB-415 may be safer for cardiac patients due to TNF-alpha"
echo "  pathway modulation. Consider preferential enrollment of"
echo "  cardiac-at-risk subjects in CB-415 cohorts."
echo ""

# ============================================================================
# SCENARIO 4: Cross-Compound Analysis - Biomarker Pathway Discovery
# ============================================================================

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "SCENARIO 4: Cross-compound analysis - pathway convergence"
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "Query: 'Which compounds share biomarker pathways?'"
echo ""

echo "Searching for CRP (inflammation marker) connections..."
smriti search "crp" | head -10

echo ""
echo "CRITICAL DISCOVERY:"
echo "  Both compounds affect inflammatory pathways but differently:"
echo ""
echo "  CB-209 (PKC inhibitor):"
echo "    └→ Anti-inflammatory through PKC-mediated immune suppression"
echo "    └→ Potential cardiac toxicity (dose-dependent)"
echo ""
echo "  CB-415 (TNF-alpha modulator):"
echo "    └→ Direct CRP reduction via TNFR2 modulation"
echo "    └→ Protective cardiac effects (BNP stable/reduced)"
echo ""

# ============================================================================
# SCENARIO 5: Adverse Event Pattern Recognition
# ============================================================================

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "SCENARIO 5: Adverse event pattern analysis"
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "Query: 'Retrieve all adverse events and their dose relationships'"
echo ""

smriti search "adverse\|ae-log" | head -15

echo ""
echo "AE PATTERN ANALYSIS:"
echo "  CB-209:"
echo "    └─ GI upset (20% at 100mg, 60% at 300mg) - DOSE DEPENDENT"
echo "    └─ Cardiac concern (Troponin elevation at 300mg) - DOSE LIMITING"
echo ""
echo "  CB-415:"
echo "    └─ Headache (15% at 150mg, 60% at 450mg) - MANAGEABLE"
echo "    └─ Hepatic (ALT elevation at 450mg) - DOSE LIMITING"
echo ""
echo "OPTIMIZATION DECISION:"
echo "  CB-209: Optimal dose = 200mg (between 100 and 300)"
echo "  CB-415: Optimal dose = 300mg (between 150 and 450)"
echo ""

# ============================================================================
# SCENARIO 6: Simulating Lab Assistant Optimization Workflow
# ============================================================================

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "SCENARIO 6: AI Lab Assistant - Optimization Workflow"
echo "════════════════════════════════════════════════════════════════"
echo ""

echo "STEP 1: Retrieve all dosage curves for comparison"
echo "────────────────────────────────────────────────"
smriti search "dosage.*curve" | head -20
echo ""

echo "STEP 2: Evaluate efficacy vs. safety trade-offs"
echo "────────────────────────────────────────────────"
echo ""
echo "CB-209 Analysis:"
echo "  100mg:   Safe + minimal efficacy"
echo "  200mg:   (inferred optimal) Efficacy gain + acceptable risk"
echo "  300mg:   Maximum efficacy BUT cardiac toxicity risk"
echo ""
echo "CB-415 Analysis:"
echo "  150mg:   Safe + moderate efficacy + favorable cardiac profile"
echo "  300mg:   (inferred optimal) Strong CRP reduction + acceptable hepatic risk"
echo "  450mg:   Maximum efficacy BUT hepatic monitoring burden"
echo ""

echo "STEP 3: Protocol Optimization Recommendations"
echo "──────────────────────────────────────────────"
echo ""
echo "RECOMMENDED PHASE II EXPANSION DESIGN:"
echo ""
echo "  Cohort A: CB-415 150mg (cardiac-safe, anti-inflammatory efficacy)"
echo "    • Low cardiac risk → good for cardiac comorbidity subjects"
echo "    • CRP reduction ~20% → validates inflammation target"
echo "    • Safety: headache manageable with OTC medication"
echo "    • Biomarkers: Troponin-T stable, BNP maintained"
echo ""
echo "  Cohort B: CB-415 300mg (optimized efficacy)"
echo "    • CRP reduction ~35% → strong efficacy signal"
echo "    • Hepatic monitoring required (ALT trending)"
echo "    • Consider enrollment of subjects with baseline LFT in upper normal"
echo ""
echo "  Cohort C: CB-209 200mg (with intensive cardiac monitoring)"
echo "    • Intermediate cardiac risk (estimated from 100/300 data)"
echo "    • Efficacy expected between 100/300mg"
echo "    • Restrict to subjects with normal baseline Troponin-T"
echo "    • Daily Troponin-T monitoring recommended"
echo ""

echo "STEP 4: Cross-compound Synergy Consideration"
echo "───────────────────────────────────────────"
echo ""
echo "DISCOVERY: CB-209 and CB-415 target different pathways"
echo "  • CB-209: PKC inhibition → anti-inflammatory"
echo "  • CB-415: TNF-alpha modulation → anti-inflammatory + cardiac protective"
echo ""
echo "FUTURE OPTIMIZATION:"
echo "  Consider Phase IIb combination study (CB-209 + CB-415)"
echo "  with potential for synergistic anti-inflammatory effect"
echo "  while maintaining cardiac safety (CB-415 offset CB-209 risk)"
echo ""

# ============================================================================
# SCENARIO 7: Knowledge Graph Visualization Summary
# ============================================================================

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "SCENARIO 7: Knowledge Graph Visualization"
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "The AI lab assistant has built the following decision graph:"
echo ""
echo "EFFICACY PATHWAY:"
echo "  CB-209 (PKC) ───→ ↓PKC activity ───→ ↓Inflammation ───→ Therapeutic benefit"
echo "         ╰→ [[Dosage-Curve]] ───→ ↑Troponin-T ───→ Cardiac risk"
echo ""
echo "SAFETY PATHWAY:"
echo "  CB-415 (TNFR2) ──→ ↓TNF-alpha ───→ ↓CRP + ↓BNP ───→ Safe + efficacious"
echo "          ╰→ [[Dosage-Curve]] ───→ ↑ALT ───→ Hepatic monitoring needed"
echo ""
echo "BIOMARKER CONVERGENCE:"
echo "  Both compounds reduce CRP, but via different mechanisms"
echo "  CB-415 additionally improves cardiac biomarkers (BNP)"
echo "  This suggests CB-415 primary compound + CB-209 as secondary"
echo ""

# ============================================================================
# FINAL SUMMARY
# ============================================================================

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "OPTIMIZATION SUMMARY"
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "KEY INSIGHTS FROM KNOWLEDGE GRAPH TRAVERSAL:"
echo ""
echo "✓ Identified optimal doses through safety-efficacy analysis"
echo "✓ Discovered cross-compound biomarker pathway convergence"
echo "✓ Revealed CB-415 cardiac protective advantage"
echo "✓ Detected dose-dependent adverse event patterns"
echo "✓ Enabled data-driven protocol optimization"
echo ""
echo "RECOMMENDATIONS FOR NEXT PHASE:"
echo ""
echo "1. Prioritize CB-415 for Phase II expansion (safer cardiac profile)"
echo "2. Continue CB-209 with enhanced cardiac monitoring (Troponin-T daily)"
echo "3. Optimize CB-415 dose to 300mg for efficacy target"
echo "4. Consider combination therapy for Phase IIb planning"
echo "5. Implement adaptive trial design based on biomarker response"
echo ""
echo "SMRITI VALUE PROPOSITION:"
echo "  Without graph knowledge base: Manual Excel/PDF searches = hours"
echo "  With Smriti: Automated discovery + relationship mapping = minutes"
echo ""
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "Try additional manual queries:"
echo "  smriti list                          # Show all notes"
echo "  smriti search 'phase'                # Find protocols"
echo "  smriti graph --note Biomarker-BNP --depth 3  # BNP analysis"
echo ""
