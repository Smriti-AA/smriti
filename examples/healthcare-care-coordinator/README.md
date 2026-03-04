# Healthcare Care Coordinator Example

## Overview

This example demonstrates how Smriti can function as a **digital care coordinator** that monitors and connects patient health data across multiple domains. The system uses wiki-links to create a semantic graph of patient records, vitals, medications, and risk assessments—enabling rapid discovery of critical connections in clinical workflows.

## Use Case

A care coordinator needs to:
- Monitor patients with chronic conditions (e.g., atrial fibrillation, hypertension)
- Track wearable device data (heart rate, sleep, blood oxygen)
- Link patient conditions to current medications
- Identify cardiovascular risk based on biomarkers
- Discover alerts when vitals exceed safe thresholds
- Navigate the relationships between patient, conditions, medications, and monitoring data

## What This Example Demonstrates

- **Patient Profiles**: Notes containing demographics, medical history, and alerts
- **Wearable Integration**: Real-time vital signs from connected devices (heart rate, SpO2, sleep quality)
- **Medication Records**: Prescriptions linked to the conditions they treat
- **Risk Assessment**: Cardiac risk scoring based on biomarkers and comorbidities
- **Care Plans**: Personalized treatment strategies linked to conditions and medications
- **Graph Navigation**: Discovering all connected records from a single patient entry

## Prerequisites

- Smriti installed: `cargo install smriti`
- Bash shell
- Read/write access to a directory for the Smriti knowledge base

## How to Run

1. **Initialize the knowledge base:**
   ```bash
   ./setup.sh
   ```
   This creates 8-10 interconnected notes representing patient records, vitals, medications, and risk assessments.

2. **Run the demonstration:**
   ```bash
   ./demo.sh
   ```
   This shows how the care coordinator discovers connections by searching and traversing the graph.

3. **Explore manually:**
   ```bash
   smriti list                    # View all notes
   smriti read <ID>              # Read a specific note
   smriti graph --note <ID> --depth 2  # See connections from a patient
   smriti search "cardiac"       # Find cardiac-related records
   smriti search "SpO2 alert"    # Find wearable alerts
   ```

## Expected Output

After running `setup.sh`:
- 10 notes created representing a care coordination workflow
- Notes linked via wiki-links like `[[Patient: James Mitchell]]`, `[[Condition: Atrial Fibrillation]]`, etc.

After running `demo.sh`:
- Search results showing cardiac-related notes
- Graph traversals revealing the patient → condition → medication → wearable data pathway
- Alerts and risk assessments flagged for immediate attention

## What to Look For in Graph Connections

1. **Patient → Conditions**: Each patient links to their diagnoses
2. **Conditions → Medications**: Treatments link back to the problems they address
3. **Medications → Vitals**: Medications affect observable biomarkers
4. **Vitals → Alerts**: Out-of-range readings trigger care coordinator flags
5. **Risk Assessment → Biomarkers**: Cardiac risk is calculated from linked vital signs

## Clinical Workflow Example

```
Patient: James Mitchell (Age 67)
  ├─→ Condition: Atrial Fibrillation
  ├─→ Medication: Apixaban 5mg
  ├─→ Wearable Data: Heart Rate 89 bpm (irregular)
  ├─→ Cardiac Risk: HIGH (based on linked biomarkers)
  └─→ Care Plan: Ensure anticoagulation compliance, monitor for bleeding
```

## Notes on Realism

- Patient names and ages are fictional
- Vital signs are realistic ranges for the conditions
- Medications and dosages follow standard clinical practice
- Risk assessments use simplified but realistic scoring
- Alerts follow established thresholds (e.g., SpO2 <94% is concerning)

## Next Steps

After exploring this example, consider:
- Adding appointment history notes linked to conditions
- Integrating lab results with biomarkers
- Creating escalation chains for critical alerts
- Linking to educational resources for patient conditions
- Building care plan compliance tracking
