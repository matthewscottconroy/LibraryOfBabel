# The Intermediate Value Theorem

A continuous function cannot "jump over" a value. If $f$ is continuous on $[a,b]$ and $f(a) = 2$ while $f(b) = 5$, then $f$ must take the value $3$ somewhere in between — it cannot pass from $2$ to $5$ while skipping $3$, because to do so it would have to make a discontinuous leap. This is the Intermediate Value Theorem, and its proof is a direct application of the Completeness Axiom.

## Statement and Proof

**Theorem (Intermediate Value Theorem).** Let $f: [a,b] \to \mathbb{R}$ be continuous. If $c$ is any real number strictly between $f(a)$ and $f(b)$ (meaning $\min(f(a), f(b)) < c < \max(f(a), f(b))$), then there exists $x_0 \in (a,b)$ with $f(x_0) = c$.

*Proof.* Assume without loss of generality that $f(a) < c < f(b)$ (if $f(a) > f(b)$, apply the same argument to $-f$). Let
$$S = \{x \in [a,b] : f(x) \leq c\}.$$

$S$ is nonempty ($a \in S$ since $f(a) < c \leq c$) and bounded above by $b$. By the Completeness Axiom, $x_0 = \sup S$ exists.

We claim $f(x_0) = c$. 

**Case $f(x_0) < c$.** Since $f$ is continuous at $x_0$, there exists $\delta > 0$ with $|f(x) - f(x_0)| < c - f(x_0)$ for $|x - x_0| < \delta$. In particular, $f(x) < c$ for $x \in (x_0, x_0 + \delta)$, so these $x$ lie in $S$. But $x_0 + \delta/2 > x_0 = \sup S$ and $x_0 + \delta/2 \in S$, a contradiction.

**Case $f(x_0) > c$.** Similarly, there exists $\delta > 0$ with $f(x) > c$ for $|x - x_0| < \delta$. Then no element of $S$ lies in $(x_0 - \delta, x_0]$, so $x_0 - \delta/2$ is an upper bound of $S$ smaller than $x_0 = \sup S$, a contradiction.

Therefore $f(x_0) = c$. Since $f(a) < c < f(b)$, we have $x_0 \neq a$ and $x_0 \neq b$, so $x_0 \in (a,b)$. $\square$

## Why Continuity Is Essential

The IVT fails without continuity. The function
$$f(x) = \begin{cases} 0 & x \in [0,1] \\ 2 & x \in (1, 2] \end{cases}$$
satisfies $f(0) = 0$ and $f(2) = 2$ but never takes the value $1$.

## Applications: Existence of Solutions

The IVT is an existence theorem: it guarantees that solutions exist without constructing them. This philosophical role reappears throughout analysis.

**Corollary.** Every polynomial of odd degree has at least one real root.

*Proof.* Let $p(x) = x^{2n+1} + a_{2n}x^{2n} + \cdots + a_0$. For large $|x|$, the leading term dominates: $p(M) > 0$ and $p(-M) < 0$ for large $M$. By IVT, $p$ has a root in $(-M, M)$. $\square$

**Corollary.** If $f: [0,1] \to [0,1]$ is continuous, then $f$ has a fixed point (a point with $f(x_0) = x_0$).

*Proof.* Let $g(x) = f(x) - x$. Then $g(0) = f(0) \geq 0$ and $g(1) = f(1) - 1 \leq 0$. By IVT, there exists $x_0$ with $g(x_0) = 0$, i.e., $f(x_0) = x_0$. $\square$

This fixed-point result is the one-dimensional case of Brouwer's Fixed-Point Theorem, which underlies existence proofs for nonlinear ODEs via degree-theoretic methods.

**Corollary (Root Finding).** If $f$ is continuous on $[a,b]$ with $f(a)$ and $f(b)$ of opposite sign, there is a root of $f$ in $(a,b)$. The bisection method, which recursively halves the interval, converges to a root — and IVT guarantees that each halved interval still contains a root.

## Connection to the Completeness Axiom

The IVT is a consequence of completeness and would fail in $\mathbb{Q}$. The function $f(x) = x^2 - 2$ is continuous on $[1,2]$ (viewed as a function on $\mathbb{Q}$), with $f(1) = -1 < 0$ and $f(2) = 2 > 0$, but $f(x) = 0$ has no solution in $\mathbb{Q}$. The IVT fails in $\mathbb{Q}$ because $\sqrt{2} \notin \mathbb{Q}$.

## Intermediate Value Property vs. Continuity

The IVT says continuous functions have the "intermediate value property" (IVP): the image of any interval under $f$ is an interval. Surprisingly, the converse is false: there exist functions with the IVP that are not continuous at any point. (Darboux functions, defined using the axiom of choice.) So the IVP is a consequence but not a characterization of continuity.

However, for monotone functions, the IVP does imply continuity. This is used in the theory of inverse functions: if $f$ is strictly monotone and has the IVP, then $f$ is continuous and its inverse is also continuous.

## The IVT and Differential Equations

In ODE theory, existence proofs often reduce to showing that a certain operator has a fixed point. The Schauder fixed-point theorem (a generalization of the above corollary to infinite-dimensional spaces) is the main tool for existence when uniqueness fails. The IVT itself is used in simpler contexts: to show that a boundary value problem $y'' = f(x, y)$ with $y(a) = \alpha$, $y(b) = \beta$ has a solution, one considers the "shooting" function $s(\mu) = y(b; \mu) - \beta$, where $y(b; \mu)$ is the value at $b$ of the solution with initial slope $\mu$. If $s$ is continuous and changes sign, the IVT guarantees a solution.
