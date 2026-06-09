# Chapter 7 — Ergodic Theory

> *Time averages equal space averages — when the system is ergodic. This is not a physical intuition but a theorem, and the proof reveals the mathematical structure underlying statistical mechanics, information theory, and number theory.*

**Prerequisites:** Chapters 2 (measure theory, $L^p$ spaces, conditional expectation), 6 (topological dynamics, invariant measures).

**What this chapter builds:** Measure-preserving transformations as the central object; the Poincaré Recurrence Theorem in measure; the Birkhoff and von Neumann Ergodic Theorems; ergodicity and mixing as spectral properties of the Koopman operator; the Kolmogorov-Sinai entropy as the measure-theoretic invariant; and Ornstein's classification theorem.

---

## 7.1 Measure-Preserving Transformations

### 7.1.1 Definitions

**Definition 7.1.1.** A *measure-preserving transformation (MPT)* is a quadruple $(X, \mathcal{B}, \mu, f)$ where $(X, \mathcal{B}, \mu)$ is a probability space and $f: X \to X$ is measurable with $\mu(f^{-1}(A)) = \mu(A)$ for all $A \in \mathcal{B}$.

If $f$ is invertible (bijective, with $f^{-1}$ measurable), the system is an *invertible MPT* or an *automorphism*.

**Definition 7.1.2.** Two MPTs $(X, \mathcal{B}, \mu, f)$ and $(Y, \mathcal{C}, \nu, g)$ are *measurably isomorphic* if there exists a measure-preserving bijection $\varphi: (X, \mu) \to (Y, \nu)$ with $\varphi \circ f = g \circ \varphi$ $\mu$-a.e.

### 7.1.2 Standard Examples

**Example 7.1.3 (Circle Rotation).** $X = {\mathbb T} = {\mathbb R}/{\mathbb Z}$, $\mu$ = Lebesgue measure, $R_\alpha(x) = x + \alpha \pmod{1}$.
- $\mu(R_\alpha^{-1}(A)) = \mu(A + \alpha) = \mu(A)$ (Lebesgue measure is translation-invariant).

**Example 7.1.4 (Doubling Map).** $X = [0,1]$, $\mu$ = Lebesgue, $f(x) = 2x \pmod{1}$.
- Check: $f^{-1}([a,b]) = [a/2, b/2] \cup [(a+1)/2, (b+1)/2]$, which has measure $b-a = \mu([a,b])$.

**Example 7.1.5 (Bernoulli Shift).** $X = \{0, 1, \ldots, k-1\}^{\mathbb Z}$ (bi-infinite sequences), $\mu = p^{\otimes {\mathbb Z}}$ (product measure with weights $p_0, \ldots, p_{k-1}$), $\sigma(x)_n = x_{n+1}$ (left shift).
- The Bernoulli shift is the fundamental model of an independent process.

**Example 7.1.6 (Toral Automorphism).** $X = {\mathbb T}^2$, $\mu$ = Lebesgue, $f_A: (x,y) \mapsto (2x+y, x+y) \pmod{1}$ where $A = \begin{pmatrix} 2 & 1 \\ 1 & 1 \end{pmatrix} \in SL(2,{\mathbb Z})$. Since $\det(A) = 1$, $f_A$ preserves area (Lebesgue measure).

**Example 7.1.7 (Gauss Map).** $X = [0,1]$, $\mu_G = \frac{1}{\ln 2} \frac{dx}{1+x}$ (Gauss measure), $G(x) = \{1/x\}$ (fractional part of $1/x$). This models the continued fraction expansion of $x$.

---

## 7.2 Poincaré Recurrence Theorem

**Theorem 7.2.1 (Poincaré Recurrence).** Let $(X, \mathcal{B}, \mu, f)$ be an MPT and $A \in \mathcal{B}$ with $\mu(A) > 0$. Then $\mu$-a.e. point $x \in A$ returns to $A$ infinitely often: for a.e. $x \in A$, the set $\{n \geq 1 : f^n(x) \in A\}$ is infinite.

