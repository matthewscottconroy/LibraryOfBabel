# Section 38.1: Deriving the Schwarzschild Metric

---

## The Symmetry Ansatz

To find a spherically symmetric, static, asymptotically flat solution to the vacuum Einstein equations $R_{\mu\nu} = 0$, we begin with the most general metric consistent with these symmetries.

**Spherical symmetry** means the metric is invariant under $SO(3)$ rotations — the metric looks the same from any direction. This requires that the angular part of the metric be the standard round 2-sphere metric $r^2 d\Omega^2 = r^2(d\theta^2 + \sin^2\theta\,d\phi^2)$, where $r$ is the area-radius coordinate (so a sphere of coordinate radius $r$ has area $4\pi r^2$).

**Staticity** means the metric is invariant under time translation ($\partial/\partial t$ is a Killing vector) and time reversal ($t \to -t$). This eliminates $dt\,dr$ cross terms.

The most general static, spherically symmetric metric is therefore:
$$ds^2 = -e^{2\alpha(r)}c^2dt^2 + e^{2\beta(r)}dr^2 + r^2d\Omega^2$$
where $\alpha(r)$ and $\beta(r)$ are arbitrary functions to be determined by the field equations.

**Asymptotic flatness**: at large $r$, the metric must approach Minkowski spacetime:
$$e^{2\alpha(r)}\to 1, \quad e^{2\beta(r)}\to 1 \quad \text{as } r\to\infty$$

---

## Computing the Ricci Tensor

From the ansatz, the Christoffel symbols are:
$$\Gamma^t_{tr} = \alpha'(r), \quad \Gamma^r_{tt} = \alpha'e^{2(\alpha-\beta)}, \quad \Gamma^r_{rr} = \beta'(r)$$
$$\Gamma^r_{\theta\theta} = -re^{-2\beta}, \quad \Gamma^r_{\phi\phi} = -r\sin^2\theta\,e^{-2\beta}$$
$$\Gamma^\theta_{r\theta} = 1/r, \quad \Gamma^\theta_{\phi\phi} = -\sin\theta\cos\theta$$
$$\Gamma^\phi_{r\phi} = 1/r, \quad \Gamma^\phi_{\theta\phi} = \cot\theta$$
(Primes denote $d/dr$.)

The non-zero Ricci tensor components (using $R_{\mu\nu} = \partial_\rho\Gamma^\rho_{\mu\nu} - \partial_\nu\Gamma^\rho_{\mu\rho} + \Gamma^\rho_{\rho\lambda}\Gamma^\lambda_{\mu\nu} - \Gamma^\rho_{\nu\lambda}\Gamma^\lambda_{\mu\rho}$) are:

$$R_{tt} = e^{2(\alpha-\beta)}\left(\alpha'' + (\alpha')^2 - \alpha'\beta' + \frac{2\alpha'}{r}\right)$$

$$R_{rr} = -\alpha'' - (\alpha')^2 + \alpha'\beta' + \frac{2\beta'}{r}$$

$$R_{\theta\theta} = 1 - e^{-2\beta}\left(1 + r(\alpha' - \beta')\right)$$

$$R_{\phi\phi} = \sin^2\theta\,R_{\theta\theta}$$

---

## The Vacuum Equations and Birkhoff's Theorem

Setting $R_{\mu\nu} = 0$:

From $e^{-2\beta}R_{tt}/\alpha' + R_{rr}/\alpha' = 0$ (combining $R_{tt}$ and $R_{rr}$):
$$\frac{2(\alpha'+\beta')}{r} = 0 \implies \alpha' + \beta' = 0 \implies \alpha(r) + \beta(r) = \text{const}$$

With asymptotic flatness ($\alpha = \beta = 0$ at infinity): $\alpha(r) = -\beta(r)$.

