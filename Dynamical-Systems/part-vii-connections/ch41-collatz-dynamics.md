# Chapter 41 — The Collatz Conjecture as a Dynamical System

> *Take any positive integer. If even, halve it. If odd, triple it and add one. Repeat. The conjecture: you always reach 1. This is the simplest unsolved problem in mathematics — and it is a problem about the ergodic theory of a number-theoretic dynamical system.*

**Prerequisites:** Chapter 6 (topological dynamics), Chapter 7 (ergodic theory), Chapter 31 (ergodic number theory), Chapter 27 (computability and dynamics).

---

## 41.1 The Collatz Map: Definitions and Basic Properties

**Definition 41.1.1.** The *Collatz map* (also called the $3x+1$ map) on ${\mathbb N}$ is:
$$C(n) = \begin{cases}n/2 & \text{if } n \equiv 0 \pmod 2 \\ 3n+1 & \text{if } n \equiv 1 \pmod 2\end{cases}$$

The *Collatz conjecture*: for every $n \in {\mathbb N}^+$, there exists $k$ with $C^k(n) = 1$.

**Verified range:** All $n \leq 2^{68}$ have been verified computationally (as of 2023).

**Definition 41.1.2.** The *stopping time* $\sigma(n) = \min\{k : C^k(n) < n\}$ (first time the orbit drops below the starting value).

**Definition 41.1.3.** The *total stopping time* $\tau(n) = \min\{k : C^k(n) = 1\}$.

**Theorem 41.1.4 (Terras, 1976).** For almost all $n$ (in the density sense):
$$\sigma(n) \sim \frac{\log n}{\log(4/3)}.$$

The stopping time grows logarithmically — orbits reach below $n$ in about $\log n / \log(4/3)$ steps.

---

## 41.2 2-Adic Extension and Ergodicity

**Definition 41.2.1.** The *2-adic integers* ${\mathbb Z}_2 = \varprojlim {\mathbb Z}/2^n{\mathbb Z}$ is the completion of ${\mathbb Z}$ with respect to the 2-adic absolute value $|n|_2 = 2^{-v_2(n)}$ (where $v_2(n)$ is the 2-adic valuation).

As a set: ${\mathbb Z}_2 = \{x = \sum_{i=0}^\infty a_i 2^i : a_i \in \{0,1\}\}$ (formal power series in $2$).

**Theorem 41.2.2 (2-Adic Extension of $C$).** The Collatz map $C$ extends to a continuous map $C: {\mathbb Z}_2 \to {\mathbb Z}_2$ given by:
$$C(x) = \begin{cases}x/2 & \text{if } x \equiv 0 \pmod 2 \\ (3x+1)/2 & \text{if } x \equiv 1 \pmod 2\end{cases} \quad (\text{accelerated map})$$

The *accelerated* version $(3x+1)/2$ (combining two steps for odd $n$) is smoother in ${\mathbb Z}_2$.

**Theorem 41.2.3 (Ergodicity of $C$ on ${\mathbb Z}_2$).** The accelerated Collatz map $\tilde{C}: {\mathbb Z}_2 \to {\mathbb Z}_2$ is measure-preserving and ergodic with respect to Haar measure on ${\mathbb Z}_2$.

*(sketch)* The Haar measure on ${\mathbb Z}_2$ is the unique probability measure invariant under translation. The map $\tilde{C}$ locally expands by factor $3/2$ on odd elements and contracts by $1/2$ on even — on average, $\tilde{C}$ is "neutral." The ergodicity is proved using the spectral gap of the associated Markov operator.

**Interpretation:** Ergodicity of $C$ on ${\mathbb Z}_2$ means that a "random" 2-adic integer has an orbit that visits every measurable set with the correct frequency. The Collatz conjecture for positive integers asks about the behavior of the *positive* integers within ${\mathbb Z}_2$ — a measure-zero subset.

---

## 41.3 Statistical Model of the Collatz Map

**The Heuristic:** Consider the orbit $n, C(n), C^2(n), \ldots$. If consecutive bits of $n$ are "random" (i.i.d. Bernoulli(1/2)):
- Each step: halve with probability 1/2 (if even), apply $3n+1$ with probability 1/2 (if odd)
- After one step: $E[C(n)] \approx n/2 \cdot 1/2 + (3n+1)/2 \cdot 1/2 \approx n \cdot 3/4$

Since $3/4 < 1$: each step reduces $n$ by factor $3/4$ on average. So orbits should reach 1 in $O(\log n)$ steps.

**Definition 41.3.1.** The *stochastic process* model replaces $C$ by a random walk on $(0,\infty)$: $X_0 = \log n$, $X_{t+1} = X_t + \varepsilon_t$ where $\varepsilon_t \in \{-\log 2, +\log(3/2)\}$ i.i.d. with equal probability.

Since $E[\varepsilon_t] = \frac{1}{2}(-\log 2 + \log(3/2)) = \frac{1}{2}\log(3/4) < 0$, the random walk has a negative drift: $X_t \to -\infty$ a.s. This supports the Collatz conjecture probabilistically.

**Theorem 41.3.2 (Wirsching, 1998).** Under the stochastic model, the time to reach 0 (corresponding to reaching 1) is:
$$\tau \approx \frac{\log n}{|\log(3/4)|/2} = \frac{2\log n}{\log(4/3)}.$$

This matches the observed empirical stopping times.

---

## 41.4 Tao's Progress (2022)

**Theorem 41.4.1 (Tao, 2022).** Almost all Collatz orbits attain almost bounded values. More precisely, for any function $f: {\mathbb N} \to {\mathbb R}$ with $f(n) \to \infty$ (however slowly):
$$\#\{n \leq N : \min_k C^k(n) \leq f(n)\} = (1 + o(1))N.$$

