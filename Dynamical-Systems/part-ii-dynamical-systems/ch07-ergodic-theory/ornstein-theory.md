# 7.8 Ornstein Theory

In 1970, Donald Ornstein proved one of the deepest theorems in ergodic theory. The theorem says: among Bernoulli shifts, entropy is a *complete* invariant. Two Bernoulli shifts with the same entropy are measurably isomorphic.

This is remarkable. Bernoulli shifts can be built from completely different alphabets and probability distributions. The shift on $\{H, T\}^{\mathbb Z}$ with fair coin measure and the shift on $\{1, 2, 3, 4, 5, 6\}^{\mathbb Z}$ with the distribution $p_i = 1/6$ are both Bernoulli shifts with entropy $\log 2$ and $\log 6$ respectively. But two shifts with the same entropy $\log 2$ — regardless of how different their alphabets look — are measurably isomorphic. Entropy is the only thing that matters.

**Theorem 7.8.1 (Ornstein, 1970).** Two Bernoulli shifts with the same entropy are measurably isomorphic.

This is a profound theorem: entropy is a *complete* invariant for the Bernoulli shifts. Despite having the same abstract description (product spaces), Bernoulli shifts with different entropy $H(p) = -\sum p_i \log p_i$ are non-isomorphic.

*(proof outline)* The key ingredient is the notion of *$\bar{d}$-distance* (distribution distance) between two processes. One shows that given two Bernoulli shifts with equal entropy and $\varepsilon > 0$, one can find a "good" way to compare their orbits that makes them $\varepsilon$-close in $\bar{d}$. This is done via the *Finitary Ornstein theorem* and careful matching of names.

The $\bar{d}$-metric is the key invention. Given two processes (two systems with a common time axis), the $\bar{d}$-distance measures how well you can couple the processes so that their orbits agree on a large fraction of time. Ornstein showed that if two Bernoulli shifts have the same entropy, you can always find a coupling where the orbits agree on almost all of the time. That's the isomorphism.

---

## The Bernoulli Class

Ornstein's theorem identifies which systems are "really" Bernoulli — not just Bernoulli shifts, but measurably isomorphic to Bernoulli shifts.

**Definition 7.8.2.** A process $(X, f, \mu)$ is *Bernoulli* if it is measurably isomorphic to a Bernoulli shift.

**Theorem 7.8.3.** The following systems are Bernoulli:
- All Bernoulli shifts (by definition)
- Anosov diffeomorphisms of compact manifolds (Sinai)
- The geodesic flow on surfaces of constant negative curvature (Ornstein-Weiss)
- Billiards in convex domains (Chernov-Sinai)

**Corollary 7.8.4.** Two Anosov diffeomorphisms with the same Lyapunov exponent sums are measurably isomorphic (even though they may be topologically distinct).

The corollary is striking. The cat map and a "perturbed" version of the cat map, if they have the same sum of positive Lyapunov exponents (and hence the same KS entropy by Pesin's formula), are measurably isomorphic. They are dynamically identical from the measure-theoretic point of view, even if their topological structure looks different. Ergodic theory cannot distinguish them.

This is both a theorem and a warning. Ornstein's theory shows that the measure-theoretic classification is coarser than the topological one. Two systems can be topologically inequivalent but ergodically the same. The right invariant to use depends on what question you're asking.

In the final section of this chapter, we develop joinings — the framework that makes it possible to compare two different dynamical systems precisely.
