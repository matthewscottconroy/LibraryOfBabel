# Chapter 25 — Chaos, Randomness, and Computation

> *Positive Lyapunov exponent = exponential divergence of orbits = information production. A chaotic system generates one bit per $1/\lambda$ time units. This is why long-term prediction is impossible and why chaos can be a source of pseudo-randomness.*

**Prerequisites:** Chapters 11 (chaos, Lyapunov exponents), 18 (algorithmic randomness, ML-randomness).

---

## 25.1 Chaos as Information Production

**The Central Identity:** For an ergodic system with KS entropy $h = h_\mu(f)$ and Pesin's formula:
$$h = \sum_{\lambda_i > 0} \lambda_i \quad \text{(sum of positive Lyapunov exponents)}.$$

Each positive Lyapunov exponent $\lambda_i$ contributes information production at rate $\lambda_i$ bits per unit time (using natural log; divide by $\log 2$ for bits).

**Operational Meaning:** A binary description of an orbit of length $T$ with precision $\varepsilon$ requires $\approx hT/\log 2$ bits. Specifying the initial condition to accuracy $\varepsilon e^{-\lambda T}$ costs $hT/\log 2$ bits and predicts the orbit to time $T$ at accuracy $\varepsilon$.

---

## 25.2 Pseudo-Randomness from Chaotic Maps

**The Doubling Map as Generator:** The doubling map $T(x) = 2x \pmod 1$ with $x_0 \in [0,1]$ generates the sequence of bits $b_n = \lfloor 2^n x_0 \rfloor \pmod 2$ — the binary expansion of $x_0$. For Lebesgue-a.e. $x_0$, this is a Martin-Löf random sequence (Section 18.8).

**BUT:** If $x_0$ is computable (rational), the sequence is periodic and NOT random. The "randomness" comes from the initial condition, not from the map.

**Theorem 25.2.1 (Pseudo-Randomness via Chaos).** For the logistic map $f_4(x) = 4x(1-x)$: the orbit $\{f_4^n(x)\}$ with $x$ chosen uniformly has the distribution of i.i.d. arcsine-distributed random variables. The sequence of bits $b_n = \lfloor f_4^n(x) \cdot 2 \rfloor$ is i.i.d. Bernoulli($1/2$) for Lebesgue-a.e. $x$.

---

## 25.3 Algorithmic Randomness of Chaotic Orbits

**Theorem 25.3.1 (Fouché, strengthened).** For the doubling map $T$ and Lebesgue measure:
- A point $x \in [0,1]$ has a Martin-Löf random binary expansion iff the symbolic orbit $(\lfloor 2^n x \rfloor \pmod 2)$ is ML-random as a sequence.
- ML-random points form a set of measure 1.
- Computable initial conditions give periodic or eventually periodic orbits — never random.

**Theorem 25.3.2 (Effective Birkhoff Theorem).** For a computable ergodic system and a computable integrable function $\varphi$:
- If $x$ is ML-random, the time average $\frac{1}{n}\sum_{k<n} \varphi(f^k(x))$ converges to $\int \varphi\,d\mu$ at the ergodic rate.
- The convergence is *effectively computable*: the modulus of convergence is computable from $x$.

---

## 25.4 Undecidability in Dynamical Systems

**Theorem 25.4.1 (Undecidable Properties of Dynamical Systems).** The following problems are undecidable (no algorithm can solve them for all inputs):
1. **Transitivity**: given a computable dynamical system, is it topologically transitive?
2. **Positive entropy**: given a cellular automaton rule, does it have positive topological entropy?
3. **Tiling**: given a finite set of Wang tiles, can they tile the plane?
4. **Emptiness of 2D SFTs**: given a finite set of forbidden patterns, is the corresponding 2D SFT nonempty?

*(These all reduce to the halting problem.)*

**Theorem 25.4.2 (Berger 1966).** The Wang tiling problem is undecidable. As a consequence, there exist *aperiodic* tile sets — sets of Wang tiles that tile the plane but only aperiodically (Berger; Robinson's simpler example).

**Connection to Subshifts:** A set of Wang tiles defines a 2D subshift $X_\tau$. The emptiness problem for $X_\tau$ is $\Pi_1^0$-complete (all computably enumerable sets are involved). 2D SFTs can simulate Turing machines.

---

## 25.5 Computable Analysis and Dynamical Systems

**Definition 25.5.1.** A real number $x \in [0,1]$ is *computable* if there is a Turing machine that, given $n$, outputs a rational $q$ with $|x - q| < 2^{-n}$.

**Definition 25.5.2.** A function $f: [0,1] \to [0,1]$ is *computable* if there is a Turing machine that, given a computable $x$ and $n$, outputs a rational $q$ with $|f(x) - q| < 2^{-n}$.

**Theorem 25.5.3 (Braverman-Yampolsky 2006, 2008).** For the quadratic family $f_c$:
- The Mandelbrot set $\mathcal{M}$ is computable as a subset of ${\mathbb C}$ (given $c$ and $\varepsilon$, one can decide if $c$ is within $\varepsilon$ of $\mathcal{M}$).
- Julia sets $\mathcal{J}(f_c)$ are in general NOT computable on $\partial\mathcal{M}$: there exist $c \in \partial\mathcal{M}$ for which no algorithm can approximate $\mathcal{J}(f_c)$.

---

## 25.6 Computational Complexity of Dynamical Properties

**Lyapunov Exponents:** Computing Lyapunov exponents exactly is $\#P$-hard in general (for piecewise linear maps with rational data). But for generic smooth systems, numerical algorithms converge.

**Entropy:** Computing the topological entropy of a piecewise affine map on the interval is polynomial-time. For 2D piecewise affine maps, it is $\#P$-complete.

**Periodic Points:** Counting periodic points of period $n$ for polynomial maps is $\#P$-complete (related to counting roots of polynomial equations).

---

## Exercises

**Exercise 25.1.** Show that the sequence generated by the doubling map starting at a rational $x_0 = p/q$ (with $\gcd(p,q)=1$ and $q$ odd) is eventually periodic with period equal to the multiplicative order of $2$ modulo $q$. What is the period for $x_0 = 1/3$?

**Exercise 25.2.** (Chaos and Prediction) A chaotic system has Lyapunov exponent $\lambda = 1$ per second. To predict the orbit at time $T = 10$ seconds with accuracy $\varepsilon = 10^{-2}$, what initial condition accuracy $\delta$ is needed? How many bits does specifying $\delta$ require?

**Exercise 25.3.** Describe a 2D Wang tile set that simulates a Turing machine. Show that if the simulated Turing machine halts, the tiling eventually becomes periodic; if it doesn't halt, the tiling is aperiodic.

---

## Chapter Notes

For undecidability of dynamical properties: Kari's *Theory of Cellular Automata* and Lindenmayer-Rozenberg *Automata, Languages, Development* cover the 2D tiling connections. Berger's original paper is *The Undecidability of the Domino Problem* (1966).

For computable analysis and Julia sets: Braverman's thesis and the Rettinger-Weihrauch papers. Weihrauch's *Computable Analysis* provides the general framework.
