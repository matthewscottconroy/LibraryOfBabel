# Improper Integrals via Residues

Many improper integrals $\int_{-\infty}^\infty f(x)\, dx$ that cannot be evaluated by elementary antiderivatives are accessible via contour integration. The strategy is to close the real-line integral with an arc in the upper or lower half-plane, show the arc contributes zero, and apply the residue theorem to the closed contour. The technique requires verifying that $f$ decays fast enough at infinity to justify taking $R \to \infty$.

## The Semicircular Contour Method

Let $f(z)$ be a rational function with no real poles and with $\deg(\text{denominator}) \geq \deg(\text{numerator}) + 2$. Consider the contour $C_R$ consisting of:
- The real segment $[-R, R]$, traversed left to right.
- The upper semicircle $\Gamma_R = \{Re^{i\theta} : \theta \in [0, \pi]\}$, traversed counterclockwise.

By the residue theorem:
$$\int_{-R}^R f(x)\, dx + \int_{\Gamma_R} f(z)\, dz = 2\pi i\sum_{\mathrm{Im}(z_k)>0}\mathrm{Res}(f; z_k).$$

As $R \to \infty$: the integral over $\Gamma_R$ satisfies $\left|\int_{\Gamma_R}\right| \leq M_R \cdot \pi R$, where $M_R = \max_{|z|=R}|f(z)| = O(1/R^2)$, so $M_R \pi R = O(1/R) \to 0$. Therefore:
$$\int_{-\infty}^\infty f(x)\, dx = 2\pi i\sum_{\mathrm{Im}(z_k)>0}\mathrm{Res}(f; z_k).$$

## Worked Examples

**Example 1.** Evaluate $\displaystyle I = \int_{-\infty}^\infty \frac{dx}{1 + x^2}$.

Poles of $f(z) = 1/(1+z^2)$: at $z = \pm i$. Only $z = i$ is in the upper half-plane.
$\mathrm{Res}(f; i) = \frac{1}{2i}$.
$I = 2\pi i \cdot \frac{1}{2i} = \pi$. $\square$

**Example 2.** Evaluate $\displaystyle I = \int_{-\infty}^\infty \frac{x^2\, dx}{(x^2 + 1)(x^2 + 4)}$.

Poles in upper half-plane: $z = i$ and $z = 2i$.
$\mathrm{Res}$ at $z = i$: $\frac{i^2}{(i^2+4)(2i)} = \frac{-1}{3 \cdot 2i} = \frac{-1}{6i} = \frac{i}{6}$.
$\mathrm{Res}$ at $z = 2i$: $\frac{(2i)^2}{((2i)^2+1)(2\cdot 2i)} = \frac{-4}{-3 \cdot 4i} = \frac{-4}{-12i} = \frac{1}{3i} = \frac{-i}{3}$.

$I = 2\pi i\left(\frac{i}{6} - \frac{i}{3}\right) = 2\pi i \cdot \frac{-i}{6} = 2\pi i \cdot \frac{-i}{6} = \frac{2\pi}{6} = \frac{\pi}{3}$. $\square$

**Example 3.** Evaluate $\displaystyle I = \int_0^\infty \frac{dx}{x^4 + 1}$.

Since the integrand is even: $I = \frac{1}{2}\int_{-\infty}^\infty \frac{dx}{x^4+1}$.

Roots of $z^4 + 1 = 0$: $z^4 = -1 = e^{i\pi}$, so $z_k = e^{i(\pi + 2\pi k)/4}$, $k = 0,1,2,3$.
Upper half-plane roots: $z_0 = e^{i\pi/4} = \frac{1+i}{\sqrt{2}}$ and $z_1 = e^{3i\pi/4} = \frac{-1+i}{\sqrt{2}}$.

$\mathrm{Res}(f; z_k) = \frac{1}{4z_k^3} = \frac{z_k}{4z_k^4} = \frac{z_k}{4(-1)} = \frac{-z_k}{4}$ (using $z_k^4 = -1$).

$\mathrm{Res}$ at $z_0$: $-e^{i\pi/4}/4$. $\mathrm{Res}$ at $z_1$: $-e^{3i\pi/4}/4$.

