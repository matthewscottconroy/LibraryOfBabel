# Section 7.2: Functions of Several Variables

---

## Section Introduction

A function of several variables assigns a real number (or a vector of real numbers) to each point in $\mathbb{R}^n$. Such functions describe every physical field: temperature $T(x, y, z)$ at each point in space; electric potential $\phi(x, y, z)$; fluid velocity $\mathbf{v}(x, y, z, t)$; the gravitational metric components $g_{\mu\nu}(x^0, x^1, x^2, x^3)$. Before we can differentiate or integrate such functions, we need to understand what it means for them to be continuous, bounded, or defined only on a subset of $\mathbb{R}^n$.

The key concept is **continuity** at a point $\mathbf{a}$: $f$ is continuous at $\mathbf{a}$ if for every $\varepsilon > 0$ there exists $\delta > 0$ such that $\|\mathbf{x} - \mathbf{a}\| < \delta$ implies $|f(\mathbf{x}) - f(\mathbf{a})| < \varepsilon$. This is the same $\varepsilon$-$\delta$ definition as in one variable, now with the multivariable distance $\|\mathbf{x} - \mathbf{a}\|$. The extension is straightforward, but the consequences are richer: limits in $\mathbb{R}^n$ can be approached along any path, and a function can have directional limits that exist but differ from path to path, preventing continuity.

A fundamental theorem: a continuous function on a **compact** set (closed and bounded in $\mathbb{R}^n$) attains its maximum and minimum values. This is the Extreme Value Theorem in $n$ dimensions. Compactness is not merely a technical condition — it is the key property that makes analysis in higher dimensions tractable and that generalizes to manifolds in GR.

Vector-valued functions $\mathbf{F}: \mathbb{R}^n \to \mathbb{R}^m$ are equally important. They describe coordinate transformations, flow maps, and mappings between manifolds. The **Jacobian matrix** of a vector-valued function encodes all the information about its linear approximation — it is the multivariable generalization of the derivative, and it is the precursor to the tangent map between manifolds (Section 7.4).

---

## Subsections

- [7.2.1: Scalar and Vector Fields](7.2.1-scalar-vector-fields.md)
- [7.2.2: Limits and Continuity](7.2.2-limits-continuity.md)
- [7.2.3: Paths and Directional Approach](7.2.3-paths.md)
- [7.2.4: Compact Sets and Extreme Values](7.2.4-compact-sets.md)
- [7.2.5: Vector-Valued Functions](7.2.5-vector-valued.md)
