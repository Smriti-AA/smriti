#!/bin/bash

# Multi-Agent Collaboration Setup for Smriti
# Simulates three AI agents independently creating notes in a shared knowledge graph
# Phase 1: Care Coordinator (Patient management)
# Phase 2: Lab Assistant (Research & biomarkers)
# Phase 3: Clinical Decision Agent prepares to query the graph

set -e

echo "=========================================="
echo "Smriti Multi-Agent Collaboration Setup"
echo "=========================================="
echo ""

# Phase 1: Care Coordinator Agent
echo "PHASE 1: Care Coordinator Agent"
echo "Domain: Patient care management and clinical history"
echo "Task: Document patient conditions and symptoms"
echo "-------------------------------------------"
echo ""

echo "Care Coordinator: Creating patient profile - John Doe"
smriti create "Patient: John Doe - Medical History" \
  --content "John Doe, 62-year-old male. Presented with chest pain and dyspnea on 2026-02-15. EKG shows ST elevation. Diagnosed with [[Acute Myocardial Infarction]]. Critical biomarkers: [[Troponin-T]], [[CK-MB]]. Patient requires ICU monitoring and aggressive intervention. Related conditions: [[Hypertension]], [[Diabetes Type 2]]. Care coordination focus: Post-MI recovery and rehabilitation."

echo "Care Coordinator: Creating patient profile - Sarah Chen"
smriti create "Patient: Sarah Chen - Cardiology Profile" \
  --content "Sarah Chen, 58-year-old female. Chronic heart failure (NYHA Class III). Ejection fraction 35%. Key monitoring markers: [[Troponin-T]], [[BNP]]. Recent exacerbation treated with diuretics and ACE inhibitors. Needs continuous monitoring of [[Cardiac Biomarkers]]. Related comorbidities: [[Atrial Fibrillation]], [[Chronic Kidney Disease]]. Follow-up scheduled every 2 weeks."

echo "Care Coordinator: Creating patient profile - Robert Martinez"
smriti create "Patient: Robert Martinez - MI History" \
  --content "Robert Martinez, 71-year-old male. Suffered two MIs in the past 5 years (2021, 2024). Highly susceptible to cardiac events. Primary biomarkers monitored: [[Troponin-T]], [[Myoglobin]]. Recent troponin spike on 2026-03-01 required urgent re-evaluation. Baseline cardiac risk assessment shows elevated [[Cardiac Biomarkers]]. Candidate for advanced monitoring programs."

echo "Care Coordinator: Creating symptom note"
smriti create "Clinical Observation: Cardiac Biomarker Elevation Patterns" \
  --content "Multiple patients in our clinic show concerning patterns of [[Troponin-T]] elevation independent of acute MI diagnosis. Observation suggests [[Cardiac Biomarkers]] may be more sensitive indicators of subclinical cardiac stress than previously thought. This pattern correlates with [[Chronic Kidney Disease]] progression. Recommend investigation into biomarker elevation as early warning system."

echo ""
echo "PHASE 1 COMPLETE"
echo "Care Coordinator has added 4 patient and clinical notes"
echo ""
sleep 1

# Phase 2: Lab Assistant Agent
echo "=========================================="
echo "PHASE 2: Lab Assistant Agent"
echo "Domain: Biomedical research and clinical trials"
echo "Task: Document trials and biomarker research"
echo "-------------------------------------------"
echo ""

echo "Lab Assistant: Creating trial information - CARDIAC-2024"
smriti create "Clinical Trial: CARDIAC-2024 Post-MI Recovery Protocol" \
  --content "CARDIAC-2024 is a Phase 3 randomized controlled trial investigating novel [[Troponin-T]] monitoring protocols in post-MI patients. Primary endpoint: 30-day troponin normalization rate. Secondary endpoints: Infarct size reduction and LV remodeling prevention. Biomarker focus: [[Troponin-T]], [[CK-MB]], [[Myoglobin]]. Inclusion criteria: Acute MI confirmed by [[Cardiac Biomarkers]]. Target enrollment: 500 patients. Status: Currently recruiting."

echo "Lab Assistant: Creating trial information - BNP-HEART Study"
smriti create "Clinical Trial: BNP-HEART Study on Heart Failure Progression" \
  --content "BNP-HEART is an observational study tracking [[BNP]] elevation patterns in [[Chronic Heart Failure]] patients. Monitors progression markers: [[Troponin-T]], [[BNP]], [[NT-proBNP]]. Focuses on [[Cardiac Biomarkers]] as predictors of decompensation. 200 patient cohort enrolled. Collects data on [[Atrial Fibrillation]] comorbidity impact. Expected completion: 2027."

echo "Lab Assistant: Creating research note on troponin assays"
smriti create "Research Notes: Advanced Troponin-T Assay Development" \
  --content "Development of high-sensitivity [[Troponin-T]] assay improved detection window from 24 to 72 hours. New methodology allows earlier detection of [[Cardiac Biomarkers]] in [[Acute Myocardial Infarction]] cases. Preliminary data shows 98% specificity for [[Troponin-T]] elevation in post-MI patients. Assay also demonstrates utility in [[Chronic Kidney Disease]] populations with cardiac involvement. Patent pending."

echo "Lab Assistant: Creating biomarker correlation data"
smriti create "Lab Analysis: Biomarker Correlation Matrix" \
  --content "Analyzed 5000 patient samples for [[Cardiac Biomarkers]] correlation. Strong correlation observed between [[Troponin-T]] and CK-MB in acute settings (r=0.89). [[Troponin-T]] elevation in [[Chronic Kidney Disease]] patients shows unique pattern compared to [[Acute Myocardial Infarction]] (different kinetics, plateau timing). Suggests [[Troponin-T]] may be sensitive indicator of [[Chronic Kidney Disease]]-related cardiac stress. Recommend separate reference ranges for CKD populations."

echo ""
echo "PHASE 2 COMPLETE"
echo "Lab Assistant has added 4 trial and research notes"
echo ""
sleep 1

# Phase 3: Preparation for Clinical Decision Agent
echo "=========================================="
echo "PHASE 3: Clinical Decision Agent Preparation"
echo "Domain: Data synthesis and clinical decision support"
echo "Task: Prepare to query the graph and discover connections"
echo "-------------------------------------------"
echo ""

echo "Clinical Decision Agent: Initializing knowledge graph analysis..."
echo "Querying available notes in the system..."
echo ""

echo "Total notes created:"
smriti list | wc -l
echo ""

echo "Core biomarkers in the knowledge graph:"
smriti search "Cardiac Biomarkers" || true
echo ""

echo "Setup complete! The knowledge graph now contains:"
echo "  - 4 patient notes with condition and biomarker references (Care Coordinator)"
echo "  - 4 trial and research notes with biomarker requirements (Lab Assistant)"
echo "  - Automatic cross-links through shared wiki-link references"
echo ""
echo "Ready for cross-agent discovery in demo.sh!"
echo ""
