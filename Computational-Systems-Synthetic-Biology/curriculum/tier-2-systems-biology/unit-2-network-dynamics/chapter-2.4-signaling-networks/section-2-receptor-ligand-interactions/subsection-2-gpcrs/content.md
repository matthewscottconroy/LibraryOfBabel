# G Protein-Coupled Receptors

## The Dominant Receptor Family

The next time your heart races before a presentation, blame your GPCRs. Epinephrine is flooding your bloodstream, and somewhere on every cardiac myocyte, a beta-adrenergic receptor — one of the ~800 **G protein-coupled receptors (GPCRs)** in your genome — is catching that adrenaline molecule and translating chemical panic into the faster heartbeat you feel. GPCRs constitute the largest family of cell surface receptors in the human genome, targeting ~34% of approved drugs. They mediate responses to an extraordinary range of stimuli — neurotransmitters, hormones, odorants, taste molecules, photons (via rhodopsin), and even mechanical force.

GPCRs are also called **seven-transmembrane (7-TM) receptors** because of their characteristic structure: a single polypeptide that traverses the membrane seven times, with three extracellular and three intracellular loops, an extracellular N-terminus, and an intracellular C-terminus.

## Activation Mechanism

The **activation cycle** proceeds through a stereotyped sequence:

**Step 1: Ligand binding and conformational change**

An agonist binds to the extracellular ligand-binding pocket (formed by TM3, TM5, TM6, TM7). This induces a conformational change that propagates through the helix bundle: TM6 rotates outward by ~6-14 Å on the intracellular side, opening a cavity for G protein binding.

**Step 2: G protein activation**

The inactive G protein heterotrimer (Gα-GDP·Gβγ) is recruited to the activated receptor. The receptor acts as a **guanine nucleotide exchange factor (GEF)**: it accelerates exchange of GDP for GTP in the Gα subunit $\sim 10^4$-fold. Gα-GTP has low affinity for Gβγ → they dissociate. Each active receptor can activate multiple G proteins (catalytic amplification: ~1,000-fold).

**Step 3: Effector activation by Gα-GTP and Gβγ**

The dissociated subunits independently activate effectors:

| Gα subtype | Effector | Second messenger |
|---|---|---|
| Gαs | Adenylyl cyclase ↑ | cAMP ↑ → PKA |
| Gαi | Adenylyl cyclase ↓ | cAMP ↓ |
| Gαq | PLC-β | IP3 + DAG → Ca²⁺ + PKC |
| Gα12/13 | RhoGEF | RhoA → cytoskeleton |
| Gβγ | PI3Kγ, ion channels, PLC-β | Multiple |

**Step 4: Signal termination**

- Gα-GTP is inactivated by its intrinsic GTPase activity (GTP → GDP, minutes timescale)
- RGS proteins (Regulators of G-protein Signaling) accelerate GTPase activity ~100-1000-fold
- GRK (GPCR kinase) phosphorylates intracellular loops/C-terminus of activated receptor
- Phosphorylated receptor recruits β-arrestin → uncouples from G protein (desensitization)
- β-arrestin recruits clathrin → receptor internalization (endocytosis)

## The cAMP-PKA Signaling Branch

For Gαs-coupled receptors (β-adrenergic receptors, glucagon receptor, many others):

$$\text{Agonist} \to \text{GPCR} \to \text{Gαs-GTP} \to \text{Adenylyl Cyclase (AC)} \to \text{cAMP} \to \text{PKA}$$

cAMP is produced from ATP by AC; degraded to AMP by phosphodiesterases (PDEs). At steady state:

$$[\text{cAMP}]_{\text{ss}} = \frac{k_{\text{AC}} [\text{AC}_{\text{active}}]}{k_{\text{PDE}} [\text{PDE}]}$$

cAMP binds regulatory subunits of **PKA (Protein Kinase A)**: $K_d \approx 100-300$ nM. At $[\text{cAMP}] > K_d$: regulatory subunits dissociate from catalytic subunits → free catalytic PKA phosphorylates hundreds of substrates.

**Worked example — β-adrenergic signaling in cardiac myocytes:**

Epinephrine (adrenaline) binding to β1-AR → Gαs → cAMP ↑ → PKA activates:
- Phospholamban phosphorylation → increased SERCA pump activity → faster calcium reuptake → faster relaxation
- RyR2 phosphorylation → increased Ca²⁺ release → stronger contraction
- Troponin I phosphorylation → reduced myofilament Ca²⁺ sensitivity → faster cross-bridge cycling

Net effect: increased heart rate and contractility (the "fight or flight" response). A single signaling pathway achieves coordinated activation of multiple targets through PKA-mediated phosphorylation.

## Mathematical Modeling of GPCR Signaling

```python
import numpy as np
from scipy.integrate import solve_ivp

def gpcr_camp_model(t, y, params):
    """
    Minimal GPCR → G protein → cAMP → PKA model.
    y: [Lbound, Gactive, cAMP, PKAactive]
    """
    Lbound, Gactive, cAMP, PKAactive = y
    
    L = params['L']          # free ligand (constant, excess)
    Rtotal = params['Rtotal'] # total receptor
    Gtotal = params['Gtotal'] # total G protein
    
    # Receptor activation
    kon, koff = params['kon'], params['koff']
    R_free = Rtotal - Lbound
    dLbound = kon * L * R_free - koff * Lbound
    
    # G protein activation (catalytic)
    kact = params['kact']
    kinact = params['kinact']  # GTPase + RGS
    dGactive = kact * Lbound * (Gtotal - Gactive) - kinact * Gactive
    
    # cAMP synthesis/degradation
    kAC = params['kAC']
    kPDE = params['kPDE']
    dcAMP = kAC * Gactive - kPDE * cAMP
    
    # PKA activation (simplified Hill function)
    Kpka = params['Kpka']
    n = params['n_pka']
    PKA_fraction = cAMP**n / (Kpka**n + cAMP**n)
    dPKAactive = params['kpka'] * (PKA_fraction * params['PKAtotal'] - PKAactive)
    
    return [dLbound, dGactive, dcAMP, dPKAactive]

params = {
    'L': 1e-8, 'Rtotal': 1e-8, 'Gtotal': 1e-6,
    'kon': 1e6, 'koff': 0.01, 'kact': 0.5, 'kinact': 0.1,
    'kAC': 0.1, 'kPDE': 0.5, 'Kpka': 1e-7, 'n_pka': 2,
    'kpka': 0.5, 'PKAtotal': 1e-7
}

sol = solve_ivp(gpcr_camp_model, [0, 200], [0, 0, 0, 0],
                args=(params,), method='Radau', rtol=1e-8)
```

## β-Arrestin: A Second Signaling Mode

Recent discoveries revealed that β-arrestin, originally described only as a desensitization molecule, also acts as a scaffold for signaling complexes — a process called **biased agonism**:

- Classical signaling: ligand → G protein → second messengers
- β-arrestin signaling: ligand → β-arrestin recruitment → distinct signaling complex (ERK, etc.)

Different ligands can preferentially activate G protein vs. β-arrestin signaling at the same receptor. This **functional selectivity** provides a potential basis for drugs that activate only the desired signaling arm while avoiding side effects from the other.

## Why This Matters

GPCRs are the largest drug target family in medicine: antihistamines, beta blockers, antidepressants (via serotonin receptors), antipsychotics, opioid analgesics, and many others all target GPCRs. Understanding the G protein activation cycle, second messenger kinetics, and desensitization mechanisms at the quantitative level is prerequisite to understanding dose-response relationships, tolerance (desensitization), and the rational design of biased agonists that maximize therapeutic benefit while minimizing side effects.
