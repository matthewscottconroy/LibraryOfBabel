# 34.2 Bowen's Sofic Entropy

Here's the core idea. Classical KS entropy measures how many distinct "histories" a dynamical system can produce. The Ornstein-Weiss definition uses Følner sets — finite windows that approximate the group — and counts the number of "names" (patterns) that appear in those windows.

For non-amenable groups, there are no Følner sets. But there are sofic approximations. Bowen's idea: use those approximations to count the number of ways to "simulate" the action on a finite set.

## 34.2.1 Definition via Microstates

**Setup:** Let $\Gamma \curvearrowright (X, \mu)$ be a free measure-preserving action of a sofic group $\Gamma$. Let $\Sigma = (\sigma_n: \Gamma \to \text{Sym}(d_n))$ be a sofic approximation.

**Definition 34.2.1 (Bowen, 2010).** Let $\xi = \{A_1, \ldots, A_k\}$ be a measurable partition of $X$. For a sofic approximation $\Sigma$ and $n$ large:
- A *microstate* is a map $\phi: [d_n] \to \{1,\ldots,k\}$ (a coloring of $[d_n]$) that "looks like" the partition $\xi$ of $X$ with respect to the $\Gamma$-action.
- The *microstate space* $\text{Map}(\xi, F, \delta, \sigma_n)$ is the set of colorings $\phi$ such that for $s \in F$ and $i$ in a $(1-\delta)d_n$-fraction of $[d_n]$: $\phi(\sigma_n(s)(i)) = j$ iff $s\cdot x \in A_j$ (approximately equivariant)

Each microstate is a finite "model" of the action: a coloring of the $d_n$ "atoms" of the finite approximation that approximately respects how $\Gamma$ acts. The microstate space counts how many such finite models exist.

**Definition 34.2.2.** The *sofic entropy* of the partition $\xi$ with respect to $\Sigma$ is:
$$h_\Sigma(\xi, \Gamma \curvearrowright X) = \inf_{F, \delta} \limsup_{n\to\infty} \frac{1}{d_n} \log |\text{Map}(\xi, F, \delta, \sigma_n)|.$$

The *sofic entropy* of the action is $h_\Sigma(\Gamma \curvearrowright X) = \sup_\xi h_\Sigma(\xi, \Gamma \curvearrowright X)$.

We take the infimum over approximations $(F, \delta)$ to get the "true" count, and the limsup as $n \to \infty$ to get the asymptotic rate.

## 34.2.2 Properties of Sofic Entropy

**Theorem 34.2.3 (Bowen, 2010).** For a free ergodic action of a sofic group $\Gamma$:
1. $h_\Sigma(\Gamma \curvearrowright X) \in [-\infty, \infty]$ (can be $-\infty$)
2. $h_\Sigma$ is an invariant of the action (independent of the choice of sofic approximation $\Sigma$, for Bernoulli actions)
3. For Bernoulli actions $\Gamma \curvearrowright (X_0, \mu_0)^\Gamma$: $h_\Sigma = H(\mu_0)$ (the base entropy)
4. If $\Gamma$ is amenable: $h_\Sigma$ equals the classical KS entropy

The fact that sofic entropy can be $-\infty$ is a genuine departure from KS entropy, which is always $\geq 0$. For certain non-Bernoulli actions of non-amenable groups, the sofic entropy is $-\infty$ — meaning the system has "no good finite models."

**Theorem 34.2.4 (Kerr-Li, 2011).** Sofic entropy is independent of the sofic approximation for all actions (not just Bernoulli), confirming it is a genuine invariant.

This is not obvious — in principle, different sofic approximations could give different entropy values. Kerr and Li proved this doesn't happen, using operator-algebraic techniques. The proof is substantial.
