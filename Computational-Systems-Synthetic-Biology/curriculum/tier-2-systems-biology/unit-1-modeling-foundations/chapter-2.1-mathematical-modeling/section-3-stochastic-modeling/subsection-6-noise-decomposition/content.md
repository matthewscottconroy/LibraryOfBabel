# Noise Decomposition: Intrinsic and Extrinsic Noise

## The Two-Reporter Assay

In 2002, Michael Elowitz and colleagues published an experiment in *Science* that changed how biologists think about gene expression. The question they asked was deceptively simple: when two genetically identical cells express the same gene differently, is that because of random molecular events inside each cell, or because the cells themselves are in different states?

Their approach was elegant. They placed two different fluorescent reporters — cyan fluorescent protein (CFP) and yellow fluorescent protein (YFP) — under control of identical promoters in the same *E. coli* cell. If all variation were due to differences between cells (cell size, ribosome content, etc.), the two reporters would always be correlated: both high in big cells, both low in small cells. But if variation arose from the random timing of individual molecular events, the two reporters within the same cell would differ from each other — one high while the other is low — simply by chance.

When they looked at the cells under a fluorescence microscope, they saw both. Some cells had both reporters high, some had both low — that was extrinsic variation, cells differing from each other. But within individual cells, CFP and YFP levels were clearly not always equal — that was intrinsic variation, each gene's expression fluctuating independently even in the same cellular environment. The field had its first quantitative decomposition of the sources of biological noise.

## Definitions

**Intrinsic noise** arises from the inherently stochastic nature of molecular reactions: the random timing of transcription initiation events, mRNA degradation, translation, and protein degradation. It causes the two identical reporters in the same cell to differ from each other. Intrinsic noise is specific to the gene of interest — each gene has its own independent source of intrinsic fluctuations.

**Extrinsic noise** arises from cell-to-cell differences in the cellular environment: total RNA polymerase concentration, ribosome abundance, cell volume, DNA copy number, and global regulatory factors. It affects all genes simultaneously and coherently, causing both reporters in a cell to fluctuate together.

## Quantitative Framework

Let $u$ and $v$ be the expression levels of the two reporters (CFP and YFP) in a cell. Define:
- **Total noise**: $\eta_\text{tot}^2 = \text{Var}(u)/\langle u \rangle^2 = \text{CV}^2(u)$ (or equivalently for $v$)
- **Intrinsic noise**: $\eta_\text{int}^2 = \frac{\langle (u - v)^2 \rangle}{2 \langle u \rangle \langle v \rangle}$
- **Extrinsic noise**: $\eta_\text{ext}^2 = \frac{\text{Cov}(u, v)}{\langle u \rangle \langle v \rangle}$

These satisfy:

$$\eta_\text{tot}^2 = \eta_\text{int}^2 + \eta_\text{ext}^2$$

This decomposition assumes the two reporters have identical kinetics and are subject to the same extrinsic fluctuations.

**Intrinsic noise** measures the average squared difference between the two reporters, normalized by their means. If the cell produces exactly the same amount of both proteins (perfect molecular fidelity), $\eta_\text{int} = 0$.

**Extrinsic noise** measures the correlation between the two reporters across cells. If all variation is extrinsic (same cellular environment → same expression), then $u \approx v$ in every cell and $\text{Cov}(u, v) \approx \text{Var}(u)$, giving $\eta_\text{ext} \approx \eta_\text{tot}$.

The sum $\eta_\text{int}^2 + \eta_\text{ext}^2 = \eta_\text{tot}^2$ is an exact identity — not an approximation — under the two-reporter framework. It is the quantitative statement that total noise has two independent sources.

## Noise Metrics

**Coefficient of Variation (CV)**: $\text{CV} = \sigma/\mu$

The most common metric for noise. For a Poisson process, $\text{CV} = 1/\sqrt{\mu}$, so CV decreases with increasing mean. This means highly expressed genes have lower relative noise — a form of noise buffering through abundance. You might expect this to mean that cells prefer high expression to reduce noise, but high expression is metabolically costly. The cell must balance expression level against noise tolerance.

**Fano Factor**: $F = \sigma^2/\mu$

For a Poisson process, Fano = 1. The Fano factor is independent of mean expression level, making it better for comparing noise across different expression levels. Super-Poisson noise (Fano > 1) indicates clustering or bursting — production events are not independent.

**Measuring burstiness**: If mRNA is produced in transcriptional bursts of mean size $b$ and mean burst frequency $k_\text{on}$, the steady-state Fano factor is approximately:

