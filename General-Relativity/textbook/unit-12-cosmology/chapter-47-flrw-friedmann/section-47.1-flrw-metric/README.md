# Section 47.1: The FLRW Metric

---

## The Cosmological Principle

The starting point of modern cosmology is an assumption so sweeping that it might seem unfounded: the universe, on large scales, is the same everywhere and in every direction. This is the **cosmological principle** — homogeneity and isotropy.

It sounds like a philosopher's simplification. But it is observationally confirmed:

- **Galaxy surveys** (SDSS, 2dFGRS, DESI) show that the galaxy distribution, when averaged over volumes larger than $\sim (300 \text{ Mpc})^3$, is uniform to better than 0.1%
- **The cosmic microwave background** (CMB) is isotropic to 1 part in $10^5$ — after subtracting the dipole anisotropy from our own motion
- **Radio source counts** at different frequencies show isotropy to $\sim 1$%
- **Gamma-ray burst distributions** are isotropic across the sky

The inhomogeneities we see — galaxies, clusters, filaments, voids — are small perturbations on top of a smooth background. They can be treated as cosmological perturbations on the FLRW background.

Why should the universe be so uniform? The answer from inflation is that the observable universe expanded from a region much smaller than the Hubble radius at the end of inflation, smoothing out any initial inhomogeneities exponentially. But even without inflation, the fact that the CMB temperature is uniform to $10^{-5}$ across the sky is deeply puzzling — different patches of the CMB we see were not in causal contact at the time of emission (this is the **horizon problem**, which inflation solves).

---

## Deriving the FLRW Metric

The most general spacetime metric consistent with spatial homogeneity and isotropy is the **Friedmann-Lemaître-Robertson-Walker (FLRW) metric**. Robertson and Walker (1935–36) proved this rigorously; Friedmann and Lemaître had used it earlier on physical grounds.

**Step 1: The spatial metric.** A homogeneous, isotropic 3-space has constant curvature. There are only three possibilities:
- $k = +1$: constant positive curvature (a 3-sphere $S^3$)
- $k = 0$: flat (Euclidean 3-space $\mathbb{R}^3$)
- $k = -1$: constant negative curvature (hyperbolic 3-space $H^3$)

The spatial line element is:
$$d\ell^2 = \frac{dr^2}{1-kr^2} + r^2(d\theta^2 + \sin^2\theta\, d\phi^2)$$

where $r$ is the comoving radial coordinate normalized so that the radius of curvature is unity (for $k = \pm 1$). In the physical universe, the radius of curvature is $\ell_{\rm curv} = c/H_0 / \sqrt{|\Omega_k|}$ — currently $> 100$ Gpc.

**Step 2: The full spacetime metric.** Time translation symmetry (homogeneity in time would be too strong — the universe is evolving) is broken, but we can choose a preferred cosmic time $t$ = the proper time of comoving observers. Isotropy then forces the metric to take the form:
$$ds^2 = -c^2 dt^2 + a(t)^2\left[\frac{dr^2}{1-kr^2} + r^2 d\Omega^2\right]$$

where $a(t)$ is the **scale factor** — a function of time only, describing how the spatial sections stretch uniformly. This is the FLRW metric.

**Alternative forms.** Using the comoving angular diameter distance $\chi$ defined by $d\chi = dr/\sqrt{1-kr^2}$:
$$ds^2 = -c^2 dt^2 + a(t)^2\left[d\chi^2 + f_k(\chi)^2 d\Omega^2\right]$$

where:
$$f_k(\chi) = \begin{cases}\sin\chi & k = +1\\ \chi & k = 0\\ \sinh\chi & k = -1\end{cases}$$

Using **conformal time** $\eta$ defined by $d\eta = dt/a(t)$ (so $a\,d\eta = dt$):
$$ds^2 = a(\eta)^2\left[-c^2 d\eta^2 + d\chi^2 + f_k(\chi)^2 d\Omega^2\right]$$

In conformal coordinates, the metric is conformally flat — it is Minkowski spacetime multiplied by the conformal factor $a(\eta)^2$. This makes null geodesics (light rays) travel along $45°$ lines in the $(\eta, \chi)$ plane, just as in special relativity. Conformal time is invaluable for computing particle horizons and the CMB.

---

## Comoving Coordinates and Physical Distances

In the FLRW metric, coordinates $(r, \theta, \phi)$ or $(\chi, \theta, \phi)$ are **comoving coordinates** — they are fixed to the matter of the universe, which is at rest in the cosmological fluid. A galaxy with no peculiar velocity has fixed comoving coordinates.

The **proper distance** between two comoving points (at constant $t$) is:
$$d(t) = a(t)\int_0^{\chi}\frac{d\chi'}{\sqrt{1-k\chi'^2}} = a(t)\chi$$

for the flat case. The velocity at which two galaxies separate is:
$$\dot{d} = \dot{a}\chi = \frac{\dot{a}}{a}a\chi = H(t)d$$

This is **Hubble's law**: $v = H_0 d$ (at the present epoch). It is not a velocity of recession in the special relativistic sense — it is the expansion of the coordinate grid itself. For $d > c/H_0 \approx 4.3$ Gpc (the Hubble radius), the recession "velocity" exceeds $c$. This does not violate special relativity because no signal is being sent; the relationship is purely kinematic from the metric.

