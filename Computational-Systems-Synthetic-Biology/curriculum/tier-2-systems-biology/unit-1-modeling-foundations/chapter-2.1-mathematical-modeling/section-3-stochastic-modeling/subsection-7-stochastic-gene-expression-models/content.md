# Stochastic Gene Expression Models

## Beyond the Constitutive Case

Here is a puzzle. If you measure the number of mRNA molecules of a typical gene in a population of genetically identical cells — using single-molecule FISH, which counts each mRNA one by one — you find a distribution that is not Poisson. It is broader. In mammalian cells, Fano factors of 2–50 are common. In bacteria, values of 2–10 are typical for well-studied genes.

From the CME analysis of constitutive expression, we know that if mRNA is produced at a constant rate $\alpha$ and degrades at rate $\delta$, the steady-state distribution is Poisson with Fano = 1. So the excess noise above 1 is telling us something important: mRNA is *not* produced at a constant rate. It comes in bursts.

This is **transcriptional bursting**: periods of intense transcriptional activity separated by silent intervals. A gene doesn't produce mRNA at a smooth, constant rate; it turns on, makes a burst of transcripts, turns off, waits, then turns on again. The randomness in burst timing and burst size creates the super-Poisson distributions we observe. Understanding this phenomenon requires models that explicitly account for promoter state dynamics.

## The Two-State Promoter Model

The **two-state promoter model** (also called the telegraph model or random telegraph process) is the minimal model of transcriptional bursting:

$$\text{Promoter}_\text{OFF} \underset{k_\text{off}}{\overset{k_\text{on}}{\rightleftharpoons}} \text{Promoter}_\text{ON}$$
$$\text{Promoter}_\text{ON} \xrightarrow{\alpha} \text{Promoter}_\text{ON} + \text{mRNA}$$
$$\text{mRNA} \xrightarrow{\delta} \emptyset$$

The model has four parameters: $k_\text{on}$, $k_\text{off}$, $\alpha$ (transcription rate when ON), and $\delta$ (mRNA degradation rate).

**Analytical steady-state solution**: The CME for this model can be solved exactly. The mRNA distribution follows a **negative binomial** (or Beta-Binomial, depending on parameterization):

$$P(n) = \binom{n + r - 1}{n} p^r (1-p)^n$$

where $r = k_\text{on}/\delta$ and $p = \delta/(\alpha + \delta)$ (these are the negative binomial parameters). The moments are:

$$\langle n \rangle = \frac{\alpha k_\text{on}}{\delta (k_\text{on} + k_\text{off})}$$

$$\text{Fano} = \frac{\sigma^2}{\langle n \rangle} = 1 + \frac{\alpha k_\text{off}}{\delta (k_\text{on} + k_\text{off})}$$

The Fano factor exceeds 1, quantifying the excess variance due to bursting. The second term is the **burst contribution**: it increases with burst size ($\alpha/\delta$) and promoter OFF fraction ($k_\text{off}/(k_\text{on}+k_\text{off})$). A promoter that is OFF most of the time but highly active when ON will produce large, infrequent bursts — and therefore large Fano factors.

## Limiting Regimes

The parameter space of the two-state model has three qualitatively distinct regimes, each with a characteristic mRNA distribution. Understanding these regimes gives you physical intuition that the general formula alone cannot provide.

**Fast switching limit** ($k_\text{on}, k_\text{off} \gg \delta$): The promoter switches between ON and OFF many times per mRNA lifetime. The mRNA distribution approaches **Poisson** with mean $\alpha k_\text{on} / [\delta(k_\text{on}+k_\text{off})]$. The promoter appears as a single effective state with reduced production rate. This is the regime where averaging is valid — interestingly, it is also the regime that looks most like the constitutive model, even though the promoter is actually switching.

**Slow switching limit** ($k_\text{on}, k_\text{off} \ll \delta$): Many mRNAs are produced and degraded during a single ON episode. The distribution becomes **bimodal**: cells are either in the OFF state (near zero mRNA) or in the ON state (Poisson-distributed mRNA with mean $\alpha/\delta$). The mixing probability is $k_\text{on}/(k_\text{on}+k_\text{off})$. This is the regime where single-cell measurements most differ from bulk averages — the average mRNA level might be 3, but most cells have either ~0 or ~10.

**Burst regime** ($k_\text{off} \gg k_\text{on}$ and $\alpha \gg \delta$): Promoter is mostly OFF; when it turns ON, it rapidly produces many mRNAs before turning OFF again. The mRNA distribution is approximately **geometric** (the discrete analogue of an exponential distribution for burst sizes). In this regime, the mean burst size is $\alpha/(\delta + k_\text{off}) \approx \alpha/k_\text{off}$ and the burst frequency is approximately $k_\text{on}$.

