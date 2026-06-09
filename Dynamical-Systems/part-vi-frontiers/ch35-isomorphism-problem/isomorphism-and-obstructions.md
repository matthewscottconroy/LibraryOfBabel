# 35.1 Isomorphism and Its Obstructions

When are two measure-preserving systems the same? The formal definition is conjugacy: there should be a measure-preserving bijection that intertwines the dynamics. But the real question is whether we can tell, from some computable or classifiable invariant, when two systems are conjugate.

**Definition 35.1.1.** Two MPTs $(X, \mu, T)$ and $(Y, \nu, S)$ are *isomorphic* (conjugate) if there is a measure-space isomorphism $\phi: X \to Y$ with $\phi \circ T = S \circ \phi$ a.e.

**Definition 35.1.2.** A *complete invariant* for a class $\mathcal{C}$ of MPTs is a Borel function $I: \mathcal{C} \to \mathcal{I}$ (to some standard Borel space $\mathcal{I}$) with $T \cong S \iff I(T) = I(S)$.

The prototype is entropy for Bernoulli shifts.

**Ornstein's Theorem (Complete Invariant for Bernoulli Shifts).** Entropy $h: \text{Bernoulli} \to [0, \infty]$ is a complete invariant for Bernoulli shifts (among all free ergodic MPTs). This is the central theorem of Chapter 7.

Entropy is computable, it takes values in $[0, \infty]$, and it perfectly separates Bernoulli shifts. This is exactly what a complete invariant should be.

**The Problem:** Does a complete invariant exist for all ergodic MPTs?

The answer is no, but making "no" precise requires the machinery of descriptive set theory. The isomorphism relation on ergodic systems is a complicated subset of $\text{Aut}(X, \mu) \times \text{Aut}(X, \mu)$, and we can ask: how complex is this set in the Borel hierarchy? If it were Borel — or at least not too complex — we'd expect a reasonably simple invariant to exist. The Foreman-Rudolph-Weiss theorem says the complexity is maximal.
