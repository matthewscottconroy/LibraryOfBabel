# The Strict Echo State Property

## 29.5.1 Motivation: From Measurable to Continuous

The (standard) echo state property, as developed in Section 29.2, guarantees that the echo function $E_t: u \mapsto \mathbf{x}^*(t,u)$ is well-defined and measurable for $\mathbb{P}$-almost every input sequence $u$. This is sufficient for many theoretical results: ridge regression converges, training is initialization-independent, and time averages of the readout equal ensemble averages.

However, measurability is not sufficient for the most important application of the echo state: approximating a **fading-memory functional**. The Boyd-Chua theorem (Section 26.3) requires continuity of the approximating functional, not merely measurability. A reservoir whose echo function is merely measurable but not continuous may not implement a fading-memory approximation — it may produce wildly different outputs for input sequences that are close in the weighted norm $\|\cdot\|_w$.

This motivates the **strict echo state property** [Jaeger 2001]: a strengthening of the ESP that requires the echo function to be continuous.

## 29.5.2 Definition of the Strict ESP

**Definition 29.5 (Strict Echo State Property).** A driven reservoir satisfies the **strict echo state property** (strict ESP) with respect to weight sequence $w$ if:

1. The reservoir satisfies the (standard) ESP: the echo function $E_t: u \mapsto \mathbf{x}^*(t,u)$ is well-defined.

2. The echo function is **continuous** in the weighted norm: for each $t$, the map $E_t: (\ell^\infty_w, \|\cdot\|_w) \to (\mathbb{R}^N, \|\cdot\|_2)$ is continuous.

Equivalently, for any $\varepsilon > 0$, there exists $\delta > 0$ such that

$$
\|u - v\|_w < \delta \implies \|E_t(u) - E_t(v)\|_2 < \varepsilon.
$$

**Interpretation.** The strict ESP means: if two input sequences are close in the weighted norm (i.e., they agree on recent inputs even if they disagree on distant past), then the corresponding echo states are close. The reservoir state is a *continuous* function of the recent input history.

## 29.5.3 Why Strictness Is Required for Fading Memory

**Theorem 29.5 (Strict ESP is Necessary for Fading Memory).** A reservoir readout $y = \mathbf{w}^T E_t(u)$ has the **fading memory property** (FMP) with respect to $w$ only if the reservoir satisfies the strict ESP with respect to $w$.

*Proof.* The readout $y = f(u) = \mathbf{w}^T E_t(u)$ is a composition of $\mathbf{w}^T$ (which is continuous as a linear functional on $\mathbb{R}^N$) and $E_t$. For $f$ to be continuous in $\|\cdot\|_w$ (FMP), $E_t$ must also be continuous in $\|\cdot\|_w$ (since $\|\mathbf{w}^T(\mathbf{x} - \mathbf{y})\| \leq \|\mathbf{w}\|\|\mathbf{x}-\mathbf{y}\|$, continuity of $E_t$ implies continuity of $f$; and conversely, if $E_t$ is discontinuous, there exist $u_n \to u$ with $E_t(u_n) \not\to E_t(u)$, so we can choose $\mathbf{w}$ to detect this discontinuity). $\square$

**Corollary 29.6.** If a reservoir does not satisfy the strict ESP, it cannot approximate any fading-memory functional to arbitrary accuracy — regardless of the readout.

This corollary shows that the strict ESP is a *necessary* condition for the Boyd-Chua approximation theorem to apply. It is the bridge between the dynamical systems condition (ESP) and the functional analysis condition (fading memory).

## 29.5.4 The Lipschitz Echo Function

A quantitative strengthening of the strict ESP requires the echo function to be not just continuous but **Lipschitz**: there exists $L > 0$ such that

$$
\|E_t(u) - E_t(v)\|_2 \leq L\|u - v\|_w \quad \text{for all } u, v \in \ell^\infty_w. \tag{29.1}
$$

The Lipschitz constant $L$ controls the sensitivity of the reservoir state to input perturbations.

**Theorem 29.7 (Sufficient Condition for Lipschitz Echo Function [Buehner & Young 2006]).** If the reservoir map $F: \mathbb{R}^N \times \mathcal{U} \to \mathbb{R}^N$ satisfies a **uniform contractivity condition**: for all $\mathbf{x}, \mathbf{x}' \in \mathbb{R}^N$ and all $u \in \mathcal{U}$,

