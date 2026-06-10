# Section 42.1: The Kerr Metric

---

## Section Introduction

The **Kerr solution** (1963) is the unique stationary, asymptotically flat, vacuum solution to Einstein's equations with nonzero angular momentum — the rotating black hole. It took 47 years after Schwarzschild's solution for Kerr to find this metric, despite its fundamental importance. The difficulty is that rotation breaks spherical symmetry, requiring a more complex ansatz and formidable calculations (Kerr used the Newman-Penrose formalism to find it).

The Kerr metric in **Boyer-Lindquist coordinates** $(t, r, \theta, \phi)$ is:
$$ds^2 = -\left(1-\frac{r_s r}{\Sigma}\right)c^2dt^2 - \frac{2r_s ra\sin^2\theta}{\Sigma}c\,dt\,d\phi + \frac{\Sigma}{\Delta}dr^2 + \Sigma\,d\theta^2 + \left(r^2+a^2+\frac{r_s ra^2\sin^2\theta}{\Sigma}\right)\sin^2\theta\,d\phi^2$$
where $r_s = 2GM/c^2$, $a = J/(Mc)$ is the specific angular momentum (with dimensions of length), $\Sigma = r^2 + a^2\cos^2\theta$, and $\Delta = r^2 - r_s r + a^2$.

Several features of this metric require comment. The off-diagonal term $g_{t\phi}\propto dt\,d\phi$ couples time and the azimuthal angle — this is the mathematical signature of rotation, and it has the physical consequence of **frame dragging**: spacetime is "dragged" in the direction of rotation near the black hole. The metric reduces to Schwarzschild when $a = 0$. For $a = r_s/2$ (the extremal case), the outer and inner horizons merge. For $|a| > r_s/2$, there are no horizons and the singularity is naked — but this extreme case is believed not to arise in nature.

The **singularity** in Kerr is not a point but a **ring**: in the equatorial plane at $r = 0$, $\Sigma = 0$, and the curvature diverges. The ring singularity is connected to a region with closed timelike curves (the solution can be extended through the ring to a region where $r < 0$ in Boyer-Lindquist notation), but these features are probably artifacts of the exact solution and not physically realizable.

---

## Subsections

- [42.1.1: Boyer-Lindquist Coordinates and Metric Components](42.1.1-coordinates.md)
- [42.1.2: Angular Momentum Parameter and Extremal Limit](42.1.2-angular.md)
- [42.1.3: Horizons: Roots of Δ](42.1.3-horizons.md)
- [42.1.4: The Ring Singularity](42.1.4-singularity.md)
- [42.1.5: Reduction to Schwarzschild and Flat Space Limits](42.1.5-limits.md)