*(proof)* Let $B = \{x \in A : f^n(x) \notin A \text{ for all } n \geq 1\}$ (the set of points that never return). The sets $B, f^{-1}(B), f^{-2}(B), \ldots$ are pairwise disjoint:
if $x \in f^{-m}(B) \cap f^{-n}(B)$ with $m < n$, then $f^m(x) \in B$, so $f^{n-m}(f^m(x)) = f^n(x) \notin B$ (since $f^m(x)$ never visits $A$), but then $f^n(x) \notin B$, contradiction. Since all $f^{-k}(B)$ have the same measure (MPT) and are disjoint, and $\mu(X) = 1$, we need $\sum_k \mu(f^{-k}(B)) = \sum_k \mu(B) \leq 1$, forcing $\mu(B) = 0$. So a.e. point of $A$ returns at least once. Applying this to $f^n(A)$ for each $n$ shows infinitely many returns.

**Remark 7.2.2.** The measure-theoretic version is much stronger than the topological Poincaré theorem: it says a.e. point (not just some point) returns to every set of positive measure.

---

## 7.3 The Ergodic Theorems

### 7.3.1 Von Neumann's Mean Ergodic Theorem

**Theorem 7.3.1 (Von Neumann, 1931).** Let $(X, \mathcal{B}, \mu, f)$ be an MPT. For $\varphi \in L^2(\mu)$, the time averages $A_N \varphi = \frac{1}{N}\sum_{n=0}^{N-1} \varphi \circ f^n$ converge in $L^2$ to the projection $P\varphi$ onto the closed subspace of $f$-invariant functions: $\{g \in L^2 : g \circ f = g \text{ a.e.}\}$.

*(proof)* The Koopman operator $U_f: L^2 \to L^2$, $U_f(\varphi) = \varphi \circ f$, is an isometry (since $\mu$ is $f$-invariant) and hence unitary if $f$ is invertible. The theorem reduces to showing $(1/N)\sum_{n=0}^{N-1} U_f^n$ converges strongly to the orthogonal projection onto $\ker(U_f - I)$. This follows from the spectral theory: $P$ is the spectral projection onto eigenvalue $1$.

### 7.3.2 Birkhoff's Pointwise Ergodic Theorem

**Theorem 7.3.2 (Birkhoff, 1931).** Let $(X, \mathcal{B}, \mu, f)$ be an MPT and $\varphi \in L^1(\mu)$. Then for $\mu$-a.e. $x$, the time averages converge:
$$\lim_{N \to \infty} \frac{1}{N} \sum_{n=0}^{N-1} \varphi(f^n(x)) = \varphi^*(x),$$
where $\varphi^* \in L^1(\mu)$ satisfies $\varphi^* \circ f = \varphi^*$ a.e. (invariance) and $\int \varphi^*\,d\mu = \int \varphi\,d\mu$.