From $R_{\theta\theta} = 0$:
$$1 - e^{-2\beta}\left(1 + r(\alpha'-\beta')\right) = 0$$
With $\alpha = -\beta$, so $\alpha' = -\beta'$:
$$1 - e^{2\alpha}(1 + 2r\alpha') = 0 \implies \frac{d}{dr}(re^{2\alpha}) = 1 \implies re^{2\alpha} = r - r_s$$

where $r_s$ is an integration constant. Therefore:
$$e^{2\alpha} = 1 - \frac{r_s}{r}$$

The integration constant $r_s$ is fixed by the Newtonian limit: as $r\to\infty$, $g_{00} \approx -(1 - r_s/r)$, and the geodesic equation gives $d^2\mathbf{x}/dt^2 = -\frac{c^2}{2}\nabla g_{00} = -\frac{r_s c^2}{2r^2}\hat{r}$. Comparing to $d^2\mathbf{x}/dt^2 = -GM/r^2\hat{r}$:
$$r_s = \frac{2GM}{c^2}$$

This is the **Schwarzschild radius**. The metric is:
$$\boxed{ds^2 = -\left(1 - \frac{r_s}{r}\right)c^2dt^2 + \left(1 - \frac{r_s}{r}\right)^{-1}dr^2 + r^2d\Omega^2}$$

**Birkhoff's theorem** (1923): The Schwarzschild metric is the *unique* spherically symmetric solution to the vacuum Einstein equations (without cosmological constant). This is GR's analog of Gauss's law in electrostatics: the exterior field of any spherically symmetric matter distribution is Schwarzschild, regardless of the details of the interior.

Consequences:
- A pulsating spherical star has a static Schwarzschild exterior — no monopole gravitational radiation.
- A spherical collapse produces a Schwarzschild black hole, regardless of the collapse dynamics.

---

## The Schwarzschild Radius

$$r_s = \frac{2GM}{c^2}$$

| Object | Mass | Schwarzschild radius |
|---|---|---|
| Earth | $5.97\times 10^{24}$ kg | $\sim 9$ mm |
| Sun | $1.99\times 10^{30}$ kg | $\sim 3$ km |
| Stellar black hole | $10 M_\odot$ | $\sim 30$ km |
| Supermassive BH (M87*) | $6.5\times 10^9 M_\odot$ | $\sim 2\times 10^{13}$ m $\sim 0.13$ AU |
| Proton | $1.67\times 10^{-27}$ kg | $\sim 2.5\times 10^{-54}$ m |

For normal matter, the Schwarzschild radius is far inside the physical radius. The Sun's $r_s = 3$ km is well inside the Sun (radius 696,000 km); the Schwarzschild metric applies only outside the Sun's surface. For a black hole, the object has collapsed below its own Schwarzschild radius, and the Schwarzschild metric applies all the way down to $r = 0$ (the singularity).

---

## The Christoffel Symbols of the Schwarzschild Metric

For later use (computing geodesic equations, Riemann tensor), the non-zero Christoffel symbols of the Schwarzschild metric (with $f(r) = 1 - r_s/r$) are:

**Time-time-radial:** $\Gamma^t_{tr} = \Gamma^t_{rt} = r_s/(2r^2 f)$

**Radial-time-time:** $\Gamma^r_{tt} = f r_s c^2/(2r^2)$

**Radial-radial-radial:** $\Gamma^r_{rr} = -r_s/(2r^2 f)$

**Radial-angular-angular:** $\Gamma^r_{\theta\theta} = -rf$, $\Gamma^r_{\phi\phi} = -rf\sin^2\theta$

**Angular connections:** $\Gamma^\theta_{r\theta} = \Gamma^\phi_{r\phi} = 1/r$, $\Gamma^\theta_{\phi\phi} = -\sin\theta\cos\theta$, $\Gamma^\phi_{\theta\phi} = \cot\theta$

These can be computed directly from the metric formula $\Gamma^\rho_{\mu\nu} = \frac{1}{2}g^{\rho\sigma}(\partial_\mu g_{\nu\sigma} + \partial_\nu g_{\mu\sigma} - \partial_\sigma g_{\mu\nu})$.

---

## Verifying the Vacuum Equations

With the Schwarzschild Christoffel symbols in hand, one can verify $R_{\mu\nu} = 0$ directly. The computation is straightforward (if tedious). The key fact is that:

- $R_{tt} = R_{rr} = 0$ follows from $\alpha = -\beta$
- $R_{\theta\theta} = R_{\phi\phi} = 0$ follows from $e^{2\alpha} = 1 - r_s/r$
- All off-diagonal components vanish by symmetry

The Riemann curvature tensor $R^\rho_{\ \sigma\mu\nu}$ is *not* zero — spacetime is curved — but all Ricci tensor contractions vanish. The non-zero curvature is entirely in the Weyl tensor. For Schwarzschild, the Kretschner invariant is:
$$R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma} = \frac{48G^2M^2}{r^6 c^4}$$
This diverges at $r = 0$ — the true curvature singularity — and is perfectly regular at $r = r_s$ (the apparent "Schwarzschild singularity" is a coordinate artifact).

