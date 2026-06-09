# The Cauchy Integral Formula

The Cauchy integral formula is one of the most remarkable results in all of mathematics. It asserts that the value of an analytic function at any interior point of a region is completely determined by the function's values on the boundary of that region. This has no analogue in real analysis: a real differentiable function on an interval is not determined by its endpoint values. The formula transforms complex analysis into a subject where local information propagates globally, where boundary data controls interior behavior, and where integrals and function values are intimately linked.

## Statement and Proof

**Theorem (Cauchy Integral Formula).** Let $D$ be a simply connected domain with boundary $C$ (a simple closed contour traversed counterclockwise). If $f$ is analytic on and inside $C$, then for any $z_0$ in the interior of $C$:
$$f(z_0) = \frac{1}{2\pi i} \oint_C \frac{f(z)}{z - z_0}\, dz.$$

**Proof.** Since $z_0$ is in the interior of $C$, choose $\varepsilon > 0$ small enough that the circle $C_\varepsilon : |z - z_0| = \varepsilon$ lies entirely inside $C$. The function $g(z) = \frac{f(z)}{z - z_0}$ is analytic on the region between $C$ and $C_\varepsilon$ (the singularity $z_0$ is inside $C_\varepsilon$). By the deformation principle:
$$\oint_C \frac{f(z)}{z - z_0}\, dz = \oint_{C_\varepsilon} \frac{f(z)}{z - z_0}\, dz.$$

Now write $f(z) = f(z_0) + (f(z) - f(z_0))$:
$$\oint_{C_\varepsilon} \frac{f(z)}{z - z_0}\, dz = f(z_0)\oint_{C_\varepsilon} \frac{dz}{z - z_0} + \oint_{C_\varepsilon} \frac{f(z) - f(z_0)}{z - z_0}\, dz.$$

The first integral is $f(z_0) \cdot 2\pi i$ (computed in Chapter 01). For the second integral, since $f$ is continuous at $z_0$, for any $\eta > 0$ there exists $\varepsilon$ small enough that $|f(z) - f(z_0)| < \eta$ for all $z$ on $C_\varepsilon$. By ML:
$$\left|\oint_{C_\varepsilon} \frac{f(z) - f(z_0)}{z - z_0}\, dz\right| \leq \frac{\eta}{\varepsilon} \cdot 2\pi\varepsilon = 2\pi\eta.$$

Since $\eta$ is arbitrary, the second integral is $0$. Therefore:
$$\oint_C \frac{f(z)}{z - z_0}\, dz = 2\pi i f(z_0). \quad \square$$

## Computing Integrals via the Formula

The Cauchy integral formula allows us to evaluate integrals of the form $\oint_C \frac{f(z)}{z - z_0}\, dz$ (where $f$ is analytic and $z_0$ is a simple pole inside $C$) by simply reading off the value $2\pi i f(z_0)$.

**Worked example.** Evaluate $\oint_{|z|=2} \frac{e^z}{z - 1}\, dz$.

Here $f(z) = e^z$ (entire) and $z_0 = 1$ (inside $|z| = 2$). By the formula:
$$\oint_{|z|=2} \frac{e^z}{z-1}\, dz = 2\pi i e^1 = 2\pi i e.$$

**Worked example.** Evaluate $\oint_{|z|=2} \frac{\sin z}{z}\, dz$.

Here $f(z) = \sin z$ and $z_0 = 0$:
$$\oint_{|z|=2} \frac{\sin z}{z}\, dz = 2\pi i \sin(0) = 0.$$

**Worked example.** Evaluate $\oint_{|z-i|=2} \frac{z^2 + 1}{z - i}\, dz$.

Note $z^2 + 1 = (z-i)(z+i)$. Factor: $\frac{z^2+1}{z-i} = z + i$ for $z \neq i$. But by the formula with $f(z) = z^2 + 1$ and $z_0 = i$ (inside the contour):
$$\oint \frac{z^2+1}{z-i}\, dz = 2\pi i f(i) = 2\pi i (i^2 + 1) = 2\pi i (0) = 0.$$

Alternatively, $z^2 + 1 = (z-i)(z+i)$, so $\frac{z^2+1}{z-i} = z+i$ which is analytic, and by Cauchy's theorem the integral is $0$. $\square$

## The Extended Cauchy Formula for Multiply Connected Domains

If $C$ is not simple but is a piecewise smooth closed curve with winding number $n(C, z_0)$ around $z_0$, the formula generalizes to:
$$\frac{1}{2\pi i}\oint_C \frac{f(z)}{z - z_0}\, dz = n(C, z_0) f(z_0).$$

## The Mean Value Property

Taking $C$ to be the circle $|z - z_0| = r$ in the Cauchy integral formula:
$$f(z_0) = \frac{1}{2\pi i}\oint_{|z-z_0|=r}\frac{f(z)}{z-z_0}\, dz.$$
Parametrize $z = z_0 + re^{i\theta}$, $dz = ire^{i\theta}\, d\theta$:
$$f(z_0) = \frac{1}{2\pi}\int_0^{2\pi} f(z_0 + re^{i\theta})\, d\theta.$$

This is the **mean value property of analytic functions**: the value at the center of any disk equals the average of the values on the circle.

## Maximum Modulus Principle

**Theorem.** If $f$ is analytic and nonconstant on a domain $D$, then $|f|$ cannot attain a maximum in the interior of $D$.

**Proof sketch.** Suppose $|f(z_0)| = M$ is maximum in a neighborhood of $z_0$. By the mean value property applied to $|f|$ (via the estimate $|f(z_0)| \leq \frac{1}{2\pi}\int_0^{2\pi}|f(z_0 + re^{i\theta})|\, d\theta$), the equality implies $|f(z_0 + re^{i\theta})| = M$ for all $\theta$. Applying this to all sufficiently small $r$ shows $|f|$ is identically $M$ in a neighborhood of $z_0$. By the open mapping theorem and connectedness, $f$ is constant on $D$. $\square$

The maximum modulus principle has many applications: proving uniqueness of solutions to Laplace's equation, estimating polynomial growth, and the Schwarz lemma in the unit disk.

## Liouville's Theorem Preview

The Cauchy integral formula for $f'$ gives the bound $|f'(z_0)| \leq M/R$ where $M = \sup|f|$ and $R$ is the radius of any circle around $z_0$. If $f$ is entire and bounded, then $R \to \infty$ forces $f'(z_0) = 0$ for all $z_0$, hence $f$ is constant. This is Liouville's theorem, proved in detail in the next section.
