# Universal Approximation for Temporal Functionals: The Full Story

## 26.4.1 The Three Universality Theorems

The theoretical foundation of reservoir computing rests on three universality theorems, each proved over a span of fifteen years and each subsuming the previous. This section states all three precisely, derives their relationships, and discusses what they do and do not imply for the practice of reservoir computing.

**Setup.** Throughout, let $\ell^\infty_w(\mathbb{Z}_-)$ be the weighted input space with weight $w$ satisfying $w(0) = 1$, $w(k) \to 0$ as $k \to -\infty$ (Section 26.2). Let $\mathcal{F}_{CTI}$ denote the class of **causal, time-invariant, fading-memory** functionals $F: \ell^\infty_w \to \mathbb{R}$. These are the target functions for approximation.

## 26.4.2 Result 1: Boyd-Chua and Volterra Series Approximation

**Theorem 26.3 (Boyd-Chua 1985).** For any $F \in \mathcal{F}_{CTI}$, any bounded subset $K \subseteq \ell^\infty_w$, and any $\varepsilon > 0$, there exists a finite-order truncated Volterra series

$$
V(u) = v_0 + \sum_{n=1}^P \sum_{k_1, \ldots, k_n = 0}^\infty h_n(k_1, \ldots, k_n) \prod_{j=1}^n u(-k_j),
$$

such that $\sup_{u \in K} |F(u) - V(u)| < \varepsilon$.

*Proof idea.* The key step is to identify the compact domain $K$ (compact by Lemma 26.2 in Section 26.2) and apply the Stone-Weierstrass theorem. The algebra of polynomials in $\{u(-k)\}_{k \geq 0}$ separates points and contains constants, so by Stone-Weierstrass it is dense in $C(K)$. Volterra series are exactly these polynomials. $\square$

**Limitation.** The Volterra series may require infinite order ($P \to \infty$ and infinite-dimensional kernels $h_n$). Boyd-Chua showed that a *finite* truncation suffices only on compact sets; the kernels may still be infinite sequences.

## 26.4.3 Result 2: Sandberg and Finite-Dimensional State Spaces

**Theorem 26.4 (Sandberg 1991).** For any $F \in \mathcal{F}_{CTI}$ and any $\varepsilon > 0$, $F$ can be approximated to within $\varepsilon$ by a finite-dimensional state-space system: there exist $N < \infty$, a map $G: \mathbb{R}^N \to \mathbb{R}$, and a causal map $\Phi: \ell^\infty_w \to \mathbb{R}^N$ (the "state function") such that

$$
\sup_{u \in K} |F(u) - G(\Phi(u))| < \varepsilon.
$$

Moreover, $\Phi$ can be chosen to be the state of a finite-dimensional autonomous system driven by $u$.

*Significance.* Sandberg's theorem is stronger than Boyd-Chua in that it guarantees a *finite-dimensional* approximation. The state $\Phi(u) \in \mathbb{R}^N$ captures the relevant "memory" of the input. This is the mathematical precursor to the echo state: the reservoir state is exactly this finite-dimensional summary of the input history.

**Connection to Boyd-Chua.** Boyd-Chua's Volterra series can always be reorganized into a finite-dimensional state-space form by using the delayed inputs $\{u(-k)\}_{k=0}^K$ as the state (with appropriate $K$). Thus Sandberg's result follows from Boyd-Chua, but Sandberg's proof is more direct and constructive.

## 26.4.4 Result 3: Maass-Sontag and Recurrent Networks

**Theorem 26.5 (Maass & Sontag 1999).** For any $F \in \mathcal{F}_{CTI}$, any compact $K \subseteq \ell^\infty_w$, and any $\varepsilon > 0$, there exists a recurrent neural network with sigmoid activations and a linear readout such that

$$
\sup_{u \in K} |F(u) - F_{\mathrm{RNN}}(u)| < \varepsilon,
$$

where $F_{\mathrm{RNN}}$ is the functional computed by the RNN.

**Corollary 26.6 (ESN Universality [Maass et al. 2002, Jaeger 2001]).** The class of echo state networks (with fixed random reservoir and trained linear readout) is a universal approximator for $\mathcal{F}_{CTI}$, subject to the echo state property.

