# 2.1.1 — Fermat's Principle

## The Statement

Pierre de Fermat proposed his principle of least time in 1662: *light traveling between two points takes the path that requires the least time* [1]. The principle was controversial — critics asked how light could "know" which path to take before taking it, which seemed to require prescience. The modern formulation resolves this: light takes the path of *stationary optical path length*, which is a precise variational statement that does not require the light to know anything in advance.

**Fermat's principle (modern form)**: The optical path length $\mathcal{L}$ along the actual ray path from point $A$ to point $B$ is *stationary* with respect to small variations of the path:

$$\delta \mathcal{L} = \delta \int_A^B n(\mathbf{r}) \, ds = 0$$

where $n(\mathbf{r})$ is the refractive index at position $\mathbf{r}$ and $ds$ is the arc length element along the path.

The optical path length $n \, ds$ is the physical path length multiplied by the refractive index — it is proportional to the *phase accumulated* by the wave as it travels. A path of optical path length $\mathcal{L}$ accumulates a phase $\phi = (2\pi/\lambda_0) \mathcal{L}$, where $\lambda_0$ is the free-space wavelength. So Fermat's principle says: the actual ray path is the one along which the phase is stationary.

## Why "Stationary" Rather Than "Minimum"

The word "stationary" is important. In many common situations, the actual path is the *minimum* optical path length (Fermat's original "least time" formulation). But this is not always true:
- For reflection off a concave mirror, the actual path can be a *maximum* of the optical path length.
- At a focus, many paths have the same optical path length (a *saddle point* rather than a minimum or maximum).

The correct statement — stationary, meaning the first variation vanishes — covers all cases. Snell's law, the law of reflection, and the focusing property of lenses all follow from applying this variational principle.

## From Fermat to Snell: A Preview

Consider a ray traveling from medium 1 (refractive index $n_1$) to medium 2 ($n_2$), crossing a flat interface. The optical path length from a point $A$ in medium 1 to a point $B$ in medium 2 is:

$$\mathcal{L} = n_1 \sqrt{x^2 + d_1^2} + n_2 \sqrt{(L-x)^2 + d_2^2}$$

where $x$ is the horizontal position of the crossing point, $d_1$ and $d_2$ are the perpendicular distances from $A$ and $B$ to the interface, and $L$ is the total horizontal separation. Setting $d\mathcal{L}/dx = 0$:

$$n_1 \frac{x}{\sqrt{x^2 + d_1^2}} = n_2 \frac{L-x}{\sqrt{(L-x)^2 + d_2^2}}$$

The left side is $n_1 \sin\theta_1$ (where $\theta_1$ is the angle of incidence), and the right side is $n_2 \sin\theta_2$. The stationary condition gives:

$$n_1 \sin\theta_1 = n_2 \sin\theta_2$$

This is Snell's law. It is derived from Fermat's principle with nothing more than calculus. The full wave derivation (Section 2.1.2) will show that Fermat's principle itself follows from the wave nature of light — a deeper derivation that reveals why geometric optics is the limit of wave optics.

## The Connection to Wave Optics: Stationary Phase

The reason Fermat's principle is true is a consequence of wave interference. Consider all possible paths from $A$ to $B$. Each path accumulates a phase $\phi = (2\pi/\lambda_0)\mathcal{L}$. If the optical path lengths along nearby paths are very different, the phases are random and the contributions from those paths cancel — destructive interference. But near a path where $\mathcal{L}$ is stationary, nearby paths have nearly the same optical path length and nearly the same phase — constructive interference. The ray is the path of constructive interference.

This argument — that the classical trajectory is the path of stationary phase — is the semiclassical limit of quantum mechanics (Feynman's path integral formulation) [2]. Fermat's principle is not a separate law; it is wave optics in the limit $\lambda \to 0$, the geometric optics limit.

**Why this matters for photonic computing**: The argument explains when geometric optics fails. It fails when the spread of optical path lengths among nearby paths is comparable to a wavelength — that is, when path differences of order $\lambda$ are accumulated over the relevant scale. In a silicon waveguide narrower than a wavelength, or in an MZI with path differences of a fraction of a wavelength, we are definitively not in the geometric optics limit. We need wave optics.

## The Eikonal Equation

For completeness, the formal geometric optics limit of wave optics is expressed by the *eikonal equation*. Starting from the Helmholtz equation (the time-independent wave equation):

$$\nabla^2 \mathbf{E} + k_0^2 n^2(\mathbf{r}) \mathbf{E} = 0$$

and substituting the ansatz $\mathbf{E} = \mathbf{A}(\mathbf{r}) e^{ik_0 S(\mathbf{r})}$ (where $S$ is the *eikonal* and $\mathbf{A}$ is a slowly varying amplitude):

$$(\nabla S)^2 = n^2(\mathbf{r})$$

This is the eikonal equation. Its solutions give the wavefronts of geometric optics: surfaces of constant $S$ are the wavefronts, and the rays are the normals to these wavefronts (the direction of $\nabla S$). Fermat's principle follows from the eikonal equation as a variational theorem.

The eikonal approximation is valid when $|\nabla^2 \mathbf{A}| \ll k_0 |\nabla S| |\mathbf{A}|$ — that is, when the amplitude varies slowly compared to the wavelength. In photonic devices operating at wavelength scale, this condition fails, and we must use the full wave equation.

## Summary

- Fermat's principle: the actual ray path makes the optical path length $\int n \, ds$ stationary.
- This follows from the constructive interference of nearby wave paths (stationary phase argument).
- Geometric optics is the limit $\lambda \to 0$ of wave optics; the eikonal equation is the formal expression of this limit.
- In photonic computing, wavelengths are always comparable to device dimensions; geometric optics is always an approximation that must be used with care.

---

*References*

[1] Fermat, P. de (1662). Letter to Cureau de la Chambre. In: *Œuvres de Fermat*, Vol. 2. Paris, 1894. [Fermat's original statement of the principle of least time.]

[2] Feynman, R.P. & Hibbs, A.R. (1965). *Quantum Mechanics and Path Integrals*. McGraw-Hill. [The path integral formulation of quantum mechanics, which contains Fermat's principle as its classical limit via the stationary phase approximation.]
