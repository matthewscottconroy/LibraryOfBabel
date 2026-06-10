# Fading Memory as a Banach Space Topology

## 26.2.1 The Problem of Infinite History

A reservoir computing system processes an input sequence $u: \mathbb{Z}_- \to \mathbb{R}$ (where $\mathbb{Z}_- = \{\ldots, -2, -1, 0\}$ denotes the non-positive integers) and produces an output $y = F(u)$. The functional $F$ maps infinite histories to real outputs. This formulation raises an immediate technical difficulty: how do we define continuity and approximation for functionals on infinite-dimensional spaces?

The standard $\ell^\infty$ topology is too coarse: it treats a perturbation at $t = -10^{10}$ as equally significant as one at $t = 0$. Physical systems have **fading memory**: distant past inputs matter less than recent ones. To capture this, we equip the input space with a topology that down-weights the distant past.

## 26.2.2 The Weighted Sequence Space

**Definition 26.4 (Weight Sequence).** A **weight sequence** is a function $w: \mathbb{Z}_- \to (0, \infty)$ satisfying:
1. $w(0) = 1$ (normalization)
2. $w(k) \to 0$ as $k \to -\infty$ (fading)
3. $w$ is non-increasing: $w(k) \geq w(k-1)$ for all $k \leq 0$

**Definition 26.5 (Weighted Sequence Space).** The weighted sequence space is

$$
\ell^\infty_w(\mathbb{Z}_-) = \left\{u: \mathbb{Z}_- \to \mathbb{R} \;\middle|\; \|u\|_w := \sup_{k \leq 0} |u(k)|\, w(-k) < \infty\right\}.
$$

The norm $\|u\|_w = \sup_{k \leq 0} |u(k)|\, w(-k)$ penalizes past values by $w(-k)$, which decreases as $k \to -\infty$. Recent values ($k$ near $0$) are weighted by $w(0) = 1$; ancient values ($k \ll 0$) are weighted by $w(-k) \approx 0$.

**Proposition 26.1.** $(\ell^\infty_w(\mathbb{Z}_-), \|\cdot\|_w)$ is a Banach space.

*Proof.* Completeness follows from the completeness of $\ell^\infty$ and the fact that the weight $w$ is bounded below by $0$. The norm axioms are straightforward to verify. $\square$

## 26.2.3 The Fading Memory Property

**Definition 26.6 (Fading Memory).** A functional $F: \ell^\infty_w(\mathbb{Z}_-) \to \mathbb{R}$ has the **fading memory property** (FMP) with respect to weight $w$ if $F$ is continuous in the $\|\cdot\|_w$ norm: for any $u \in \ell^\infty_w$ and $\varepsilon > 0$, there exists $\delta > 0$ such that

$$
\|u - v\|_w < \delta \implies |F(u) - F(v)| < \varepsilon.
$$

Equivalently, $F$ has the FMP iff for any $u$ and any sequence $\{v_n\}$ with $\|u - v_n\|_w \to 0$, we have $F(u) = \lim_n F(v_n)$.

**Interpretation.** The $\|\cdot\|_w$ norm is small when $u$ and $v$ agree on recent inputs even if they disagree on distant past inputs (since $w(-k) \to 0$ suppresses the effect of distant differences). Thus, $F$ has the FMP iff the output $F(u)$ is insensitive to perturbations of the distant past — the physical content of "fading memory."

**Example.** The linear filter $F(u) = \sum_{k=0}^\infty h(k) u(-k)$ has the FMP with respect to weight $w$ if the filter kernel $h$ decays faster than $w$: $|h(k)| \leq C w(k)$ for some $C < \infty$. In this case, $|F(u) - F(v)| \leq C \|u-v\|_w \sum_k w(k)/w(k) = C\|u-v\|_w \cdot \text{const}$.

## 26.2.4 Compactness: The Tychonoff Argument

The key property of the weighted topology is compactness of bounded sets. Let $\mathcal{B}_w = \{u \in \ell^\infty_w : \|u\|_w \leq 1\}$ be the unit ball in $\ell^\infty_w$.

**Lemma 26.2 (Compactness of Bounded Sets).** The set

$$
K = \{u \in \ell^\infty : |u(k)| \leq 1/w(-k) \text{ for all } k \leq 0\}
$$

is compact in the **pointwise convergence topology** on $\mathbb{R}^{\mathbb{Z}_-}$.

