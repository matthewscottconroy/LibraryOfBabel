# The Laplacian

The heat equation, the wave equation, Poisson's equation in electrostatics, the Schrödinger equation of quantum mechanics — these foundational equations of mathematical physics all contain a single differential operator acting on a scalar function: the Laplacian. Denoted $\nabla^2$ or $\Delta$, the Laplacian is defined as the divergence of the gradient. Its prominence throughout mathematics and physics is not accidental: it is the natural measure of how a function compares to its average over surrounding regions, and this comparison property is the key to understanding physical equilibrium, wave propagation, and diffusion.

## Definition

For a twice continuously differentiable scalar field $f: D \subseteq \mathbb{R}^3 \to \mathbb{R}$, the **Laplacian** of $f$ is

$$\nabla^2 f = \nabla \cdot (\nabla f) = \frac{\partial^2 f}{\partial x^2} + \frac{\partial^2 f}{\partial y^2} + \frac{\partial^2 f}{\partial z^2}.$$

The notation $\Delta f$ is also standard and is preferred in some texts on PDEs. In two dimensions:

$$\nabla^2 f = \frac{\partial^2 f}{\partial x^2} + \frac{\partial^2 f}{\partial y^2}.$$

## Geometric Interpretation: Comparison with Local Average

The Laplacian measures, in a precise sense, how the value of $f$ at a point differs from the average value of $f$ in a small neighborhood. Specifically, for a ball $B_\varepsilon(\mathbf{p})$ of radius $\varepsilon$:

$$f(\mathbf{p}) \approx \langle f \rangle_{B_\varepsilon(\mathbf{p})} - \frac{\varepsilon^2}{2(n+2)}\nabla^2 f(\mathbf{p}) + O(\varepsilon^4),$$

where $\langle f \rangle$ denotes the average over the ball and $n$ is the dimension. Rearranging:

$$\nabla^2 f(\mathbf{p}) \approx \frac{2(n+2)}{\varepsilon^2}\left(\langle f \rangle_{B_\varepsilon} - f(\mathbf{p})\right).$$

**Positive Laplacian:** $f(\mathbf{p})$ is less than its local average — the function curves upward, like the bottom of a bowl.
**Negative Laplacian:** $f(\mathbf{p})$ exceeds its local average — the function curves downward, like the top of a hill.
**Zero Laplacian:** $f(\mathbf{p})$ equals its local average — harmonic functions; no local extrema in the interior.

## Harmonic Functions

A function satisfying $\nabla^2 f = 0$ on a domain $D$ is called **harmonic** on $D$. Harmonic functions appear throughout mathematics and physics:

