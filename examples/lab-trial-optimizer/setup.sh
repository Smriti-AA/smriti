#!/bin/bash

# Smriti Lab Trial Optimizer - Setup Script
# Creates an interconnected knowledge base for pharmaceutical research

echo "========================================"
echo "Smriti Lab Trial Optimizer - Setup"
echo "========================================"
echo ""

# Check if smriti is installed
if ! command -v smriti &> /dev/null; then
    echo "ERROR: smriti not found. Install with: cargo install smriti"
    exit 1
fi

# Initialize Smriti if not already done
if [ ! -d ".smriti" ]; then
    echo "Initializing Smriti database..."
    smriti init
fi

echo "Creating knowledge base for pharmaceutical trial optimization..."
echo ""

# ============================================================================
# COMPOUND NOTES
# ============================================================================

echo "[1/10] Creating compound: CB-209"
smriti create "Compound-CB-209" --content "
# CB-209: Novel PKC Inhibitor

**Type**: Kinase inhibitor targeting Protein Kinase C (PKC)
**Molecular Weight**: 487.3 g/mol
**Log P**: 2.8 (good membrane permeability)
**Half-life**: 12-14 hours (suitable for BID dosing)
**Metabolism**: CYP3A4 primary, CYP2C9 secondary

## Pharmacological Properties
- **Mechanism**: Selective PKC inhibitor with anti-inflammatory effects
- **Primary Target**: PKC-alpha, PKC-beta isoforms
- **Secondary Targets**: Some activity against Akt kinase
- **Therapeutic Class**: Cardioprotective, anti-inflammatory

## Preclinical Data
- **IC50 (PKC)**: 45 nM
- **Clearance**: 0.8 L/min (moderate)
- **Protein Binding**: 92%

## Related Notes
- [[Dosage-Curve-CB209-100mg]]
- [[Dosage-Curve-CB209-300mg]]
- [[Biomarker-TroponinT]]
- [[AE-Log-CB209-GI-Upset]]
- [[Protocol-Phase-II-Cardiotoxicity]]

#compound #pkc-inhibitor #cardioprotective
"

echo "[2/10] Creating compound: CB-415"
smriti create "Compound-CB-415" --content "
# CB-415: TNF-alpha Pathway Modulator

**Type**: Immunomodulatory compound
**Molecular Weight**: 521.2 g/mol
**Log P**: 3.1 (high membrane penetration)
**Half-life**: 18-20 hours (once-daily dosing)
**Metabolism**: CYP2D6 primary, hepatic glucuronidation

## Pharmacological Properties
- **Mechanism**: Selective TNF-alpha receptor modulation
- **Primary Target**: TNFR2 (TNF receptor 2)
- **Secondary Targets**: Some IL-6 pathway activity
- **Therapeutic Class**: Immunomodulatory, anti-inflammatory

## Preclinical Data
- **IC50 (TNFR2)**: 23 nM
- **Clearance**: 1.2 L/min (moderate-high)
- **Protein Binding**: 88%

## Related Notes
- [[Dosage-Curve-CB415-150mg]]
- [[Dosage-Curve-CB415-450mg]]
- [[Biomarker-BNP]]
- [[Biomarker-CRP]]
- [[AE-Log-CB415-Headache]]

#compound #immunomodulator #anti-inflammatory
"

# ============================================================================
# BIOMARKER NOTES
# ============================================================================

echo "[3/10] Creating biomarker: Troponin-T"
smriti create "Biomarker-TroponinT" --content "
# Troponin-T (cTnT): Cardiac Injury Marker

**Clinical Significance**: Highly sensitive and specific indicator of myocardial injury
**Reference Range**: < 0.03 ng/mL (normal)
**Critical Threshold**: > 0.1 ng/mL suggests acute myocardial damage

## Role in Drug Safety
Troponin-T elevation indicates cardiotoxicity—a critical safety parameter for kinase inhibitors.
Multiple kinase inhibitors (including PKC inhibitors) can cause off-target cardiac effects.

## Monitoring Protocol
- **Frequency**: Baseline, Day 7, Day 14, Day 28
- **Method**: High-sensitivity troponin assay
- **Clinical Action**: If > 0.5 ng/mL, consider dose reduction or discontinuation

## Compounds Affecting This Biomarker
- [[Compound-CB-209]] (PKC inhibitor - moderate cardiac potential)

## Literature References
- [[Literature-Troponin-Cardiotoxicity]]

## Related Dosage Curves
- [[Dosage-Curve-CB209-100mg]]
- [[Dosage-Curve-CB209-300mg]]

#biomarker #cardiac-safety #troponin #cardiotoxicity
"

echo "[4/10] Creating biomarker: BNP"
smriti create "Biomarker-BNP" --content "
# BNP: B-Type Natriuretic Peptide

**Clinical Significance**: Marker of heart failure and cardiac stress
**Reference Range**: < 100 pg/mL (normal)
**Elevated**: > 400 pg/mL suggests cardiac decompensation

