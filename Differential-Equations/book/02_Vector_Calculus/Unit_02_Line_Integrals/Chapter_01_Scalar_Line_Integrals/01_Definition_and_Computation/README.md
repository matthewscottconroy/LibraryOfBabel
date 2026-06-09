# Definition and Computation of Scalar Line Integrals

Consider a thin wire bent into the shape of a helix. The wire has variable density — denser at the bottom, less dense near the top. To find the wire's total mass, you cannot simply multiply density by length (the density is not constant) and you cannot use an ordinary one-variable integral (the wire is not a straight segment). What you need is a device for summing the product of density and length element along a curve in three-dimensional space. That device is the scalar line integral.

## Informal Definition via Riemann Sums

Let $C$ be a smooth curve in $\mathbb{R}^n$ and $f$ a continuous scalar function defined on $C$. Subdivide $C$ into $n$ arcs of lengths $\Delta s_1, \Delta s_2, \ldots, \Delta s_n$. In each arc, pick a sample point $\mathbf{p}_i^*$. Form the Riemann sum

$$\sum_{i=1}^n f(\mathbf{p}_i^*)\,\Delta s_i.$$

As the maximum arc length $\max \Delta s_i \to 0$, this sum converges (under appropriate continuity conditions on $f$ and $C$) to the **scalar line integral**

$$\int_C f\,ds.$$

## Formal Definition via Parametrization

To compute the integral, we use a parametrization. Let $\mathbf{r}: [a, b] \to \mathbb{R}^n$ be a $C^1$ parametrization of $C$ with $|\mathbf{r}'(t)| > 0$ for all $t \in (a,b)$ (the curve is traversed without stopping). Then

$$\int_C f\,ds = \int_a^b f(\mathbf{r}(t))\,|\mathbf{r}'(t)|\,dt.$$

**Why this formula.** The arc length element satisfies $ds = |\mathbf{r}'(t)|\,dt$: in a small time interval $[t, t+dt]$, the curve moves approximately $|\mathbf{r}'(t)|\,dt$ units of length. Substituting into the Riemann sum and passing to the limit yields the formula.

**Independence from parametrization.** Suppose $\mathbf{r}_1$ and $\mathbf{r}_2$ are two smooth parametrizations of the same curve $C$ (with possibly different parameter intervals). Then a change-of-variables argument shows that the two formulas yield the same value. The scalar line integral is a property of the curve and the function, not of the choice of parameter.

**Independence from direction.** Reversing the direction of traversal replaces $\mathbf{r}(t)$ with $\mathbf{r}(a + b - t)$, but $|\mathbf{r}'(a+b-t)| = |\mathbf{r}'(a+b-t)|$ and the integral over the parameter interval is unchanged after the substitution $t \mapsto a+b-t$.

## Computation in Two Dimensions

For $\mathbf{r}(t) = (x(t), y(t))$, we have $|\mathbf{r}'(t)| = \sqrt{(x'(t))^2 + (y'(t))^2}$, and

$$\int_C f\,ds = \int_a^b f(x(t), y(t))\sqrt{[x'(t)]^2 + [y'(t)]^2}\,dt.$$

**Example 1.** Let $C$ be the upper semicircle of radius 2 from $(2,0)$ to $(-2,0)$, and $f(x,y) = y$.

Parametrize: $\mathbf{r}(t) = (2\cos t, 2\sin t)$, $t \in [0, \pi]$.

$|\mathbf{r}'(t)| = |(-2\sin t, 2\cos t)| = 2$.

$$\int_C f\,ds = \int_0^\pi (2\sin t)\cdot 2\,dt = 4\int_0^\pi \sin t\,dt = 4[-\cos t]_0^\pi = 4(1 + 1) = 8.$$

**Interpretation.** If $f(x,y) = y$ represents the density (mass per unit length) of a semicircular wire, the total mass is 8 (in appropriate units).

**Example 2.** Let $C$ be the line segment from $(0,0)$ to $(1,2)$, and $f(x,y) = x + y^2$.

Parametrize: $\mathbf{r}(t) = (t, 2t)$, $t \in [0,1]$.

$|\mathbf{r}'(t)| = |(1, 2)| = \sqrt{5}$.

$$\int_C f\,ds = \int_0^1 (t + (2t)^2)\sqrt{5}\,dt = \sqrt{5}\int_0^1 (t + 4t^2)\,dt = \sqrt{5}\left[\frac{t^2}{2} + \frac{4t^3}{3}\right]_0^1 = \sqrt{5}\cdot\frac{11}{6}.$$

## Computation in Three Dimensions

For $\mathbf{r}(t) = (x(t), y(t), z(t))$, $|\mathbf{r}'(t)| = \sqrt{[x'(t)]^2 + [y'(t)]^2 + [z'(t)]^2}$, and

