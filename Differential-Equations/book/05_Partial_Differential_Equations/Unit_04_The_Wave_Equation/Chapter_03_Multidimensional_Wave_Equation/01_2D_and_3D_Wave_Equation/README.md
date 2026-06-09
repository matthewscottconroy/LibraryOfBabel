# The 2D and 3D Wave Equation

The wave equation in two and three spatial dimensions is the fundamental model for acoustic waves, electromagnetic radiation, and gravitational waves. The explicit solution formulas — Kirchhoff's formula in 3D and Poisson's formula in 2D — reveal a striking and physically important difference: in 3D, a sharp pulse produces a sharp signal at each observation point (a "click"), while in 2D, the same pulse produces a signal that begins at a definite time but then persists indefinitely (a "boom").

## Solution in Three Dimensions: Kirchhoff's Formula

The Cauchy problem for the 3D wave equation:

$$u_{tt} = c^2\Delta u, \quad \mathbf{x}\in\mathbb{R}^3, \; t>0, \quad u(\mathbf{x},0)=\phi(\mathbf{x}), \quad u_t(\mathbf{x},0)=\psi(\mathbf{x}).$$

**Kirchhoff's formula:**

$$u(\mathbf{x},t) = \frac{\partial}{\partial t}\left[\frac{1}{4\pi c^2 t}\oint_{|\mathbf{y}-\mathbf{x}|=ct}\phi(\mathbf{y})\,dS\right] + \frac{1}{4\pi c^2 t}\oint_{|\mathbf{y}-\mathbf{x}|=ct}\psi(\mathbf{y})\,dS. \tag{1}$$

The integrals are surface integrals over the sphere of radius $ct$ centered at $\mathbf{x}$. In terms of spherical averages:

$$\bar{\phi}(\mathbf{x},t) = \frac{1}{4\pi}\int_{|\boldsymbol{\omega}|=1}\phi(\mathbf{x}+ct\boldsymbol{\omega})\,dS(\boldsymbol{\omega}),$$

the formula is $u = \partial_t[t\bar\phi] + t\bar\psi$.

**Strong Huygens principle in 3D:** If $\phi$ and $\psi$ are supported in a ball $B_R(\mathbf{0})$, then $u(\mathbf{x},t) = 0$ for $||\mathbf{x}| - ct| > R$ — the signal is nonzero only when the sphere of radius $ct$ centered at $\mathbf{x}$ intersects the support of the initial data. Both the "leading front" at $|\mathbf{x}| = ct - R$ and the "trailing front" at $|\mathbf{x}| = ct + R$ are sharp. After the sphere has swept through the support (i.e., for $ct > |\mathbf{x}| + R$), the signal is exactly zero again.

This is the 3D counterpart of the observation that sound (or light) from a distant explosion arrives as a sharp pulse: you hear a crack, not a prolonged drone. In 2D, by contrast, you would hear the crack followed by a prolonged sound.

## Derivation via Radial Functions

For radially symmetric data $\phi(\mathbf{x}) = \phi(r)$ and $\psi(\mathbf{x}) = \psi(r)$ with $r = |\mathbf{x}|$, set $v = ru$. Then $v$ satisfies the 1D wave equation $v_{tt} = c^2 v_{rr}$ for $r > 0$ with $v(0,t) = 0$ (regularity at origin). D'Alembert's formula gives:

$$v(r,t) = \frac{1}{2}[(r+ct)\phi(r+ct) + (r-ct)\phi(r-ct)] + \frac{1}{2c}\int_{r-ct}^{r+ct}s\psi(s)\,ds.$$

The formula for $u = v/r$ recovers Kirchhoff's formula in the radially symmetric case.

## Solution in Two Dimensions: Poisson's Formula

The 2D Cauchy problem is solved by the **method of descent**: reduce to 3D by treating 2D initial data $\phi(x_1,x_2)$, $\psi(x_1,x_2)$ as 3D data independent of $x_3$, and apply Kirchhoff's formula.

Integrating over $x_3$ on the sphere of radius $ct$ gives the **Poisson formula** for 2D:

$$u(x_1,x_2,t) = \frac{1}{2\pi c}\frac{\partial}{\partial t}\left[\int_{|\mathbf{y}'-(x_1,x_2)|<ct}\frac{\phi(\mathbf{y}')}{\sqrt{c^2t^2-|\mathbf{y}'-(x_1,x_2)|^2}}\,d\mathbf{y}'\right] + \frac{1}{2\pi c}\int_{|\mathbf{y}'-(x_1,x_2)|<ct}\frac{\psi(\mathbf{y}')}{\sqrt{c^2t^2-|\mathbf{y}'-(x_1,x_2)|^2}}\,d\mathbf{y}'.$$

**Key difference from 3D:** The integrals are over disks (interiors of circles of radius $ct$), not just the boundary circles. This means:

**Weak Huygens principle in 2D:** If the initial data are supported in a ball $B_R(\mathbf{0})$, the signal at $\mathbf{x}$ is nonzero for $|\mathbf{x}| - R \leq ct$ (it turns on when the circle first reaches the support), but it does not turn off again — the disk $|\mathbf{y}'- \mathbf{x}| < ct$ continues to overlap the support $B_R(\mathbf{0})$ for all larger $t$.

This is the "afterglow" effect: in 2D, a sharp pulse of sound produces a signal that begins at a definite time (arrival of the wavefront) but then persists indefinitely.

## Comparison of Dimensions

| Dimension | Formula type | Huygens principle | Physical consequence |
|-----------|-------------|-------------------|---------------------|
| 1 | D'Alembert (interval) | Weak (trailing signal) | Pulse has trailing "tail" |
| 2 | Poisson (disk integral) | Weak (trailing signal) | Sound persists after arrival |
| 3 | Kirchhoff (sphere integral) | Strong (sharp wavefronts) | Sharp acoustic click |
| $n$ odd $\geq 3$ | Spherical mean | Strong | Sharp signals |
| $n$ even | Ball integral | Weak | Trailing signals |

The general pattern — odd dimensions satisfy the strong Huygens principle, even dimensions do not — follows from the theory of Riesz potentials and the structure of the fundamental solution, which involves distributions supported on the light cone for odd dimensions and involving the interior of the light cone for even dimensions.

## Fundamental Solution in 3D

The fundamental solution (Green's function for the 3D wave equation) is the distributional solution of $u_{tt} - c^2\Delta u = \delta(\mathbf{x})\delta(t)$:

$$E(\mathbf{x},t) = \frac{\delta(t - |\mathbf{x}|/c)}{4\pi c|\mathbf{x}|}.$$

This is supported exactly on the forward light cone $|\mathbf{x}| = ct$, $t > 0$ — a delta function on the sphere of radius $ct$. The convolution $u = E * f$ with initial data gives Kirchhoff's formula.

In 2D, the fundamental solution has support on and inside the light cone — the entire disk $|\mathbf{x}| \leq ct$ — reflecting the weak Huygens principle.
