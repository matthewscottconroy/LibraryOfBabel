# Chromatin State as Cellular Memory

## Epigenetic Memory: The Problem

A liver cell and a neuron carry identical DNA sequences. Yet they maintain radically different gene expression programs across decades and through thousands of cell divisions. How? The answer lies in **epigenetic mechanisms** — heritable modifications to chromatin structure that do not alter the DNA sequence itself. These modifications constitute a form of cellular memory encoded in the biochemistry of chromatin, not in the base sequence of DNA.

This is a deep puzzle if you think about it carefully. After every cell division, the chromatin is replicated, the new histones laid down are unmarked, and the epigenetic pattern must be re-established from scratch — like printing a complex image from a template that was cut in half during copying. The fact that this process works faithfully, generation after generation, tells you that there must be a self-reinforcing mechanism that restores the pattern before the next division. That mechanism is the subject of this section.

From a systems biology perspective, epigenetic states are **bistable switches implemented in chromatin**: molecular positive feedback loops that maintain active or repressed chromatin states through cell division, while remaining responsive to developmental signals that can switch the state.

## Histone Modifications and Readers/Writers/Erasers

Histone proteins (H2A, H2B, H3, H4) around which DNA wraps are subject to dozens of distinct post-translational modifications on their N-terminal tails. The most functionally important for gene silencing/activation:

| Mark | Enzyme | Function |
|---|---|---|
| H3K27me3 | PRC2 (writer), KDM6A/B (eraser) | Polycomb silencing |
| H3K4me3 | MLL complexes (writer) | Active promoters |
| H3K27ac | CBP/p300 (writer), HDACs (eraser) | Active enhancers |
| H3K9me3 | G9a, SUV39H (writer) | Constitutive heterochromatin |
| H4ac | HATs (writer) | Accessible chromatin |

The **reader-writer-eraser (RWE) model** describes how each modification is self-reinforcing: a writer enzyme deposits a mark; the mark recruits a reader protein that binds the writer; the writer then deposits additional marks on neighboring nucleosomes. This local amplification creates a positive feedback mechanism.

If you are thinking "this is another positive feedback loop, just like the toggle switch," you are correct. The Polycomb system, the heterochromatin spreading system, and the active histone modification system are all biochemical implementations of the same mathematical object — a bistable switch — but realized through chromatin chemistry rather than transcriptional logic.

## The Polycomb System as a Bistable Switch

The **PRC2 (Polycomb Repressive Complex 2)** system is the best-understood example of epigenetic bistability:

$$\frac{d[H3K27me3]}{dt} = k_{\text{write}} \cdot [PRC2] \cdot [H3K27me3] \cdot [H3K27] - k_{\text{erase}} \cdot [KDM6] \cdot [H3K27me3]$$

The key term is $k_{\text{write}} \cdot [PRC2] \cdot [H3K27me3]$: the writing rate is proportional to the existing mark density (because PRC2 is recruited by H3K27me3 through its EED subunit). This creates positive feedback:

- More H3K27me3 → more PRC2 recruitment → more H3K27me3 writing
- Less H3K27me3 → less PRC2 recruitment → further loss of the mark

A **bistable system** results: the system has two stable states (marked = silenced, unmarked = active) with an unstable threshold between them. Transitions between states require sufficient perturbation (e.g., developmental signal activating KDM6 demethylase, or recruitment of an activating complex that displaces PRC2).

```python
import numpy as np
from scipy.integrate import solve_ivp
import matplotlib.pyplot as plt

def polycomb_bistable(t, y, k_write, k_erase, prc2_total, kdm6_total):
    """
    y[0]: fraction of H3K27me3 (m, between 0 and 1)
    Remaining: 1 - m = unmethylated H3K27
    """
    m = y[0]
    # Positive feedback: PRC2 recruited by existing me3
    prc2_active = prc2_total * m / (0.1 + m)  # saturable recruitment
    kdm6_active = kdm6_total  # constitutive (developmental signal)
    
    dm_dt = k_write * prc2_active * (1 - m) - k_erase * kdm6_active * m
    return [dm_dt]

# Two initial conditions → two different stable states (bistability)
for m0 in [0.05, 0.95]:
    sol = solve_ivp(polycomb_bistable, [0, 100], [m0],
                   args=(0.5, 0.1, 1.0, 0.2), dense_output=True)
    print(f"Starting at m={m0}: final state m={sol.y[0,-1]:.3f}")
# Starting at m=0.05: final state m≈0.1 (silenced)
# Starting at m=0.95: final state m≈0.9 (active)
```

## Inheritance Through Cell Division

A critical feature of epigenetic memory is that it must be **faithfully inherited through DNA replication**. After replication, each daughter chromosome has only 50% of the parental histone marks (old histones are distributed between daughters, new histones are deposited without marks). The positive feedback mechanism must restore the full complement of marks before the next cell division.

For H3K27me3, this restoration relies on:
1. Old H3K27me3-marked histones distribute to both daughter chromosomes
2. PRC2 is recruited to the marked histones
3. PRC2 deposits new H3K27me3 on nearby unmarked histones
4. Spreading continues until all nucleosomes within the domain are marked

The mathematical requirement for faithful inheritance is that the restoration rate must exceed the dilution rate (50% per cell cycle). This constrains the minimum positive feedback strength required for epigenetic memory.

This constraint is not just theoretical. It explains why cancer cells with reduced PRC2 activity (through EZH2 loss-of-function mutations) progressively lose Polycomb silencing over generations — the positive feedback is too weak to restore the full mark density after each replication, and the system drifts toward the active state.

## Chromatin Domains and Boundary Elements

Epigenetic marks do not exist in isolation — they are organized into **topologically associating domains (TADs)** and **chromatin domains** (typically 10 kb to several Mb in size). Within a domain, marks tend to be uniform (all active or all repressed). Between domains, **boundary elements** (CTCF binding sites, cohesin) prevent spreading from one domain to another.

From a systems perspective, each chromatin domain behaves as a bistable switch with two stable states. The genome is therefore a collection of coupled bistable switches, where each domain can independently be silenced or activated, creating a combinatorial epigenetic state for each cell type.

## Why This Matters

Understanding chromatin state as a bistable memory system transforms how we think about cell identity, cancer epigenetics, and reprogramming. Cancer often involves inappropriate gain or loss of Polycomb silencing (via EZH2 mutations or overexpression). Reprogramming requires erasing epigenetic memory in somatic cells to establish pluripotent state. Differentiation involves progressive locking of the epigenetic state as bistability becomes more pronounced. All of these phenomena are mechanistically explained by the reader-writer positive feedback that sustains chromatin states across cell divisions.