## Role in Drug Safety
BNP elevation indicates cardiac hemodynamic stress. Important for monitoring compounds with potential cardiac effects.

## Monitoring Protocol
- **Frequency**: Baseline, weekly during Phase I/II
- **Method**: Immunoassay
- **Clinical Action**: Progressive elevation warrants cardiac imaging

## Compounds Affecting This Biomarker
- [[Compound-CB-209]] (secondary cardiac effects possible)
- [[Compound-CB-415]] (TNF modulation may reduce cardiac stress)

## Related Dosage Curves
- [[Dosage-Curve-CB209-300mg]]
- [[Dosage-Curve-CB415-150mg]]

#biomarker #cardiac-function #bnp #heart-failure
"

echo "[5/10] Creating biomarker: CRP"
smriti create "Biomarker-CRP" --content "
# CRP: C-Reactive Protein

**Clinical Significance**: Non-specific inflammatory marker
**Reference Range**: < 3.0 mg/L (normal)
**Elevated**: > 10 mg/L indicates significant systemic inflammation

## Role in Drug Efficacy
CRP reduction can demonstrate anti-inflammatory efficacy. Used as a biomarker for immunomodulatory drug effect.

## Monitoring Protocol
- **Frequency**: Baseline, Day 7, Day 14, Day 28
- **Method**: High-sensitivity CRP assay
- **Clinical Action**: Successful drugs should reduce CRP by > 30% by Day 28

## Compounds Affecting This Biomarker
- [[Compound-CB-415]] (TNF-alpha modulation reduces CRP)

## Related Dosage Curves
- [[Dosage-Curve-CB415-150mg]]
- [[Dosage-Curve-CB415-450mg]]

#biomarker #inflammation #crp #anti-inflammatory
"

# ============================================================================
# DOSAGE CURVE NOTES
# ============================================================================

echo "[6/10] Creating dosage curve: CB-209 at 100mg"
smriti create "Dosage-Curve-CB209-100mg" --content "
# CB-209 Dosage Curve: 100 mg BID

**Compound**: [[Compound-CB-209]]
**Dose Level**: 100 mg twice daily
**Duration**: 28-day dosing

## Pharmacokinetic Profile
- **Cmax**: 1.2 µM (achieved at ~2 hours)
- **Trough (Ctrough)**: 0.4 µM
- **Steady-state**: Achieved by Day 5

## Efficacy Markers
- **PKC Inhibition**: ~70% at Cmax
- **Target Engagement**: Sustained above IC50 for 16+ hours/day

## Safety Profile at This Dose
- **Well-tolerated**: No dose-limiting toxicities observed
- **GI Effects**: Mild in 20% of subjects
- **Cardiac Safety**: Troponin-T stable, no elevation

## Related Biomarkers
- [[Biomarker-TroponinT]] — Stable at this dose
- [[Biomarker-BNP]] — Minimal change

## Adverse Events
- None severe at this dose level

## Recommendation
Suitable for Phase II dose escalation. Consider next dose level 300 mg to explore efficacy plateau.

#dosage-curve #pk-profile #phase-ii
"

echo "[7/10] Creating dosage curve: CB-209 at 300mg"
smriti create "Dosage-Curve-CB209-300mg" --content "
# CB-209 Dosage Curve: 300 mg BID

**Compound**: [[Compound-CB-209]]
**Dose Level**: 300 mg twice daily
**Duration**: 28-day dosing

## Pharmacokinetic Profile
- **Cmax**: 3.8 µM (achieved at ~2 hours)
- **Trough (Ctrough)**: 1.4 µM
- **Steady-state**: Achieved by Day 4

## Efficacy Markers
- **PKC Inhibition**: ~95% at Cmax
- **Target Engagement**: Maintained above IC50 for 18+ hours/day
- **Anti-inflammatory Response**: More pronounced than 100 mg dose

## Safety Profile at This Dose
- **Cardiac Concern**: Troponin-T elevation observed in 30% of subjects
- **GI Effects**: Nausea/loose stools in 60% of subjects
- **Other**: Mild headache, fatigue

## Related Biomarkers
- [[Biomarker-TroponinT]] — Elevated in some subjects (max 0.08 ng/mL)
- [[Biomarker-BNP]] — Slight elevation in 20% of subjects

## Adverse Events
- [[AE-Log-CB209-GI-Upset]] — Common at this dose
- Troponin elevation suggests dose-dependent cardiac risk

## Recommendation
Efficacy gains vs. safety concerns. Recommend limiting to subjects with baseline cardiac monitoring. Consider 200 mg as optimized dose for Phase II expansion.

#dosage-curve #pk-profile #safety-concern
"

echo "[8/10] Creating dosage curve: CB-415 at 150mg"
smriti create "Dosage-Curve-CB415-150mg" --content "
# CB-415 Dosage Curve: 150 mg QD

**Compound**: [[Compound-CB-415]]
**Dose Level**: 150 mg once daily
**Duration**: 28-day dosing

## Pharmacokinetic Profile
- **Cmax**: 2.1 µM (achieved at ~3 hours)
- **Ctrough**: 0.8 µM
- **Steady-state**: Achieved by Day 6

