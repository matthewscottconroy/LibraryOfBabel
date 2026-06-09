# 2.8 Measures on Topological Spaces

So far, measures have lived on abstract measurable spaces — just a set with a $\sigma$-algebra. But in dynamical systems, the underlying space is typically a compact metric space or smooth manifold, and we want the measure to be compatible with the topology. This section develops that compatibility and gives the compactness theorem for measures that makes existence proofs work.

When $X$ is a topological space, we want the measure to "see" the topology:

**Definition 2.8.1.** A *Borel measure* on a topological space $X$ is a measure on $(X, \mathcal{B}(X))$. A *Radon measure* is a Borel measure that is finite on compact sets, inner regular (every measurable set is approximated from within by compact sets), and outer regular (approximated from outside by open sets).

Radon measures are the "tame" measures on topological spaces — they interact nicely with both the topology and the measure structure. On compact metric spaces, every Borel probability measure is a Radon measure, so the distinction is mainly relevant in non-compact or non-metrizable settings.

The topology on the space of measures is the *weak topology* (also called the weak-* topology in functional analysis):

**Definition 2.8.2.** A sequence of Borel probability measures $(\mu_n)$ *converges weakly* (or *in the weak-* topology*) to $\mu$ if $\int f\,d\mu_n \to \int f\,d\mu$ for all bounded continuous functions $f$.

Weak convergence is the "right" notion of convergence for probability measures. It doesn't require pointwise convergence of the measures on every measurable set — just convergence of integrals against continuous functions. This is weaker than convergence in total variation, but much more flexible.

The compactness theorem for probability measures is the Prokhorov theorem — the measure-theoretic analog of Arzelà-Ascoli:

**Theorem 2.8.3 (Prokhorov's Theorem).** A family $\{\mu_\alpha\}$ of probability measures on a complete separable metric space $X$ has a weakly convergent subsequence if and only if it is *tight*: for every $\varepsilon > 0$ there is a compact $K \subseteq X$ with $\mu_\alpha(K) > 1 - \varepsilon$ for all $\alpha$.

Tightness is the condition that no mass "escapes to infinity" — every measure in the family puts at least $1-\varepsilon$ of its mass inside a compact set. This is exactly what makes subsequential limits possible: if mass can escape, the sequence might not converge.

**Application in Dynamics.** Prokhorov's theorem is the primary tool for proving existence of invariant measures, and one of the most-used theorems in all of ergodic theory. Here's the argument:

Given a continuous map $f: X \to X$ on a compact metric space and any starting measure $\mu$, form the Cesàro averages:
$$\mu_N = \frac{1}{N}\sum_{n=0}^{N-1} f^n_*\mu.$$

Since $X$ is compact, the sequence $(\mu_N)$ is automatically tight (every measure is supported on $X$). By Prokhorov, there is a weakly convergent subsequence $\mu_{N_k} \to \mu^*$. One can verify that any such limit $\mu^*$ satisfies $f_*\mu^* = \mu^*$ — it's an $f$-invariant probability measure.

This argument — compact space + Cesàro averages + Prokhorov — gives the existence of at least one invariant measure for any continuous map of a compact metric space. It's a beautiful application of the measure-theoretic machinery.
