# Chromatin Accessibility Models

## What Chromatin Accessibility Encodes

Gene regulation requires physical access. A transcription factor carrying the perfect binding domain for a promoter element is useless if a nucleosome is sitting on that element, occluding it behind 146 base pairs of tightly wrapped DNA. Before any of the transcription factor logic you have studied — the activations, the repressions, the cooperative binding — can operate, the regulatory DNA must be physically available. Chromatin accessibility is therefore the gate through which all gene regulation must pass.

**Chromatin accessibility** refers to the degree to which chromosomal DNA is physically available for protein binding — including transcription factors, RNA polymerase, and other regulatory machinery. Inaccessible (closed) chromatin is wound tightly around nucleosomes, occluding TF binding sites. Accessible (open) chromatin has nucleosomes displaced or repositioned, exposing regulatory sequences.

Accessibility is the gate through which all gene regulation must pass: a TF cannot regulate a gene if it cannot reach the DNA. Consequently, the landscape of chromatin accessibility essentially defines the **regulatory potential** of a cell — which TF binding sites are physically accessible and can transmit regulatory signals.

## Thermodynamic Model of Nucleosome Positioning

The probability that a specific TF is bound to its site depends on competition with a nucleosome for the same DNA sequence. The **thermodynamic model of regulatory element occupancy** (Segal & Widom 2009) treats this as an equilibrium binding problem:

$$P(\text{TF bound at site } s) = \frac{K_{\text{TF}} [c_{\text{TF}}] \cdot e^{-\Delta G_{\text{nuc}}(s)/k_BT}}{Z}$$

where:
- $K_{\text{TF}}$: TF-DNA affinity (derived from PWM score)
- $[c_{\text{TF}}]$: TF concentration in nucleus
- $\Delta G_{\text{nuc}}(s)$: energetic cost of displacing the nucleosome centered near site $s$ (sequence-dependent)
- $Z$: partition function summing all configurations

The key insight is that nucleosome positioning is **sequence-dependent**: poly(A/T) tracts and specific sequences with 10 bp periodicity in CG content are unfavorable for nucleosome wrapping (because the minor groove must face inward at regular intervals). Sequences with unfavorable nucleosome binding energy naturally have lower occupancy — they are intrinsically more accessible.

This provides a partial answer to one of the most puzzling questions in cell biology: why do different cell types have different open chromatin landscapes even when they express many of the same TFs? Part of the answer is written in the DNA sequence itself: some regulatory regions have intrinsically low nucleosome affinity and are accessible in many cell types, while others have high nucleosome affinity and require active displacement mechanisms specific to certain cell lineages.

## Sequence-Based Accessibility Prediction

**Nucleosome occupancy** can be predicted from DNA sequence using learned models:
- **Kaplan model** (Kaplan et al. 2009): trained on *S. cerevisiae* MNase-seq; predicts nucleosome occupancy from 5-mers
- **DANPOS**: predicts nucleosome positioning from sequence + histone modification data
- **DeepSEA / Basenji**: deep learning models predicting chromatin accessibility (ATAC-seq peaks) from sequence