The corollary follows by noting that the RNN in Theorem 26.5 can be decomposed into a fixed random projection (the reservoir) followed by a linear readout (the trained layer). The Boyd-Chua and Sandberg results establish that the function class is rich enough; the ESP guarantees that the reservoir state is a well-defined function of the input history.

## 26.4.5 The Inclusion Hierarchy

The three results can be organized by generality:

$$
\text{Volterra series} \supseteq \text{Finite-dim. state-space} \supseteq \text{RNN/ESN}
$$

Reading from left to right: any function computable by an RNN/ESN is also expressible as a finite-dimensional state-space system, which is expressible as a Volterra series. Reading from right to left: the Volterra approximation may require infinite order; the state-space approximation requires finite but possibly large $N$; the RNN/ESN requires the additional constraint that the state update takes a specific recurrent form.

The practical implication: the ESN is the most constrained of the three, but it is the one that is easiest to train (no backpropagation through time needed). The universality theorem guarantees that this constraint is not limiting for fading-memory functionals on compact sets.

## 26.4.6 What These Results Do Not Say

It is important to be precise about the limitations of the universality theorems:

**1. No rate of approximation.** The theorems guarantee existence of an approximating ESN but do not specify how large $N$ must be to achieve accuracy $\varepsilon$. The approximation may require $N = O(\varepsilon^{-1/\alpha})$ for some problem-dependent $\alpha$. Sharp approximation rates are established only in special cases (see Section 26.5 on approximation rates).

**2. No learnability guarantee.** Universality is about *existence* of an approximating network, not about *finding* it from data. The statistical learning bounds of Chapter 28 are needed to understand when ridge regression on a finite dataset produces a solution that generalizes.

**3. Compact sets only.** All three theorems hold on compact subsets $K$ of the input space. For inputs not in $K$, there is no guarantee of approximation. In practice, the training distribution must cover the test distribution for the bound to be meaningful.

**4. Fixed-point vs. fading memory.** The results apply to functionals with fading memory with respect to *some* weight $w$. A functional with infinite memory (e.g., a recurrent system with purely imaginary eigenvalues, which sustains oscillations indefinitely) is not in $\mathcal{F}_{CTI}$ and cannot be approximated by an ESN.

## 26.4.7 The Approximation-Learnability Gap

The juxtaposition of universality (expressiveness) with the sample complexity bounds of Chapter 28 reveals an important gap. Universality tells us: for any $F \in \mathcal{F}_{CTI}$ and any $\varepsilon > 0$, there exists an ESN with $N$ neurons that approximates $F$ to within $\varepsilon$. Statistical learning theory tells us: to learn the readout of this ESN from $T$ training examples, we need

$$
T \gtrsim \frac{N}{\varepsilon}\log\frac{N}{\varepsilon}
$$

examples (VC bound). As $\varepsilon \to 0$, both $N$ and $T$ grow — but at what rate? If $N(\varepsilon) \sim \varepsilon^{-\beta}$ for the optimal approximation, then $T(\varepsilon) \sim \varepsilon^{-\beta-1}\log(\varepsilon^{-1})$. The exponent $\beta$ depends on the smoothness of $F$ and the architecture of the approximating network — a topic studied in approximation theory but not resolved in full generality for reservoir computing.

This gap is not a defect of reservoir computing; it appears for any function approximation scheme. But it means that theoretical guarantees of approximation (universality) should not be conflated with practical guarantees of learning (sample complexity). The two are distinct — and both are needed for a complete theoretical picture.

## References

- Boyd, S. and Chua, L. O. (1985). Fading memory and the problem of approximating nonlinear operators with Volterra series. *IEEE Transactions on Circuits and Systems*, 32(11), 1150–1161.
- Maass, W., Natschläger, T., and Markram, H. (2002). Real-time computing without stable states: A new framework for neural computation based on perturbations. *Neural Computation*, 14(11), 2531–2560.
- Maass, W. and Sontag, E. D. (1999). Analog neural nets with Gaussian or other common noise distributions cannot recognize arbitrary regular languages. *Neural Computation*, 11(3), 771–782.
- Sandberg, I. W. (1991). Approximation theorems for discrete-time systems. *IEEE Transactions on Circuits and Systems*, 38(5), 564–566.
- Sandberg, I. W. (2001). Notes on uniform approximation of time-varying systems on finite time intervals. *IEEE Transactions on Circuits and Systems I*, 48(4), 500–504.
