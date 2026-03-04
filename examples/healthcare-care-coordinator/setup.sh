#!/bin/bash

# Healthcare Care Coordinator Example - Setup Script
# This script creates 8-10 interconnected notes demonstrating a digital care coordinator
# monitoring patient records, wearable data, medications, and risk assessments.

# Color codes for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Setting up Healthcare Care Coordinator Knowledge Base...${NC}\n"

# Note 1: Patient Profile - James Mitchell
echo -e "${GREEN}[1/10]${NC} Creating patient profile: James Mitchell"
smriti create "Patient: James Mitchell" \
  --content "## Demographics
- Age: 67
- Gender: Male
- Primary Care Physician: Dr. Sarah Chen
- Medical Record #: MR-2024-089401

## Active Conditions
- [[Condition: Atrial Fibrillation]] (diagnosed 2022)
- [[Condition: Hypertension]] (diagnosed 2018)
- [[Condition: Type 2 Diabetes]] (diagnosed 2015)

## Alerts
⚠️ Recent irregular heartbeat detected by [[Wearable: Apple Watch Series 8]]
⚠️ Blood pressure elevated at morning reading

## Insurance
- Blue Cross Blue Shield
- Copay: \$35 office visits

#cardiovascular #elderly #priority-monitoring"

# Note 2: Patient Profile - Maria Gonzalez
echo -e "${GREEN}[2/10]${NC} Creating patient profile: Maria Gonzalez"
smriti create "Patient: Maria Gonzalez" \
  --content "## Demographics
- Age: 54
- Gender: Female
- Primary Care Physician: Dr. James Wilson
- Medical Record #: MR-2024-089402

## Active Conditions
- [[Condition: Hypertension]] (diagnosed 2010)
- [[Condition: Chronic Kidney Disease]] Stage 2 (diagnosed 2022)

## Current Treatment
- [[Medication: Lisinopril 10mg]]
- [[Medication: Amlodipine 5mg]]

## Alerts
✓ Stable BP control
✓ eGFR stable at 62

#hypertension #kidney-disease #stable"

# Note 3: Condition - Atrial Fibrillation
echo -e "${GREEN}[3/10]${NC} Creating condition note: Atrial Fibrillation"
smriti create "Condition: Atrial Fibrillation" \
  --content "## Clinical Definition
Irregular heart rhythm (arrhythmia) affecting approximately 6 million Americans.

## Patients
- [[Patient: James Mitchell]]

## Recommended Medications
- [[Medication: Apixaban 5mg]] (anticoagulant)
- [[Medication: Metoprolol 50mg]] (rate control)
- [[Medication: Amiodarone 200mg]] (rhythm control - alternative)

## Monitoring Parameters
- Target heart rate: 60-100 bpm at rest (lenient control)
- Monitor for stroke risk using CHA₂DS₂-VASc score
- Regular ECG assessment

## Risk Assessment
- [[Cardiac Risk Assessment: James Mitchell AFib]]

## Related Biomarkers
- Linked to elevated troponin
- Associated with elevated BNP levels

#cardiovascular #arrhythmia #critical"

# Note 4: Condition - Hypertension
echo -e "${GREEN}[4/10]${NC} Creating condition note: Hypertension"
smriti create "Condition: Hypertension" \
  --content "## Clinical Definition
High blood pressure (systolic ≥140 mmHg or diastolic ≥90 mmHg). Major risk factor for cardiovascular disease.

## Patients
- [[Patient: James Mitchell]]
- [[Patient: Maria Gonzalez]]

## Recommended Medications
- [[Medication: Lisinopril 10mg]] (ACE inhibitor)
- [[Medication: Amlodipine 5mg]] (calcium channel blocker)
- [[Medication: Metoprolol 50mg]] (beta blocker)

## Monitoring Parameters
- Target BP: <130/80 mmHg
- Check daily with home monitoring device
- Recheck in clinic every 3-6 months

## Related Wearables
- Smart blood pressure cuff monitoring
- Smartwatch systolic/diastolic trending

#cardiovascular #chronic-disease"

# Note 5: Condition - Type 2 Diabetes
echo -e "${GREEN}[5/10]${NC} Creating condition note: Type 2 Diabetes"
smriti create "Condition: Type 2 Diabetes" \
  --content "## Clinical Definition
Metabolic disorder characterized by elevated fasting glucose >126 mg/dL.

## Patients
- [[Patient: James Mitchell]]

## Medications
- [[Medication: Metformin 1000mg]]

## Monitoring Parameters
- Fasting glucose target: 100-150 mg/dL
- HbA1c target: <7%
- Annual eye exam, foot exam, kidney function

## Complications
- Increases cardiovascular disease risk (patient has [[Condition: Atrial Fibrillation]])
- Linked to [[Condition: Chronic Kidney Disease]] progression

#metabolic #chronic-disease #elderly"

# Note 6: Medication - Apixaban
echo -e "${GREEN}[6/10]${NC} Creating medication note: Apixaban 5mg"
smriti create "Medication: Apixaban 5mg" \
  --content "## Drug Information
- Brand: Eliquat
- Strength: 5 mg tablet
- Classification: Direct oral anticoagulant (DOAC)

## Indications
- [[Condition: Atrial Fibrillation]] (stroke prevention)
- Deep vein thrombosis prevention
- Pulmonary embolism treatment

## Patient Taking This
- [[Patient: James Mitchell]] - Dose 2x daily with food

## Monitoring
- No INR monitoring required (unlike warfarin)
- Monitor for bleeding signs
- Renal function assessment (annual)

## Common Side Effects
- Bleeding (major concern)
- Bruising
- Nosebleeds

## Drug Interactions
- Avoid NSAIDs
- Caution with aspirin

