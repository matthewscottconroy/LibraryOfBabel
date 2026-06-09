# 9.6 SRB Measures

Not all invariant measures for a hyperbolic system are "physically relevant." A hyperbolic attractor can have uncountably many invariant measures — one for each invariant probability on the symbolic system. But most of these measures are "exotic": they describe statistical behavior that no physical observer would ever see.

The physically relevant measure is the one seen by Lebesgue-typical initial conditions. This is the SRB measure.

Not all invariant measures for a hyperbolic system are "physically relevant." The SRB measures are the ones seen by Lebesgue-typical initial conditions.

**Definition 9.6.1.** For a diffeomorphism $f$ with a hyperbolic attractor $\Lambda$, a measure $\mu$ is an *SRB measure* (Sinai-Ruelle-Bowen, or *physical measure*) if for Lebesgue-a.e. $x$ in the basin of attraction:
$$\frac{1}{N} \sum_{n=0}^{N-1} \varphi(f^n(x)) \to \int \varphi\,d\mu \quad \text{for all continuous } \varphi.$$

This is exactly the Birkhoff ergodic theorem, but with "Lebesgue-a.e." instead of "$\mu$-a.e." The SRB measure describes the statistics seen by an observer who picks initial conditions uniformly (i.e., from Lebesgue measure) in the basin, not from the invariant measure itself. This is the physically natural choice: you don't know the invariant measure before you start the experiment.

**Theorem 9.6.2 (Sinai-Ruelle-Bowen).** Every Axiom A attractor has a unique SRB measure. The SRB measure:
- Is ergodic
- Is absolutely continuous on unstable manifolds (but may be singular w.r.t. Lebesgue)
- Satisfies Pesin's formula: $h_\mu(f) = \sum_{\lambda_i > 0} \lambda_i$

The theorem was proved independently by Sinai (1972), Ruelle (1976), and Bowen-Ruelle (1975). It's one of the central results of smooth ergodic theory.

*SRB measure characterization:* $\mu$ is SRB iff it satisfies Pesin's formula AND it has absolutely continuous conditional measures on unstable manifolds.

The "absolutely continuous on unstable manifolds" condition is the geometric characterization of the SRB measure. On stable manifolds, the SRB measure can be singular (concentrated on a fractal-like set). But on unstable manifolds — where the dynamics expands — the SRB measure must be absolutely continuous. This is the measure-theoretic signature of the physical relevance: the invariant measure is smeared out along the expanding directions in the right way.

**Example 9.6.3.** For linear toral automorphisms: the SRB measure is Lebesgue measure (since the system preserves Lebesgue measure and is ergodic, Lebesgue is both SRB and the unique ergodic measure).

For the cat map, Lebesgue measure is the SRB measure because the cat map preserves area. For non-area-preserving hyperbolic attractors (like the Lorenz attractor), the SRB measure is singular with respect to Lebesgue but still describes the long-run statistics of Lebesgue-typical orbits.

The existence and uniqueness of SRB measures is what makes hyperbolic dynamics tractable for physics: there's a canonical "natural" measure associated to each hyperbolic attractor, and it's the one any physical experiment will converge to.
