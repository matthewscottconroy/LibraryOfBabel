# The Extreme Value Theorem

A continuous function on a closed bounded interval is guaranteed to attain its maximum and minimum. This is the Extreme Value Theorem (EVT), and it is not at all obvious: the function could conceivably approach but never reach its supremum. The theorem asserts that this cannot happen, and its proof uses the full power of the Completeness Axiom combined with sequential compactness.

## Statement and Proof

**Theorem (Extreme Value Theorem).** Let $f: [a,b] \to \mathbb{R}$ be continuous. Then $f$ attains its maximum and minimum on $[a,b]$: there exist $x_{\max}, x_{\min} \in [a,b]$ with
$$f(x_{\min}) \leq f(x) \leq f(x_{\max}) \quad \text{for all } x \in [a,b].$$

*Proof.* We prove $f$ attains its supremum; the infimum argument is analogous.

**Step 1.** $f$ is bounded above. Suppose not. Then for each $n$, there exists $x_n \in [a,b]$ with $f(x_n) > n$. The sequence $(x_n)$ lies in $[a,b]$ and is bounded, so by Bolzano-Weierstrass it has a convergent subsequence $x_{n_k} \to x^* \in [a,b]$. By continuity, $f(x_{n_k}) \to f(x^*)$, which is finite. But $f(x_{n_k}) > n_k \to \infty$, a contradiction. So $f$ is bounded above.

**Step 2.** Let $M = \sup_{x \in [a,b]} f(x)$, which exists by Step 1 and the Completeness Axiom. For each $n$, there exists $x_n \in [a,b]$ with $f(x_n) > M - 1/n$ (by the characterization of the supremum). Again, $(x_n)$ is bounded, so by Bolzano-Weierstrass, some subsequence $x_{n_k} \to x_{\max} \in [a,b]$. By continuity:
$$f(x_{\max}) = \lim_{k\to\infty} f(x_{n_k}) \geq \lim_{k\to\infty} \left(M - \frac{1}{n_k}\right) = M.$$
Also $f(x_{\max}) \leq M$ since $M$ is the supremum. So $f(x_{\max}) = M$. $\square$

## Why Each Hypothesis Is Necessary

The EVT fails if any of its three hypotheses — continuity, closedness of $[a,b]$, or boundedness of $[a,b]$ — is dropped.

**Without continuity.** Define $f: [0,1] \to \mathbb{R}$ by $f(0) = 0$ and $f(x) = 1/x$ for $x > 0$. This is unbounded on $[0,1]$ and has no maximum.

**Without closedness.** Define $f(x) = x$ on $(0,1)$. Then $\sup f = 1$, but $f(x) < 1$ for all $x \in (0,1)$, so the supremum is not attained.

**Without boundedness.** Define $f(x) = x$ on $[0,\infty)$. This is continuous but unbounded, with no finite maximum.

All three conditions work together: continuity ensures $f$ behaves predictably; the closed interval captures any limit points of sequences in the domain; the boundedness prevents blow-up.

## The Concept of Compactness

The key property of $[a,b]$ used in the proof is not "closed and bounded" per se but a property called **compactness**. A subset $K \subseteq \mathbb{R}$ is compact if every sequence in $K$ has a subsequence converging to a point of $K$. The Heine-Borel theorem states that a subset of $\mathbb{R}$ is compact if and only if it is closed and bounded.

The EVT is really the statement: a continuous function on a compact set is bounded and attains its extrema. This generalizes beyond $\mathbb{R}$: in any metric space, continuous functions on compact sets attain their extrema.

## Applications

**Optimization.** To find the maximum of a differentiable function on $[a,b]$: by Fermat's theorem (proved in Chapter 6), the maximum is attained either at a critical point (where $f' = 0$ or $f'$ does not exist) or at an endpoint. The EVT guarantees a maximum exists; calculus locates it.

**Bounding solutions.** In ODE theory, if a solution $y(t)$ is continuous on a closed interval $[t_0, t_0 + T]$ — which follows from the existence theorem — the EVT guarantees that $y$ is bounded on that interval, say $|y(t)| \leq M$. This bound is used to extend the local existence result to a global one.

**Error bounds.** If $e(x)$ is the error in an approximation and $e$ is continuous on $[a,b]$, the EVT guarantees the maximum error $\max_{[a,b]} |e(x)|$ is attained. Numerical analysis frequently reports this maximum error.

## Corollary: Continuity Preserves Compactness

**Corollary.** If $f: [a,b] \to \mathbb{R}$ is continuous, then $f([a,b]) = \{f(x): x \in [a,b]\}$ is a closed bounded interval $[m, M]$ where $m = \min f$ and $M = \max f$.

*Proof.* By the EVT, $m$ and $M$ exist. By the IVT, $f$ takes every value between $m$ and $M$. So $f([a,b]) = [m, M]$. $\square$

This clean description of the image of a closed interval under a continuous function is used when analyzing what values a solution to an ODE can take.

## Common Pitfalls

**Assuming the maximum is attained at a critical point.** On an open interval $(a,b)$, the supremum of a continuous function may not be attained, and even if the function is differentiable with a critical point, the supremum might be attained at an endpoint of a closed interval instead. The EVT applies to the closed interval.

**Forgetting that the infimum is also attained.** Both extrema are guaranteed. In optimization, one must check both maximum and minimum.