In particular: for almost all $n$ (in the density sense), the orbit of $n$ eventually reaches a value $\leq f(n)$ for any unbounded function $f$.

**Proof Strategy:**
1. *Syracuse function*: Work with the map $S: 2{\mathbb Z}+1 \to 2{\mathbb Z}+1$ (odd numbers only), $S(n) = (3n+1)/2^{v_2(3n+1)}$
2. *$p$-adic analysis*: Track the 2-adic valuation of iterates
3. *Fourier analysis on ${\mathbb Z}_2$*: Use the circle of 2-adic characters to bound exponential sums
4. *Sieve theory*: Control the density of orbits not reaching small values
5. *Probabilistic argument*: Convert the Fourier bounds to density statements

**The Gap from Full Conjecture:** Tao's theorem says orbits reach "small values" (any $f(n) \to \infty$) but not necessarily "1." The full conjecture requires reaching exactly 1. The final step — showing orbits do not "escape to infinity" — remains open.

---

## 41.5 Connections to Other Dynamical Systems

### 41.5.1 Connections to Number Theory

**Theorem 41.5.1 (Connection to $p$-adic Dynamics).** The Collatz map is a special case of a $(p,q)$-Collatz-type map: $C(n) = n/q$ if $q|n$, else $C(n) = pn+r$. The $3x+1$ map is $(p,q) = (3,2)$. These are related to Mahler's $p$-adic measure theory.

**Connection to Automatic Sequences:** A sequence $a_n$ is *$k$-automatic* if it is computed by a $k$-state automaton reading the base-$k$ expansion of $n$. The parity sequence of Collatz orbits — $b_n = C^n(m) \pmod 2$ — is not automatic (it's "too complex"). This is evidence that Collatz is not a simple recurrence.

### 41.5.2 Connections to Symbolic Dynamics

**Definition 41.5.2.** The *Collatz graph* $\mathcal{G}$ has vertices ${\mathbb N}$ and edges $n \to C(n)$. The conjecture says $\mathcal{G}$ has a unique absorbing strongly connected component $\{1, 2, 4\}$ (the $1 \to 2 \to 1$ loop and $4 \to 2 \to 1 \to 4$ cycle, with $4 \to 2 \to 1$ the absorbing path).

**Theorem 41.5.3 (Symbolic Representation).** Encode the Collatz orbit of $n$ by the sequence $\omega_n = (\omega_0, \omega_1, \ldots)$ where $\omega_k = C^k(n) \pmod 2$. This is a binary sequence (the parity sequence). The Collatz conjecture is equivalent to: every parity sequence is eventually periodic with the unique period $(0, 0)$ (the cycle $1 \to 2 \to 1$) or $(1, 1, 0)$ (the cycle $1 \to 4 \to 2 \to 1$).

---

## 41.6 Open Research Directions

**Direction 41.6.1 (Spectral Gap of Collatz Operator).** The transfer operator $\mathcal{L}f(n) = \sum_{C(m)=n} f(m)$ on $\ell^2({\mathbb N})$ or on $L^2({\mathbb Z}_2)$ may have a spectral gap, which would give polynomial convergence of orbits — a key step toward the conjecture.

**Direction 41.6.2 (Random Matrix Methods).** Model the Collatz map as a random matrix: $C = M_0 \cdot 1/2 + M_1 \cdot 3/2$ where $M_0, M_1$ alternate randomly. Products of random matrices (Furstenberg theory) have well-defined Lyapunov exponents: $\lambda = \frac{1}{2}\log(1/2) + \frac{1}{2}\log(3/2) = \frac{1}{2}\log(3/4) < 0$. This rigorizes the heuristic that orbits shrink on average.

**Direction 41.6.3 (Connections to the Riemann Hypothesis).** Various authors have noted numerological connections between Collatz-type sums and $L$-functions. These are mostly speculative but point toward deeper number-theoretic structure.

---

## Exercises

**Exercise 41.1.** Compute the Collatz orbit of $n = 27$: find the stopping time $\sigma(27)$ and the total stopping time $\tau(27)$. What is the maximum value reached?

**Exercise 41.2.** (2-Adic Extension) Express $n = 5$ as a 2-adic integer $x = 1 + 0 \cdot 2 + 1 \cdot 4 + \cdots$. Apply $\tilde{C}$ (the accelerated map) and verify it matches applying $C$ twice to $n = 5$ (since $5$ is odd).

**Exercise 41.3.** (Stochastic Model) Simulate 1000 random walks on $(0, \infty)$ starting at $\log 100$ with steps $\pm\log(3/2)$ or $-\log 2$ (each with probability 1/2). What fraction hit 0 within 100 steps? Compare to $\tau(n)$ for actual Collatz orbits starting near $100$.

**Exercise 41.4.** (Research) Read Tao's 2022 paper abstract. The key tool is "exponential sum estimates on ${\mathbb Z}_2$." Describe what an exponential sum on ${\mathbb Z}_2$ is, and why bounding it gives control over Collatz orbit densities.

---

## Chapter Notes

Lagarias's survey: *The $3x+1$ Problem and its Generalizations* (AMS Monthly, 1985) and the comprehensive collection *The Ultimate Challenge: The $3x+1$ Problem* (AMS, 2010) edited by Lagarias — the definitive reference. Contains reprints of all the key papers.

Tao's 2022 result: *Almost all orbits of the Collatz map attain almost bounded values* (Forum of Mathematics, Pi, 2022). A blog post by Tao explains the proof strategy accessibly.

The 2-adic formulation is in Bernstein-Lagarias's *The 3x+1 Conjugacy Map* (Canadian J. Math., 1996). The connection to ergodic theory is developed in Wirsching's *The Dynamical System Generated by the $3n+1$ Function* (Springer LNM 1681, 1998).
