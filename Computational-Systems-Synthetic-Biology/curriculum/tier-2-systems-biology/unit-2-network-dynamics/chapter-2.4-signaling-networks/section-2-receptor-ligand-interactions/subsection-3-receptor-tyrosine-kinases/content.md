# Receptor Tyrosine Kinases

## Structure and Mechanism

In about 25% of breast cancers, a receptor sits at the cell surface in a permanently excited state, broadcasting a growth signal the cell never asked for. HER2 — a member of the receptor tyrosine kinase family — is overexpressed to such a degree that the sheer density of receptors drives spontaneous dimerization and constitutive kinase activity, uncoupled from any ligand. The result is uncontrolled proliferation. The drug trastuzumab (Herceptin) blocks this, and its clinical success is a direct testament to how well we now understand RTK biology.

**Receptor tyrosine kinases (RTKs)** are a family of ~58 transmembrane receptors in the human genome that possess intrinsic tyrosine kinase activity. Unlike GPCRs (which rely on G proteins as intermediaries), RTKs directly phosphorylate their own intracellular domains and downstream substrates upon ligand binding.

The canonical RTK structure:
- **Extracellular domain**: ligand-binding, diverse structure (Ig-like, cysteine-rich, leucine-rich)
- **Single transmembrane helix**: anchors the receptor
- **Juxtamembrane domain**: regulatory, often contains phosphorylation sites that modulate kinase activity
- **Kinase domain**: bilobal structure with an activation loop containing key tyrosine(s)
- **C-terminal regulatory tail**: contains multiple tyrosine phosphorylation sites that recruit signaling proteins

## Activation by Dimerization

Most RTKs are monomers in the resting state. Ligand binding induces **receptor dimerization** (or oligomerization), which is required for activation:

1. Ligand binds extracellular domain (often by bridging two receptor monomers as a dimer — e.g., EGF forms bivalent 1:1 complexes with EGFR)
2. Receptor dimerization brings two intracellular kinase domains into proximity
3. **Transautophosphorylation**: each kinase phosphorylates the partner receptor's activation loop tyrosine — cross-activation
4. Phosphorylated activation loop opens the kinase active site → constitutive kinase activity
5. Additional regulatory tyrosines in the kinase domain and C-terminal tail are phosphorylated (multiple sites on EGFR, PDGFR, etc.)

**Notable exception**: HER2 (ERBB2) has no known ligand — it exists at the cell surface in a pre-extended conformation ready for dimerization with other ERBB family members. Its overexpression (25% of breast cancers) drives ligand-independent receptor dimerization and constitutive signaling.

## Recruitment of Downstream Signaling Proteins

Phosphotyrosine residues on activated RTKs function as docking sites for proteins containing **SH2 (Src Homology 2)** or **PTB (Phosphotyrosine Binding)** domains. Each phosphotyrosine site recruits a specific set of adaptor proteins, creating a combinatorial signaling complex:

**EGFR signaling scaffold:**
- pY1068: recruits GRB2 (via SH2) → SOS (RAS-GEF) → RAS → RAF/MEK/ERK
- pY1173: recruits SHC → GRB2 → RAS (alternative RAS activation route)
- pY1045: recruits CBL (E3 ubiquitin ligase) → receptor ubiquitination → degradation
- pY992, pY1045: recruit PLC-γ → IP3 + DAG → Ca²⁺ + PKC

The specificity of SH2 domain binding for particular phosphotyrosine motifs (defined by the 3 amino acids C-terminal to the pY) creates **combinatorial specificity**: different RTKs recruit different signaling proteins even though they all use SH2-phosphotyrosine interactions.

## RAS Activation: The Pivot from Receptor to Intracellular

The most critical downstream pathway from most RTKs is RAS activation via the GRB2-SOS complex:

$$\text{pRTK} \xrightarrow{+\text{GRB2}} \text{pRTK:GRB2:SOS} \to \text{RAS-GTP} \to \text{RAF} \to \text{MEK} \to \text{ERK}$$