#anticoagulant #afib #critical-medication"

# Note 7: Medication - Metoprolol
echo -e "${GREEN}[7/10]${NC} Creating medication note: Metoprolol 50mg"
smriti create "Medication: Metoprolol 50mg" \
  --content "## Drug Information
- Brand: Lopressor, Toprol-XL
- Strength: 50 mg (immediate release)
- Classification: Beta blocker

## Indications
- [[Condition: Hypertension]] (blood pressure control)
- [[Condition: Atrial Fibrillation]] (rate control - target 60-100 bpm)
- Post-MI protection

## Patients
- [[Patient: James Mitchell]] - Once daily

## Monitoring
- Resting heart rate should be 60-80 bpm
- BP monitoring
- Watch for fatigue, dizziness, bradycardia

## Side Effects
- Fatigue
- Dizziness on position changes
- Sexual dysfunction
- Reduced exercise tolerance

#beta-blocker #hypertension #afib"

# Note 8: Wearable Data - James Mitchell (Current)
echo -e "${GREEN}[8/10]${NC} Creating wearable data note: James Mitchell Apple Watch"
smriti create "Wearable: Apple Watch Series 8 - James Mitchell" \
  --content "## Device Info
- Device: Apple Watch Series 8
- Patient: [[Patient: James Mitchell]]
- Data sync: Real-time via HealthKit integration
- Last update: Today 08:30 AM

## Current Vital Signs (Last 24 Hours)
- **Heart Rate**: 89 bpm (resting) - ALERT: Irregular rhythm detected ⚠️
- **SpO2**: 96% (normal)
- **Sleep**: 6.2 hours (target: 7-9 hours) - SUBOPTIMAL
- **Steps**: 4,821 (target: 10,000)
- **Activity Rings**: Mostly closed (68%)

## Abnormal Readings
- Irregular heartbeat flagged 3 times in past week
- Related to [[Condition: Atrial Fibrillation]]
- Recommend cardiology review

## Connected Medications
- [[Medication: Metoprolol 50mg]] should improve rate control
- [[Medication: Apixaban 5mg]] managing stroke risk

#wearable #cardiac-monitoring #real-time-data"

# Note 9: Cardiac Risk Assessment
echo -e "${GREEN}[9/10]${NC} Creating risk assessment: James Mitchell AFib Risk"
smriti create "Cardiac Risk Assessment: James Mitchell AFib" \
  --content "## Patient
[[Patient: James Mitchell]]

## CHA₂DS₂-VASc Score Components
| Factor | Score |
|--------|-------|
| Congestive Heart Failure | 0 |
| Hypertension | 1 (on [[Medication: Metoprolol 50mg]]) |
| Age ≥75 | 0 (age 67) |
| Diabetes | 1 ([[Condition: Type 2 Diabetes]]) |
| Stroke/TIA/Thromboembolism | 0 |
| Vascular disease | 0 |
| Age 65-74 | 1 |
| Sex (Female) | 0 |
| **TOTAL SCORE** | **3 (HIGH RISK)** |

## Recommendations
- **Anticoagulation**: YES - currently on [[Medication: Apixaban 5mg]]
- **Stroke Risk**: 4.0% per year
- **Bleeding Risk**: Monitor for signs, especially on anticoagulation

## Related Vitals
- HR irregular as detected on [[Wearable: Apple Watch Series 8 - James Mitchell]]
- BP elevated at home monitoring

## Care Plan
- Ensure medication compliance ([[Medication: Apixaban 5mg]], [[Medication: Metoprolol 50mg]])
- Encourage sleep optimization (currently 6.2 hours)
- Reduce sodium intake

#cardiac-risk #afib #elderly #high-risk"

# Note 10: Care Plan
echo -e "${GREEN}[10/10]${NC} Creating care plan: James Mitchell Management"
smriti create "Care Plan: James Mitchell Quarterly Review" \
  --content "## Patient
[[Patient: James Mitchell]]

## Review Date
Q1 2024

## Active Conditions Managed
1. [[Condition: Atrial Fibrillation]]
   - Medication: [[Medication: Apixaban 5mg]] (anticoagulation)
   - Rate control: [[Medication: Metoprolol 50mg]]
   - Monitoring: Wearable device tracking

2. [[Condition: Hypertension]]
   - Medication: [[Medication: Metoprolol 50mg]]
   - Target BP: <130/80 mmHg
   - Home monitoring: Daily

3. [[Condition: Type 2 Diabetes]]
   - Medication: [[Medication: Metformin 1000mg]]
   - Next A1c check: 3 months

## Vital Signs Summary
- Current HR (via [[Wearable: Apple Watch Series 8 - James Mitchell]]): 89 bpm with irregular rhythm
- Current BP: Morning reading elevated
- Sleep quality: Suboptimal at 6.2 hours

## Risk Assessment
- [[Cardiac Risk Assessment: James Mitchell AFib]]: HIGH RISK (score 3)

## Action Items
1. ☐ Verify Apixaban adherence and assess for bleeding
2. ☐ Refer to cardiology for AFib management review
3. ☐ Recommend sleep hygiene intervention (target 7-9 hours)
4. ☐ Schedule follow-up in 6 weeks

## Next Appointment
- Primary Care: TBD
- Cardiology: Urgent

#care-plan #management #quarterly-review #follow-up"

echo -e "\n${BLUE}Knowledge base setup complete!${NC}"
echo -e "${BLUE}Total notes created: 10${NC}\n"

echo "Next steps:"
echo "  1. Run: smriti list               # View all created notes"
echo "  2. Run: smriti graph --note 1    # View the knowledge graph"
echo "  3. Run: ./demo.sh                # See the care coordinator in action"
