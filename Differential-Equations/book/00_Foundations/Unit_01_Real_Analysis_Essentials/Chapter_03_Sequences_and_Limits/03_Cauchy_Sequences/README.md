# Cauchy Sequences

The epsilon-N definition of convergence requires knowing the limit $L$ in advance: one must verify that terms eventually cluster near this specific value. In practice, however, one often needs to prove that a sequence converges without knowing what the limit is — the limit might be a new mathematical object that doesn't yet have a name. Augustin-Louis Cauchy identified an internal criterion for convergence that requires no reference to the limit: the terms of the sequence must eventually become close to each other.

## Definition

**Definition.** A sequence $(a_n)$ is a **Cauchy sequence** if
$$\forall \varepsilon > 0,\ \exists N \in \mathbb{N},\ \forall m, n > N,\ |a_m - a_n| < \varepsilon.$$

The key difference from convergence: instead of requiring each term to be close to a fixed $L$, we require any two sufficiently late terms to be close to each other. The existence of a limit point is not assumed.

**Intuition.** Imagine walking along the sequence. In a Cauchy sequence, after some point you never stray far from where you were. In a divergent sequence, you either keep moving off to infinity or keep returning to different neighborhoods of different values.

## The Cauchy Criterion

**Theorem (Cauchy Criterion).** A sequence of real numbers converges if and only if it is a Cauchy sequence.

This theorem has two directions.

**Convergent $\Rightarrow$ Cauchy:** If $a_n \to L$, let $\varepsilon > 0$. Choose $N$ so that $|a_n - L| < \varepsilon/2$ for all $n > N$. For $m, n > N$:
$$|a_m - a_n| = |(a_m - L) - (a_n - L)| \leq |a_m - L| + |a_n - L| < \frac{\varepsilon}{2} + \frac{\varepsilon}{2} = \varepsilon.$$

**Cauchy $\Rightarrow$ Convergent:** This is the substantive direction and requires completeness of $\mathbb{R}$.

*Proof.* 
*Step 1.* Every Cauchy sequence is bounded. Take $\varepsilon = 1$; choose $N$ so that $|a_m - a_n| < 1$ for all $m, n > N$. In particular $|a_n - a_{N+1}| < 1$ for all $n > N$, so $|a_n| < |a_{N+1}| + 1$ for $n > N$. The bound $M = \max(|a_1|, \ldots, |a_N|, |a_{N+1}| + 1)$ works for all $n$.

*Step 2.* By the Bolzano-Weierstrass theorem, the bounded sequence $(a_n)$ has a convergent subsequence $a_{n_k} \to L$.

*Step 3.* The Cauchy sequence converges to the same limit as its convergent subsequence. Let $\varepsilon > 0$. Choose $N$ so that $|a_m - a_n| < \varepsilon/2$ for all $m, n > N$. Choose $K$ so that $|a_{n_k} - L| < \varepsilon/2$ for all $k > K$ and $n_K > N$. For $n > N$, pick $k$ with $n_k > N$ and $|a_{n_k} - L| < \varepsilon/2$. Then:
$$|a_n - L| \leq |a_n - a_{n_k}| + |a_{n_k} - L| < \frac{\varepsilon}{2} + \frac{\varepsilon}{2} = \varepsilon. \quad \square$$

## Why Completeness Is Necessary

The Cauchy Criterion fails in $\mathbb{Q}$: there exist Cauchy sequences of rationals that do not converge in $\mathbb{Q}$. The sequence of rational approximations to $\sqrt{2}$ — for instance, $a_n$ defined by the recursion $a_{n+1} = (a_n + 2/a_n)/2$ starting from $a_1 = 1$ — is Cauchy but converges to $\sqrt{2} \notin \mathbb{Q}$.

This is precisely why completeness of $\mathbb{R}$ is needed: it is the property that guarantees every Cauchy sequence converges within $\mathbb{R}$. The converse (convergent $\Rightarrow$ Cauchy) holds in any metric space, but the forward direction (Cauchy $\Rightarrow$ convergent) is what defines a **complete metric space**.

## Complete Metric Spaces

The concept of a Cauchy sequence generalizes from $\mathbb{R}$ to any metric space $(X, d)$: a sequence $(x_n)$ is Cauchy if $d(x_m, x_n) \to 0$ as $m, n \to \infty$. A metric space is **complete** if every Cauchy sequence converges. The real line $\mathbb{R}$ is complete; the rationals $\mathbb{Q}$ are not.

For differential equations, the relevant complete metric space is $C([a, b])$, the space of continuous functions on $[a, b]$, with the **sup-norm**:
$$\|f\| = \sup_{x \in [a,b]} |f(x)|.$$
The metric $d(f, g) = \|f - g\|$ makes $C([a,b])$ a complete metric space (this is a theorem, not a definition). It is the completeness of this function space — which inherits from the completeness of $\mathbb{R}$ — that makes Picard iteration work.

## The Banach Fixed-Point Theorem

The Cauchy criterion lies at the heart of the most powerful tool for proving existence and uniqueness:

**Theorem (Banach Fixed-Point Theorem, or Contraction Mapping Principle).** Let $(X, d)$ be a complete metric space and $T: X \to X$ a contraction, meaning there exists $k \in (0,1)$ such that $d(Tx, Ty) \leq k \cdot d(x, y)$ for all $x, y \in X$. Then $T$ has a unique fixed point $x^*$ (with $T(x^*) = x^*$), and for any starting point $x_0$, the iteration $x_{n+1} = T(x_n)$ converges to $x^*$.

*Proof sketch.* The sequence $x_0, x_1 = Tx_0, x_2 = T^2 x_0, \ldots$ satisfies $d(x_{n+1}, x_n) \leq k^n d(x_1, x_0)$. By the triangle inequality:
$$d(x_{n+p}, x_n) \leq \sum_{j=0}^{p-1} d(x_{n+j+1}, x_{n+j}) \leq d(x_1, x_0) \sum_{j=0}^{p-1} k^{n+j} \leq \frac{k^n}{1-k} d(x_1, x_0) \to 0.$$
So $(x_n)$ is Cauchy, hence converges to some $x^*$ by completeness. Continuity of $T$ gives $T(x^*) = x^*$. Uniqueness: if $T(y) = y$ and $T(x^*) = x^*$, then $d(x^*, y) = d(T(x^*), T(y)) \leq k \cdot d(x^*, y)$, so $(1-k) d(x^*, y) \leq 0$, giving $d(x^*, y) = 0$. $\square$

The Picard-Lindelof existence theorem for ODEs ($y' = f(x,y)$, $y(x_0) = y_0$) is proved by showing that the Picard iteration operator is a contraction on $C([x_0 - h, x_0 + h])$ for small enough $h$, and applying this theorem. The solution is the fixed point.

## Common Pitfalls

**Consecutive terms converging is not enough.** If $|a_{n+1} - a_n| \to 0$, the sequence need not be Cauchy and need not converge. The harmonic partial sums $S_n = \sum_{k=1}^n 1/k$ satisfy $S_{n+1} - S_n = 1/(n+1) \to 0$, yet $S_n \to \infty$.

**The Cauchy condition requires uniform control.** For all $m, n > N$, not just adjacent pairs. The "for all $m, n > N$" quantification is stronger than "for consecutive terms."
