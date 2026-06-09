# 6.1 Discrete Dynamical Systems

## Setup and Basic Definitions

Let's start with the minimal setup. We want to study what happens when you apply a map over and over again. The phase space should be compact — so orbits can't escape to infinity — and the map should be continuous — so nearby points behave similarly, at least initially. That's it. No differential equations, no measures, no coordinates.

**Definition 6.1.1.** A *topological dynamical system* (TDS) is a pair $(X, f)$ where $X$ is a compact metrizable space and $f: X \to X$ is continuous. (If $f$ is a homeomorphism, the system is *invertible*.)

The *orbit* of $x \in X$ under $f$ is $\mathcal{O}(x) = \{f^n(x) : n \in {\mathbb N}\}$ (or $n \in {\mathbb Z}$ if $f$ is invertible).

The orbit is the sequence of positions the point visits. The central question of dynamics — where does the orbit go? — is answered, in the topological setting, by the omega-limit set.

**Definition 6.1.2.** The *omega-limit set* of $x$ is
$$\omega_f(x) = \bigcap_{N \geq 0} \overline{\{f^n(x) : n \geq N\}} = \{y : f^{n_k}(x) \to y \text{ for some } n_k \to \infty\}.$$

Read this definition carefully. The omega-limit set is not just the set of accumulation points of the orbit — it's the set of accumulation points that persist no matter how far along the orbit you start. The inner definition, $\{y : f^{n_k}(x) \to y\}$, captures exactly this: $y \in \omega_f(x)$ means the orbit returns arbitrarily close to $y$ infinitely often.

**Proposition 6.1.3.** For compact $X$ and continuous $f$:
1. $\omega_f(x)$ is nonempty, closed, and $f$-invariant ($f(\omega_f(x)) \subseteq \omega_f(x)$).
2. If $f$ is a homeomorphism, $f(\omega_f(x)) = \omega_f(x)$ (positively and negatively invariant).
3. $\omega_f(x)$ is connected if $X$ is connected.

*(proof of 1)* Nonempty: the sequence $(f^n(x))_n$ has a convergent subsequence by compactness. Closed: direct from definition as an intersection of closed sets. Invariance: if $y = \lim_k f^{n_k}(x)$, then $f(y) = \lim_k f^{n_k+1}(x) \in \omega_f(x)$.

The proof is a short exercise in topology, but the conclusion is important: every orbit, no matter how complicated, has a well-defined "eventual home." That home is compact, closed, and preserved by the map.

---

## Periodic Points

Among the simplest orbits are periodic ones — those that eventually return exactly to where they started.

**Definition 6.1.4.** A point $x$ is *periodic* with (minimal) period $n \geq 1$ if $f^n(x) = x$ and $f^k(x) \neq x$ for $0 < k < n$. A period-1 point is a *fixed point*.

The set of periodic points of $f$ is $\text{Per}(f) = \bigcup_{n \geq 1} \text{Fix}(f^n)$.

Periodic points are not just a special case — they're often a kind of skeleton for the whole system. When periodic points are dense, they constrain the behavior of every orbit. One of the hallmarks of chaos (as we'll see in Section 6.7) is having periodic points everywhere, yet behaving unpredictably in between them.

**Example 6.1.5 (Quadratic Maps).** For $f_c: {\mathbb R} \to {\mathbb R}$, $f_c(x) = x^2 + c$:
- $c = 0$: $x = 0$ is a fixed point (attracting); $x = 1$ is also fixed (repelling).
- $c = -2$: $f_{-2}$ on $[-2, 2]$ has dense periodic points (topologically conjugate to the tent map).
- $c = -1$: period-2 orbit at $\{0, -1\}$.

The quadratic family $f_c$ is the standard testing ground for one-dimensional dynamics. By varying the single parameter $c$, you can produce fixed points, period-2 orbits, period-4 orbits, chaos, and everything in between. We'll return to this family repeatedly throughout Part II.

The next section asks a more subtle question than "does the orbit return exactly?" It asks: does the orbit *come back close*?
