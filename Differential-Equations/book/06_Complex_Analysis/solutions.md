# Solutions: Complex Analysis

## Problem 1: Cauchy-Riemann Equations and Analyticity

**Problem.** Verify that $f(z) = z^3$ satisfies the Cauchy-Riemann equations and find the real and imaginary parts.

**Solution.** $z = x+iy$, $z^3 = (x+iy)^3 = x^3 + 3x^2(iy) + 3x(iy)^2 + (iy)^3 = x^3 - 3xy^2 + i(3x^2y - y^3)$.

$u = x^3 - 3xy^2$, $v = 3x^2y - y^3$.

Cauchy-Riemann: $u_x = 3x^2 - 3y^2$ and $v_y = 3x^2 - 3y^2$. Equal. $u_y = -6xy$ and $-v_x = -6xy$. Equal. Verified.

Derivative: $f'(z) = u_x + iv_x = (3x^2-3y^2) + i(6xy) = 3(x^2 - y^2 + 2ixy) = 3(x+iy)^2 = 3z^2$. Consistent with power rule.

---

## Problem 2: Contour Integration via Cauchy's Formula

**Problem.** Compute $\oint_{|z|=2}\frac{e^z}{z^2(z-1)}\,dz$.

**Solution.** Singularities inside $|z| = 2$: $z = 0$ (pole of order 2) and $z = 1$ (simple pole).

**Residue at $z=1$:** $\text{Res}(f;1) = \lim_{z\to 1}\frac{e^z}{z^2} = e$.

**Residue at $z=0$ (order-2 pole):** $\text{Res}(f;0) = \lim_{z\to 0}\frac{d}{dz}\!\left[z^2\cdot\frac{e^z}{z^2(z-1)}\right] = \lim_{z\to 0}\frac{d}{dz}\frac{e^z}{z-1}$.

$\frac{d}{dz}\frac{e^z}{z-1} = \frac{e^z(z-1) - e^z}{(z-1)^2} = \frac{e^z(z-2)}{(z-1)^2}$.

At $z=0$: $e^0(0-2)/(0-1)^2 = -2$.

By the Residue Theorem: $\oint = 2\pi i(\text{Res at }0 + \text{Res at }1) = 2\pi i(-2+e)$.

---

## Problem 3: Real Integral by Residues

**Problem.** Evaluate $\int_{-\infty}^\infty\frac{\cos x}{x^2+1}\,dx$.

**Solution.** Consider $f(z) = e^{iz}/(z^2+1)$. Poles: $z = \pm i$. For the upper half-plane contour, only $z = i$ is inside.

$\text{Res}(f;i) = \lim_{z\to i}\frac{(z-i)e^{iz}}{(z-i)(z+i)} = \frac{e^{i\cdot i}}{2i} = \frac{e^{-1}}{2i}$.

Semicircular arc: for $z = Re^{i\theta}$, $|e^{iz}| = e^{-R\sin\theta} \to 0$ uniformly for $\theta \in (0,\pi)$ as $R\to\infty$ (Jordan's lemma). So the arc contributes zero.

$\int_{-\infty}^\infty\frac{e^{ix}}{x^2+1}\,dx = 2\pi i\cdot\frac{e^{-1}}{2i} = \frac{\pi}{e}$.

Taking real parts: $\int_{-\infty}^\infty\frac{\cos x}{x^2+1}\,dx = \frac{\pi}{e}$.

(Taking imaginary parts: $\int_{-\infty}^\infty\frac{\sin x}{x^2+1}\,dx = 0$, which is expected since the integrand is odd.)

---

## Problem 4: Laurent Series and Classification of Singularities

**Problem.** Find the Laurent series of $f(z) = \frac{e^z - 1}{z^3}$ near $z = 0$ and classify the singularity.

**Solution.** $e^z = 1 + z + z^2/2! + z^3/3! + z^4/4! + \cdots$

$e^z - 1 = z + z^2/2 + z^3/6 + z^4/24 + \cdots$

$f(z) = \frac{z + z^2/2 + z^3/6 + z^4/24 + \cdots}{z^3} = z^{-2} + \frac{1}{2}z^{-1} + \frac{1}{6} + \frac{z}{24} + \cdots$

Principal part: $z^{-2} + (1/2)z^{-1}$. This terminates at order $-2$, so $z = 0$ is a **pole of order 2**.

Residue: the coefficient of $z^{-1}$ is $\boxed{1/2}$.

**Common mistake.** Confusing the order of the pole with the leading power. A pole of order $m$ means the principal part has finitely many terms, with the lowest term being $z^{-m}$.

---

## Problem 5: Analytic Continuation and Monodromy

**Problem.** Describe the analytic continuation of $f(z) = \sqrt{z} = e^{(1/2)\log z}$ around a loop encircling the origin.

**Solution.** Starting with the principal branch $\sqrt{z} = e^{(1/2)\text{Log}(z)}$ (using $\text{Log}$ = principal logarithm, $\text{arg} \in (-\pi, \pi)$). At $z = 1$: $\sqrt{1} = 1$.

Analytically continue along the circle $|z| = 1$: at angle $\theta$, $\sqrt{re^{i\theta}} = \sqrt{r}e^{i\theta/2}$.

After a full loop $\theta: 0 \to 2\pi$: the function value at $z = 1$ becomes $e^{i\cdot 2\pi/2} = e^{i\pi} = -1$.

A second loop returns to $1$. So $z = 0$ is a **branch point of order 2**: the function changes value after one loop and returns after two. The monodromy is multiplication by $-1$.

The standard resolution: introduce a branch cut (e.g., the negative real axis) and restrict $\text{arg}$ to $(-\pi, \pi)$. This gives a single-valued analytic function on $\mathbb{C}\setminus(-\infty, 0]$. Alternatively, work on the two-sheeted Riemann surface of $\sqrt{z}$.

---

## Problem 6: Conformal Mapping

**Problem.** Find a conformal map from the upper half-plane $\text{Im}(z) > 0$ to the strip $0 < \text{Im}(w) < 1$.

**Solution.** The map $w = \log(z)/\pi$ (with log = principal logarithm) maps the upper half-plane as follows:

For $z = re^{i\theta}$ with $\theta \in (0,\pi)$: $w = (\ln r + i\theta)/\pi$. So $\text{Re}(w) = \ln r/\pi$ (ranges over all of $\mathbb{R}$) and $\text{Im}(w) = \theta/\pi \in (0,1)$.

This maps the upper half-plane bijectively to the horizontal strip $\{0 < \text{Im}(w) < 1\}$.

Conformal: $dw/dz = 1/(\pi z) \neq 0$ for $z \neq 0$ in the upper half-plane.

**Application.** Harmonic functions on the strip can be transported to the upper half-plane (where they are simpler to solve), then mapped back. This is the method of conformal mapping for solving Laplace's equation on strips.
