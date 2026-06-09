# Chapter 22 — Entropy in Dynamical Systems

> *The entropy of a dynamical system is Shannon entropy applied to the orbit structure. The variational principle says: topological entropy is the maximum, achieved by the measure of maximal entropy. This is the deepest single theorem connecting the two pillars of this book.*

**Prerequisites:** Chapter 7 (ergodic theory, KS entropy), Chapter 12 (symbolic dynamics, topological entropy), Chapter 16 (Shannon entropy, AEP).

---

## 22.1 The Dictionary: Dynamics ↔ Information

| Dynamical Concept | Information-Theoretic Concept |
|---|---|
| Partition $\xi$ of $(X, \mu)$ | Source alphabet |
| KS entropy $h_\mu(f, \xi)$ | Entropy rate of the coded process |
| Generating partition | Sufficient statistics |
| Topological entropy $h_{\text{top}}(f)$ | Max achievable entropy rate |
| Measure of maximal entropy | Capacity-achieving input distribution |
| Variational principle | Duality: channel capacity = max mutual information |
| Ornstein's theorem | Entropy classifies Bernoulli shifts (up to isomorphism) |
| Shannon-McMillan-Breiman | Ergodic AEP (see Chapter 23) |

---

## 22.2 Topological Entropy — Bowen's Definition

**Definition 22.2.1 (Bowen, 1971).** For a continuous map $f$ on a compact metric space $(X, d)$:

A set $E \subseteq X$ is $(n, \varepsilon)$-*separated* if for all distinct $x, y \in E$: $\max_{0 \leq k \leq n-1} d(f^k(x), f^k(y)) > \varepsilon$.

Let $s_n(\varepsilon) = $ maximum size of an $(n, \varepsilon)$-separated set. The *topological entropy* is:
$$h_{\text{top}}(f) = \lim_{\varepsilon \to 0} \limsup_{n\to\infty} \frac{1}{n}\log s_n(\varepsilon).$$

**Theorem 22.2.2.** Bowen's definition agrees with the Adler-Konheim-McAndrew (open cover) definition and with the growth rate of periodic orbits (for Axiom A systems).

**Interpretation:** $s_n(\varepsilon)$ counts how many initial conditions are "distinguishable" by observing $n$ steps of the orbit at resolution $\varepsilon$. The topological entropy is the exponential growth rate of distinguishable orbits — the number of bits per unit time needed to specify an orbit.

---

## 22.3 The Variational Principle

**Theorem 22.3.1 (Variational Principle — Goodwyn 1969, Dinaburg 1970, Bowen 1971).** For a continuous map $f$ on a compact metric space:
$$h_{\text{top}}(f) = \sup_\mu h_\mu(f),$$
where the supremum is over all $f$-invariant Borel probability measures $\mu$.

*(proof sketch)* 
**Upper bound ($h_\mu \leq h_{\text{top}}$):** For any finite partition $\xi$ and invariant measure $\mu$, the information $H(\bigvee_{k=0}^{n-1} f^{-k}\xi)$ is bounded by $\log s_n(\varepsilon)$ for $\varepsilon$ = the minimum partition diameter. Taking the limit gives $h_\mu(f, \xi) \leq h_{\text{top}}(f)$.

**Lower bound (achieved by some $\mu$):** For each $n$, take a maximal $(n,\varepsilon)$-separated set and put a uniform measure on it. The weak limit of the Cesàro averages of these measures is an invariant measure achieving entropy $\geq h_{\text{top}}$.

---

## 22.4 Measures of Maximal Entropy

**Definition 22.4.1.** A *measure of maximal entropy (MME)* is an invariant measure $\mu$ achieving $h_\mu(f) = h_{\text{top}}(f)$.

**Theorem 22.4.2 (Existence and Uniqueness for SFTs).** Every irreducible subshift of finite type has a unique MME — the *Parry measure* (see Section 12.9).

**Theorem 22.4.3 (MMEs for Axiom A Systems — Bowen, Ruelle).** Every Axiom A attractor has a unique MME. It is ergodic and is supported on the closure of periodic orbits.

**Non-uniqueness:** For general continuous maps, multiple MMEs can coexist. For the quadratic family $f_\mu$ at special parameter values, the MME can be supported on a strange attractor.

---

## 22.5 The Pressure Function

**Definition 22.5.1.** For a continuous map $f$ on a compact metric space and a continuous function $\phi: X \to {\mathbb R}$ (potential), the *topological pressure* is:
$$P(f, \phi) = \sup_\mu \left[h_\mu(f) + \int \phi\,d\mu\right].$$