## Efficacy Markers
- **TNFR2 Target Engagement**: ~65% at Cmax
- **CRP Reduction**: ~20% by Day 28

## Safety Profile at This Dose
- **Well-tolerated**: Excellent safety profile
- **Cardiac Markers**: BNP stable or slightly reduced
- **GI Effects**: Minimal

## Related Biomarkers
- [[Biomarker-BNP]] — Stable or improved
- [[Biomarker-CRP]] — Meaningful reduction

## Adverse Events
- Minor headache in 15% of subjects (grade 1)

## Recommendation
Excellent risk-benefit at this dose. Clear efficacy with minimal toxicity. Advance to Phase II with expansion cohort.

#dosage-curve #pk-profile #well-tolerated
"

echo "[9/10] Creating dosage curve: CB-415 at 450mg"
smriti create "Dosage-Curve-CB415-450mg" --content "
# CB-415 Dosage Curve: 450 mg QD

**Compound**: [[Compound-CB-415]]
**Dose Level**: 450 mg once daily
**Duration**: 14-day dosing (higher dose cohort)

## Pharmacokinetic Profile
- **Cmax**: 6.8 µM (achieved at ~3 hours)
- **Ctrough**: 2.9 µM
- **Steady-state**: Achieved by Day 5

## Efficacy Markers
- **TNFR2 Target Engagement**: ~92% at Cmax
- **CRP Reduction**: ~35% by Day 14

## Safety Profile at This Dose
- **Hepatic Concern**: ALT elevation (2-3x ULN) in 40% of subjects
- **GI Effects**: Nausea in 50%, loose stools in 35%
- **Neurological**: Headaches in 60% (manageable with acetaminophen)

## Related Biomarkers
- [[Biomarker-BNP]] — Stable
- [[Biomarker-CRP]] — Significant reduction

## Adverse Events
- [[AE-Log-CB415-Headache]] — Common at this dose
- Hepatic monitoring required

## Recommendation
Dose-limiting efficacy gains. Hepatic concerns argue against escalation beyond 300 mg for next cohort. Consider 300 mg as optimal dose.

#dosage-curve #pk-profile #dose-limiting
"

# ============================================================================
# ADVERSE EVENT LOG
# ============================================================================

echo "[10/10] Creating adverse event logs"
smriti create "AE-Log-CB209-GI-Upset" --content "
# Adverse Event Log: CB-209 GI Upset

**Compound**: [[Compound-CB-209]]
**Event Type**: Gastrointestinal upset
**Severity**: Grade 1-2 (mild to moderate)
**Dose Relationship**: [[Dosage-Curve-CB209-300mg]]

## Incidence
- 100 mg dose: 20% incidence
- 300 mg dose: 60% incidence

## Manifestations
- Nausea (most common)
- Loose stools
- Abdominal discomfort
- Loss of appetite

## Mechanism
Likely due to direct GI epithelial irritation from high local concentration or gut microbiome disruption from PKC inhibition.

## Management
- Dose reduction recommended
- Food co-administration may help
- Anti-emetic support if needed

## Clinical Impact
Manageable but affects quality of life. Important consideration for long-term dosing strategies.

#adverse-event #gastrointestinal #cb-209
"

smriti create "AE-Log-CB415-Headache" --content "
# Adverse Event Log: CB-415 Headache

**Compound**: [[Compound-CB-415]]
**Event Type**: Headache
**Severity**: Grade 1-2 (mild to moderate)
**Dose Relationship**: [[Dosage-Curve-CB415-450mg]]

## Incidence
- 150 mg dose: 15% incidence
- 450 mg dose: 60% incidence

## Characteristics
- Tension-type headaches, bilateral
- Onset within 2-4 hours of dosing
- Duration: 4-8 hours
- Responsive to acetaminophen or ibuprofen

## Mechanism
Possibly related to vasodilatory effects from TNF-alpha pathway modulation or direct CNS penetration at higher doses.

## Management
- OTC analgesics effective
- Dose timing optimization (evening dosing may help)
- Prophylactic acetaminophen for high-dose cohorts

## Clinical Impact
Generally well-tolerated but affects tolerability. May limit dose escalation in sensitive subjects.

#adverse-event #headache #cb-415 #dose-related
"

echo ""
echo "========================================"
echo "Setup Complete!"
echo "========================================"
echo ""
echo "Knowledge base created with:"
echo "  - 2 compounds (CB-209, CB-415)"
echo "  - 3 biomarkers (Troponin-T, BNP, CRP)"
echo "  - 4 dosage curves (100mg, 300mg, 150mg, 450mg)"
echo "  - 2 adverse event logs"
echo ""
echo "Run the demo with:"
echo "  bash demo.sh"
echo ""
echo "Or explore manually with:"
echo "  smriti list              # See all notes"
echo "  smriti search 'compound' # Search by keyword"
echo "  smriti graph --note Compound-CB-209 --depth 3  # Traverse relationships"
echo ""
