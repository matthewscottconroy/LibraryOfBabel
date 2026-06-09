# Chapter 18 — Algorithmic Information Theory

> *Shannon entropy is the entropy of a distribution. Kolmogorov complexity is the entropy of an individual object — defined without any probability. This is the right framework for asking: how complex is this string? This orbit? This theorem?*

**Prerequisites:** Chapter 16 (Shannon entropy), basic computability theory (Turing machines, halting problem).

**What this chapter builds:** Turing machines and computability; Kolmogorov complexity and its universality; the incompressibility method; algorithmic probability (Solomonoff); Martin-Löf randomness as the definitive notion of a random infinite sequence; the connections between algorithmic randomness and dynamical systems.

---

## 18.1 Computability — Background

**Definition 18.1.1.** A *Turing machine* is an abstract device with a finite control (states), an infinite read-write tape, and a transition function. A function $f: \{0,1\}^* \to \{0,1\}^*$ is *computable* if some Turing machine computes it. A set $A \subseteq {\mathbb N}$ is *recursively enumerable (r.e.)* if some Turing machine enumerates it; *decidable* (recursive) if both $A$ and $A^c$ are r.e.

**Theorem 18.1.2 (Halting Problem).** The set $K = \{(M, x) : M\text{ halts on input }x\}$ is r.e. but not decidable. There is no Turing machine that decides, on input $(M, x)$, whether $M$ halts on $x$.

**Theorem 18.1.3 (Rice's Theorem).** Any nontrivial property of the function computed by a Turing machine is undecidable.

---

## 18.2 Kolmogorov Complexity

**Definition 18.2.1.** Fix a universal Turing machine $U$. The *Kolmogorov complexity* (or *plain complexity*) of a binary string $x \in \{0,1\}^*$ is:
$$C(x) = \min\{|p| : U(p) = x\},$$
the length of the shortest program $p$ such that $U$ on input $p$ outputs $x$.

*Interpretation:* $C(x)$ is the minimum number of bits needed to describe $x$ — the shortest "explanation" of $x$.

**Theorem 18.2.2 (Invariance Theorem).** For any universal Turing machine $U'$, the complexity $C_{U'}(x)$ defined using $U'$ differs from $C(x)$ by at most a constant:
$$|C(x) - C_{U'}(x)| \leq c_{U, U'}$$
for some constant depending only on $U$ and $U'$ but not on $x$.

*Proof:* Since $U$ is universal, there is a constant-length description of how to simulate $U'$. So $C(x) \leq C_{U'}(x) + c$. Symmetrically for the other direction.

**Consequence:** The choice of universal Turing machine does not matter up to an additive constant. Kolmogorov complexity is well-defined up to this constant.

**Theorem 18.2.3 (Complexity of Most Strings).** For any $k > 0$:
$$|\{x \in \{0,1\}^n : C(x) < n - k\}| < 2^{n-k}.$$

Almost all strings of length $n$ have complexity $\geq n - O(1)$ — they are *incompressible*.

*Proof:* There are at most $\sum_{j < n-k} 2^j = 2^{n-k} - 1$ programs of length $< n-k$, so at most $2^{n-k}-1$ strings with complexity $< n-k$.

---

## 18.3 Prefix-Free Complexity

**Motivation:** Plain complexity $C$ has an unfortunate property: $C(x,y) \leq C(x) + C(y) + 2\log|x|$ (a $2\log|x|$ overhead). *Prefix-free complexity* $K$ satisfies $K(x,y) \leq K(x) + K(y) + O(1)$.

**Definition 18.3.1.** A *prefix-free machine* is a Turing machine whose domain (set of valid inputs) is a prefix-free set. The *prefix-free Kolmogorov complexity* $K(x)$ is defined using a universal prefix-free machine.

**Theorem 18.3.2 (Kraft Inequality for Complexity).** $\sum_{x \in \{0,1\}^*} 2^{-K(x)} \leq 1$.

This is the analogue of the Kraft inequality for codes: $K$ defines a valid prefix-free code.

**Key Properties:**
- $K(x) \leq |x| + O(1)$ (the identity program is short)
- $K(x) \leq K(y) + K(x|y) + O(1)$ (given $y$, we can describe $x$ from a description of $x$ given $y$)
- $K(x, y) = K(x) + K(y|x) + O(\log K(x))$ (approximate chain rule)
- $K(f(x)) \leq K(x) + K(f) + O(1)$ for computable $f$ — computation cannot increase complexity much

---

## 18.4 Algorithmic Probability