**SOS** is a **guanine nucleotide exchange factor (GEF)** for RAS: it accelerates exchange of GDP for GTP, activating RAS. RAS-GTP activates RAF (MAPKKK), initiating the MAPK cascade.

RAS is inactivated by its intrinsic GTPase activity (slow, ~5 min), accelerated by GAP proteins (RAS-GTPase activating proteins). Oncogenic RAS mutations (KRAS G12D, NRAS Q61K) eliminate GTPase activity → constitutive RAS-GTP → constitutive MAPK activation → cancer.

## PI3K-AKT Pathway

Many RTKs also activate the **phosphoinositide 3-kinase (PI3K) → AKT** survival pathway:

$$\text{pRTK} \to \text{PI3K} \to \text{PIP3} \to \text{AKT (PKB)} \to \text{mTORC1, FOXO, BAD, ...}$$

PI3K phosphorylates PIP2 → PIP3. PIP3 recruits AKT (via PH domain) and PDK1 to the membrane, where PDK1 phosphorylates AKT at T308 → partial activation. mTORC2 phosphorylates AKT at S473 → full activation.

Active AKT promotes:
- Cell survival (phosphorylates BAD → releases BCL-2 from BAD sequestration)
- Cell growth (activates mTORC1 → protein synthesis, ribosome biogenesis)
- Cell proliferation (phosphorylates FOXO transcription factors → nuclear exclusion → reduced CDK inhibitor expression)

**PTEN** (phosphatase and tensin homolog) is the tumor suppressor that dephosphorylates PIP3 → PIP2, opposing PI3K. Loss of PTEN (one of the most common tumor suppressor deletions in cancer) → constitutive AKT activation.

## Receptor Internalization and Spatial Signaling

RTK signaling does not end at the cell surface. After internalization:
- **Sorting endosomes**: receptors continue signaling from endosomes, potentially with different substrate access and duration
- **Multivesicular bodies**: continued signaling, different protein complex composition
- **Recycling**: receptor returns to surface (fast recycling via Rab4, slow recycling via Rab11)
- **Lysosomal degradation**: signal termination

The **subcellular location** of RTK signaling matters: ERK activation from the plasma membrane vs. endosomes has different kinetics and activates different nuclear targets. This spatial encoding is a mechanism for signal diversification from a single receptor.

## Worked Example: EGFR Dose-Response

```python
import numpy as np

def egfr_dose_response(EGF_conc, Kd=1e-9, n_dimers=2, 
                        K_EC50_ras=0.3, alpha=0.8):
    """
    Simplified EGFR → RAS → ERK dose-response model.
    EGF_conc: EGF concentration (M)
    Returns: predicted ERK activation (normalized 0-1)
    """
    # Receptor occupancy (assume Kd for dimerization-triggering binding)
    theta = EGF_conc**n_dimers / (Kd**n_dimers + EGF_conc**n_dimers)
    # RAS activation (proportional to receptor dimerization)
    ras_active = theta
    # ERK activation (sigmoidal response to RAS)
    erk = ras_active**2 / (K_EC50_ras**2 + ras_active**2)
    return erk

EGF_range = np.logspace(-12, -6, 100)
erk_response = [egfr_dose_response(c) for c in EGF_range]
ec50_idx = np.argmin(np.abs(np.array(erk_response) - 0.5))
print(f"ERK EC50 ≈ {EGF_range[ec50_idx]*1e9:.2f} nM")
```

## Why This Matters

RTKs are major cancer drivers and drug targets: EGFR (lung cancer), HER2 (breast cancer), PDGFR (GIST), KIT (GIST, AML), VEGFR (angiogenesis target) are all clinically targeted by approved inhibitors. Understanding RTK activation mechanisms, downstream pathway connections, and feedback regulation is essential for predicting drug effects, mechanisms of acquired resistance (e.g., EGFR T790M mutation), and rational combination therapy design.