$$\int_C f\,ds = \int_a^b f(x(t),y(t),z(t))\sqrt{[x']^2+[y']^2+[z']^2}\,dt.$$

**Example 3.** Let $C$ be the helix $\mathbf{r}(t) = (\cos t, \sin t, t)$, $t \in [0, 2\pi]$, and $f(x,y,z) = z$.

$|\mathbf{r}'(t)| = |(-\sin t, \cos t, 1)| = \sqrt{\sin^2 t + \cos^2 t + 1} = \sqrt{2}$.

$$\int_C f\,ds = \int_0^{2\pi} t \cdot \sqrt{2}\,dt = \sqrt{2}\cdot\frac{(2\pi)^2}{2} = 2\pi^2\sqrt{2}.$$

If the helix is a wire with density proportional to height $z$, then $2\pi^2\sqrt{2}$ (times the proportionality constant) is the total mass.

## Piecewise Smooth Curves

If $C$ is a piecewise smooth curve — the union of smooth arcs $C_1, C_2, \ldots, C_n$ joined end to end — then

$$\int_C f\,ds = \int_{C_1} f\,ds + \int_{C_2} f\,ds + \cdots + \int_{C_n} f\,ds.$$

**Example 4.** Let $C$ consist of the segment from $(0,0)$ to $(1,0)$ (call it $C_1$) followed by the segment from $(1,0)$ to $(1,1)$ (call it $C_2$). Integrate $f(x,y) = x + y$.

$C_1$: $\mathbf{r}(t) = (t, 0)$, $t \in [0,1]$, $|\mathbf{r}'| = 1$. $\int_{C_1} f\,ds = \int_0^1 t\,dt = 1/2$.

$C_2$: $\mathbf{r}(t) = (1, t)$, $t \in [0,1]$, $|\mathbf{r}'| = 1$. $\int_{C_2} f\,ds = \int_0^1 (1+t)\,dt = 3/2$.

Total: $\int_C f\,ds = 1/2 + 3/2 = 2$.

## Using the Natural Parametrization

When a curve is given in a form other than explicit parametrization, it may be convenient to use the curve's natural description directly.

**Curves of the form $y = g(x)$.** Parametrize by $\mathbf{r}(t) = (t, g(t))$, so $|\mathbf{r}'(t)| = \sqrt{1 + [g'(t)]^2}$, and

$$\int_C f\,ds = \int_a^b f(x, g(x))\sqrt{1 + [g'(x)]^2}\,dx.$$

**Polar curves $r = h(\theta)$.** Use $\mathbf{r}(\theta) = (h(\theta)\cos\theta, h(\theta)\sin\theta)$. Then

$$|\mathbf{r}'(\theta)| = \sqrt{[h'(\theta)]^2 + [h(\theta)]^2},$$

and the scalar line integral becomes a definite integral in $\theta$.

## Common Mistakes

**Forgetting the speed factor.** The most frequent error is writing $\int_a^b f(\mathbf{r}(t))\,dt$ without the $|\mathbf{r}'(t)|$ factor. This mistake changes the value of the integral unless the curve happens to be parametrized at unit speed.

**Confusing scalar and vector line integrals.** The scalar line integral uses $ds = |\mathbf{r}'|\,dt$, a positive scalar. The vector line integral uses $d\mathbf{r} = \mathbf{r}'(t)\,dt$, a vector. These measure completely different things.

## Summary

The scalar line integral $\int_C f\,ds$ accumulates the values of a scalar function along a curve weighted by arc length. It is computed by choosing any smooth parametrization $\mathbf{r}(t)$ and evaluating $\int_a^b f(\mathbf{r}(t))|\mathbf{r}'(t)|\,dt$. The result is independent of both the parametrization and the direction of traversal. Its primary physical application is integrating a density function along a curve to find total mass or similar extensive quantities. The arc length formula (the special case $f=1$) is its most fundamental instance.
