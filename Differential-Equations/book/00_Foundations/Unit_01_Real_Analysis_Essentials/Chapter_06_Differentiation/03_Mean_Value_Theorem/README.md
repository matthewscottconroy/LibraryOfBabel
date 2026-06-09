# The Mean Value Theorem

The Mean Value Theorem (MVT) is the bridge between the local notion of a derivative — defined at a single point — and global information about a function over an interval. It states that for any smooth function, the instantaneous rate of change at some interior point equals the average rate of change over the whole interval. From this single statement, a remarkable array of consequences follow: monotonicity criteria, uniqueness arguments for ODEs, error estimates for numerical methods, and the entire theory of antiderivatives.

## Rolle's Theorem

The MVT is proved via Rolle's theorem, which handles the special case where the function values at the endpoints are equal.

**Theorem (Rolle).** Let $f$ be continuous on $[a,b]$, differentiable on $(a,b)$, and $f(a) = f(b)$. Then there exists $c \in (a,b)$ with $f'(c) = 0$.

*Proof.* By the Extreme Value Theorem, $f$ attains its maximum $M$ and minimum $m$ on $[a,b]$.

- If $M = m$: $f$ is constant, so $f'(c) = 0$ for all $c \in (a,b)$.
- If $M > m$: at least one of the extrema is attained at an interior point $c \in (a,b)$ (if both were at endpoints, $M = m$ since $f(a) = f(b)$). At an interior extremum, Fermat's theorem (below) gives $f'(c) = 0$. $\square$

**Fermat's Theorem.** If $f$ is differentiable at $c$ and $c$ is a local extremum of $f$, then $f'(c) = 0$.

*Proof.* If $c$ is a local max, then for $h > 0$ small, $f(c+h) \leq f(c)$, so $(f(c+h)-f(c))/h \leq 0$; taking $h \to 0^+$, $f'(c) \leq 0$. For $h < 0$ small, $(f(c+h)-f(c))/h \geq 0$; taking $h \to 0^-$, $f'(c) \geq 0$. So $f'(c) = 0$. $\square$

Note: $f'(c) = 0$ does not imply $c$ is a local extremum ($f(x) = x^3$ at $c = 0$). Fermat's theorem gives a necessary, not sufficient, condition.

## The Mean Value Theorem

**Theorem (Mean Value Theorem).** Let $f$ be continuous on $[a,b]$ and differentiable on $(a,b)$. Then there exists $c \in (a,b)$ with
$$f'(c) = \frac{f(b) - f(a)}{b - a}.$$

*Proof.* The right-hand side is the slope of the secant line from $(a, f(a))$ to $(b, f(b))$. Define the "adjusted" function
$$g(x) = f(x) - f(a) - \frac{f(b)-f(a)}{b-a}(x-a).$$
This $g$ satisfies $g(a) = 0 = g(b)$, and $g$ is continuous on $[a,b]$ and differentiable on $(a,b)$. By Rolle's theorem, there exists $c \in (a,b)$ with $g'(c) = 0$. But $g'(x) = f'(x) - \frac{f(b)-f(a)}{b-a}$, so $g'(c) = 0$ gives $f'(c) = \frac{f(b)-f(a)}{b-a}$. $\square$

## Consequences of the MVT

**Corollary 1.** If $f'(x) = 0$ for all $x \in (a,b)$, then $f$ is constant on $(a,b)$.

*Proof.* For any $x \in (a,b)$, apply MVT on $[a,x]$: $f(x) - f(a) = f'(c)(x-a) = 0$. $\square$

**Corollary 2.** If $f'(x) > 0$ for all $x \in (a,b)$, then $f$ is strictly increasing on $(a,b)$.

*Proof.* For $a < x < y < b$, MVT gives $f(y) - f(x) = f'(c)(y-x) > 0$. $\square$

Similarly, $f' < 0$ implies strictly decreasing.

**Corollary 3 (Uniqueness for ODEs).** If $f'(x) = g'(x)$ for all $x \in (a,b)$, then $f - g$ is constant.

*Proof.* $(f-g)' = 0$, so $f - g$ is constant. $\square$

This implies that any two antiderivatives of the same function differ by a constant — the foundation of the Fundamental Theorem of Calculus.

**Corollary 4 (Lipschitz bound).** If $|f'(x)| \leq K$ for all $x \in (a,b)$, then $|f(x) - f(y)| \leq K|x-y|$ for all $x, y \in (a,b)$. That is, $f$ is Lipschitz with constant $K$.

This is the connection between bounded derivatives and Lipschitz continuity. The Picard-Lindelof theorem requires the right-hand side $f(t,y)$ to be Lipschitz in $y$; by this corollary, boundedness of $\partial f/\partial y$ is sufficient.

## The Cauchy Mean Value Theorem

**Theorem.** Let $f$ and $g$ be continuous on $[a,b]$ and differentiable on $(a,b)$, with $g'(x) \neq 0$ on $(a,b)$. Then there exists $c \in (a,b)$ with
$$\frac{f'(c)}{g'(c)} = \frac{f(b)-f(a)}{g(b)-g(a)}.$$

*Proof.* Apply Rolle's theorem to $h(x) = f(x) - f(a) - \frac{f(b)-f(a)}{g(b)-g(a)}(g(x)-g(a))$. $\square$

The Cauchy MVT reduces to the ordinary MVT when $g(x) = x$, and it is the key to proving L'Hopital's rule.

## Application to Error Estimates

**Taylor's error bound (preview).** The MVT gives $f(x) = f(a) + f'(c)(x-a)$ for some $c$ between $a$ and $x$. If $|f'| \leq M$ on $[a,b]$, then $|f(x) - f(a)| \leq M|x-a|$. This is the simplest case of the Taylor remainder estimate.

**Euler's method.** In the Euler method for $y' = f(t,y)$ with step size $h$, the local truncation error is $y(t+h) - y(t) - hf(t,y(t))$. By Taylor's theorem (essentially a refined MVT), this error is $O(h^2)$. The global error over $n = T/h$ steps is $O(h)$.

## Common Pitfalls

**Assuming the MVT gives a unique $c$.** The theorem asserts existence of at least one $c$; there may be many.

**Applying the MVT on an open interval.** The function must be continuous on the closed interval $[a,b]$ and differentiable on the open interval $(a,b)$. If differentiability fails at an endpoint, the MVT is still valid.

**Confusing $f'(c) = 0$ with a local extremum.** Fermat's theorem says extrema have zero derivative; it does not say zero derivative implies an extremum.
