# The Monotone Convergence Theorem

Monotone sequences are among the most naturally occurring objects in analysis. They arise from iterative algorithms (where each step improves on the last), from cumulative sums (which can only increase), and from approximation schemes (which approach their target from one direction). For monotone sequences, there is a remarkably clean criterion for convergence: boundedness alone is sufficient.

## Statement and Proof

**Theorem (Monotone Convergence Theorem).** Let $(a_n)$ be a monotone sequence.
1. If $(a_n)$ is increasing ($a_n \leq a_{n+1}$ for all $n$) and bounded above, then $(a_n)$ converges, and $\lim_{n\to\infty} a_n = \sup\{a_n : n \in \mathbb{N}\}$.
2. If $(a_n)$ is decreasing ($a_n \geq a_{n+1}$ for all $n$) and bounded below, then $(a_n)$ converges, and $\lim_{n\to\infty} a_n = \inf\{a_n : n \in \mathbb{N}\}$.

*Proof of (1).* Let $S = \{a_n : n \in \mathbb{N}\}$. Since $(a_n)$ is bounded above, $S$ is a nonempty bounded-above subset of $\mathbb{R}$. By the Completeness Axiom, $\alpha = \sup S$ exists.

Claim: $a_n \to \alpha$. Let $\varepsilon > 0$. Since $\alpha - \varepsilon$ is not an upper bound of $S$, there exists $N$ such that $a_N > \alpha - \varepsilon$. Since $(a_n)$ is increasing, for all $n > N$ we have $a_n \geq a_N > \alpha - \varepsilon$. Also, $a_n \leq \alpha$ (since $\alpha$ is an upper bound). Therefore $\alpha - \varepsilon < a_n \leq \alpha < \alpha + \varepsilon$, so $|a_n - \alpha| < \varepsilon$. $\square$

Part (2) follows by applying part (1) to the sequence $(-a_n)$.

## Why This Theorem Matters

The Monotone Convergence Theorem is the most economical convergence result in analysis: to prove a monotone sequence converges, find a single bound and check monotonicity — no epsilon-N computation needed. The limit is automatically the supremum (or infimum), even if one cannot compute it explicitly.

This is not a weakness but a strength: in many applications, one proves convergence to show that a limit exists, and then uses other methods to identify what the limit is.

## The Number $e$

**Example.** The sequence $a_n = \left(1 + \frac{1}{n}\right)^n$ is increasing and bounded above by $3$.

*Monotonicity sketch.* By the AM-GM inequality or by expanding via the binomial theorem and comparing term counts, $a_n \leq a_{n+1}$.

*Boundedness.* By the binomial theorem:
$$a_n = \sum_{k=0}^n \binom{n}{k} \frac{1}{n^k} = \sum_{k=0}^n \frac{1}{k!} \cdot \frac{n!}{n^k(n-k)!} \leq \sum_{k=0}^n \frac{1}{k!} \leq \sum_{k=0}^\infty \frac{1}{k!}.$$
The series $\sum 1/k!$ converges (compare to the geometric series $\sum 1/2^{k-1}$) and its sum is less than $3$. So $a_n < 3$.

By the Monotone Convergence Theorem, $\lim a_n$ exists. This limit is defined to be $e \approx 2.71828\ldots$

## Recursive Sequences

Monotone sequences frequently arise from recursions. A recursion defines $a_{n+1} = f(a_n)$ for some function $f$. To apply the Monotone Convergence Theorem:

1. Prove monotonicity by induction: show $a_1 \leq a_2$ and then $a_n \leq a_{n+1} \Rightarrow a_{n+1} \leq a_{n+2}$.
2. Prove boundedness: show $a_n \leq M$ for all $n$ (again by induction).
3. Conclude convergence, and find the limit by letting $n \to \infty$ in the recursion: if $a_n \to L$, then $a_{n+1} = f(a_n) \to f(L)$ (assuming $f$ is continuous), giving $L = f(L)$.

**Example.** Define $a_1 = 1$ and $a_{n+1} = \frac{1}{2}\left(a_n + \frac{2}{a_n}\right)$. (This is Newton's method for $\sqrt{2}$.)

*Claim.* $(a_n)_{n \geq 2}$ is decreasing and bounded below by $\sqrt{2}$.

*Boundedness below.* By AM-GM, $\frac{1}{2}(x + 2/x) \geq \sqrt{x \cdot 2/x} = \sqrt{2}$ for $x > 0$. So $a_n \geq \sqrt{2}$ for $n \geq 2$.

*Decreasing.* For $n \geq 2$, $a_n \geq \sqrt{2} > 0$ and $a_{n+1} = \frac{1}{2}(a_n + 2/a_n) \leq a_n$ iff $2/a_n \leq a_n$ iff $2 \leq a_n^2$, which holds since $a_n \geq \sqrt{2}$.

By the Monotone Convergence Theorem, $L = \lim a_n$ exists. Taking limits in the recursion: $L = \frac{1}{2}(L + 2/L)$, giving $2L^2 = L^2 + 2$, so $L^2 = 2$, $L = \sqrt{2}$ (positive root since $a_n > 0$).

This computation also demonstrates the link to differential equations: Newton's method is a discrete analog of continuous gradient flow, and its convergence analysis uses exactly the same monotone sequence argument.

## Connection to Series

The Monotone Convergence Theorem is the foundation of convergence theory for series with nonnegative terms. The sequence of partial sums $S_n = \sum_{k=1}^n a_k$ is increasing whenever $a_k \geq 0$. It converges if and only if it is bounded above, which is the essence of the comparison test for series (treated in Chapter 4).

## Connection to the Picard Iteration

In the proof of the Picard-Lindelof theorem (existence and uniqueness for ODEs), one approach constructs the approximate solutions $\phi_n$ and shows that in a suitable norm, the sequence $\|\phi_n\|$ is bounded and that the sequence converges in a monotone fashion. While the general proof uses the contraction mapping principle, a simpler monotone iteration approach works for special classes of ODEs (notably those with monotone right-hand sides), where the Monotone Convergence Theorem applies directly at the function-space level.

## Worked Example in Full

**Problem.** Let $a_1 = 2$ and $a_{n+1} = \sqrt{2 a_n}$. Prove that $(a_n)$ converges and find the limit.

**Step 1: Show $a_n \leq 4$ for all $n$ (by induction).** Base: $a_1 = 2 \leq 4$. Inductive step: if $a_n \leq 4$, then $a_{n+1} = \sqrt{2a_n} \leq \sqrt{8} = 2\sqrt{2} < 4$.

**Step 2: Show $(a_n)$ is increasing.** We need $a_{n+1} \geq a_n$, i.e., $\sqrt{2a_n} \geq a_n$, i.e., $2a_n \geq a_n^2$, i.e., $a_n \leq 2$. But we need to check this: $a_1 = 2 \leq 2$, and if $a_n \leq 2$ then $a_{n+1} = \sqrt{2a_n} \leq \sqrt{4} = 2$. So by induction $a_n \leq 2$ for all $n$, confirming the sequence is increasing.

**Step 3: Conclude.** By the Monotone Convergence Theorem, $L = \lim a_n$ exists. Taking limits: $L = \sqrt{2L}$, so $L^2 = 2L$, giving $L(L-2) = 0$. Thus $L = 0$ or $L = 2$. Since $a_n \geq a_1 = 2$ for all $n$ (the sequence is increasing starting at $2$), $L \geq 2$. So $L = 2$.