$$F \approx 1 + b$$

A Fano factor of 5 suggests an average burst size of ~4 mRNA molecules per burst event. This can be measured directly from single-molecule FISH data — counting individual mRNA molecules in individual cells and computing the variance-to-mean ratio. The fact that this number tells you the burst size is a beautiful example of how a statistical measurement connects directly to a molecular mechanism.

## Biological Sources of Extrinsic Noise

Major contributors to extrinsic noise include:

**Cell-cycle position**: gene copy number varies 2-fold from G1 (one copy) to post-replication (two copies). Protein levels inherited from the mother cell dilute as the cell grows. Cell size (volume) affects concentrations.

**RNAP and ribosome abundance**: fluctuations in global translation and transcription capacity affect all genes simultaneously, creating correlated extrinsic noise. A cell that happens to have slightly more ribosomes will translate all its mRNAs more efficiently — a global effect that is extrinsic to any individual gene.

**Regulatory hub proteins**: transcription factors that regulate hundreds of genes (sigma factors in bacteria, master TFs in eukaryotes) introduce correlated fluctuations across their target genes. The noise in the master regulator becomes shared noise for its entire regulon.

## Controlling Noise

Network topology can control noise levels:

**Negative autoregulation** reduces intrinsic noise by creating a homeostatic feedback that dampens fluctuations. A transcription factor that represses its own gene maintains its concentration closer to the set point. Quantitatively, negative autoregulation reduces the CV of protein expression — a prediction that was made from ODE models and verified by comparing natural negative autoregulators to engineered non-autoregulating controls.

**Increasing mRNA stability** (longer half-life) allows more protein molecules per mRNA molecule (larger burst size $b$), increasing Fano factor but potentially leaving total protein CV unchanged if production rate is adjusted.

**Shared extrinsic noise** can be filtered by ratio sensing: if a cell measures the ratio of two proteins subject to the same extrinsic noise, the correlated fluctuations cancel. This is why cells often use ratios of two proteins as regulatory signals rather than absolute concentrations.

```python
import numpy as np
from scipy.stats import pearsonr

def two_reporter_experiment(n_cells=1000, alpha=10, delta=1, 
                             sigma_ext=0.3, seed=42):
    """Simulate two-reporter assay with intrinsic and extrinsic noise."""
    rng = np.random.default_rng(seed)
    
    # Extrinsic: global scaling of production rate
    ext_factor = rng.lognormal(0, sigma_ext, n_cells)
    
    # Intrinsic: independent Poisson sampling for each reporter
    mean_u = alpha / delta * ext_factor
    mean_v = alpha / delta * ext_factor
    u = rng.poisson(mean_u)
    v = rng.poisson(mean_v)
    
    mu_u, mu_v = u.mean(), v.mean()
    eta_int2 = np.mean((u - v)**2) / (2 * mu_u * mu_v)
    eta_ext2 = np.cov(u, v)[0,1] / (mu_u * mu_v)
    eta_tot2 = np.var(u) / mu_u**2
    
    print(f"η²_tot = {eta_tot2:.4f}")
    print(f"η²_int = {eta_int2:.4f}")
    print(f"η²_ext = {eta_ext2:.4f}")
    print(f"Sum check: {eta_int2 + eta_ext2:.4f}")
    return u, v

u, v = two_reporter_experiment()
```

## Why This Matters

Noise decomposition transformed our understanding of gene expression variability from a curiosity into a quantitative phenomenon with molecular explanations. It revealed that most cell-to-cell variation in highly expressed genes is extrinsic — driven by global cellular state — while variability in lowly expressed genes (especially transcription factors) has a larger intrinsic component.

This distinction has practical consequences: strategies to reduce cell-to-cell variability in synthetic gene circuits must target the dominant noise source, which differs between genes. If you want to engineer a more reliable gene circuit, you need to know whether the variability you're fighting is intrinsic (fix the gene's own regulation) or extrinsic (fix the global cellular environment or filter it out via ratiometric sensing). The noise decomposition framework gives you the tools to diagnose which problem you have.

For gene therapy and cell-based therapies, controlling expression variability is critical for predictable therapeutic outcomes. A therapeutic gene that is expressed at the right level in 80% of cells but at dangerous levels in 20% is not a viable therapy. The noise decomposition framework — and the mechanistic understanding of what drives each component — is the foundation for engineering more reliable biological systems.