```python
import numpy as np
from scipy.stats import nbinom
import matplotlib.pyplot as plt

def telegraph_moments(k_on, k_off, alpha, delta):
    """Compute mean and Fano for the two-state promoter model."""
    mean = alpha * k_on / (delta * (k_on + k_off))
    fano = 1 + alpha * k_off / (delta * (k_on + k_off))
    return mean, fano

# Analytical negative binomial distribution
def telegraph_distribution(k_on, k_off, alpha, delta, n_max=50):
    """Steady-state mRNA distribution from two-state promoter model."""
    r = k_on / delta
    p_nb = delta / (alpha + delta)
    n = np.arange(n_max)
    probs = nbinom.pmf(n, r, p_nb)
    return n, probs

# Compare three regimes
fig, axes = plt.subplots(1, 3, figsize=(14, 5))
params_list = [
    (1.0, 1.0, 10.0, 1.0, 'Fast switching (Poisson-like)'),
    (0.1, 0.1, 10.0, 1.0, 'Slow switching (bimodal)'),
    (0.1, 1.0, 20.0, 1.0, 'Bursty (geometric-like)'),
]
for ax, (k_on, k_off, alpha, delta, title) in zip(axes, params_list):
    n, probs = telegraph_distribution(k_on, k_off, alpha, delta)
    mean, fano = telegraph_moments(k_on, k_off, alpha, delta)
    ax.bar(n, probs, color='steelblue', alpha=0.8)
    ax.set_title(f'{title}\nMean={mean:.1f}, Fano={fano:.2f}')
    ax.set_xlabel('mRNA count'); ax.set_ylabel('Probability')
plt.tight_layout()
```

## The Protein Distribution

If protein is produced from mRNA at rate $\beta$ and degrades at rate $\gamma$, and if protein lifetime is much longer than mRNA lifetime ($\gamma \ll \delta$), then each mRNA produces a geometric number of proteins before it degrades. The protein distribution becomes a **compound negative binomial** or, in certain limits, a **gamma distribution**.

The key result is that transcriptional bursting produces protein distributions with Fano factor:

$$\text{Fano}_\text{protein} \approx 1 + \frac{\beta}{\gamma} + \frac{\beta}{\gamma} \cdot \frac{\alpha k_\text{off}}{\delta(k_\text{on}+k_\text{off})}$$

where $\beta/\gamma$ is the mean number of proteins per mRNA (translational burst size) and the last term is the transcriptional burst contribution.

This expression is worth parsing: even without transcriptional bursting (setting $k_\text{off} = 0$), the protein Fano factor is $1 + \beta/\gamma > 1$ just from translational bursting — each mRNA produces a random geometric number of proteins before degrading. Transcriptional bursting adds further variance on top. The total protein noise is therefore the sum of two burst contributions, operating at different timescales.

## Connecting to Single-Molecule Data

smFISH (single-molecule fluorescence in situ hybridization) directly counts individual mRNA molecules per cell. Fitting the two-state model to smFISH data extracts $k_\text{on}$, $k_\text{off}$, and $\alpha$, providing physical interpretation of bursting parameters.

In mammalian cells, a typical pattern is:
- Burst size ($\alpha/\delta$): 1–20 mRNAs per burst
- Burst frequency ($k_\text{on}/(k_\text{on}+k_\text{off})$): 0.01–0.5
- Fano factors: 2–50

Chromatin accessibility determines burst frequency: more accessible chromatin allows the transcription machinery to engage more readily, increasing $k_\text{on}$. Enhancer activity and transcription factor binding kinetics also modulate burst frequency. RNAPII pause-release dynamics — how long RNA polymerase stalls near the promoter before elongating — modulate burst size. These mechanistic connections between molecular events and statistical parameters are what make the two-state model genuinely useful rather than merely descriptive.

## Why This Matters

The two-state promoter model is the foundation of quantitative transcriptomics. It underlies the statistical models used to identify differentially expressed genes, to model gene expression in single-cell RNA-seq, and to interpret chromosome conformation data in terms of transcriptional bursting.

It also explains why gene expression is variable: promoter switching is an inherently stochastic process, and unless $k_\text{on}$ and $k_\text{off}$ are both much faster than mRNA turnover, bursty production is unavoidable. Evolution has apparently decided that this noise is acceptable or even beneficial — the diversity it creates in cell populations provides phenotypic flexibility. But for synthetic biology, where predictable expression levels matter for circuit performance, engineering gene circuits with controlled noise levels requires tuning these switching rates. The design principle flows directly from the model: slow switching creates high noise; fast switching (relative to mRNA lifetime) suppresses it. This is not intuitive from verbal reasoning — it emerges from the mathematics of the two-state model, and it has direct applications in designing reliable therapeutic gene circuits.
