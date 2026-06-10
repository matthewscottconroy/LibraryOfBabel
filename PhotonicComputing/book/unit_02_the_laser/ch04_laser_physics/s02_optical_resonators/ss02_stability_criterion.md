# 4.2.2 Resonator Stability Criterion

## The ABCD Matrix Approach

In Chapter 2, we developed the ray transfer (ABCD) matrix formalism for paraxial optics. A resonator is stable if a paraxial ray, after many round trips, does not diverge to infinity. Using the ABCD matrices of Chapter 2, a round trip through a resonator of length $L$ with mirrors of radii of curvature $R_1$ and $R_2$ is:

$$M_{RT} = M_1 \cdot M_{prop} \cdot M_2 \cdot M_{prop}$$

$$= \begin{pmatrix}1 & 0 \\ -2/R_1 & 1\end{pmatrix} \begin{pmatrix}1 & L \\ 0 & 1\end{pmatrix} \begin{pmatrix}1 & 0 \\ -2/R_2 & 1\end{pmatrix} \begin{pmatrix}1 & L \\ 0 & 1\end{pmatrix}$$

Working out the matrix product, the trace of the round-trip matrix is:

$$\text{tr}(M_{RT}) = A + D = 2\left(1 - \frac{2L}{R_1}\right)\left(1 - \frac{2L}{R_2}\right) - 2 + 2$$

The stability criterion derived from Floquet theory for periodic optical systems is:

$$\left|\frac{A+D}{2}\right| \leq 1$$

which can be written in terms of the g-parameters:

$$g_1 = 1 - \frac{L}{R_1}, \quad g_2 = 1 - \frac{L}{R_2}$$

**Stability condition: $0 \leq g_1 g_2 \leq 1$**

## Stability Diagram

The stability diagram plots $g_2$ vs. $g_1$; stable resonators fall inside the hyperbolic region bounded by $g_1 g_2 = 0$ and $g_1 g_2 = 1$.

Key points on the stability diagram:

| Configuration | $g_1, g_2$ | Stability |
|---|---|---|
| Plane-plane ($R_1 = R_2 = \infty$) | (1, 1) | Marginally stable (boundary) |
| Symmetric confocal ($R_1 = R_2 = L$) | (0, 0) | Marginally stable (center) |
| Symmetric concentric ($R_1 = R_2 = L/2$) | (−1, −1) | Marginally stable (boundary) |
| Plano-concave ($R_1 = \infty$, $R_2 = L$) | (1, 0) | Marginally stable |
| General curved ($R_1, R_2 > L$) | $(0,1) \times (0,1)$ | Stable |

The plane-plane cavity used in the simplest Fabry-Pérot lasers is marginally stable — it sits on the stability boundary. In practice, the laser waveguide (which confines the mode laterally) provides the effective stabilization that keeps the mode from diverging.

## Why This Matters for Photonic Computing

Most integrated photonic computing systems use waveguide-confined lasers (semiconductor ridge waveguides, DFB gratings) where the mode confinement is provided by the waveguide, not by curved mirror geometry. However, the stability analysis matters for:

1. **External-cavity tunable lasers**: Widely-tunable lasers for WDM photonic computing use external cavities with lenses and gratings; their alignment sensitivity and mode stability depend directly on the resonator stability analysis.

2. **Microresonator lasers**: Ring resonators used as laser cavities (e.g., InP ring lasers) are analyzed as waveguide resonators, but their coupling geometry must satisfy analogous mode stability conditions.

3. **Free-space optical computing**: Systems that implement optical neural networks in free space (as in large-scale diffractive optical processors) require stable beam propagation over distances of tens of centimeters, which demands proper resonator or beam relay design.
