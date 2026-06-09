# Convergence and Divergence

Convergence is a special property, not the generic behavior of sequences. Many sequences diverge, and they do so in qualitatively different ways. Understanding both convergence and divergence — what each looks like, what properties it implies, and how to distinguish one from the other — is essential for the theory of differential equations, where one must determine whether solution approximations settle to a true solution or drift away.

## Properties of Convergent Sequences

**Theorem.** Every convergent sequence is bounded: if $a_n \to L$, then there exists $M > 0$ such that $|a_n| \leq M$ for all $n$.

*Proof.* Take $\varepsilon = 1$. There exists $N$ such that $|a_n - L| < 1$ for all $n > N$. By the triangle inequality, $|a_n| = |a_n - L + L| \leq |a_n - L| + |L| < 1 + |L|$ for $n > N$. Set $M = \max(|a_1|, |a_2|, \ldots, |a_N|, 1 + |L|)$. Then $|a_n| \leq M$ for all $n$. $\square$

The contrapositive is useful: an unbounded sequence diverges. But boundedness alone does not guarantee convergence — the sequence $(-1)^n$ is bounded yet diverges.

**Theorem.** Limits preserve inequalities: if $a_n \to L$ and $b_n \to M$ and $a_n \leq b_n$ for all sufficiently large $n$, then $L \leq M$.

Note: strict inequality $a_n < b_n$ implies only $L \leq M$, not $L < M$. For example, $1/n < 1$ for all $n$, but $\lim 1/n = 0 \leq 1 = \lim 1$.

**Corollary.** If $a_n \to L$ and $a_n \geq 0$ for all $n$, then $L \geq 0$.

## Subsequences

A **subsequence** of $(a_n)$ is a sequence $(a_{n_k})_{k=1}^\infty$ where $n_1 < n_2 < n_3 < \cdots$ is a strictly increasing sequence of natural numbers. Intuitively, a subsequence selects an infinite collection of terms from the original sequence, in order.

**Theorem.** If $a_n \to L$, then every subsequence of $(a_n)$ also converges to $L$.

*Proof.* Let $\varepsilon > 0$. Choose $N$ so that $|a_n - L| < \varepsilon$ for all $n > N$. Since the indices $n_k$ are strictly increasing natural numbers, $n_k \geq k$ for all $k$. So for $k > N$, $n_k \geq k > N$, giving $|a_{n_k} - L| < \varepsilon$. $\square$

The contrapositive: if $(a_n)$ has two subsequences converging to different limits, then $(a_n)$ diverges. For $(-1)^n$, the even-indexed subsequence converges to $1$ and the odd-indexed subsequence converges to $-1$; since $1 \neq -1$, $(-1)^n$ diverges.

## The Bolzano-Weierstrass Theorem

**Theorem (Bolzano-Weierstrass).** Every bounded sequence of real numbers has a convergent subsequence.

*Proof sketch.* Let $(a_n)$ be bounded, so $a_n \in [c_0, d_0]$ for all $n$. Bisect: $[c_0, d_0] = [c_0, m] \cup [m, d_0]$ where $m = (c_0 + d_0)/2$. At least one half contains infinitely many terms; call it $[c_1, d_1]$. Pick any $a_{n_1}$ from it. Bisect again; pick $a_{n_2}$ from the half with infinitely many terms, with $n_2 > n_1$. Continue. The resulting subsequence $(a_{n_k})$ lies in intervals $[c_k, d_k]$ of length $(d_0 - c_0)/2^k \to 0$. The nested interval property gives a unique point $L \in \bigcap_k [c_k, d_k]$, and $|a_{n_k} - L| \leq d_k - c_k \to 0$. $\square$

This theorem is fundamental: it converts the abstract property of boundedness into an existential claim about convergent subsequences. It is used in the proof of the Cauchy criterion, the extreme value theorem, and the existence of eigenvalues.

## Modes of Divergence

**Divergence to infinity.** $(a_n)$ **diverges to $+\infty$**, written $a_n \to +\infty$, if for every $M > 0$ there exists $N$ such that $a_n > M$ for all $n > N$. Similarly for $-\infty$. Examples: $a_n = n$, $a_n = n^2$, $a_n = 2^n$.

**Oscillatory divergence.** $(a_n)$ diverges without going to $\pm\infty$; it has at least two subsequences converging to different limits (called **subsequential limits**). The sequence $(-1)^n$ has subsequential limits $\{-1, 1\}$; the sequence $\sin(n\pi/2)$ cycles through $\{0, 1, 0, -1\}$ as subsequential limits.

The **limit superior** and **limit inferior** of a sequence capture the largest and smallest subsequential limits:
$$\limsup_{n\to\infty} a_n = \lim_{n\to\infty} \sup_{k \geq n} a_k, \qquad \liminf_{n\to\infty} a_n = \lim_{n\to\infty} \inf_{k \geq n} a_k.$$
These always exist (in $[-\infty, +\infty]$) for any bounded sequence, and a sequence converges if and only if $\limsup a_n = \liminf a_n$.

## Worked Examples

**Example 1.** Prove that $\lim_{n\to\infty} r^n = 0$ for $|r| < 1$.

If $r = 0$, the conclusion is immediate. Assume $0 < |r| < 1$. Write $|r| = 1/(1 + \alpha)$ where $\alpha > 0$. By Bernoulli's inequality (proved by induction), $(1+\alpha)^n \geq 1 + n\alpha$. So $|r^n| = |r|^n = 1/(1+\alpha)^n \leq 1/(1 + n\alpha)$. Given $\varepsilon > 0$, choose $N > (1/\varepsilon - 1)/\alpha$; then for $n > N$, $|r^n| \leq 1/(1+n\alpha) < \varepsilon$. $\square$

**Example 2.** Prove that $\left(\frac{n+1}{n}\right)^n \to e$ (given the definition $e = \lim_{n}(1+1/n)^n$).

Write $\left(\frac{n+1}{n}\right)^n = \left(1 + \frac{1}{n}\right)^n$. This is exactly the sequence defining $e$. $\square$

## Common Pitfalls

**Assuming convergence from terms getting close.** The harmonic series' partial sums $S_n = 1 + 1/2 + \cdots + 1/n$ satisfy $S_{n+1} - S_n = 1/(n+1) \to 0$, yet $S_n \to \infty$. Consecutive terms getting close does not imply convergence.

**Ignoring finitely many terms.** Convergence is a tail property: the behavior of $a_1, \ldots, a_N$ is irrelevant. Only the terms beyond some $N$ matter.

**Confusing bounded with convergent.** Bounded sequences need only have a convergent subsequence (Bolzano-Weierstrass), not converge themselves. The Monotone Convergence Theorem (next section) gives conditions under which a bounded sequence must actually converge.

## Connection to Differential Equations

Solutions to differential equations are often constructed as limits of sequences of approximations. The Picard iteration produces $\phi_0, \phi_1, \phi_2, \ldots$ and the argument that this converges uses both Bolzano-Weierstrass (for compactness of function spaces) and direct epsilon-N arguments. Understanding what it means for a sequence to converge — and recognizing the pathologies that can prevent convergence — is prerequisite to understanding why existence theorems require the hypotheses they do.
