# DNA Methylation Dynamics

## The Biochemistry of CpG Methylation

Every time a cell divides, its DNA methylation pattern faces a crisis. The methylation marks on the parental DNA strand are on cytosines, but after replication, the newly synthesized strand is unmethylated — the double helix is suddenly hemimethylated on one strand and blank on the other. If nothing intervenes, successive rounds of replication would passively dilute the methylation pattern by 50% per generation, and within a few cell cycles the epigenetic identity encoded in methylation would be gone. That this does not happen is a remarkable feat of molecular bookkeeping.

**DNA methylation** at cytosines in CpG dinucleotides is a heritable, covalent epigenetic mark. In mammalian genomes:
- Approximately 70–80% of CpG sites are methylated in somatic cells
- **CpG islands** (CG-rich regions near gene promoters) are typically unmethylated in transcriptionally active genes
- Promoter methylation generally correlates with gene silencing (blocks TF binding and recruits methyl-CpG binding proteins)

The enzymes governing methylation form a coupled system with distinct biochemical roles:

**Writers:**
- **DNMT3A, DNMT3B**: *de novo* methyltransferases — add methyl groups to previously unmethylated cytosines
- **DNMT1**: maintenance methyltransferase — after DNA replication, restores methylation on the newly synthesized strand using the parental strand as template

**Erasers (TET-mediated demethylation):**
$$5\text{mC} \xrightarrow{\text{TET}} 5\text{hmC} \xrightarrow{\text{TET}} 5\text{fC} \xrightarrow{\text{TET}} 5\text{caC} \xrightarrow{\text{TDG}} \text{C}$$

TET enzymes oxidize 5-methylcytosine (5mC) to 5-hydroxymethylcytosine (5hmC), which can be further oxidized to 5-formylcytosine (5fC) and 5-carboxylcytosine (5caC). The last two forms are substrates for base excision repair, ultimately returning an unmethylated cytosine.

## Mathematical Model of Methylation Dynamics

At a single CpG site, the methylation state $m \in [0,1]$ (fraction of cells with the site methylated) evolves as:

$$\frac{dm}{dt} = k_m (1-m) - k_d \cdot m$$

where $k_m$ is the *de novo* methylation rate (DNMT3A/B activity) and $k_d$ is the demethylation rate (TET + BER activity plus passive dilution from replication if DNMT1 is inefficient). At steady state:

$$m^* = \frac{k_m}{k_m + k_d}$$

This simple model predicts that methylation levels are determined by the balance of writing and erasing activities — not by the initial state. However, the actual dynamics are more complex due to:

1. **Replication-coupled dynamics**: at each S phase, methylation on the new strand must be re-established by DNMT1. If DNMT1 fails (DNMT1 inhibition), methylation is passively diluted by 50% per generation.
2. **Cooperative spreading**: DNMT3A/B have higher affinity for hemimethylated CpG sites, and PRC2 recruits DNMT3A to some loci, creating local amplification.
3. **Sequence context**: non-CpG methylation (CH methylation) occurs significantly in neurons; CpG density affects spreading kinetics.

## Modeling Maintenance Methylation

The maintenance methylation fidelity $\epsilon$ (probability that DNMT1 re-methylates each hemimethylated site per replication cycle) determines the error rate:

$$m(g+1) = \epsilon \cdot m(g) + k_m (1 - m(g)) \cdot \Delta t - k_d \cdot m(g) \cdot \Delta t$$

For typical DNMT1 fidelity $\epsilon \approx 0.97$, and in the absence of *de novo* methylation ($k_m = 0$), a fully methylated site loses $\approx 3\%$ of methylation per generation — consistent with observed methylation drift in long-term cultured cells and aging.

```python
import numpy as np

def methylation_dynamics(m0, k_m, k_d, epsilon=0.97, n_generations=100):
    """
    Simulate methylation dynamics over cell divisions.
    m0: initial methylation fraction
    k_m: de novo methylation rate per generation  
    k_d: demethylation rate per generation
    epsilon: maintenance methylation fidelity
    """
    m = [m0]
    for g in range(n_generations):
        m_prev = m[-1]
        # After replication: hemimethylated sites (m_prev/2 fully methylated,
        # m_prev/2 hemimethylated requiring maintenance)
        m_after_rep = epsilon * m_prev + (1 - epsilon) * m_prev * 0.5
        # De novo methylation of unmethylated sites
        m_after_dn = m_after_rep + k_m * (1 - m_after_rep)
        # Demethylation by TET
        m_new = m_after_dn - k_d * m_after_dn
        m.append(np.clip(m_new, 0, 1))
    return np.array(m)

# Normal maintenance: methylation preserved
traj_normal = methylation_dynamics(0.9, k_m=0.01, k_d=0.01)

# DNMT1 knockout: rapid demethylation
traj_ko = methylation_dynamics(0.9, k_m=0.01, k_d=0.01, epsilon=0.0)

print(f"Normal: final m = {traj_normal[-1]:.3f}")
print(f"DNMT1 KO: final m = {traj_ko[-1]:.6f}")
```

## Methylation in Single Cells vs. Population Averages

A critical insight from single-cell bisulfite sequencing: **CpG methylation is NOT binary at the population level**. Bulk bisulfite sequencing reports the population average, which can be any value between 0 and 1. This intermediate methylation arises not because individual molecules are "half methylated" but because different cells in the population have different binary methylation states at the same CpG.

This means that:
- A locus with 50% bulk methylation may be fully methylated in half the cells and fully unmethylated in the other half (bimodal distribution — consistent with bistability)
- Or it may be uniformly ~50% methylated in all cells (unimodal distribution — reflecting a steady-state balance of writing and erasing)

Single-cell bisulfite sequencing distinguishes these cases and reveals which loci are epigenetically heterogeneous (bistable) versus uniformly intermediate.

This distinction is not merely academic. A locus with bimodal methylation in a tumor may represent an epigenetically unstable state — a population of cells with two distinct fates, one of which leads to drug resistance. A locus with uniformly intermediate methylation represents a different kind of regulatory state entirely, one set by a dynamic equilibrium rather than by bistable switching.

## Allele-Specific Methylation and Imprinting

**Genomic imprinting** is the clearest demonstration that DNA methylation encodes gene expression memory: some loci are methylated exclusively on the maternal or paternal allele, creating permanent mono-allelic gene expression. This methylation is established in the germline, partially erased and re-established during embryonic reprogramming, and maintained faithfully in all somatic cells.

The mathematical model for imprinting maintenance is the same as above ($\epsilon$ close to 1 for imprinted loci) but with specific methylation patterns on individual chromosomes. Loss of imprinting (LOI) — observed in many cancers — corresponds to failure of maintenance methylation at specific loci.

## Why This Matters

DNA methylation modeling provides a quantitative framework for understanding epigenetic drift (aging), reprogramming efficiency, and cancer methylation patterns. Therapeutically, DNMT inhibitors (5-azacytidine, decitabine) work by blocking maintenance methylation, causing passive demethylation. The mathematical model predicts that these drugs require multiple cell cycles to substantially reduce methylation — explaining their clinical pharmacodynamics and guiding dosing strategies.
