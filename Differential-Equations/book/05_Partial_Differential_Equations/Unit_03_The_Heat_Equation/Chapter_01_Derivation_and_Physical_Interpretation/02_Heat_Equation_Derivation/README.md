# Derivation of the Heat Equation

Fourier's law specifies the heat flux; the heat equation itself follows by combining this flux law with the principle of conservation of energy. The derivation is a model of how physical laws are translated into PDEs, and understanding it in detail builds the intuition needed to modify the equation for more complex situations (anisotropic materials, heat sources, phase changes).

## Conservation of Energy

Consider a fixed (Eulerian) control volume $\Omega \subset \mathbb{R}^3$, a region of space through which material may flow or in which heat may be stored and released. The thermal energy (heat content) stored in $\Omega$ is

$$E(t) = \int_\Omega \rho(\mathbf{x})\,c_p(\mathbf{x})\,u(\mathbf{x},t)\,d\mathbf{x},$$

where $\rho$ is the mass density (kg/m³), $c_p$ is the specific heat at constant pressure (J/(kg·K)), and $u$ is temperature. The conservation of energy principle states that the rate of change of $E$ equals the rate of heat flowing in through $\partial\Omega$ plus the rate of heat generated internally:

$$\frac{dE}{dt} = -\oint_{\partial\Omega}\mathbf{q}\cdot\hat{n}\,dS + \int_\Omega Q(\mathbf{x},t)\,d\mathbf{x},$$

where $\hat{n}$ is the outward unit normal to $\partial\Omega$, and $Q$ (W/m³) is the volumetric heat generation rate (from chemical reactions, electrical resistance heating, radioactive decay, etc.). The negative sign accounts for the convention that $\hat{n}$ points outward: heat flowing in corresponds to $\mathbf{q}\cdot\hat{n} < 0$.

## Applying Fourier's Law

Substituting $\mathbf{q} = -k\nabla u$ (Fourier's law) and applying the divergence theorem:

$$\oint_{\partial\Omega}\mathbf{q}\cdot\hat{n}\,dS = -\oint_{\partial\Omega}k\nabla u\cdot\hat{n}\,dS = -\int_\Omega\nabla\cdot(k\nabla u)\,d\mathbf{x}.$$

Therefore the energy balance becomes:

$$\int_\Omega\rho c_p\,u_t\,d\mathbf{x} = \int_\Omega\bigl[\nabla\cdot(k\nabla u) + Q\bigr]\,d\mathbf{x}.$$

Since $\Omega$ is arbitrary and all integrands are assumed continuous, the integrands must be equal pointwise:

$$\rho c_p\,u_t = \nabla\cdot(k\nabla u) + Q. \tag{1}$$

This is the general **heat equation** (or heat conduction equation).

## The Standard Heat Equation

For a homogeneous isotropic medium with constant thermal properties ($k$, $\rho$, $c_p$ all independent of position and temperature) and no internal heat sources ($Q=0$):

$$\rho c_p\,u_t = k\,\Delta u \implies u_t = \kappa\,\Delta u, \qquad \kappa = \frac{k}{\rho c_p}, \tag{2}$$

where $\kappa$ (m²/s) is the **thermal diffusivity**. This is the PDE we will study in this unit. In one spatial dimension: $u_t = \kappa\,u_{xx}$.

## Meaning of Each Term

- $\rho c_p\,u_t$: rate of thermal energy accumulation per unit volume. Positive means the temperature is rising.
- $\nabla\cdot(k\nabla u) = k\Delta u$: the Laplacian measures the excess of the average temperature on a sphere over the central value. If $\Delta u > 0$ at a point, the point is cooler than its surroundings, and heat flows in to raise its temperature. The heat equation says: temperature rises where it is below the local average.
- $Q$: any additional heat input, which would appear as a source term.

## Boundary Conditions

To determine a unique solution, equation (2) must be supplemented by:
1. **Initial condition:** $u(\mathbf{x},0) = u_0(\mathbf{x})$, specifying the initial temperature distribution.
2. **Boundary conditions** on $\partial\Omega$ for all $t > 0$: Dirichlet ($u = g$), Neumann ($\partial u/\partial n = h$), or Robin ($k\partial u/\partial n + \alpha u = \beta$), depending on the physical situation at the boundary.

## The Heat Equation in Various Coordinate Systems

**Cartesian (1D):**
$$u_t = \kappa\,u_{xx}.$$

**Cartesian (3D):**
$$u_t = \kappa\,(u_{xx} + u_{yy} + u_{zz}).$$

**Cylindrical (radially symmetric):**
$$u_t = \kappa\left(u_{rr} + \frac{1}{r}u_r\right) = \kappa\,\frac{1}{r}\frac{\partial}{\partial r}\left(r\frac{\partial u}{\partial r}\right),$$
for a function $u(r,t)$ independent of $\theta$ and $z$.

**Spherical (radially symmetric):**
$$u_t = \kappa\left(u_{rr} + \frac{2}{r}u_r\right) = \kappa\,\frac{1}{r^2}\frac{\partial}{\partial r}\left(r^2\frac{\partial u}{\partial r}\right).$$

These alternative forms arise naturally when the geometry of the domain has cylindrical or spherical symmetry, and they are treated in detail in Chapter 4.

## Nonhomogeneous and Nonlinear Variations

**Nonhomogeneous heat equation:** $u_t = \kappa\Delta u + f(\mathbf{x},t)$, where $f$ represents an external heat source. This is treated by superposition: find a particular solution of the nonhomogeneous equation and add the general solution of the homogeneous equation.

**Temperature-dependent conductivity:** If $k = k(u)$, the heat equation becomes $\rho c_p u_t = \nabla\cdot(k(u)\nabla u)$, which is nonlinear (quasilinear parabolic). The porous medium equation $u_t = \Delta(u^m)$ for $m > 1$ is a related example with a free boundary (the edge of the diffusing region).

**Nonlinear source terms:** Reaction-diffusion equations $u_t = \kappa\Delta u + f(u)$ combine diffusion with local reactions. Fisher's equation ($f(u) = ru(1-u)$, modeling population dynamics) and the Newell-Whitehead-Segel equation are important examples, treated in Unit 7.

## Energy Estimate and Well-Posedness

Multiplying (2) by $u$ and integrating over $\Omega$ (with Dirichlet boundary conditions $u|_{\partial\Omega}=0$):

$$\frac{1}{2}\frac{d}{dt}\int_\Omega u^2\,d\mathbf{x} = \kappa\int_\Omega u\Delta u\,d\mathbf{x} = -\kappa\int_\Omega|\nabla u|^2\,d\mathbf{x} \leq 0.$$

The $L^2$ norm of the solution is nonincreasing: $\|u(\cdot,t)\|_{L^2(\Omega)} \leq \|u(\cdot,0)\|_{L^2(\Omega)}$ for all $t \geq 0$. This is the energy estimate, and it immediately implies both uniqueness (the difference of two solutions has zero $L^2$ norm) and continuous dependence on initial data (small $L^2$ errors stay small). Together with an existence theorem (via separation of variables or semigroup theory), it establishes well-posedness of the initial-boundary value problem.
