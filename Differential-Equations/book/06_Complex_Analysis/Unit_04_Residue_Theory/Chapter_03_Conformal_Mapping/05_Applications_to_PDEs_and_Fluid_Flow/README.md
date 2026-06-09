# Applications of Conformal Mapping to PDEs and Fluid Flow

Conformal mapping transforms difficult boundary value problems on complicated domains into equivalent problems on simple reference domains (the unit disk, the upper half-plane, or a strip), where the solution is known or easily computed. This technique is most powerful for two-dimensional problems governed by Laplace's equation: electrostatics, steady-state heat conduction, gravitational potential, and irrotational incompressible fluid flow. The key fact is that harmonic functions and the Laplace operator transform in a particularly clean way under conformal maps.

## Invariance of Laplace's Equation Under Conformal Mapping

**Theorem.** Let $f : D \to \Omega$ be a conformal map, and let $u : \Omega \to \mathbb{R}$ be harmonic. Then $u \circ f : D \to \mathbb{R}$ is harmonic on $D$.

**Proof.** Since $f$ is analytic, we can write $f = g$ analytically, and the composition of an analytic function with a harmonic function is harmonic. More explicitly: $\Delta(u \circ f) = |f'|^2 \Delta u \circ f$, which is $0$ whenever $\Delta u = 0$ (and $f' \neq 0$). $\square$

This means: solve $\Delta u = 0$ on $\Omega$ with boundary conditions, then pull back via $f^{-1}$ to get the solution on $D$.

## The Dirichlet Problem via Conformal Mapping

**Setup.** Solve $\Delta u = 0$ on a simply connected domain $D$, with $u = g$ on $\partial D$.

**Method.**
1. Find a conformal map $F : D \to \mathbb{D}$ (or $D \to H$).
2. Transform the boundary condition: $u$ on $\partial D$ corresponds to $\tilde{u} = u \circ F^{-1}$ on $\partial\mathbb{D}$.
3. Solve $\Delta\tilde{u} = 0$ on $\mathbb{D}$ with the given boundary data, using the Poisson formula:
$$\tilde{u}(re^{i\phi}) = \frac{1}{2\pi}\int_0^{2\pi}\frac{(1-r^2)g(\theta)}{1 - 2r\cos(\phi-\theta) + r^2}\, d\theta.$$
4. Pull back: $u(z) = \tilde{u}(F(z))$.

**Worked example.** Solve $\Delta u = 0$ on the upper half-plane $H$ with $u(x, 0) = \begin{cases} 1 & x > 0 \\ 0 & x < 0 \end{cases}$.

The solution is $u(x,y) = \frac{1}{\pi}\mathrm{Arg}(x + iy) + c$... The boundary condition gives $u = 1$ for $x > 0$ (arg $= 0$) and $u = 0$ for $x < 0$ (arg $= \pi$). So $u(x,y) = 1 - \frac{1}{\pi}\arctan(y/x)$ (using the appropriate branch). $\square$

## The Neumann Problem

For the Neumann problem ($\partial u/\partial n = g$ on $\partial D$), conformal mapping works analogously: the normal derivative transforms as $\partial(u \circ F^{-1})/\partial n = |{F^{-1}}'|^{-1}\partial u/\partial n$ on the boundary, so the boundary condition scales by the derivative of the conformal map.

## Two-Dimensional Fluid Flow

The complex potential for a two-dimensional, irrotational, incompressible flow is:
$$\Omega(z) = \phi(x,y) + i\psi(x,y),$$
where $\phi$ is the velocity potential (satisfying $\Delta\phi = 0$) and $\psi$ is the stream function. The complex velocity is $\overline{d\Omega/dz} = V_x - iV_y$, where $(V_x, V_y)$ is the fluid velocity. Streamlines are level curves $\{\psi = c\}$, and equipotential lines are $\{\phi = c\}$.