*(proof sketch)* The hard part is the a.e. convergence. The key tool is the *Maximal Ergodic Theorem*: $\mu\{x : \sup_N A_N \varphi(x) > \alpha\} \leq \frac{1}{\alpha}\int_{\{\sup A_N \varphi > \alpha\}} \varphi\,d\mu$. From this, one shows the set where $\limsup A_N \varphi > \liminf A_N \varphi$ has measure zero, using a "truncation and approximate" argument (Riesz's sunrise lemma).

**Key observation:** If $f$ is ergodic (Definition 7.4.1), then $\varphi^*$ is constant a.e., equal to $\int \varphi\,d\mu$. This is the precise statement that time averages equal space averages.

**Applications:**
- Normal numbers: $x \in [0,1]$ is *normal in base 2* if the density of 1s in its binary expansion is $1/2$. Birkhoff's theorem applied to the doubling map and $\varphi = \mathbf{1}_{[1/2,1]}$ gives: Lebesgue a.e. $x$ is normal in base 2.
- The Borel-Cantelli lemma follows from Birkhoff applied to suitable characteristic functions.

---

## 7.4 Ergodicity

### 7.4.1 Definition and Characterizations

**Definition 7.4.1.** An MPT $(X, \mathcal{B}, \mu, f)$ is *ergodic* if every $f$-invariant set has measure 0 or 1: $f^{-1}(A) = A$ implies $\mu(A) \in \{0, 1\}$.

**Theorem 7.4.2 (Equivalences for Ergodicity).** The following are equivalent:
1. $(X, \mathcal{B}, \mu, f)$ is ergodic
2. The only $f$-invariant functions in $L^1(\mu)$ are constants a.e.
3. For $\mu$-a.e. $x$, the orbit of $x$ equidistributes: $\frac{1}{N}\sum_{n=0}^{N-1} \varphi(f^n(x)) \to \int \varphi\,d\mu$ for all $\varphi \in L^1$
4. The Koopman operator $U_f$ has $1$ as a simple eigenvalue
5. For all $A, B \in \mathcal{B}$ with positive measure: $\frac{1}{N}\sum_{n=0}^{N-1} \mu(f^{-n}(A) \cap B) \to \mu(A)\mu(B)$ (Cesàro mixing)

**Example 7.4.3.**
- Irrational rotation $R_\alpha$ is ergodic. Proof: if $f \in L^2$ is $R_\alpha$-invariant, its Fourier coefficients $\hat{f}(k) = \int f(x) e^{-2\pi i kx} dx$ satisfy $\hat{f}(k) = e^{2\pi i k\alpha} \hat{f}(k)$, so $\hat{f}(k) = 0$ for all $k \neq 0$ (since $e^{2\pi i k\alpha} \neq 1$ for $k \neq 0$, $\alpha \notin {\mathbb Q}$). Hence $f$ is constant.
- Doubling map $f(x) = 2x \pmod{1}$ is ergodic. Same Fourier argument: $\hat{f}(k) = \hat{f}(2k)$ for all $k$, so $\hat{f}(k) = \hat{f}(2^n k) \to 0$ for $k \neq 0$.

**Theorem 7.4.4 (Ergodic Decomposition).** Every MPT $(X, \mathcal{B}, \mu, f)$ decomposes as an integral over ergodic measures: $\mu = \int \mu_x\,d\mu(x)$, where each $\mu_x$ is an ergodic $f$-invariant measure and $\mu_x = \mu_y$ whenever $y$ is in the orbit of $x$.

---

## 7.5 Mixing

**Definition 7.5.1.** An ergodic MPT $(X, \mathcal{B}, \mu, f)$ is:
- *Weakly mixing* if for all $A, B \in \mathcal{B}$: $\frac{1}{N}\sum_{n=0}^{N-1} |\mu(f^{-n}(A) \cap B) - \mu(A)\mu(B)| \to 0$
- *Strongly mixing* (or just *mixing*) if for all $A, B \in \mathcal{B}$: $\mu(f^{-n}(A) \cap B) \to \mu(A)\mu(B)$ as $n \to \infty$

Mixing $\Rightarrow$ weak mixing $\Rightarrow$ ergodic.

**Spectral Characterizations:**
- $f$ is ergodic iff $1$ is a simple eigenvalue of $U_f$
- $f$ is weakly mixing iff $U_f$ has no eigenvalue other than $1$ (equivalently, $U_f$ has purely continuous spectrum on $L^2_0 = \{g \in L^2 : \int g = 0\}$)
- $f$ is mixing iff $\langle U_f^n g, h \rangle \to 0$ for all $g, h \in L^2_0$ (matrix elements of $U_f^n$ tend to 0)

**Example 7.5.2 (Mixing and Nonmixing).**
- Bernoulli shifts are mixing: $\mu(\sigma^{-n}(A) \cap B) \to \mu(A)\mu(B)$ since distant coordinates are independent.
- Irrational rotation $R_\alpha$ is NOT mixing: take $A = B = [0, \varepsilon]$. Along the subsequence where $n\alpha$ is close to 0, $\mu(R_\alpha^{-n}(A) \cap A) \approx \mu(A)^2 \cdot 1/\mu(A) = \mu(A)$, not $\mu(A)^2$.
- The Chacon system is weakly mixing but not mixing.

---

## 7.6 The Koopman Operator and Spectral Theory

**Definition 7.6.1.** The *Koopman operator* of $(X, \mathcal{B}, \mu, f)$ is $U_f: L^2(\mu) \to L^2(\mu)$, $U_f \varphi = \varphi \circ f$.

For an invertible MPT, $U_f$ is unitary: $U_f^* = U_f^{-1} = U_{f^{-1}}$.

**Definition 7.6.2.** The *spectral measure* of $\varphi \in L^2$ is the Borel measure $\sigma_\varphi$ on $S^1$ defined by $\widehat{\sigma_\varphi}(n) = \langle U_f^n \varphi, \varphi \rangle = \int \varphi \circ f^n \cdot \bar{\varphi}\,d\mu$.

The spectral type of $f$ is the equivalence class of measures $\sigma_\varphi$ (under mutual absolute continuity) as $\varphi$ ranges over a cyclic vector.

**Theorem 7.6.3 (Spectral Isomorphism).** Two MPTs with the same spectral type (as abstract unitary operators) are *spectrally isomorphic* — they have the same eigenvalues, spectral measures, and mixing properties. However, spectral isomorphism is weaker than measurable isomorphism.

**Examples of Spectra:**
- Irrational rotation $R_\alpha$: pure point spectrum $\{e^{2\pi i n\alpha} : n \in {\mathbb Z}\}$ — all eigenvalues with eigenfunctions $e^{2\pi i n x}$.
- Bernoulli shift: Lebesgue spectrum — every cyclic component is spectrally equivalent to Lebesgue measure on $S^1$.
- Anosov diffeomorphisms: Lebesgue spectrum (with multiplicity).

---

## 7.7 Entropy

### 7.7.1 Partitions and Information

**Definition 7.7.1.** A *measurable partition* $\xi = \{A_1, \ldots, A_k\}$ of $(X, \mu)$ is a finite collection of disjoint measurable sets with $\mu(\bigcup_i A_i) = 1$.

**Definition 7.7.2.** The *information function* of $\xi$ is $I(\xi)(x) = -\log \mu(A_i)$ for $x \in A_i$.

The *entropy* of $\xi$ is $H(\xi) = \int I(\xi)\,d\mu = -\sum_i \mu(A_i) \log \mu(A_i)$ (Shannon entropy of the distribution $(\mu(A_1), \ldots, \mu(A_k))$).

**Definition 7.7.3.** The *join* $\xi \vee \eta$ of two partitions is the finest partition coarser than both: $\{A_i \cap B_j : \mu(A_i \cap B_j) > 0\}$.

**Definition 7.7.4.** The *conditional entropy* of $\xi$ given $\eta$ is $H(\xi | \eta) = H(\xi \vee \eta) - H(\eta)$.

### 7.7.2 The Kolmogorov-Sinai Entropy

**Definition 7.7.5.** For an MPT $(X, \mathcal{B}, \mu, f)$ and partition $\xi$, the *entropy of $f$ with respect to $\xi$* is
$$h(f, \xi) = \lim_{N \to \infty} \frac{1}{N} H\left(\bigvee_{n=0}^{N-1} f^{-n}\xi\right).$$

The limit exists by subadditivity of entropy: $H(\xi \vee f^{-1}\xi \vee \cdots \vee f^{-(N-1)}\xi) \leq N \cdot H(\xi)$.

*Interpretation:* $h(f, \xi)$ measures the average information gained per iterate about which element of $\xi$ the orbit visits.

**Definition 7.7.6 (Kolmogorov-Sinai Entropy).** The *metric entropy* of $(X, \mathcal{B}, \mu, f)$ is
$$h_\mu(f) = \sup_\xi h(f, \xi)$$
where the supremum is over all finite measurable partitions.

**Theorem 7.7.7 (Sinai's Generator Theorem).** If $\xi$ is a *generating partition* (i.e., $\bigvee_{n=-\infty}^{\infty} f^{-n}\xi = \mathcal{B}$ mod $\mu$), then $h_\mu(f) = h(f, \xi)$.

The generator theorem makes entropy computable: one need not take the supremum over all partitions.

**Theorem 7.7.8 (Entropy of Basic Examples).**
- Irrational rotation: $h_\mu(R_\alpha) = 0$ (any partition generates entropy $\to 0$)
- Bernoulli shift $(p_0, \ldots, p_{k-1})$: $h_\mu(\sigma) = -\sum_i p_i \log p_i = H(p)$ (Shannon entropy)
- Toral automorphism $f_A$: $h_\mu(f_A) = \sum_{\lambda > 1} \log |\lambda|$ (sum of logs of expanding eigenvalues)

**Proof for Bernoulli:** The partition $\xi = \{[0], [1], \ldots, [k-1]\}$ (by the 0-th coordinate) is a generator. The atoms of $\bigvee_{n=0}^{N-1} \sigma^{-n}\xi$ are cylinder sets of length $N$, each with measure $p_{i_0} p_{i_1} \cdots p_{i_{N-1}}$. By independence (product measure): $H(\bigvee_{n<N} \sigma^{-n}\xi) = N \cdot H(\xi) = N \cdot (-\sum p_i \log p_i)$.

---

## 7.8 Ornstein Theory

**Theorem 7.8.1 (Ornstein, 1970).** Two Bernoulli shifts with the same entropy are measurably isomorphic.

This is a profound theorem: entropy is a *complete* invariant for the Bernoulli shifts. Despite having the same abstract description (product spaces), Bernoulli shifts with different entropy $H(p) = -\sum p_i \log p_i$ are non-isomorphic.

*(proof outline)* The key ingredient is the notion of *$\bar{d}$-distance* (distribution distance) between two processes. One shows that given two Bernoulli shifts with equal entropy and $\varepsilon > 0$, one can find a "good" way to compare their orbits that makes them $\varepsilon$-close in $\bar{d}$. This is done via the *Finitary Ornstein theorem* and careful matching of names.

**Definition 7.8.2.** A process $(X, f, \mu)$ is *Bernoulli* if it is measurably isomorphic to a Bernoulli shift.

**Theorem 7.8.3.** The following systems are Bernoulli:
- All Bernoulli shifts (by definition)
- Anosov diffeomorphisms of compact manifolds (Sinai)
- The geodesic flow on surfaces of constant negative curvature (Ornstein-Weiss)
- Billiards in convex domains (Chernov-Sinai)

**Corollary 7.8.4.** Two Anosov diffeomorphisms with the same Lyapunov exponent sums are measurably isomorphic (even though they may be topologically distinct).

---

## 7.9 Joinings

**Definition 7.9.1.** A *joining* of two MPTs $(X, \mu, f)$ and $(Y, \nu, g)$ is an $(f \times g)$-invariant measure $\lambda$ on $X \times Y$ with marginals $\mu$ and $\nu$.

The product measure $\mu \otimes \nu$ is always a joining ("independence"). Other joinings capture correlations between the systems.

**Theorem 7.9.2 (Furstenberg).** $(X, f)$ and $(Y, g)$ are disjoint (the only joining is the product) if and only if... (one is weakly mixing and the other has singular spectrum, or similar conditions). Disjointness is the strongest possible "independence" between two systems.

**Application:** Joinings are the natural language for expressing that two dynamical systems are "independent." Furstenberg used joinings to prove his multiple recurrence theorem (the ergodic-theoretic foundation for Szemerédi's theorem on arithmetic progressions in dense sets).

---

## Exercises

**Exercise 7.1.** Prove that the Gauss map $G(x) = \{1/x\}$ preserves the Gauss measure $\mu_G = \frac{1}{\ln 2} \frac{dx}{1+x}$. (*Hint:* Compute $\mu_G(G^{-1}([a,b]))$ directly.)

**Exercise 7.2.** Prove the Mean Ergodic Theorem in the following form: if $U$ is a unitary operator on a Hilbert space $H$, then $\frac{1}{N}\sum_{n=0}^{N-1} U^n \varphi \to P\varphi$ in $H$, where $P$ is the orthogonal projection onto $\ker(U - I)$.

**Exercise 7.3.** Let $(X, \mu, f)$ be ergodic. Show that for any $\varphi, \psi \in L^2(\mu)$:
$$\lim_{N \to \infty} \frac{1}{N} \sum_{n=0}^{N-1} \langle U_f^n \varphi, \psi \rangle = \langle \varphi, 1 \rangle \langle 1, \psi \rangle = \left(\int \varphi\right)\left(\int \psi\right).$$

**Exercise 7.4.** (Ergodicity of the Doubling Map) Prove that the doubling map $f(x) = 2x \pmod{1}$ is ergodic with respect to Lebesgue measure, using Fourier analysis on $[0,1]$.

**Exercise 7.5.** (Entropy Computation) Compute the KS entropy of the $\frac{1}{3}$-$\frac{2}{3}$ Bernoulli shift (where $p_0 = 1/3$, $p_1 = 2/3$). Compare to the KS entropy of the fair coin shift ($p_0 = p_1 = 1/2$).

**Exercise 7.6.** Show that $h_\mu(f^n) = n \cdot h_\mu(f)$ for any MPT $f$ and $n \geq 1$.

**Exercise 7.7.** (Ornstein connection) The Arnold cat map $A = \begin{pmatrix} 2 & 1 \\ 1 & 1 \end{pmatrix}$ on ${\mathbb T}^2$ has eigenvalues $\lambda = \frac{3 \pm \sqrt{5}}{2}$. Compute the KS entropy using Pesin's formula. Conclude that the cat map is Bernoulli.

**Exercise 7.8.** Prove the Poincaré Recurrence Theorem directly from Birkhoff's theorem: if $\mu(A) > 0$, apply Birkhoff to $\varphi = \mathbf{1}_A$ and show $\varphi^*(x) = \mu(A) > 0$ for a.e. $x \in A$, which implies infinitely many returns.

---

## Chapter Notes

Ergodic theory has two classical texts: Walters' *An Introduction to Ergodic Theory* is accessible and complete; Cornfeld-Fomin-Sinai's *Ergodic Theory* is the classical Soviet treatment. The modern research-level text is Einsiedler-Ward's *Ergodic Theory with a View Towards Number Theory* — outstanding for the connections to number theory and Ratner's theorem.

The Birkhoff Ergodic Theorem (1931) is the true starting point of the subject. The original proof used the "sunrise lemma" (a geometric argument with a one-dimensional flavor). Modern proofs use the maximal ergodic theorem; see Halmos' *Lectures on Ergodic Theory* for the cleanest version.

Ornstein's theorem (Section 7.8) is one of the deepest results in the subject. The key new technique was the $\bar{d}$-metric on processes, which allows one to compare orbits of different systems. See Ornstein's *Ergodic Theory, Randomness, and Dynamical Systems* for the full development.

The connection to information theory runs deep: the KS entropy (Section 7.7) is the dynamical-systems version of Shannon entropy, and the Shannon-McMillan-Breiman theorem (Chapter 23) makes this precise as an "ergodic AEP."