$$
\|F(\mathbf{x}, u) - F(\mathbf{x}', u)\|_2 \leq \gamma \|\mathbf{x} - \mathbf{x}'\|_2, \quad \gamma < 1,
$$

then the reservoir satisfies the strict ESP with a Lipschitz echo function, and the Lipschitz constant satisfies

$$
L \leq \frac{\|W^{\mathrm{in}}\|_{\mathrm{op}}}{1 - \gamma},
$$

where $\|W^{\mathrm{in}}\|_{\mathrm{op}}$ is the operator norm of the input weight matrix.

*Proof sketch.* The contractivity condition implies $\|E_t(u) - E_t(v)\|_2 \leq \sum_{k=0}^\infty \gamma^k \|W^{\mathrm{in}}(u(-k) - v(-k))\|_2$. If $w(k) = \gamma^k$ (exponential weight), then $\sum_k \gamma^k |u(-k) - v(-k)| \leq \|u-v\|_w \cdot \|W^{\mathrm{in}}\|_\mathrm{op}$, giving (29.1) with $L = \|W^{\mathrm{in}}\|_\mathrm{op}/(1-\gamma)$. $\square$

For a tanh-ESN: $F(\mathbf{x}, u) = \tanh(W^{\mathrm{rec}}\mathbf{x} + W^{\mathrm{in}}u)$. Since $|\tanh'(z)| \leq 1$, the Lipschitz constant of $F$ in $\mathbf{x}$ is $\gamma = \|W^{\mathrm{rec}}\|_\mathrm{op}$. For the strict ESP, we require $\|W^{\mathrm{rec}}\|_\mathrm{op} < 1$.

**Remark.** $\|W^{\mathrm{rec}}\|_\mathrm{op} < 1$ is a sufficient condition for strict ESP; the spectral radius $\rho(W^{\mathrm{rec}}) < 1$ is necessary but not sufficient (operator norm $\geq$ spectral radius). In practice, the difference is small for typical reservoir matrices.

## 29.5.5 The Connection to Boyd-Chua

The strict ESP is the exact condition needed for the Boyd-Chua approximation theorem to apply:

**Theorem 29.8 (Boyd-Chua Requires Strict ESP).** Let $F_{\mathrm{target}} \in \mathcal{F}_{CTI}$ be a causal, time-invariant fading-memory functional. For a reservoir to approximate $F_{\mathrm{target}}$ via its readout, it is **necessary** that the reservoir satisfies the strict ESP with respect to the same weight $w$ as $F_{\mathrm{target}}$.

*Proof.* $F_{\mathrm{target}} \in \mathcal{F}_{CTI}$ means $F_{\mathrm{target}}$ is continuous in $\|\cdot\|_w$. If the reservoir does not satisfy the strict ESP, then (by Theorem 29.5) no readout can produce a continuous function of the input — hence no readout can approximate $F_{\mathrm{target}}$. $\square$

This theorem clarifies the role of the strict ESP in the theoretical framework: it is not an additional technical assumption but a *necessary* condition for the universality property to hold.

## 29.5.6 Strict ESP vs. ESP: A Hierarchy

The relationship between the various ESP conditions can be summarized as:

$$
\text{Strict ESP (Lipschitz)} \;\subset\; \text{Strict ESP (Continuous)} \;\subset\; \text{ESP (Measurable)} \;\subset\; \text{No ESP}
$$

From right to left, each condition is strictly stronger and implies the previous:
- **No ESP:** Multiple stationary measures; training is initialization-dependent.
- **ESP:** Unique stationary measure; training converges to a unique readout; Birkhoff's theorem applies.
- **Strict ESP:** Echo function is continuous; fading-memory approximation is possible; Boyd-Chua applies.
- **Strict ESP (Lipschitz):** Echo function is Lipschitz; quantitative approximation rates are available.

In practice, the Lipschitz condition $\|W^{\mathrm{rec}}\|_\mathrm{op} < 1$ is the easiest to verify and most commonly assumed. The distinction between Lipschitz and merely continuous strict ESP rarely matters for practical reservoir computing.

## 29.5.7 Implications for Reservoir Design

**1. Operator norm vs. spectral radius.** The strict ESP requires $\|W^{\mathrm{rec}}\|_\mathrm{op} < 1$, not just $\rho(W^{\mathrm{rec}}) < 1$. For a non-symmetric matrix, $\|W^{\mathrm{rec}}\|_\mathrm{op} = \sigma_{\max}(W^{\mathrm{rec}})$ (largest singular value). The standard normalization to spectral radius may still leave $\sigma_{\max}(W^{\mathrm{rec}}) > 1$, violating the Lipschitz condition.

**2. Safe operating region.** For a random reservoir with i.i.d. entries $\mathcal{N}(0, \sigma^2/N)$, the largest singular value satisfies $\sigma_{\max} \to 2\sigma$ (Marchenko-Pastur). Thus $\|W^{\mathrm{rec}}\|_\mathrm{op} \approx 2\sigma$; for Lipschitz strict ESP, we need $2\sigma < 1$, i.e., $\sigma < 0.5$.

**3. Tradeoff with expressiveness.** Strict ESP with small $\|W^{\mathrm{rec}}\|_\mathrm{op}$ means the reservoir is strongly contracting, which suppresses both long-term memory and nonlinear mixing. The tension between strict ESP (expressiveness constraint) and long memory is fundamental to reservoir computing design.

## References

- Buehner, M. and Young, P. (2006). A tighter bound for the echo state property. *IEEE Transactions on Neural Networks*, 17(3), 820–824.
- Crauel, H. and Flandoli, F. (1994). Attractors for random dynamical systems. *Probability Theory and Related Fields*, 100(3), 365–393.
- Grigoryeva, L. and Ortega, J.-P. (2018). Echo state networks are universal. *Neural Networks*, 108, 495–508.
- Jaeger, H. (2001). *The "echo state" approach to analysing and training recurrent neural networks*. GMD Technical Report 148.