- The electrostatic potential $\phi$ in a charge-free region satisfies $\nabla^2 \phi = 0$ (Laplace's equation).
- The steady-state temperature distribution in a homogeneous solid with no internal heat sources satisfies $\nabla^2 T = 0$.
- The velocity potential $\phi$ of an irrotational, incompressible fluid satisfies $\nabla^2 \phi = 0$.
- The real and imaginary parts of any analytic function of a complex variable are harmonic (in two dimensions).

**Mean Value Property (Theorem).** If $f$ is harmonic on an open connected set $D \subseteq \mathbb{R}^n$ and $\overline{B_\varepsilon(\mathbf{p})} \subset D$, then

$$f(\mathbf{p}) = \frac{1}{\text{Vol}(B_\varepsilon)} \int_{B_\varepsilon(\mathbf{p})} f\,dV = \frac{1}{\text{Area}(S_\varepsilon)}\iint_{S_\varepsilon(\mathbf{p})} f\,dS.$$

The value of a harmonic function at any point equals its average over any sphere (or ball) centered there, provided the sphere lies within the domain. This is not an approximation; it holds exactly.

**Maximum Principle (Theorem).** A harmonic function on a bounded connected domain that achieves its maximum (or minimum) at an interior point must be constant. In particular, the maximum and minimum of a harmonic function on a closed bounded domain are attained on the boundary.

This theorem has physical content: in steady-state heat flow, the hottest and coldest points are always on the boundary of the region.

## Poisson's Equation

The inhomogeneous version of Laplace's equation is **Poisson's equation**:

$$\nabla^2 f = g,$$

where $g$ is a given source term. In electrostatics, $\nabla^2 \phi = -\rho/\varepsilon_0$, where $\rho$ is the charge density. In Newtonian gravity, $\nabla^2 \phi = 4\pi G\rho$, where $\rho$ is the mass density. Solving Poisson's equation is one of the central problems of mathematical physics, and the Divergence Theorem provides a key tool via Green's identities.

## Worked Examples

**Example 1.** Let $f(x,y,z) = x^2 + y^2 + z^2 = r^2$.

$$\nabla^2 f = \frac{\partial^2}{\partial x^2}(x^2) + \frac{\partial^2}{\partial y^2}(y^2) + \frac{\partial^2}{\partial z^2}(z^2) = 2 + 2 + 2 = 6.$$

At every point, $f$ curves upward. The minimum at the origin (where $f=0$) is reflected in the positive Laplacian.

**Example 2.** Let $f(x,y,z) = 1/r = (x^2+y^2+z^2)^{-1/2}$, defined for $\mathbf{r} \neq \mathbf{0}$.

We computed $\partial_x(1/r) = -x/r^3$ earlier. Differentiating again:

$$\frac{\partial^2}{\partial x^2}\left(\frac{1}{r}\right) = -\frac{r^3 - x \cdot 3r^2 \cdot (x/r)}{r^6} = \frac{3x^2 - r^2}{r^5}.$$

Summing over all three coordinates:

$$\nabla^2\left(\frac{1}{r}\right) = \frac{3x^2 - r^2 + 3y^2 - r^2 + 3z^2 - r^2}{r^5} = \frac{3r^2 - 3r^2}{r^5} = 0, \quad \mathbf{r} \neq \mathbf{0}.$$

So $1/r$ is harmonic on $\mathbb{R}^3 \setminus \{\mathbf{0}\}$. At the origin, the Laplacian is a Dirac delta: $\nabla^2(1/r) = -4\pi\delta^{(3)}(\mathbf{r})$ in the sense of distributions.

**Example 3.** Let $f(x,y) = e^x\cos y$.

$$\nabla^2 f = e^x\cos y + e^x(-\cos y) = 0.$$

This function is harmonic in $\mathbb{R}^2$. It is the real part of $e^z$ where $z = x + iy$, confirming the general principle that real parts of analytic functions are harmonic.

## The Laplacian in Curvilinear Coordinates

In cylindrical coordinates $(r, \theta, z)$:

$$\nabla^2 f = \frac{1}{r}\frac{\partial}{\partial r}\left(r\frac{\partial f}{\partial r}\right) + \frac{1}{r^2}\frac{\partial^2 f}{\partial \theta^2} + \frac{\partial^2 f}{\partial z^2}.$$

In spherical coordinates $(\rho, \theta, \phi)$ (radial distance $\rho$, polar angle $\theta$, azimuthal angle $\phi$):

$$\nabla^2 f = \frac{1}{\rho^2}\frac{\partial}{\partial \rho}\left(\rho^2\frac{\partial f}{\partial \rho}\right) + \frac{1}{\rho^2\sin\theta}\frac{\partial}{\partial\theta}\left(\sin\theta\frac{\partial f}{\partial\theta}\right) + \frac{1}{\rho^2\sin^2\theta}\frac{\partial^2 f}{\partial\phi^2}.$$

For radially symmetric functions $f(\rho)$, the spherical Laplacian simplifies dramatically:

$$\nabla^2 f = \frac{1}{\rho^2}\frac{d}{d\rho}\left(\rho^2\frac{df}{d\rho}\right) = f''(\rho) + \frac{2}{\rho}f'(\rho).$$

The harmonic radially-symmetric solutions are $f = A/\rho + B$ (in three dimensions) — the fundamental solutions that give rise to the $1/r$ potentials of gravity and electrostatics.

## The Vector Laplacian

One can also apply the Laplacian to vector fields component-wise: $\nabla^2\mathbf{F} = (\nabla^2 P)\,\mathbf{i} + (\nabla^2 Q)\,\mathbf{j} + (\nabla^2 R)\,\mathbf{k}$ in Cartesian coordinates. A key vector identity is

$$\nabla^2\mathbf{F} = \nabla(\nabla \cdot \mathbf{F}) - \nabla \times (\nabla \times \mathbf{F}).$$

This identity is fundamental in electrodynamics: the wave equation for $\mathbf{E}$ in free space, $\nabla^2\mathbf{E} = \mu_0\varepsilon_0\,\partial^2\mathbf{E}/\partial t^2$, is derived using this identity applied to Faraday's and Ampere's laws.

## Summary

The Laplacian $\nabla^2 f$ measures the deviation of $f$ at a point from its local average. Functions satisfying $\nabla^2 f = 0$ (harmonic functions) have no interior extrema, equal their averages over spheres, and arise as the steady states of physical systems governed by diffusion or as potentials of irrotational source-free fields. The Laplacian is the differential operator at the core of the classical partial differential equations of physics, making it indispensable for the rest of this course.