**The Hubble sphere**: At distance $d_H = c/H_0 \approx 4.3$ Gpc (the "Hubble radius"), galaxies have recession velocity $v = c$. For $d > d_H$, photons emitted toward us are initially moving away from us (they are in a region expanding faster than light). However, if $\ddot{a} > 0$ (as in our accelerating universe), the Hubble sphere grows, and photons from within a larger region will eventually reach us. Photons from beyond the **cosmic event horizon** $d_E = a_0 c\int_{t_0}^\infty dt/a(t)$ will never reach us.

---

## Redshift in the FLRW Universe

The cosmological redshift is a kinematic effect of the expansion — photons are stretched by the expansion of the universe between emission and reception.

Consider a photon emitted at time $t_{\rm em}$ and received at $t_0$. It travels along a null geodesic $ds^2 = 0$ in the radial direction:
$$c\,dt = a(t)\,d\chi \implies \chi = c\int_{t_{\rm em}}^{t_0}\frac{dt}{a(t)}$$

Now consider the next wavecrest of the same photon, emitted at $t_{\rm em} + \delta t_{\rm em}$ and received at $t_0 + \delta t_0$. The comoving distance is the same:
$$c\int_{t_{\rm em}+\delta t_{\rm em}}^{t_0+\delta t_0}\frac{dt}{a(t)} = c\int_{t_{\rm em}}^{t_0}\frac{dt}{a(t)}$$

Subtracting:
$$\frac{\delta t_0}{a(t_0)} = \frac{\delta t_{\rm em}}{a(t_{\rm em})}$$

Since wavelength $\propto$ period ($\lambda = c\delta t$):
$$\frac{\lambda_0}{\lambda_{\rm em}} = \frac{a(t_0)}{a(t_{\rm em})} \equiv 1 + z$$

The redshift is:
$$\boxed{1 + z = \frac{a_0}{a_{\rm em}} = \frac{1}{a_{\rm em}}}$$

(using $a_0 = 1$ today). This is exact, valid for any expansion history. For a galaxy at redshift $z = 1$, the universe was $1/2$ its present size when the light was emitted. At $z = 1100$ (CMB last scattering): the universe was $1/1101$ its present size.

**Cosmological vs. Doppler redshift**: The cosmological redshift is not a Doppler shift in the usual sense — it accumulates continuously over the photon's journey, not just at the moment of emission. For small $z$: $z \approx v/c$ (Doppler), but for $z > 1$, the two pictures diverge. The distinction is gauge-dependent in the sense that "expansion of space" vs. "motion of galaxies" depends on the coordinate choice, but the redshift $z$ is a physical observable.

---

## Horizons and the Observable Universe

In a universe with finite age and finite expansion history, light from distant regions may not have reached us. The set of events from which we can have received signals is bounded by our **particle horizon**.

The comoving distance to the particle horizon is:
$$\chi_H = c\int_0^{t_0}\frac{dt}{a(t)} = c\int_0^1\frac{da}{a^2 H(a)}$$

For a flat $\Lambda$CDM universe with $H_0 = 67.4$ km/s/Mpc: $\chi_H \approx 46.5$ Gly (comoving), giving a physical radius of $\sim 46.5$ Gly $\times a_0 = 46.5$ billion light-years for the **observable universe**.

(Note: the age of the universe is only 13.8 Gyr. The particle horizon is larger because the universe has been expanding — points now at 46.5 billion light-years emitted light 13.8 billion years ago when they were much closer.)

The **event horizon** is the farthest distance from which we can ever receive light, even waiting infinitely long:
$$d_E = a_0 c\int_{t_0}^{\infty}\frac{dt}{a(t)}$$

For a universe with $\Lambda > 0$, $a(t)\to\infty$, the integral converges, and there is a finite event horizon $\sim 16$ Gly. Galaxies beyond this distance are receding so fast that light emitted today will never reach them, and light emitted by them today will never reach us.

---

## Topology of the Universe

The FLRW metric specifies local geometry but not global topology. A flat universe ($k = 0$) could be infinite Euclidean 3-space $\mathbb{R}^3$ or could be a compact flat manifold (e.g., a 3-torus $T^3$). A positively curved universe ($k = +1$) is locally $S^3$ but the topological identification could create non-trivial topologies.

Cosmic topology is testable: if the universe is topologically compact, then the same galaxy (or the same CMB pattern) would appear in multiple directions on the sky. Searches in CMB data (Planck) have found no evidence for such "circles in the sky." The lower bound on the topology scale is $> 23$ Gpc (95% confidence from Planck).

This tells us nothing about whether the universe is infinite — only that if it is finite, it is larger than our observable horizon.

---

## Spatial Curvature and the Flatness of the Universe

The dimensionless density parameter for spatial curvature is:
$$\Omega_k = -\frac{kc^2}{H_0^2}$$

The Planck 2018 CMB analysis gives $\Omega_k = 0.0007 \pm 0.0019$ — consistent with zero. The universe appears flat to extraordinary precision.

This flatness is one of the motivations for inflation (section 50): in a universe without inflation, the flatness problem asks why $\Omega_k$ was so small at the Planck time. Since $\Omega_k \propto 1/(a^2 H^2)$ evolves away from zero during matter and radiation domination, having $|\Omega_k| < 0.002$ today requires $|\Omega_k| < 10^{-60}$ at the Planck time — an extraordinary fine-tuning. Inflation solves this by expanding the scale factor by $e^{60}$–$e^{100}$, driving $\Omega_k \to 0$ exponentially.