```python
import numpy as np

# Simplified nucleosome affinity scoring (illustrative)
# Real models use genome-wide trained parameters

def nucleosome_affinity_score(seq, k=5):
    """
    Estimate nucleosome affinity from sequence k-mer composition.
    Returns higher score = more favorable nucleosome positioning
    (= less accessible).
    """
    # Simplified: AT-rich sequences disfavor nucleosomes
    at_content = (seq.count('A') + seq.count('T')) / len(seq)
    cg_content = (seq.count('C') + seq.count('G')) / len(seq)
    
    # Poly-AT sequences are nucleosome-disfavoring (lower affinity)
    affinity = cg_content - 0.5 * at_content
    return affinity

# Regulatory element accessibility from TF binding and nucleosome competition
def compute_tf_binding_prob(tf_pwm_score, nuc_affinity, tf_conc, 
                             kBT=0.59):  # kBT in kcal/mol at 310K
    """
    Thermodynamic model of TF vs. nucleosome competition.
    tf_pwm_score: log-odds score of TF binding site (proxy for ΔG_TF)
    nuc_affinity: nucleosome affinity score
    """
    # Free energies (relative units)
    dG_TF = -tf_pwm_score  # favorable binding = negative ΔG
    dG_nuc = -nuc_affinity  # favorable nucleosome = negative ΔG
    
    w_TF = tf_conc * np.exp(-dG_TF / kBT)
    w_nuc = np.exp(-dG_nuc / kBT)
    w_empty = 1.0
    
    Z = w_TF + w_nuc + w_empty
    return w_TF / Z

# High TF concentration + weak nucleosome affinity → high TF binding
p_tf = compute_tf_binding_prob(tf_pwm_score=5.0, nuc_affinity=0.2, 
                                tf_conc=1e-7)
print(f"TF occupancy probability: {p_tf:.3f}")
```

## Pioneer Transcription Factors

A crucial concept is that of **pioneer transcription factors** (Zaret & Carroll 2011): a subset of TFs that can bind to their sites even in the context of closed, nucleosomal chromatin. Pioneer factors include FOXA1, GATA factors, and OCT4. They bind first, partially displace or remodel the nucleosome, and then recruit additional TFs and chromatin remodeling complexes that open the chromatin further.

From a modeling perspective, pioneer TFs break the standard thermodynamic model: they have a non-negligible probability of binding even at occupied (nucleosome-wrapped) sites. Their binding initiates a kinetic cascade of chromatin remodeling that is history-dependent — the order in which TFs arrive at a locus matters.

Pioneer factors also explain a longstanding puzzle in reprogramming: how can Oct4 access its genomic targets in a fibroblast, where the pluripotency regulatory regions are wrapped in closed chromatin? Because Oct4, as a pioneer factor, does not require open chromatin to bind — it can open it. This makes Oct4 the first mover in Yamanaka reprogramming, the factor that breaks into closed territory and begins to reshape the epigenetic landscape.

## Chromatin Remodeling Complexes

ATP-dependent **chromatin remodeling complexes** (SWI/SNF, ISWI, CHD, INO80 families) actively reposition or evict nucleosomes. These are not in thermodynamic equilibrium with the DNA — they consume ATP to drive nucleosome movements against the thermodynamic gradient.

Modeling chromatin remodeling requires non-equilibrium (kinetic) models:

$$\frac{d[N_{\text{occ}}]}{dt} = -k_{\text{evict}}[N_{\text{occ}}][C] + k_{\text{deposit}}[N_{\text{free}}]$$

where $[C]$ is the concentration of chromatin remodeler and $k_{\text{evict}}, k_{\text{deposit}}$ are ATP-dependent rate constants.

## Integrating ATAC-seq Data

**ATAC-seq** (Assay for Transposase-Accessible Chromatin with sequencing) provides genome-wide accessibility maps at high resolution. Integration with models:

- **Footprinting**: within ATAC-seq peaks, nucleotide-resolution patterns reveal where specific TFs are bound (TF footprints = protected regions within accessible chromatin)
- **ChromBPNet / BPNet**: deep learning models that learn sequence-to-accessibility functions from ATAC-seq data, accounting for TF binding grammar at base-pair resolution
- **Linking accessibility to expression**: correlating accessibility changes at enhancers with target gene expression changes identifies functional regulatory elements

## Why This Matters

Chromatin accessibility modeling connects sequence (the genome) to function (gene regulation) through the physical chemistry of chromatin. The thermodynamic competition model explains why the same TF can activate a gene in one cell type and fail to do so in another — the chromatin state determines accessibility, and accessibility determines TF binding. This framework is central to understanding how cell-type-specific gene expression is established, how cancer driver mutations in chromatin remodeling genes alter regulatory landscapes, and how synthetic biology tools (CRISPRa/i, pioneer factor overexpression) can be designed to predictably reprogram cellular states.
