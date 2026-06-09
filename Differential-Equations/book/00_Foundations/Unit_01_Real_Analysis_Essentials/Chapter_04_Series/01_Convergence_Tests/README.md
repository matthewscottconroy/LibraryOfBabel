# Convergence Tests

The question of whether a series $\sum_{k=1}^\infty a_k$ converges cannot always be answered by computing partial sums in closed form. The geometric series $\sum r^k$ and the telescoping series $\sum (b_k - b_{k+1})$ yield explicit formulas, but most series resist such direct computation. Convergence tests are indirect methods: they infer convergence or divergence from properties of the terms $a_k$ — their rate of decay, their relationship to known series, or the behavior of their ratios — without computing the sum.

## The Divergence Test

**Theorem.** If $\sum a_k$ converges, then $a_k \to 0$.

*Proof.* If $S_n \to S$, then $a_n = S_n - S_{n-1} \to S - S = 0$. $\square$

**Contrapositive (Divergence Test).** If $a_k \not\to 0$, then $\sum a_k$ diverges.

**Example.** $\sum_{k=1}^\infty \frac{k}{k+1}$ diverges because $k/(k+1) \to 1 \neq 0$.

Warning: $a_k \to 0$ is necessary but not sufficient for convergence. The harmonic series $\sum 1/k$ diverges despite $1/k \to 0$.

## The Comparison Test

**Theorem.** Suppose $0 \leq a_k \leq b_k$ for all sufficiently large $k$.
1. If $\sum b_k$ converges, then $\sum a_k$ converges.
2. If $\sum a_k$ diverges, then $\sum b_k$ diverges.

*Proof of (1).* Let $A_n = \sum_{k=1}^n a_k$ and $B_n = \sum_{k=1}^n b_k$. Both are increasing sequences. If $\sum b_k = B < \infty$, then $A_n \leq B_n \leq B$ for all $n$, so $(A_n)$ is increasing and bounded above, hence converges by the Monotone Convergence Theorem. $\square$

**Example.** $\sum \frac{1}{k^2 + k}$ converges, since $\frac{1}{k^2+k} < \frac{1}{k^2}$ and $\sum \frac{1}{k^2}$ converges (this can be shown by the integral test: $\int_1^\infty x^{-2}\,dx = 1$).

## The Limit Comparison Test

When the exact comparison $a_k \leq b_k$ is hard to establish, the limiting ratio often works instead.

**Theorem.** Suppose $a_k, b_k > 0$ and $\lim_{k\to\infty} \frac{a_k}{b_k} = c$ with $0 < c < \infty$. Then $\sum a_k$ and $\sum b_k$ either both converge or both diverge.

*Proof.* Since $a_k/b_k \to c$, for large $k$ the ratio lies in $(c/2, 2c)$. So $\frac{c}{2} b_k < a_k < 2c \cdot b_k$ for large $k$, and the ordinary comparison test applies in both directions. $\square$

**Example.** Does $\sum \frac{1}{k^2 - k + 1}$ converge? Compare with $b_k = 1/k^2$: $\frac{a_k}{b_k} = \frac{k^2}{k^2 - k + 1} \to 1$. Since $\sum 1/k^2$ converges, so does the given series.

## The Integral Test

**Theorem.** Let $f: [1, \infty) \to \mathbb{R}$ be positive, continuous, and decreasing. Then $\sum_{k=1}^\infty f(k)$ and $\int_1^\infty f(x)\,dx$ either both converge or both diverge.

*Proof sketch.* Since $f$ is decreasing, $f(k+1) \leq \int_k^{k+1} f(x)\,dx \leq f(k)$. Summing: $\sum_{k=2}^{n+1} f(k) \leq \int_1^{n+1} f(x)\,dx \leq \sum_{k=1}^n f(k)$. The partial sums and integral bound each other, so they stand or fall together. $\square$

**$p$-Series.** The series $\sum_{k=1}^\infty \frac{1}{k^p}$ converges iff $p > 1$.

*Proof.* The integral $\int_1^\infty x^{-p}\,dx$ converges iff $p > 1$. $\square$

This establishes $\sum 1/k^2$ convergence and $\sum 1/k$ divergence simultaneously.

## The Ratio Test

**Theorem.** Let $(a_k)$ be a sequence of nonzero terms and suppose $L = \lim_{k\to\infty} \left|\frac{a_{k+1}}{a_k}\right|$ exists (or is $\pm\infty$).
1. If $L < 1$, then $\sum a_k$ converges absolutely.
2. If $L > 1$ (or $L = +\infty$), then $\sum a_k$ diverges.
3. If $L = 1$, the test is inconclusive.

*Proof of (1).* Choose $r$ with $L < r < 1$. For large $k$, $|a_{k+1}/a_k| < r$, so $|a_{k+N}| < r^N |a_k|$ for large $k$. The series is then dominated by a convergent geometric series $\sum r^N |a_k|$. $\square$

**Example.** $\sum \frac{k!}{k^k}$: the ratio is $\frac{(k+1)!/(k+1)^{k+1}}{k!/k^k} = \frac{k^k}{(k+1)^k} = \left(\frac{k}{k+1}\right)^k = \left(1 - \frac{1}{k+1}\right)^k \to e^{-1} < 1$. The series converges.

**Example (inconclusive).** Both $\sum 1/k$ (divergent) and $\sum 1/k^2$ (convergent) have ratio $L = 1$.

## The Root Test

**Theorem.** Let $L = \limsup_{k\to\infty} |a_k|^{1/k}$.
1. If $L < 1$, then $\sum a_k$ converges absolutely.
2. If $L > 1$, then $\sum a_k$ diverges.
3. If $L = 1$, the test is inconclusive.

The root test is strictly more powerful than the ratio test: whenever the ratio test is conclusive, so is the root test (with the same conclusion), but not vice versa. However, the ratio test is typically easier to apply when the terms involve factorials or exponentials.

**Example.** $\sum \left(\frac{2k+1}{3k-1}\right)^k$: $|a_k|^{1/k} = \frac{2k+1}{3k-1} \to \frac{2}{3} < 1$. The series converges.

## The Alternating Series Test

**Theorem (Leibniz's Test).** If $(b_k)$ is a positive, decreasing sequence with $b_k \to 0$, then $\sum_{k=1}^\infty (-1)^{k+1} b_k$ converges.

*Proof sketch.* The partial sums $S_{2n}$ form an increasing bounded sequence (each pair of consecutive terms contributes a positive amount) and $S_{2n+1} = S_{2n} + b_{2n+1}$, so $S_{2n+1}$ form a decreasing bounded sequence. Both subsequences have the same limit $L$, and $|S_n - L| \leq b_{n+1}$ (the error is bounded by the first omitted term). $\square$

**Example.** $\sum_{k=1}^\infty \frac{(-1)^{k+1}}{k} = 1 - \frac{1}{2} + \frac{1}{3} - \cdots = \ln 2$.

## Practical Guide

- Use the **divergence test** first; it is cheapest.
- For terms resembling $1/k^p$ or rational functions of $k$, use **$p$-series** and **limit comparison**.
- For terms involving $k!$, $k^k$, or exponentials in $k$, use the **ratio test**.
- For terms of the form $f(k)^k$, use the **root test**.
- For alternating series with decreasing terms, use **Leibniz's test**.
- If none of these work directly, try **comparison** with a series you know.

These tests are not merely algebraic exercises. In the method of series solutions for ODEs, one applies the ratio or root test to determine the radius of convergence of a proposed power series solution, establishing where the solution is valid.
