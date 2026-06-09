# Path Dependence and Discontinuity

The preceding sections introduced the definition of a multivariable limit and continuity. The most striking feature of these concepts in higher dimensions — and the most important source of counterexamples — is path dependence: the behavior of a function as $(x, y)$ approaches a point can depend radically on which direction or curve the approach follows. A function can appear to behave well along every straight line through a point while failing catastrophically to have a limit there. This section develops techniques for detecting and exploiting this phenomenon, and uses it to build a catalog of instructive discontinuities.

## The Path Method for Non-Existence of Limits

**Theorem (Necessary Condition for Limits).** If $\lim_{(x,y)\to(a,b)} f(x,y) = L$, then for every continuous curve $\boldsymbol{\gamma}: [0,1] \to \mathbb{R}^2$ with $\boldsymbol{\gamma}(0) = (a,b)$ and $\boldsymbol{\gamma}(t) \neq (a,b)$ for $t > 0$, the limit along $\boldsymbol{\gamma}$ equals $L$:

$$\lim_{t\to 0^+} f(\boldsymbol{\gamma}(t)) = L.$$

Contrapositive: if there exist two paths $\boldsymbol{\gamma}_1$ and $\boldsymbol{\gamma}_2$ approaching $(a,b)$ along which $f$ approaches different values, then $\lim_{(x,y)\to(a,b)} f(x,y)$ does not exist.

## Standard Path Families

The most commonly tested paths through the origin are:
- Horizontal: $y = 0$, approach along $(x, 0)$ as $x\to 0$.
- Vertical: $x = 0$, approach along $(0, y)$ as $y\to 0$.
- Lines: $y = mx$, approach along $(x, mx)$ as $x\to 0$.
- Parabolas: $y = kx^2$ (or $x = ky^2$).
- Cubics and higher-degree curves.
- Paths of the form $y = x^p$ for various $p > 0$.

The strategy is to substitute the path into $f$, simplify, and take the limit as the parameter approaches $0$. If the result depends on the slope $m$ or the coefficient $k$, then different choices give different limits, so the two-variable limit does not exist.

## Worked Example 1: Linear Paths Agree, Parabolic Path Differs

Consider $f(x,y) = \frac{xy^2}{x^2+y^4}$ for $(x,y)\neq(0,0)$.

**Along any line $y = mx$:**

$$f(x, mx) = \frac{x(mx)^2}{x^2+(mx)^4} = \frac{m^2x^3}{x^2+m^4x^4} = \frac{m^2x}{1+m^4x^2} \to \frac{0}{1} = 0.$$

**Along $y = 0$:** $f(x,0) = 0 \to 0$.

So the limit along every line through the origin is $0$.

**Along the parabola $x = y^2$:**

$$f(y^2, y) = \frac{y^2\cdot y^2}{(y^2)^2+y^4} = \frac{y^4}{y^4+y^4} = \frac{y^4}{2y^4} = \frac{1}{2}.$$

The limit along this parabolic path is $1/2 \neq 0$. Therefore $\lim_{(x,y)\to(0,0)} f(x,y)$ does not exist, despite agreement along all straight lines. This is the canonical demonstration that straight-line paths are not sufficient.

## Worked Example 2: Rotating Path

Consider $f(x,y) = \frac{x^2y}{x^4+y^2}$ for $(x,y)\neq(0,0)$.

**Along lines $y = mx$:** $f(x,mx) = \frac{x^2(mx)}{x^4+(mx)^2} = \frac{mx^3}{x^4+m^2x^2} = \frac{mx}{x^2+m^2} \to 0$.

**Along $y = x^2$:** $f(x,x^2) = \frac{x^2\cdot x^2}{x^4+x^4} = \frac{1}{2}$.

Again, the two-variable limit does not exist.

**Observation:** Both examples have the pattern that the "problematic path" is $x = y^p$ (or $y = x^q$) where $p, q$ match the powers in the denominator to cause cancellation. This is a useful heuristic for constructing counterexamples.

## A Continuous Function with Vanishing Partial Derivatives at a Discontinuity... Wait

Actually, here is a more dramatic example: a function that fails to be continuous at the origin even though all directional derivatives (not just partial derivatives along the axes) exist there.

Define $f(x,y) = \frac{xy}{x^2+y^2}$ for $(x,y)\neq(0,0)$ and $f(0,0) = 0$.

Along any line $y = mx$ through the origin: $f(x,mx) = \frac{x\cdot mx}{x^2+m^2x^2} = \frac{m}{1+m^2}$ for $x\neq 0$. This limit (as $x\to 0$) equals $m/(1+m^2)$, which depends on $m$. So the limit does not exist (different lines give different values, e.g., $m=0$ gives $0$ and $m=1$ gives $1/2$).

## Functions Continuous Along Lines but Not at a Point

The example above is discontinuous at the origin because different lines give different limits. It is possible to have a function where every directional limit agrees but the function is still discontinuous. Define (using polar coordinates):

$$f(r,\theta) = r\cdot h(\theta, r), \quad h(\theta, r) = \frac{\sin(2\theta)}{2}$$

This gives $f(x,y) = xy/(x^2+y^2)$, already shown to be discontinuous. For a function that is continuous along every straight line (including not just the limit but the function on the entire line) but discontinuous at the origin, one typically uses the topologist's sine curve in two variables, or functions whose "interesting" behavior occurs along curved paths.

## Path Independence and Existence of Limits: A Sufficient Condition

While the path method can only disprove existence of limits, there is a sufficient condition for existence: if $|f(x,y) - L| \leq g(r)$ where $r = \sqrt{x^2+y^2}$ and $g(r)\to 0$ as $r\to 0$, then the limit is $L$ by the squeeze theorem. Crucially, $g$ must not depend on $\theta$ (the direction of approach). If the bound is uniform in $\theta$, path independence follows automatically.

## Connection to Differential Equations

The notion of path dependence reappears in the study of line integrals. A vector field $\mathbf{F}$ is called **conservative** if its line integrals are path-independent — the integral $\int_C \mathbf{F}\cdot d\mathbf{r}$ depends only on the endpoints of the curve $C$, not on the path itself. Conservative fields are exactly the gradients of scalar functions. The analogy with limits is precise: just as a function can fail to have a limit at a point because different paths give different values, a vector field can fail to be conservative because different paths give different integrals.

## Summary of Techniques

To determine whether $\lim_{(x,y)\to(0,0)} f(x,y)$ exists:

1. **Try the squeeze theorem**: bound $|f|$ by a function of $r = \sqrt{x^2+y^2}$ alone. If the bound goes to $0$, the limit is $0$.

2. **Try polar coordinates**: substitute $x = r\cos\theta$, $y = r\sin\theta$ and examine whether the result depends on $\theta$ as $r\to 0$.

3. **Try paths**: compute limits along $y = mx$ (all slopes), along $y = kx^2$, along $x = ky^2$, etc. If two paths give different values, the limit does not exist.

4. **If all paths agree but the bound in step 1 involves $\theta$**: be very careful — paths agreeing does not imply the limit exists. Seek a parabolic or higher-order path that gives a different value.