*Proof.* Each coordinate $u(k)$ ranges over the interval $[-1/w(-k), 1/w(-k)]$, which is compact. By Tychonoff's theorem, the product space $\prod_{k \leq 0} [-1/w(-k), 1/w(-k)]$ is compact in the product topology. The set $K$ is closed in the product topology, hence compact. $\square$

This compactness is the key tool in the Boyd-Chua proof (Section 26.3). The Stone-Weierstrass theorem requires a compact Hausdorff domain; the Tychonoff lemma provides this domain.

## 26.2.5 Connection to Topological Dynamical Systems

The fading memory framework can be rephrased in terms of topological dynamical systems. Consider the **shift map** $\sigma: \mathbb{R}^{\mathbb{Z}_-} \to \mathbb{R}^{\mathbb{Z}_-}$ defined by $(\sigma u)(k) = u(k-1)$. An input sequence $u$ generates the orbit $\{u, \sigma u, \sigma^2 u, \ldots\}$, and a time-invariant functional $F$ is constant along these orbits: $F(\sigma^t u) = F_t(u)$ where $F_t$ is the output at time $t$.

The set $K$ from Lemma 26.2 is **$\sigma$-invariant** (invariant under the shift): $\sigma(K) \subseteq K$. The pair $(K, \sigma)$ is a compact topological dynamical system. Functionals on $K$ that are continuous in the pointwise topology are exactly the fading-memory functionals.

**Causal time-invariant functionals.** A functional $F$ is:
- **Causal:** $F(u)$ depends only on $\{u(k): k \leq 0\}$ (no future inputs)
- **Time-invariant:** $F(\sigma^{-t} u)$ does not depend explicitly on $t$ (same rule at all times)

The class of causal, time-invariant, fading-memory functionals on $\ell^\infty_w$ is the natural domain of approximation theorems for reservoir computing (see Section 26.4).

## 26.2.6 Why the Weighted Topology is the Right Choice

One might ask: why use $\|\cdot\|_w$ rather than, say, $\ell^2$ or $\ell^1$ topologies? The answer is two-fold:

**Physical argument.** Physical systems with exponentially stable autonomous dynamics forget their initial conditions at exponential rates: $\|x(t) - x'(t)\| \leq C e^{-\lambda t} \|x(0) - x'(0)\|$ for $\lambda > 0$. For such systems, $u(-k)$ affects the output at time $0$ through a factor $e^{-\lambda k}$. Choosing $w(k) = e^{-\lambda k}$ matches the weighting to the actual forgetting rate of the system.

**Mathematical argument.** The weighted $\ell^\infty$ topology is the weakest topology for which bounded sets are compact (Lemma 26.2), which is the minimum requirement for applying the Stone-Weierstrass theorem. Stronger topologies (e.g., $\ell^2$) impose additional regularity that may not be satisfied by physically reasonable inputs.

## 26.2.7 Echo States in the Banach Framework

The **echo state property** can be characterized in the Banach space language. An ESN with reservoir map $T: \mathbb{R}^N \times \mathbb{R} \to \mathbb{R}^N$ has the ESP if the echo function

$$
E: u \mapsto \lim_{t \to \infty} T^t(\mathbf{x}_0, u(-t), u(-t+1), \ldots, u(0))
$$

exists and is independent of $\mathbf{x}_0 \in \mathbb{R}^N$. In Banach space terms: the ESP means that the functional $F(u) = E(u)$ is well-defined as a map $\ell^\infty_w \to \mathbb{R}^N$.

**Continuity (Strict ESP).** The **strict ESP** [Jaeger 2001] requires additionally that $E: \ell^\infty_w \to \mathbb{R}^N$ is continuous in $\|\cdot\|_w$. This is precisely the fading memory property of the echo function. A reservoir satisfies the strict ESP iff the echo function is a fading-memory functional.

## References

- Boyd, S. and Chua, L. O. (1985). Fading memory and the problem of approximating nonlinear operators with Volterra series. *IEEE Transactions on Circuits and Systems*, 32(11), 1150–1161.
- Jaeger, H. (2001). *The "echo state" approach to analysing and training recurrent neural networks*. GMD Technical Report 148.
- Rudin, W. (1991). *Functional Analysis*. 2nd ed. McGraw-Hill.
- Tychonoff, A. N. (1930). Über die topologische Erweiterung von Räumen. *Mathematische Annalen*, 102, 544–561.
