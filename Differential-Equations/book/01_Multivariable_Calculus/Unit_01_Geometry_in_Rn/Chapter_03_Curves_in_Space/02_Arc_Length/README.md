# Arc Length

How long is a curve? For a straight line segment, the answer is immediate. But for a spiral, a helix, or any other winding path through space, one needs calculus. The arc length integral is the answer: it accumulates the infinitesimal distances $\|\mathbf{r}'(t)\|\,dt$ traveled in each moment $dt$ over the entire parameter range. This integral generalizes the one-variable arc length formula and sets up the intrinsically geometric parameterization of a curve by arc length, which is the natural setting for curvature and the Frenet-Serret theory.

## The Arc Length Formula

Let $\mathbf{r}: [a, b] \to \mathbb{R}^n$ be a smooth curve. The **arc length** of $\mathbf{r}$ from $t = a$ to $t = b$ is

$$L = \int_a^b \|\mathbf{r}'(t)\|\,dt = \int_a^b \sqrt{(x'(t))^2 + (y'(t))^2 + (z'(t))^2}\,dt.$$

**Derivation.** Partition $[a, b]$ into $n$ subintervals with endpoints $t_0 = a < t_1 < \cdots < t_n = b$. Approximate the arc length by the length of the polygonal path connecting the points $\mathbf{r}(t_0), \mathbf{r}(t_1), \ldots, \mathbf{r}(t_n)$. The length of the $k$-th segment is $\|\mathbf{r}(t_k) - \mathbf{r}(t_{k-1})\|$. By the mean value theorem applied componentwise, this is approximately $\|\mathbf{r}'(\xi_k)\|\Delta t_k$ for some $\xi_k \in [t_{k-1}, t_k]$. Summing and taking the limit as the partition is refined gives the integral above.

**Reparameterization invariance.** The arc length does not depend on the parameterization. If $\mathbf{s}(u) = \mathbf{r}(\phi(u))$ with $\phi$ a smooth bijection and $\phi'(u) > 0$ (orientation-preserving), then by the chain rule and substitution, $\int \|\mathbf{s}'(u)\|\,du = \int \|\mathbf{r}'(t)\|\,dt$. This confirms that arc length is a geometric property of the curve, not of how it is parameterized.

## Worked Examples

**Example 1: Circle of radius $R$.**

$\mathbf{r}(t) = (R\cos t, R\sin t)$, $t \in [0, 2\pi]$. $\mathbf{r}'(t) = (-R\sin t, R\cos t)$, $\|\mathbf{r}'(t)\| = R$.

$$L = \int_0^{2\pi} R\,dt = 2\pi R.$$

The well-known circumference formula falls directly from the arc length integral.

**Example 2: Circular helix.**

$\mathbf{r}(t) = (a\cos t, a\sin t, bt)$, $t \in [0, 2\pi]$. $\|\mathbf{r}'(t)\| = \sqrt{a^2\sin^2 t + a^2\cos^2 t + b^2} = \sqrt{a^2 + b^2}$.

$$L = \int_0^{2\pi}\sqrt{a^2+b^2}\,dt = 2\pi\sqrt{a^2+b^2}.$$

If $b = 0$, this reduces to the circumference of a circle of radius $a$.

**Example 3: Parabolic arc.**

$\mathbf{r}(t) = (t, t^2)$, $t \in [0, 1]$. $\|\mathbf{r}'(t)\| = \sqrt{1 + 4t^2}$.

$$L = \int_0^1 \sqrt{1+4t^2}\,dt.$$

This integral requires the substitution $2t = \tan u$ or the formula for $\int\sqrt{1+u^2}\,du$; the result involves $\ln$ and is approximately $1.479$.

## The Arc Length Function and Reparameterization

Given a fixed basepoint $t_0 \in [a, b]$, define the **arc length function**

$$s(t) = \int_{t_0}^t \|\mathbf{r}'(u)\|\,du.$$

By the fundamental theorem of calculus, $s'(t) = \|\mathbf{r}'(t)\|$, which is the speed. If the curve is regular ($\|\mathbf{r}'(t)\| > 0$ everywhere), then $s(t)$ is strictly increasing and hence invertible: there is a function $t = t(s)$ such that $s(t(s)) = s$.

The **arc length reparameterization** of $\mathbf{r}$ is $\tilde{\mathbf{r}}(s) = \mathbf{r}(t(s))$. By the chain rule:

$$\tilde{\mathbf{r}}'(s) = \mathbf{r}'(t(s))\cdot t'(s) = \mathbf{r}'(t(s))\cdot\frac{1}{s'(t(s))} = \frac{\mathbf{r}'(t(s))}{\|\mathbf{r}'(t(s))\|}.$$

Thus $\|\tilde{\mathbf{r}}'(s)\| = 1$ for all $s$: the arc length parameterization traverses the curve at unit speed. The vector $\tilde{\mathbf{r}}'(s)$ is the **unit tangent vector** $\mathbf{T}(s)$.

The arc length parameterization is the natural, intrinsic parameterization of a curve — independent of any arbitrary choice of how fast to traverse it — and is the setting in which curvature and torsion are most cleanly defined.

## Arc Length in Practice

Computing the arc length parameterization explicitly is often algebraically intractable: one must integrate $\|\mathbf{r}'(t)\|$ and then invert the result, which is only possible in closed form for special curves (circles, lines, helices). For this reason, theoretical results about curvature and torsion are stated in the arc length parameterization, but actual computations use the original parameterization.

**Example: Helix in arc length.** For $\mathbf{r}(t) = (a\cos t, a\sin t, bt)$ with $c = \sqrt{a^2+b^2}$, the arc length from $t = 0$ is $s = ct$, so $t = s/c$ and

$$\tilde{\mathbf{r}}(s) = \left(a\cos\frac{s}{c},\; a\sin\frac{s}{c},\; \frac{b}{c}s\right).$$

One can verify $\|\tilde{\mathbf{r}}'(s)\| = 1$ directly: $\tilde{\mathbf{r}}'(s) = (-a\sin(s/c)/c, a\cos(s/c)/c, b/c)$, and $\|()\|^2 = a^2/c^2 + b^2/c^2 = (a^2+b^2)/c^2 = 1$.

## Common Pitfalls

The integrand is $\|\mathbf{r}'(t)\|$, the magnitude of the velocity, not the velocity itself. The arc length is a scalar (a number), not a vector.

Students sometimes forget to take the square root when computing $\|\mathbf{r}'(t)\|$, computing $\int (x'^2 + y'^2 + z'^2)\,dt$ instead of $\int \sqrt{x'^2 + y'^2 + z'^2}\,dt$. The former has no geometric meaning.

Finally, arc length integrals are often not computable in elementary terms. This is not a sign of error — it is an inherent feature of the problem. The integrand $\sqrt{1 + (f'(x))^2}$ for a graph $y = f(x)$ is only integrable in closed form for special functions $f$.
