# Uniform Continuity

Continuity at a point allows the threshold $\delta$ to depend on both the tolerance $\varepsilon$ and the base point $a$. For some functions, the required $\delta$ becomes very small as the base point moves to certain parts of the domain: for instance, $f(x) = 1/x$ is continuous at each $a > 0$, but as $a \to 0^+$, the required $\delta$ shrinks to $0$ as well. Uniform continuity demands a single $\delta$ that works simultaneously at every point in the domain.

## Definition

**Definition.** A function $f: D \to \mathbb{R}$ is **uniformly continuous** on $D$ if
$$\forall \varepsilon > 0,\ \exists \delta > 0,\ \forall x, y \in D,\ |x-y| < \delta \Rightarrow |f(x) - f(y)| < \varepsilon.$$

The difference from pointwise continuity: $\delta$ depends only on $\varepsilon$, not on any particular base point. The same $\delta$ must work for all pairs $(x,y)$ simultaneously.

**Example.** $f(x) = 3x+1$ is uniformly continuous on $\mathbb{R}$: given $\varepsilon > 0$, choose $\delta = \varepsilon/3$; then $|x-y| < \delta \Rightarrow |f(x) - f(y)| = 3|x-y| < \varepsilon$, with $\delta$ independent of location.

**Non-example.** $f(x) = x^2$ is not uniformly continuous on $\mathbb{R}$. For $\varepsilon = 1$, suppose $\delta > 0$ were claimed to work. Choose $x = 1/\delta$ and $y = 1/\delta + \delta/2$. Then $|x - y| = \delta/2 < \delta$, but
$$|f(x) - f(y)| = |x^2 - y^2| = |x+y||x-y| = \left(\frac{2}{\delta} + \frac{\delta}{2}\right)\frac{\delta}{2} = 1 + \frac{\delta^2}{4} > 1 = \varepsilon.$$
No single $\delta$ works for all $x, y \in \mathbb{R}$. However, $x^2$ is uniformly continuous on any bounded interval $[-M, M]$, because $|x+y| \leq 2M$ bounds the rate of growth.

## The Heine-Cantor Theorem

**Theorem.** If $f$ is continuous on a closed bounded interval $[a,b]$, then $f$ is uniformly continuous on $[a,b]$.

*Proof.* Suppose for contradiction that $f$ is not uniformly continuous. Then there exists $\varepsilon > 0$ such that for each $n$, there exist $x_n, y_n \in [a,b]$ with $|x_n - y_n| < 1/n$ but $|f(x_n) - f(y_n)| \geq \varepsilon$. Since $(x_n) \subseteq [a,b]$ is bounded, by Bolzano-Weierstrass there is a subsequence $x_{n_k} \to c \in [a,b]$. Since $|y_{n_k} - x_{n_k}| < 1/n_k \to 0$, also $y_{n_k} \to c$. By continuity:
$$|f(x_{n_k}) - f(y_{n_k})| \leq |f(x_{n_k}) - f(c)| + |f(c) - f(y_{n_k})| \to 0.$$
But $|f(x_{n_k}) - f(y_{n_k})| \geq \varepsilon > 0$ for all $k$ — a contradiction. $\square$

This theorem is the reason that one does not need to worry about uniform continuity when working on closed intervals: continuity suffices. The distinction matters only on unbounded or open domains.

## Characterization of Non-Uniform Continuity

By negating the definition: $f$ is **not** uniformly continuous on $D$ if there exists $\varepsilon > 0$ and sequences $(x_n), (y_n) \in D$ with $|x_n - y_n| \to 0$ but $|f(x_n) - f(y_n)| \geq \varepsilon$.

**Example.** $f(x) = \sin(1/x)$ on $(0,1]$. Take $x_n = 1/(2\pi n)$ and $y_n = 1/(2\pi n + \pi/2)$. Then $|x_n - y_n| \to 0$, but $f(x_n) = 0$ and $f(y_n) = 1$, so $|f(x_n) - f(y_n)| = 1$ for all $n$. Not uniformly continuous.

## Lipschitz Continuity

A function $f: D \to \mathbb{R}$ is **Lipschitz** with constant $K > 0$ if
$$|f(x) - f(y)| \leq K|x-y| \quad \text{for all } x, y \in D.$$

**Proposition.** Lipschitz continuity implies uniform continuity.

*Proof.* Given $\varepsilon > 0$, choose $\delta = \varepsilon/K$. Then $|x-y| < \delta \Rightarrow |f(x)-f(y)| \leq K|x-y| < K \cdot (\varepsilon/K) = \varepsilon$. $\square$

The Lipschitz condition is stronger than uniform continuity. Examples of Lipschitz functions: linear functions (constant $K = $ slope), $\sin$ and $\cos$ (constant $K = 1$, since $|\sin x - \sin y| \leq |x-y|$ by the mean value theorem). The function $f(x) = \sqrt{x}$ is uniformly continuous on $[0,\infty)$ but not Lipschitz (its derivative is unbounded near $0$).

## Extension of Uniformly Continuous Functions

**Theorem.** If $f$ is uniformly continuous on $(a,b)$, then $f$ extends uniquely to a continuous function on $[a,b]$.

*Proof.* For any sequence $x_n \to a^+$ with $x_n \in (a,b)$: since $|x_m - x_n| \to 0$, uniform continuity gives $|f(x_m) - f(x_n)| \to 0$, so $(f(x_n))$ is Cauchy. Let $L_a = \lim f(x_n)$. The limit is independent of the sequence (verify by comparing two sequences), so define $f(a) = L_a$. Similarly define $f(b)$. The extended function is continuous at $a$ and $b$. $\square$

This theorem is not true for merely pointwise continuous functions: $f(x) = \sin(1/x)$ is continuous on $(0,1)$ but cannot be continuously extended to $[0,1]$.

## Connection to Integration

Uniform continuity is exactly what ensures that the Riemann sum approximation to $\int_a^b f(x)\,dx$ converges uniformly. For a Riemann sum with partition mesh $\|\mathcal{P}\|$, the error is bounded by $|b-a| \cdot \omega(f, \|\mathcal{P}\|)$ where $\omega(f, \delta) = \sup_{|x-y|<\delta} |f(x)-f(y)|$ is the **modulus of continuity**. Uniform continuity ensures $\omega(f,\delta) \to 0$ as $\delta \to 0$, making the approximation error go to zero uniformly.

## Connection to Differential Equations

The Picard-Lindelof existence theorem for $y' = f(t, y)$ requires $f$ to be Lipschitz in $y$ (uniformly in $t$) on a suitable rectangle. Lipschitz continuity in $y$ is what enables the Picard iteration to be a contraction. Without the Lipschitz condition, solutions may fail to be unique (the Peano existence theorem guarantees existence under mere continuity, but not uniqueness). Thus uniform continuity — in its Lipschitz form — is the boundary between existence and existence-with-uniqueness for ODEs.