Sum: $-\frac{1}{4}(e^{i\pi/4} + e^{3i\pi/4}) = -\frac{1}{4} \cdot 2i\sin(\pi/4+\pi/4-\pi/4)$... 

Let me compute directly: $e^{i\pi/4} + e^{3i\pi/4} = \frac{1+i}{\sqrt{2}} + \frac{-1+i}{\sqrt{2}} = \frac{2i}{\sqrt{2}} = i\sqrt{2}$.

Sum of residues $= -i\sqrt{2}/4$.

$\int_{-\infty}^\infty \frac{dx}{x^4+1} = 2\pi i \cdot \frac{-i\sqrt{2}}{4} = \frac{2\pi\sqrt{2}}{4} = \frac{\pi\sqrt{2}}{2}$.

$I = \frac{1}{2}\cdot\frac{\pi\sqrt{2}}{2} = \frac{\pi}{2\sqrt{2}} = \frac{\pi\sqrt{2}}{4}$. $\square$

## Integrals with Poles on the Real Axis

If $f$ has simple poles on the real axis at points $x_1, \ldots, x_m$, the integral $\int_{-\infty}^\infty f(x)\, dx$ as a standard Lebesgue integral diverges. However, the Cauchy principal value may exist:
$$\mathrm{PV}\int_{-\infty}^\infty f(x)\, dx = \lim_{R \to \infty, \varepsilon \to 0}\left(\int_{-R}^{x_1-\varepsilon} + \int_{x_1+\varepsilon}^{x_2-\varepsilon} + \cdots + \int_{x_m+\varepsilon}^R\right)f(x)\, dx.$$

To evaluate this using residues, indent the contour with small semicircles of radius $\varepsilon$ around each real pole, going above the real axis (so the pole is outside the contour). As $\varepsilon \to 0$, each small semicircle contributes $-\pi i \cdot \mathrm{Res}(f; x_k)$ (half the residue with a minus sign, since the semicircle is clockwise). The result is:
$$\mathrm{PV}\int_{-\infty}^\infty f(x)\, dx = 2\pi i \sum_{\mathrm{Im}(z_k)>0}\mathrm{Res}(f; z_k) + \pi i\sum_k \mathrm{Res}(f; x_k).$$

## Integrands with Branch Cuts (Keyhole Contour)

For integrals of the form $\int_0^\infty x^\alpha f(x)\, dx$ with $-1 < \mathrm{Re}(\alpha) < 0$, use the keyhole contour: a large circle of radius $R$, a small circle of radius $\varepsilon$ around the origin, and two rays just above and just below the positive real axis. The branch cut of $x^\alpha$ along the positive real axis contributes the factor $e^{2\pi i\alpha} - 1$, which combines with residues to yield the integral.

**Example.** Evaluate $\displaystyle I = \int_0^\infty \frac{x^\alpha}{x + 1}\, dx$, $-1 < \alpha < 0$.

Using the keyhole contour and the single pole of $z^\alpha/(z+1)$ at $z = -1 = e^{i\pi}$:
$$I(1 - e^{2\pi i\alpha}) = 2\pi i \cdot \mathrm{Res}(z^\alpha/(z+1); -1) = 2\pi i \cdot (-1)^\alpha = 2\pi i\, e^{i\pi\alpha}.$$
$$I = \frac{2\pi i\, e^{i\pi\alpha}}{1 - e^{2\pi i\alpha}} = \frac{2\pi i}{e^{-i\pi\alpha} - e^{i\pi\alpha}} = \frac{2\pi i}{-2i\sin(\pi\alpha)} = \frac{-\pi}{\sin(\pi\alpha)} = \frac{\pi}{\sin(-\pi\alpha)} = \frac{\pi}{\sin(\pi(1+\alpha))}.$$

More standardly: $I = \pi/\sin(\pi\alpha)$ for $\alpha \in (-1, 0)$. This is consistent with the Beta function result $B(\alpha+1, -\alpha) = \pi/\sin(\pi(\alpha+1))$. $\square$