This is the *Legendre transform* of the entropy function.

**Theorem 22.5.2 (Variational Principle for Pressure — Walters).** For an Axiom A system:
$$P(f, \phi) = \lim_{n\to\infty} \frac{1}{n} \log \sum_{\text{Per}_n(f)} \exp\left(\sum_{k=0}^{n-1} \phi(f^k(x))\right),$$
where the sum is over periodic points of period $n$.

**Equilibrium States:** The measure achieving $h_\mu(f) + \int \phi\,d\mu = P(f,\phi)$ is the *equilibrium state for $\phi$*. For $\phi = 0$: the equilibrium state is the MME. For $\phi = -\log|Df|$ (negative Jacobian): the equilibrium state is the SRB measure (Sinai-Ruelle-Bowen).

**Connection to Statistical Mechanics:** In thermodynamics, the free energy $F = U - TS$ (internal energy minus temperature times entropy) is minimized at equilibrium. The pressure function is $-F/T$; the equilibrium state is the Gibbs distribution for potential $\phi$.

---

## 22.6 Pesin's Entropy Formula

**Theorem 22.6.1 (Pesin 1977).** For a $C^2$ diffeomorphism $f$ on a compact manifold preserving a smooth measure $\mu$ (absolutely continuous w.r.t. Lebesgue):
$$h_\mu(f) = \int_X \sum_{\lambda_i > 0} \lambda_i(x)\,d\mu(x),$$
where $\lambda_i(x)$ are the Lyapunov exponents at $x$.

**Interpretation:** The KS entropy equals the total positive Lyapunov exponent — the rate of information production equals the rate of exponential divergence of nearby orbits.

**Ruelle Inequality (general):** For any $C^1$ map and any $f$-invariant measure:
$$h_\mu(f) \leq \int \sum_{\lambda_i > 0} \lambda_i\,d\mu.$$
Equality holds iff $\mu$ is SRB (absolutely continuous on unstable manifolds).

---

## 22.7 Zeta Functions and Thermodynamic Formalism

**Definition 22.7.1.** The *Ruelle zeta function* of a flow is:
$$\zeta_R(s) = \exp\left(\sum_{\gamma \text{ periodic}} \frac{e^{-s \ell(\gamma)}}{|1 - \Lambda_\gamma^{-1}|}\right),$$
where the sum is over prime periodic orbits $\gamma$, $\ell(\gamma)$ is the period, and $\Lambda_\gamma$ is the unstable eigenvalue.

**Theorem 22.7.2.** For Axiom A flows, $\zeta_R(s)$ is meromorphic on ${\mathbb C}$ with poles and zeros related to the spectrum of the transfer operator. The smallest real zero is the topological entropy $h_{\text{top}}$.

This is the dynamical analogue of the Riemann zeta function $\zeta(s) = \prod_p (1-p^{-s})^{-1}$ — the "prime periodic orbits" play the role of rational primes.

---

## Exercises

**Exercise 22.1.** Compute $h_{\text{top}}(f)$ and find the measure of maximal entropy for: (a) the full $k$-shift; (b) the golden mean shift; (c) the cat map on ${\mathbb T}^2$.

**Exercise 22.2.** Verify the variational principle for the doubling map $f: x \mapsto 2x \pmod 1$: show $h_\mu(f) = \log 2$ for Lebesgue measure, and that no other invariant measure achieves higher entropy.

**Exercise 22.3.** (Pressure) For the doubling map with potential $\phi(x) = -t\log 2$, compute $P(f, \phi)$ for all $t \in {\mathbb R}$. Identify the equilibrium state for each $t$.

**Exercise 22.4.** Prove Pesin's formula for the doubling map: the Lyapunov exponent is $\log 2$ and the KS entropy is $\log 2$, and they agree.

---

## Chapter Notes

The variational principle (Section 22.3) is one of the most important theorems in the subject. The original proofs by Goodwyn (1969) and Dinaburg (1970) were followed by Bowen's unified approach (1971). See Walters' *An Introduction to Ergodic Theory* (Chapter 9) for the full proof.

Pesin's entropy formula (Section 22.6) is in Pesin's 1977 paper; the readable modern treatment is in Katok-Hasselblatt Chapter 9 or Mañé's *Ergodic Theory and Differentiable Dynamics*.

The pressure function and equilibrium states (Section 22.5) are Bowen's *Equilibrium States and the Ergodic Theory of Anosov Diffeomorphisms* (1975) — a short book that should be on every dynamicist's shelf.