**Conformal invariance of flow.** A conformal map $w = f(z)$ transforms a flow with potential $\Omega(w)$ in the $w$-plane to a flow with potential $\Omega(f(z))$ in the $z$-plane. The streamlines and equipotential lines are preserved as sets (since they are the level curves of $\mathrm{Im}(\Omega \circ f)$ and $\mathrm{Re}(\Omega \circ f)$, which are also harmonic conjugates of each other).

**Worked example: Flow past a cylinder.** The complex potential for uniform flow $V_\infty$ past a cylinder of radius $a$ centered at the origin is:
$$\Omega(z) = V_\infty\left(z + \frac{a^2}{z}\right).$$
The stream function is $\psi = V_\infty(y - a^2 y/r^2)$, which vanishes on $|z| = a$ (the cylinder surface is a streamline).

The Joukowski transform $w = f(z) = z + c^2/z$ maps the circle $|z| = a$ to an airfoil-like profile in the $w$-plane. The flow in the $z$-plane (past the cylinder) maps to the flow in the $w$-plane (past the airfoil), with the same complex potential. This is the basis of the classical Joukowski airfoil theory. $\square$

## Electrostatics

In two-dimensional electrostatics, the electric potential $\phi$ satisfies $\Delta\phi = 0$ in charge-free regions, and the electric field is $\mathbf{E} = -\nabla\phi$. A conformal map transforms one electrostatic configuration into another.

**Worked example: Coaxial cylinders.** Find the potential between two coaxial cylindrical conductors of radii $r_1 < r_2$, with $\phi = V_1$ on the inner and $\phi = V_2$ on the outer.

By symmetry, $\phi$ depends only on $r = |z|$. The general solution of $\Delta\phi = 0$ with circular symmetry is $\phi = A\ln r + B$.

$A\ln r_1 + B = V_1$ and $A\ln r_2 + B = V_2$:
$$A = \frac{V_1 - V_2}{\ln(r_1/r_2)}, \qquad B = V_1 - A\ln r_1.$$

The complex potential is $\Omega(z) = A\ln z + B = A\mathrm{Log}\, z + B$ (using the principal branch), and the stream lines are rays from the origin while equipotential lines are concentric circles.

The capacitance per unit length is $C = 2\pi\varepsilon_0/\ln(r_2/r_1)$, the standard formula.

## Steady-State Heat Conduction

In steady state, the temperature $T(x,y)$ satisfies $\Delta T = 0$ in the domain. Conformal mapping allows one to solve heat conduction problems on complicated domains.

**Worked example: Temperature in a wedge.** Find the steady temperature in a wedge-shaped region $\{r > 0, 0 < \theta < \alpha\}$ with $T = T_1$ on one side ($\theta = 0$) and $T = T_2$ on the other ($\theta = \alpha$).

By separation of variables in polar coordinates, $T = A + B\theta$ satisfies $\Delta T = 0$ (since $\partial^2\theta/\partial x^2 + \partial^2\theta/\partial y^2 = 0$ in a simply connected domain not containing the origin). The boundary conditions give $A = T_1$ and $B = (T_2 - T_1)/\alpha$:
$$T(r, \theta) = T_1 + \frac{T_2 - T_1}{\alpha}\,\theta.$$
The temperature is independent of $r$: it varies linearly in the angular coordinate. $\square$

## The Schwarz-Christoffel Formula in Applications

For domains bounded by straight-line segments (channels, slots, gaps), the Schwarz-Christoffel transformation provides the exact conformal map from the upper half-plane. The resulting solutions are expressed as elliptic integrals or hypergeometric functions, but for practical purposes they can be evaluated numerically to high precision. Standard reference problems include:

- Flow through a two-dimensional channel with a step change in width.
- Electrostatic fringe fields near the edge of a parallel-plate capacitor.
- Heat flow near a corner of a conducting domain with mixed boundary conditions.
- Flow over a dam with a vertical face.

These applications demonstrate that complex analysis is not merely a mathematical theory but a powerful engineering tool.
