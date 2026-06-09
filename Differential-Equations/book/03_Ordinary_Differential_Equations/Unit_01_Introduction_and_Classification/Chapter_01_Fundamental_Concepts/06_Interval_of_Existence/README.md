# Interval of Existence

Picard's theorem guarantees a solution on some interval $|x - x_0| \leq h$ around the initial point. But how large can this interval be? For linear equations, the answer is satisfyingly clean: the solution exists on the entire interval where the coefficients are defined. For nonlinear equations, the situation is more subtle and more interesting.

## The Maximal Interval

Given an IVP with a unique solution, the **maximal interval of existence** is the largest open interval $(\alpha, \beta)$ containing $x_0$ on which the solution can be defined. This interval is open, and the solution is defined on it but cannot be extended beyond it.

The maximal interval always exists and is unique. If $\phi_1$ is a solution on $(a_1, b_1)$ and $\phi_2$ is a solution on $(a_2, b_2)$, and both satisfy the same IVP, then by uniqueness they agree on the intersection, and together they define a solution on the union. Taking the union over all intervals of existence gives the maximal interval.

## The Blow-Up Theorem

**Theorem.** Let $y = \phi(x)$ be the unique maximal solution of $y' = f(x, y)$, $y(x_0) = y_0$, with maximal interval $(\alpha, \beta)$. If $\beta < \infty$, then $|\phi(x)| \to \infty$ as $x \to \beta^-$. Similarly, if $\alpha > -\infty$, then $|\phi(x)| \to \infty$ as $x \to \alpha^+$.

In plain terms: the only reason a solution can stop existing at a finite endpoint is that it blows up (becomes unbounded). A solution cannot simply "end" at a smooth point; if it is bounded, it must be extendable.

This theorem explains Example 3 from the previous section: the solution of $y' = y^2$, $y(0) = 1$ is $y = 1/(1-x)$, which blows up as $x \to 1^-$. The maximal interval is $(-\infty, 1)$.

## Proof Sketch

Suppose $|\phi(x)|$ remains bounded on $[\alpha, \beta)$. Then the solution curve stays in a compact set, on which $f$ is bounded (if $f$ is continuous). One can therefore show, by Picard's theorem applied at a point near $\beta$, that the solution can be extended slightly beyond $\beta$, contradicting maximality. Hence boundedness implies extendability, and the contrapositive gives the blow-up result.

## Linear Equations: Global Existence

For the linear equation $y' + p(x)y = q(x)$ with $p$ and $q$ continuous on an interval $I$ containing $x_0$, the solution exists on all of $I$. This follows because, in this case, $f(x, y) = -p(x)y + q(x)$ is linear in $y$, so the Lipschitz constant is simply $|p(x)|$, which is bounded on compact subintervals. The bound on $|y|$ that would be needed to stay in a compact set can be established directly, ruling out blow-up.

For the second-order equation $y'' + p(x)y' + q(x)y = g(x)$ with $p, q, g$ continuous on $I$, the same conclusion holds: the solution of any IVP at any $x_0 \in I$ exists on all of $I$. This is why one can always speak of "the solution" of a linear equation on the entire interval of continuity.

## Nonlinear Equations: Interval Depends on Data

For nonlinear equations, the maximal interval depends sensitively on the initial conditions. Consider $y' = y^2$:

- $y(0) = 1$: solution $y = 1/(1-x)$, exists on $(-\infty, 1)$.
- $y(0) = a$ for $a > 0$: solution $y = a/(1-ax)$, exists on $(-\infty, 1/a)$.
- $y(0) = -1$: solution $y = 1/(1-x) = -1/(x+1)$... let's redo: if $y(0) = -1$, then $-1/y = x + C$ gives $C = 1$, so $y = -1/(x+1)$, existing on $(-1, \infty)$.

Larger positive initial values give shorter intervals; negative initial values give solutions that persist to $+\infty$. The initial condition completely determines the interval.

## Examples of Finite-Time Blow-Up

**Example 1.** $y' = 1 + y^2$, $y(0) = 0$. Separating: $\arctan(y) = x + C$, so $y = \tan(x + C)$. Imposing $y(0) = 0$: $C = 0$, giving $y = \tan x$, which blows up at $x = \pi/2$. The maximal interval is $(-\pi/2, \pi/2)$.

**Example 2.** $y' = y^{4/3}$, $y(0) = 1$. Separating: $y^{-4/3}\,dy = dx$, so $-3y^{-1/3} = x + C$. With $y(0) = 1$: $C = -3$, so $y^{-1/3} = (3-x)/3 = 1 - x/3$, giving $y = (1 - x/3)^{-3}$. This blows up at $x = 3$, and the maximal interval is $(-\infty, 3)$.

## Global Existence Criteria for Nonlinear Equations

Several conditions ensure that nonlinear IVPs have global (all-time) solutions. One sufficient condition is a **linear growth bound**: if $|f(x, y)| \leq A(x)|y| + B(x)$ with $A, B$ integrable, then by Gronwall's inequality, $|y(x)|$ cannot blow up in finite time. The logistic equation $y' = ry(1 - y/K)$ satisfies this because the right side, while nonlinear, grows no faster than linearly in $y$ for large $|y|$ (it actually grows quadratically, but the negative $-ry^2/K$ term dominates, producing an effective linear bound).

## The Interval and the Direction Field

Geometrically, the blow-up theorem says that a solution curve, if it terminates at a finite value of $x$, must leave every bounded region of the plane. It cannot terminate at a smooth point; it must either escape to $y = +\infty$ or $y = -\infty$. In the direction field, one can sometimes see this: solution curves that start near a vertical asymptote of the field curve steeply away.

Understanding the interval of existence is practically important. When computing numerical solutions, one must not attempt to integrate past a blow-up point. When analyzing physical models, a finite blow-up time may represent a genuine physical singularity (collapse of a star, thermal runaway in a chemical reaction) or may signal a breakdown of the model's assumptions.