**Definition 18.4.1 (Solomonoff's Universal Distribution).** The *algorithmic probability* of a string $x$ is:
$$\mathbf{m}(x) = \sum_{p : U(p) = x} 2^{-|p|}.$$

By Kraft's inequality, $\sum_x \mathbf{m}(x) \leq 1$.

**Theorem 18.4.2 (Universality of $\mathbf{m}$).** For any computable probability measure $\mu$ on $\{0,1\}^*$, there exists $c > 0$ such that $\mathbf{m}(x) \geq c \cdot \mu(x)$ for all $x$.

In other words, $\mathbf{m}$ *dominates* all computable probability measures — it is a universal prior.

**Connection to $K$:** $K(x) = -\log \mathbf{m}(x) + O(1)$: the Kolmogorov complexity is (essentially) the negative log of the universal prior.

**Solomonoff Induction:** Given a sequence $x_1 x_2 \cdots x_n$, the best prediction for $x_{n+1}$ (in the sense of minimizing expected log-loss) is $\mathbf{m}(x_{n+1} | x_1 \cdots x_n)$. This is the optimal Bayesian prediction over all computable hypotheses.

---

## 18.5 AIT Analogues of Information Theory

**Complexity and Entropy:**
$$H(X) \approx E[K(X)] \quad \text{(up to } O(\log n)\text{ terms)}$$
for a random variable $X$ with finite support.

**Mutual Complexity:**
$$I(x : y) = K(x) + K(y) - K(x, y) + O(\log K(x,y))$$
(algorithmic mutual information — symmetric up to logarithmic terms).

**Complexity-Based Data Processing:**
$$K(f(x) | y) \leq K(x | y) + K(f) + O(1)$$
Computable functions don't increase complexity (conditional on a description of the function).

---

## 18.6 Randomness — The Martin-Löf Approach

**Definition 18.6.1.** A *statistical test* (in Martin-Löf's sense) is a sequence of r.e. open sets $\{U_n\}_{n \geq 1}$ with $P(U_n) \leq 2^{-n}$ for all $n$. A sequence $\omega \in \{0,1\}^{\mathbb N}$ *fails* the test if $\omega \in \bigcap_n U_n$.

**Definition 18.6.2.** A sequence $\omega$ is *Martin-Löf random* (w.r.t. a computable measure $\mu$) if it passes all statistical tests: $\omega \notin \bigcap_n U_n$ for every Martin-Löf test.

**Theorem 18.6.3 (Universal Test).** There exists a *universal Martin-Löf test* $\{V_n\}$ such that $\omega$ is ML-random iff it passes the universal test. The universal test is: $V_n = \{x : K(x_{|n}) < n - n_0\}$ for appropriate $n_0$.

**Theorem 18.6.4 (Schnorr/Levin-Schnorr).** $\omega$ is ML-random w.r.t. the uniform (fair coin) measure iff $K(\omega_{|n}) \geq n - O(1)$ for all $n$ (incompressibility of all initial segments).

*In words: a sequence is random iff no initial segment can be significantly compressed.*

**Theorem 18.6.5 (Van Lambalgen).** $(\omega, \rho)$ is ML-random (as a product sequence) iff $\omega$ is ML-random and $\rho$ is ML-random *relative to $\omega$*.

---

## 18.7 Other Randomness Notions

**Schnorr Randomness:** Defined using computable (not merely r.e.) tests, or equivalently: $\omega$ is Schnorr random iff every computable martingale succeeds on $\omega$ only at a computable rate.

**Computable Randomness:** $\omega$ is computably random iff no computable martingale succeeds on $\omega$.

**Kurtz Randomness:** The weakest notion: $\omega$ is Kurtz random iff it is not in any computable measure-zero set.

**Implications:**
$$\text{ML-random} \Rightarrow \text{Schnorr random} \Rightarrow \text{computably random} \Rightarrow \text{Kurtz random.}$$

None of these implications reverse.

---

## 18.8 Randomness and Dynamical Systems

### 18.8.1 Fouché's Theorem

**Theorem 18.8.1 (Fouché).** Under the coding of orbits of the doubling map $T(x) = 2x \pmod 1$ by binary sequences (itinerary), an initial condition $x \in [0,1]$ is ML-random (with respect to Lebesgue measure) iff its orbit coding is ML-random (as a sequence).

*This establishes the equivalence between ergodic-theoretic randomness (typicality of the orbit) and algorithmic randomness (Martin-Löf randomness of the sequence).*

### 18.8.2 Computable Analysis and Dynamics

**Definition 18.8.2.** An orbit $\{f^n(x)\}_{n \geq 0}$ of a computable dynamical system is *computably describable* if $x$ is computable (there is a Turing machine that outputs better and better rational approximations to $x$).

**Theorem 18.8.3 (Braverman-Yampolsky).** The filled Julia set $\mathcal{K}(f_c)$ is computable (as a subset of ${\mathbb C}$) iff $c$ is computable and $c \notin$ the boundary of the Mandelbrot set (or $c$ is in a specific "computable" part of the boundary).

**Theorem 18.8.4 (Rettinger-Weihrauch).** There exist $c \in \partial\mathcal{M}$ for which $\mathcal{J}(f_c)$ is not computable (as a closed subset of ${\mathbb C}$). Julia sets can be algorithmically random.

---

## 18.9 The Halting Probability Ω

**Definition 18.9.1.** The *Chaitin halting probability* is:
$$\Omega = \sum_{p : U(p) \text{ halts}} 2^{-|p|}.$$

**Theorem 18.9.2.**
1. $\Omega \in (0, 1)$.
2. $\Omega$ is Martin-Löf random.
3. $\Omega$ is computably enumerable (r.e.): one can compute better and better lower bounds.
4. The first $n$ bits of $\Omega$ allow one to decide, for all programs of length $\leq n$, whether they halt.
5. $\Omega$ is incomputable: no Turing machine computes $\Omega$ exactly.

**Remark 18.9.3.** $\Omega$ is the most information-dense number: its first $n$ bits encode the answer to all halting questions for programs of length $\leq n$. It is the "number that contains all mathematical truth" in a precise sense — and it is random.

---

## Exercises

**Exercise 18.1.** Show that $C(x) \leq |x| + O(1)$ for all strings $x$ (the identity program). Show that $C(xx) \leq C(x) + O(\log C(x))$ (two copies can be described once). Can $C(xx) = C(x) - 1$ for some $x$?

**Exercise 18.2.** (Incompressibility) Show that at least $2^n - 1$ strings of length $n$ have $C(x) \geq n$. Exhibit a string of length 1000 with $C(x) \leq 100$ bits.

**Exercise 18.3.** Prove that $K(x) + K(y) = K(x,y) + O(\log K(x))$ (approximate chain rule for prefix-free complexity). (*Hint:* Given descriptions of $x$ and $y$, we can describe $(x,y)$; given $(x,y)$, we need a way to separate the two descriptions.)

**Exercise 18.4.** (AIT and Primes) The $n$-th prime $p_n$ satisfies $K(p_n) \leq \log n + O(1)$ (given $n$, we can compute $p_n$). But most integers $x < n$ have $K(x) \approx \log n$. So the primes are "simple" (low complexity) compared to random integers of the same size. Make this precise.

**Exercise 18.5.** Show that $\Omega$ is ML-random. (*Hint:* Use the Levin-Schnorr theorem: show that $K(\Omega_{|n}) \geq n - O(1)$. If the first $n$ bits of $\Omega$ could be compressed, we could solve more halting problems than $n$ bits should allow.)

**Exercise 18.6.** (Collatz Complexity) The Collatz sequence starting at $n$ has length $\ell(n)$ before reaching 1 (stopping time). What is $K(\ell(n))$ in terms of $K(n)$? What would it mean for the Collatz conjecture if $K(\ell(n)) = K(n) + O(1)$ for all $n$?

**Exercise 18.7.** State Van Lambalgen's theorem precisely. Use it to prove: if $\omega$ is ML-random and $\rho$ is a computable sequence, then $\omega \oplus \rho$ (interleaving) is ML-random.

---

## Chapter Notes

The standard text is Li and Vitányi's *An Introduction to Kolmogorov Complexity and Its Applications* — the most comprehensive reference. Downey and Hirschfeldt's *Algorithmic Randomness and Complexity* is the research-level treatment of randomness notions.

Kolmogorov's three papers from 1965-1968 (translated in *Problems of Information Transmission*) introduced the concept. Chaitin's work (1975) introduced the halting probability $\Omega$ and prefix-free complexity. Martin-Löf's 1966 paper defined the now-standard notion of ML-randomness.

Fouché's theorem (Section 18.8.1) connects ergodic theory and algorithmic randomness: ML-random orbits of the doubling map are exactly the ML-random sequences. This is the precursor to a general theory (Kucera-Gács, Reimann-Slaman) connecting measure-theoretic and algorithmic notions of randomness.

For the connection to dynamical systems: Braverman's paper *Computational Complexity of Euclidean Sets* (2005) and the Rettinger-Weihrauch work on Julia sets are in the computable analysis literature.
