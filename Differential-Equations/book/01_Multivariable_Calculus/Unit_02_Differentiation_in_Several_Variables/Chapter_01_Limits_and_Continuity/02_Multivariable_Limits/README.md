# Multivariable Limits

In single-variable calculus, the limit $\lim_{x\to a} f(x) = L$ means that $f(x)$ can be made arbitrarily close to $L$ by taking $x$ sufficiently close to $a$. The condition is straightforward because the real line has only two directions of approach to any point: from the left or from the right. In $\mathbb{R}^n$ for $n \geq 2$, the situation is fundamentally different: a point $\mathbf{p}$ can be approached along straight lines in any direction, along curved paths, along spirals, and along infinitely more exotic trajectories. The limit $\lim_{\mathbf{x}\to\mathbf{p}} f(\mathbf{x}) = L$ requires the same value $L$ to result from every possible mode of approach. This additional constraint is the source of most of the interesting behavior of multivariable limits.

## The Formal Definition

Let $f: D \to \mathbb{R}$ be a function defined on a domain $D \subseteq \mathbb{R}^n$, and let $\mathbf{p}$ be a limit point of $D$ (not necessarily in $D$). We say $\lim_{\mathbf{x}\to\mathbf{p}} f(\mathbf{x}) = L$ if:

For every $\epsilon > 0$, there exists $\delta > 0$ such that for all $\mathbf{x} \in D$ with $0 < \|\mathbf{x} - \mathbf{p}\| < \delta$, we have $|f(\mathbf{x}) - L| < \epsilon$.

The condition $0 < \|\mathbf{x}-\mathbf{p}\|$ excludes the point $\mathbf{p}$ itself (the function need not even be defined there). The condition $\|\mathbf{x}-\mathbf{p}\| < \delta$ requires closeness in the Euclidean metric, which as noted captures approach from all directions simultaneously.

## Computing Limits: Algebraic Methods

For functions built from continuous operations — polynomials, rational functions, trigonometric functions, exponentials — limits can be computed by direct substitution whenever the function is defined at the limit point. If the function is not defined at $\mathbf{p}$ but a simplified form is, the usual algebraic techniques apply.

**Example.** $\lim_{(x,y)\to(1,2)} (x^2 + xy + y^2) = 1 + 2 + 4 = 7$. Direct substitution works because the function is a polynomial.

**Example.** $\lim_{(x,y)\to(0,0)} \frac{x^2 y}{x^2+y^2}$. Direct substitution gives $0/0$, indeterminate. Bound the expression: $\left|\frac{x^2 y}{x^2+y^2}\right| \leq \frac{x^2|y|}{x^2} = |y|$ (since $x^2+y^2 \geq x^2$). As $(x,y)\to(0,0)$, $|y|\to 0$, so the limit is $0$.

## The Squeeze Theorem

**Theorem.** If $|f(\mathbf{x}) - L| \leq g(\mathbf{x})$ for all $\mathbf{x}$ near $\mathbf{p}$, and if $\lim_{\mathbf{x}\to\mathbf{p}} g(\mathbf{x}) = 0$, then $\lim_{\mathbf{x}\to\mathbf{p}} f(\mathbf{x}) = L$.

This is the most practical tool for proving that a limit equals a given value. The key is finding an appropriate bounding function $g$ that goes to zero.

**Example.** $\lim_{(x,y)\to(0,0)} \frac{x^3 + y^3}{x^2+y^2}$.

Polar coordinates: let $x = r\cos\theta$, $y = r\sin\theta$. Then $|x^3+y^3| = |r^3\cos^3\theta + r^3\sin^3\theta| = r^3|\cos^3\theta+\sin^3\theta| \leq 2r^3$ (since $|\cos^3\theta|+|\sin^3\theta| \leq |\cos\theta|^2+|\sin\theta|^2 \leq 2$ isn't sharp, but $|\cos\theta| \leq 1$ gives $|\cos^3\theta|\leq 1$ and similarly for $\sin$). More directly, $|x^3+y^3| \leq |x|^3 + |y|^3 \leq (x^2+y^2)^{3/2} + (x^2+y^2)^{3/2} = 2r^3$ (using $|x|, |y| \leq r$). So $\left|\frac{x^3+y^3}{x^2+y^2}\right| \leq \frac{2r^3}{r^2} = 2r \to 0$ as $r\to 0$. The limit is $0$.

## The Path Method for Non-Existence

If $\lim_{\mathbf{x}\to\mathbf{p}} f(\mathbf{x}) = L$ exists, then $f(\mathbf{x})\to L$ along every path approaching $\mathbf{p}$. Equivalently: if there exist two paths approaching $\mathbf{p}$ along which $f$ approaches two different values, the limit does not exist.

**Example.** Show $f(x,y) = \frac{x^2 - y^2}{x^2+y^2}$ has no limit at $(0,0)$.

Along $y = 0$: $f(x,0) = x^2/x^2 = 1 \to 1$.
Along $x = 0$: $f(0,y) = -y^2/y^2 = -1 \to -1$.

Different limits along different paths, so $\lim_{(x,y)\to(0,0)} f(x,y)$ does not exist.

**Example (subtler).** Show $f(x,y) = \frac{xy^2}{x^2+y^4}$ has no limit at $(0,0)$.

Along $y = 0$: $f(x,0) = 0 \to 0$.
Along $y = x$ (with $x\to 0$): $f(x,x) = x^3/(x^2+x^4) = x/(1+x^2) \to 0$.
Along $x = y^2$: $f(y^2, y) = y^2\cdot y^2/(y^4+y^4) = y^4/(2y^4) = 1/2 \to 1/2$.

All straight lines give limit $0$, but the parabolic path $x = y^2$ gives limit $1/2$. Therefore the limit does not exist. This example shows that agreement along all straight lines is not sufficient for the limit to exist.

## Limits Using Polar Coordinates

For limits at the origin in $\mathbb{R}^2$, the substitution $x = r\cos\theta$, $y = r\sin\theta$ converts the limit to $\lim_{r\to 0^+}$, provided the resulting expression approaches the same value for all $\theta$.

**Caution:** If the limit in polar coordinates depends on $\theta$, the original limit does not exist. But if the polar-coordinate expression is bounded in absolute value by a function of $r$ alone that tends to $0$, then the limit is $0$ by the squeeze theorem.

## Properties of Limits

The standard limit laws hold in $\mathbb{R}^n$: if $\lim_{\mathbf{x}\to\mathbf{p}} f(\mathbf{x}) = L$ and $\lim_{\mathbf{x}\to\mathbf{p}} g(\mathbf{x}) = M$, then:
- $\lim (f + g) = L + M$
- $\lim (fg) = LM$
- $\lim (f/g) = L/M$ if $M \neq 0$

These follow immediately from the $\epsilon$-$\delta$ definition.

## Common Pitfalls

The most important pitfall is using the path method to try to prove a limit exists. Showing the limit is $L$ along several paths — even all straight lines — does not prove the limit exists; it only rules out certain modes of failure. To prove the limit is $L$, one must use the $\epsilon$-$\delta$ definition or the squeeze theorem.

Also, when the function is expressed in polar coordinates as $f(r, \theta)$, one must check that the bound on $|f|$ is independent of $\theta$. If $|f(r,\theta)| \leq g(r)$ for all $\theta$ and $g(r)\to 0$, then the limit is $0$. If the bound involves $\theta$ in a way that doesn't vanish, the squeeze theorem doesn't apply.
