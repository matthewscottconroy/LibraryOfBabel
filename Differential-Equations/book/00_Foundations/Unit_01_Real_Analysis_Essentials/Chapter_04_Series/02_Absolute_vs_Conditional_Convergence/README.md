# Absolute vs. Conditional Convergence

Not all convergence is created equal. When a series converges because of the magnitudes of its terms — because the terms are simply small — the convergence is robust. When a series converges only because of intricate cancellation between positive and negative terms, the convergence is fragile. This distinction, formalized as absolute versus conditional convergence, determines what algebraic manipulations are valid and shapes the theory of power series.

## Definitions

**Definition.** The series $\sum_{k=1}^\infty a_k$ is said to converge **absolutely** if $\sum_{k=1}^\infty |a_k|$ converges. It converges **conditionally** if $\sum a_k$ converges but $\sum |a_k|$ diverges.

**Theorem.** Absolute convergence implies convergence.

*Proof.* If $\sum |a_k|$ converges, the series of positive parts $a_k^+ = \max(a_k, 0)$ and negative parts $a_k^- = \max(-a_k, 0)$ both satisfy $a_k^+, a_k^- \leq |a_k|$, so $\sum a_k^+$ and $\sum a_k^-$ both converge by comparison. Then $\sum a_k = \sum a_k^+ - \sum a_k^-$ converges as a difference of two convergent series. $\square$

**Example.** $\sum_{k=1}^\infty \frac{(-1)^k}{k^2}$ converges absolutely, since $\sum 1/k^2$ converges.

**Example.** $\sum_{k=1}^\infty \frac{(-1)^{k+1}}{k} = 1 - \frac{1}{2} + \frac{1}{3} - \cdots$ converges conditionally. It converges by Leibniz's test, but $\sum 1/k$ diverges.

## Rearrangements and Riemann's Theorem

The dramatic difference between absolute and conditional convergence is revealed by rearrangement.

**Definition.** A **rearrangement** of $\sum a_k$ is a series $\sum a_{\sigma(k)}$ where $\sigma: \mathbb{N} \to \mathbb{N}$ is a bijection.

**Theorem.** If $\sum a_k$ converges absolutely, then every rearrangement converges to the same sum.

*Proof sketch.* Let $S = \sum a_k$. Given a rearrangement $\sum a_{\sigma(k)}$ with partial sums $T_n$, for large $N$ both $S_N$ and $T_N$ include all the largest terms, and the residuals go to zero because $\sum |a_k|$ converges. $\square$

**Theorem (Riemann Rearrangement Theorem).** If $\sum a_k$ converges conditionally, then for any $T \in \mathbb{R}$ (or $T = \pm\infty$), there exists a rearrangement of $\sum a_k$ that converges to $T$.

*Proof sketch.* Let $p_1, p_2, \ldots$ be the positive terms of $(a_k)$ in order, and $q_1, q_2, \ldots$ the absolute values of the negative terms. Since convergence is conditional, $\sum p_i = \infty$ and $\sum q_j = \infty$ (otherwise $\sum |a_k|$ would converge). To reach target $T > 0$: add positive terms until the partial sum first exceeds $T$; then subtract negative terms until it drops below $T$; repeat. Since $p_i, q_j \to 0$, the oscillations diminish and the sum approaches $T$. $\square$

**Example.** The alternating harmonic series satisfies $\sum_{k=1}^\infty \frac{(-1)^{k+1}}{k} = \ln 2$. Its rearrangement where one negative term follows every two positive terms gives:
$$1 + \frac{1}{3} - \frac{1}{2} + \frac{1}{5} + \frac{1}{7} - \frac{1}{4} + \cdots = \frac{3}{2}\ln 2.$$
A different rearrangement can produce $0$, $\pi$, or any other real number.

This is not a paradox — it is a theorem. It reveals that conditional convergence depends on the precise ordering of terms, while absolute convergence is a property of the set of terms.

## The Absolute Convergence Test via Ratio/Root

The ratio and root tests, as stated in Section 1, conclude "converges absolutely" rather than merely "converges." This is because the test compares to a geometric series, which converges absolutely.

**Corollary.** If the ratio test gives $L < 1$, the series converges absolutely. If $L > 1$, it diverges. If $L = 1$, it may converge absolutely, converge conditionally, or diverge — all three behaviors occur.

## Multiplying Series

**Cauchy Product.** Given series $\sum a_k$ and $\sum b_k$, their Cauchy product is $\sum_{k=0}^\infty c_k$ where $c_k = \sum_{j=0}^k a_j b_{k-j}$.

**Theorem (Mertens).** If $\sum a_k$ converges absolutely to $A$ and $\sum b_k$ converges (absolutely or conditionally) to $B$, then the Cauchy product converges to $AB$.

This theorem justifies multiplying power series term by term within the interval of convergence. In the theory of differential equations, when one solves $y' = ay$ by a series, the equation requires multiplying the derivative series by the coefficient series, and Mertens' theorem (via the absolute convergence of power series within their radius of convergence) guarantees this product is valid.

## Uniform Absolute Convergence of Series of Functions

For series of functions $\sum f_k(x)$, a key property is **uniform absolute convergence**: $\sum |f_k(x)|$ converges uniformly on some set $E$. By the Weierstrass M-test: if $|f_k(x)| \leq M_k$ on $E$ and $\sum M_k < \infty$, then $\sum f_k$ converges uniformly and absolutely on $E$.

This matters for series solutions to ODEs: the solution $y(x) = \sum_{k=0}^\infty c_k x^k$ must converge uniformly on compact subsets of the interval of convergence to justify differentiation term by term.

## Summary of Hierarchy

All absolutely convergent series converge; not all convergent series converge absolutely. The hierarchy is:
$$\text{absolutely convergent} \subset \text{convergent} \subset \text{sequence of partial sums bounded}.$$

For applications — especially series manipulations in ODE theory — absolute convergence is the safe assumption under which all standard algebraic operations are justified. Conditional convergence requires more care, and rearrangement in particular must be avoided.
