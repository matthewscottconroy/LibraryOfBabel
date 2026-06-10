# Section 23.2: Christoffel Symbols and the Geodesic Equation

---

## Section Introduction

The Christoffel symbols $\Gamma^\rho_{\mu\nu}$ are the coefficients of the Levi-Civita connection in a coordinate basis — the concrete, computable expression of how the metric "bends." They appear in the covariant derivative, the geodesic equation, the Riemann tensor, and the Ricci tensor. Virtually every GR calculation involves them.

This section gives the explicit formula for Christoffel symbols in terms of the metric, works through the computation for key metrics (the 2-sphere, Schwarzschild), and derives and applies the geodesic equation. The geodesic equation for the Schwarzschild metric is the central equation of GR for solar system tests (Mercury's precession, light bending, Shapiro delay, gravitational redshift).

---

## 23.2.1 The Christoffel Symbol Formula

From the derivation in Section 23.1.3 (uniqueness of the Levi-Civita connection):

$$\Gamma^\rho_{\mu\nu} = \frac{1}{2}g^{\rho\sigma}\left(\partial_\mu g_{\nu\sigma} + \partial_\nu g_{\mu\sigma} - \partial_\sigma g_{\mu\nu}\right)$$

**Properties**:
- Symmetric in lower indices: $\Gamma^\rho_{\mu\nu} = \Gamma^\rho_{\nu\mu}$
- Not a tensor: $\Gamma^\rho_{\mu\nu}$ can be set to zero at any single point by choosing Riemann normal coordinates, but not in a neighborhood unless the space is flat
- Depends on first derivatives of the metric

**In flat space (Cartesian)**: $g_{\mu\nu} = \delta_{\mu\nu}$ (or $\eta_{\mu\nu}$), all $\partial_\rho g_{\mu\nu} = 0$, so $\Gamma^\rho_{\mu\nu} = 0$. ✓

**Transformation law**: Under coordinate change $x^\mu \to x'^\mu$, the Christoffel symbols transform as:

$$\Gamma'^\rho_{\mu\nu} = \frac{\partial x'^\rho}{\partial x^\sigma}\frac{\partial x^\alpha}{\partial x'^\mu}\frac{\partial x^\beta}{\partial x'^\nu}\Gamma^\sigma_{\alpha\beta} + \frac{\partial x'^\rho}{\partial x^\sigma}\frac{\partial^2 x^\sigma}{\partial x'^\mu\partial x'^\nu}$$

The inhomogeneous second-derivative term confirms that $\Gamma^\rho_{\mu\nu}$ is not a tensor.

---

## 23.2.2 Example: The 2-Sphere $S^2$

Coordinates: $(\theta, \phi)$. Metric: $g_{\theta\theta} = r^2$, $g_{\phi\phi} = r^2\sin^2\theta$, $g_{\theta\phi} = 0$. ($r$ = radius of the sphere, treated as constant.)

Inverse metric: $g^{\theta\theta} = 1/r^2$, $g^{\phi\phi} = 1/(r^2\sin^2\theta)$, $g^{\theta\phi} = 0$.

**Non-zero Christoffel symbols**:

$\Gamma^\theta_{\phi\phi} = -\frac{1}{2}g^{\theta\theta}\partial_\theta g_{\phi\phi} = -\frac{1}{2}\cdot\frac{1}{r^2}\cdot 2r^2\sin\theta\cos\theta = -\sin\theta\cos\theta$

$\Gamma^\phi_{\theta\phi} = \Gamma^\phi_{\phi\theta} = \frac{1}{2}g^{\phi\phi}\partial_\theta g_{\phi\phi} = \frac{1}{2}\cdot\frac{1}{r^2\sin^2\theta}\cdot 2r^2\sin\theta\cos\theta = \cot\theta$

All other components vanish.

**Geodesic equation on $S^2$**: The equations $\ddot{\theta} + \Gamma^\theta_{\phi\phi}\dot\phi^2 = 0$ and $\ddot\phi + 2\Gamma^\phi_{\theta\phi}\dot\theta\dot\phi = 0$ give:

$$\ddot\theta - \sin\theta\cos\theta\,\dot\phi^2 = 0$$
$$\ddot\phi + 2\cot\theta\,\dot\theta\dot\phi = 0$$

The second equation integrates to $\frac{d}{d\lambda}(r^2\sin^2\theta\,\dot\phi) = 0$, i.e., angular momentum conservation: $\ell = r^2\sin^2\theta\,\dot\phi = $ const. The solutions are **great circles** — the spherical analogs of straight lines.

---

## 23.2.3 The Schwarzschild Christoffel Symbols

The Schwarzschild metric (in units $c = 1$; with $r_s = 2GM$):

$$ds^2 = -\left(1 - \frac{r_s}{r}\right)dt^2 + \left(1 - \frac{r_s}{r}\right)^{-1}dr^2 + r^2(d\theta^2 + \sin^2\theta\,d\phi^2)$$

Restricting to the equatorial plane ($\theta = \pi/2$, $\dot\theta = 0$), the non-vanishing Christoffel symbols are:

$$\Gamma^t_{tr} = \Gamma^t_{rt} = \frac{r_s/2}{r(r-r_s)}, \qquad \Gamma^r_{tt} = \frac{r_s(r-r_s)}{2r^3}$$

$$\Gamma^r_{rr} = -\frac{r_s/2}{r(r-r_s)}, \qquad \Gamma^r_{\phi\phi} = -(r-r_s)$$

$$\Gamma^\phi_{r\phi} = \Gamma^\phi_{\phi r} = \frac{1}{r}$$

**The geodesic equations**: Using these Christoffel symbols, the geodesic equations give (for equatorial orbits with constants of motion $E = (1-r_s/r)\dot{t}$ and $L = r^2\dot\phi$):

$$\frac{1}{2}\dot{r}^2 + V_{\rm eff}(r) = \frac{E^2 - 1}{2}$$

where $V_{\rm eff}(r) = -\frac{r_s}{2r} + \frac{L^2}{2r^2} - \frac{r_s L^2}{2r^3}$

(The three terms are Newtonian gravity, centrifugal barrier, and the GR correction.) This is exactly the Schwarzschild effective potential of Section 16.1.6.

---

## 23.2.4 The Geodesic Equation and Its Consequences

**Timelike geodesics** (massive particles, $g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = -1$ in units $c = 1$):

$$\frac{d^2x^\mu}{d\tau^2} + \Gamma^\mu_{\nu\rho}\frac{dx^\nu}{d\tau}\frac{dx^\rho}{d\tau} = 0$$

**Null geodesics** (massless particles/photons, $g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = 0$): same equation but with an affine parameter $\lambda$ instead of proper time $\tau$.

**Conserved quantities**: If the metric is independent of coordinate $x^\alpha$ (i.e., $\partial_\alpha g_{\mu\nu} = 0$), then $p_\alpha = g_{\alpha\mu}\dot{x}^\mu$ is conserved along geodesics. More precisely, $\xi^\mu = (\partial/\partial x^\alpha)^\mu$ is a Killing vector, and $p_\mu\xi^\mu = $ const. This is Noether's theorem applied to the geodesic action.

**Schwarzschild conserved quantities**:
- Independence of $t$: $E = -(1-r_s/r)\dot{t}$ (energy per unit mass, conserved)
- Independence of $\phi$: $L = r^2\sin^2\theta\,\dot\phi$ (angular momentum per unit mass, conserved)

From these and the normalization condition, all geodesics in Schwarzschild spacetime are determined — the effective potential analysis of Section 16.1.6.

**Classical tests of GR from the Schwarzschild geodesics**:

1. **Perihelion precession**: Timelike geodesics precess with rate $\Delta\phi = 6\pi r_s/(2a(1-e^2))$ per orbit (Section 16.3.4).

2. **Light deflection**: Null geodesics near the Sun are deflected by $\delta\phi = 2r_s/b = 4GM/(c^2 b)$, where $b$ is the impact parameter. For a grazing ray at the solar limb ($b = R_\odot = 6.96\times10^8$ m): $\delta\phi = 1.75$ arcseconds. Confirmed by Eddington's 1919 solar eclipse expedition (though with significant uncertainty); now measured to 0.01% precision using radio interferometry of quasars occulted by the Sun.

3. **Shapiro time delay**: A radar signal sent to a planet near superior conjunction (passing close to the Sun) takes longer than expected because the metric is not flat near the Sun — light travels along null geodesics in curved spacetime, which is longer than the Euclidean path. Delay $\Delta t = (2r_s/c)\ln(4r_1 r_2/b^2)$ (where $r_1, r_2$ are distances to Earth and planet, $b$ is closest approach). First measured by Shapiro (1964) using planetary radar; confirmed to 0.001% precision.

4. **Gravitational redshift**: A photon emitted at radius $r_1$ and received at $r_2 > r_1$ has frequency ratio $\nu_2/\nu_1 = \sqrt{(1-r_s/r_1)/(1-r_s/r_2)}$. In the weak field: $\Delta\nu/\nu = -\Delta\Phi/c^2$ (redshift from gravitational potential difference). Confirmed to 0.01% by Pound-Rebka (1959) using the Mössbauer effect.

---

## 23.2.5 Geodesic Completeness and Singularities

A geodesic is **complete** if it can be extended to all values of its affine parameter. A spacetime is **geodesically incomplete** if there exist inextendible geodesics that reach a finite value of the affine parameter — the geodesic "runs off the edge" of the manifold.

Geodesic incompleteness is the modern (rigorous) definition of a **singularity**. The Penrose singularity theorem (Section 13.3.5, proven 1965) guarantees geodesic incompleteness in spacetimes containing trapped surfaces — including all black holes and the Big Bang cosmology.

At the Schwarzschild singularity $r = 0$: tidal forces (components of the Riemann tensor) diverge, and timelike geodesics are extendible for only finite proper time. The singularity is a property of the spacetime geometry, not the coordinate system — the Schwarzschild coordinate singularity at $r = r_s$ can be removed by Kruskal-Szekeres coordinates, but the singularity at $r = 0$ cannot.

---

## References

- Christoffel, E.B. (1869). "Über die Transformation der homogenen Differentialausdrücke zweiten Grades." *Journal für die reine und angewandte Mathematik*, 70, 46–70. [The paper introducing $\Gamma^\rho_{\mu\nu}$ as the correction terms needed to transform second-degree differential forms covariantly — the birth of the Christoffel symbols.]
- Ricci-Curbastro, G. and Levi-Civita, T. (1901). "Méthodes de calcul différentiel absolu et leurs applications." *Mathematische Annalen*, 54, 125–201. [The "absolute differential calculus" — the systematic development of tensor analysis on Riemannian manifolds. This is the paper Einstein studied in 1912–1915 to learn the mathematics for GR.]
- Schwarzschild, K. (1916). "Über das Gravitationsfeld eines Massenpunktes nach der Einsteinschen Theorie." *Sitzungsberichte der Königlich Preußischen Akademie der Wissenschaften*, 189–196. [The Schwarzschild metric — the first exact solution of the Einstein equations. The computation of Christoffel symbols for this metric is the prototype for all GR calculations.]
- Shapiro, I.I. (1964). "Fourth test of general relativity." *Physical Review Letters*, 13, 789–791. [The Shapiro time delay: a fourth test of GR using radar echoes from Venus and Mercury passing near the Sun.]
- Pound, R.V. and Rebka, G.A. (1959). "Gravitational red-shift in nuclear resonance." *Physical Review Letters*, 3, 439–441. [First measurement of gravitational redshift using the Mössbauer effect: photons emitted at the bottom of a 22.5-m tower and detected at the top (and vice versa).]
