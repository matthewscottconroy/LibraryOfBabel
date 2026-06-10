# When NVAR Outperforms ESN and When It Does Not

## The Dimensional Scaling Argument

The core tradeoff between NVAR and ESN is determined by the dimensionality of the input and the degree of the polynomial expansion. For a $d$-dimensional input with $s$ delays (total lagged input dimension $ds$) and degree-$p$ polynomial expansion, the NVAR feature dimension is:

$$D_{\text{NVAR}}(d, s, p) = \binom{ds + p}{p}.$$

For degree 2:

$$D_{\text{NVAR}}(d, s, 2) = \frac{(ds)(ds+1)}{2} + ds = \frac{ds(ds+3)}{2} \approx \frac{(ds)^2}{2} \text{ for large } ds.$$

The feature dimension grows quadratically in $ds$. For $d = 3, s = 2$: $D = 27$ (manageable). For $d = 10, s = 5$: $D = \frac{50 \times 53}{2} = 1325$ (still tractable). For $d = 100, s = 5$: $D = \frac{500 \times 503}{2} = 125{,}750$ (large but solvable by ridge regression). For $d = 1000, s = 5$: $D \approx 1.25 \times 10^9$ (intractable) [Gauthier et al. 2021].

For degree 3, the feature dimension grows cubically:

$$D_{\text{NVAR}}(d, s, 3) \approx \frac{(ds)^3}{6},$$

making degree-3 NVAR impractical for $d > 20$ with $s > 3$. The curse of dimensionality is severe for NVAR, and this is its primary limitation.

## When NVAR Wins

NVAR outperforms ESN in the following scenarios:

**Low-dimensional inputs ($d \leq 10$).** The polynomial feature dimension remains manageable, all relevant nonlinearities are captured explicitly, and the ESN's random projection wastes capacity on irrelevant directions.

**Lorenz-type attractor prediction.** Systems whose dynamics are governed by polynomial differential equations (Lorenz, Rössler, van der Pol, FitzHugh-Nagumo) have quadratic or low-degree polynomial structure that NVAR captures exactly. The ESN must approximate this structure with random projections, requiring substantially more parameters for equivalent accuracy [Gauthier et al. 2021].

**Interpretability requirements.** The NVAR feature vector has a clear physical meaning: each feature is a specific monomial of delayed inputs. When understanding which combinations of past states drive the prediction is important (scientific applications, regulatory contexts), NVAR's explicit features provide insight that the ESN cannot.

**Reproducibility and determinism.** NVAR produces identical results across runs, independent of random seed. In applications where run-to-run reproducibility is required, NVAR's determinism is a significant practical advantage.

**Limited data.** The NVAR has fewer parameters than a comparably performing ESN, and therefore overfits less with small training sets. For $T \lesssim 1000$ and $d \leq 5$, NVAR's parameter efficiency often translates to better generalization.

## When ESN Wins

**High-dimensional inputs ($d \gg 10$).** The ESN projects a $d$-dimensional input into an $N$-dimensional reservoir state using a dense random matrix — cost $O(Nd)$ per step. NVAR would require $O((ds)^p)$ features — exponentially more for large $d$. For $d = 100$ and $N = 500$, ESN is vastly more parameter-efficient than degree-2 NVAR [Bollt 2021].

**Partially observed systems.** When only some state variables are observed, NVAR cannot reconstruct the full polynomial structure of the dynamics. The ESN, with its implicit memory and nonlinear mixing, can partially compensate through delay embedding. NVAR with delays can also implement the Takens embedding, but its polynomial features are less flexible than the ESN's random projections for high-dimensional embedded inputs.

**Non-polynomial temporal dependencies.** Tasks involving long-range temporal structure (e.g., paragraph-level language modeling, multi-step sequence recall) require memory that extends beyond the polynomial expansion of $s$ delayed samples. The ESN's recurrent dynamics provide implicit all-past memory; NVAR's explicit delay window has a hard cutoff at $s$ steps.

**Spiking or discrete inputs.** For binary or integer-valued inputs (spike trains, token sequences), polynomial products of delayed inputs may not be the natural feature class. The ESN treats spiking inputs through the dynamics of its analog reservoir neurons, providing richer processing.

## Summary Table

| Criterion | NVAR Preferred | ESN Preferred |
|-----------|----------------|---------------|
| Input dimension | $d \leq 10$ | $d > 20$ |
| Polynomial dynamics | Yes (Lorenz-type) | No |
| Interpretability | Required | Not required |
| Training data | $T \lesssim 1000$ | $T \gg 1000$ |
| Long-range memory | Short (polynomial) | Long (recurrent) |
| Reproducibility | Required | Acceptable |

The random reservoir's noise is sometimes a feature: the diversity of random projections provides coverage of the input space that systematic polynomial features miss. NVAR's determinism and parsimony are advantages precisely when the polynomial structure of the dynamics is known or suspected; when the relevant features are unknown, randomness as a prior can outperform structured but possibly mismatched polynomial features [Bollt 2021].

---

## References

- Gauthier, D. J., Bollt, E., Griffith, A., & Barbosa, W. A. S. (2021). Next generation reservoir computing. *Nature Communications*, 12(1), 5564.
- Bollt, E. (2021). On explaining the surprising success of reservoir computing forecaster of chaos? The universal machine learning dynamical system with contrast to VAR and DMD. *Chaos*, 31(1), 013108.
