# Physical Origins of Laplace's and Poisson's Equations

Laplace's equation $\Delta u = 0$ and Poisson's equation $\Delta u = f$ are not abstract constructions — they are the mathematical expressions of physical equilibrium. They arise in electrostatics, gravitation, steady heat conduction, and irrotational fluid flow, and the physical intuition behind each derivation informs the choice of boundary conditions and the interpretation of solutions.

## Electrostatics

The electric field $\mathbf{E}$ in a region of space satisfies Maxwell's equations. In the electrostatic case (no time variation, no magnetic fields):

$$\nabla\times\mathbf{E} = 0, \qquad \nabla\cdot\mathbf{E} = \frac{\rho}{\varepsilon_0},$$

where $\rho$ is the charge density and $\varepsilon_0$ is the permittivity of free space.

The first equation allows writing $\mathbf{E} = -\nabla V$ for a scalar potential $V$ (the electric potential). The second equation then gives:

$$-\Delta V = \frac{\rho}{\varepsilon_0}, \qquad \text{i.e.,}\quad \Delta V = -\frac{\rho}{\varepsilon_0}.$$

This is **Poisson's equation** (with $f = -\rho/\varepsilon_0$). In a charge-free region ($\rho = 0$), it reduces to **Laplace's equation** $\Delta V = 0$.

**Boundary conditions:** On the surface of a perfect conductor, the potential is constant (the conductor is an equipotential surface): Dirichlet condition $V = V_0$. On an insulating surface with prescribed surface charge density $\sigma$, the normal component of $\mathbf{E}$ is prescribed: Neumann condition $\partial V/\partial n = -\sigma/\varepsilon_0$.

## Gravitational Potential

Newton's law of gravitation gives the gravitational potential $\Phi$ (in the sign convention where the field is $\mathbf{g} = -\nabla\Phi$):

$$\Delta\Phi = 4\pi G\rho,$$

where $G$ is Newton's gravitational constant and $\rho$ is the mass density. In empty space, $\rho = 0$ and $\Delta\Phi = 0$. The fundamental solution $\Phi = -Gm/r$ for a point mass $m$ at the origin satisfies $\Delta\Phi = 4\pi Gm\delta(\mathbf{x})$ in the distributional sense.

## Steady-State Heat Conduction

As established in Unit 3, the heat equation $u_t = \kappa\Delta u$ describes thermal conduction. In the steady state ($u_t = 0$), this becomes Laplace's equation $\Delta u = 0$. With an internal heat source $Q$ (watts per volume), the steady state satisfies Poisson's equation:

$$-\kappa\Delta u = Q/(\rho c_p), \qquad \text{i.e.,}\quad \Delta u = -Q/(k),$$

where $k$ is thermal conductivity. The solution is the steady temperature distribution maintained by the external heating against the heat losses through the boundary.

## Irrotational Incompressible Fluid Flow

For an ideal (inviscid) incompressible fluid, the velocity field $\mathbf{v}$ satisfies:

$$\nabla\cdot\mathbf{v} = 0 \quad \text{(incompressibility)}, \qquad \nabla\times\mathbf{v} = 0 \quad \text{(irrotationality)}.$$

The second condition allows writing $\mathbf{v} = \nabla\phi$ for a velocity potential $\phi$. The first condition then gives $\Delta\phi = 0$ — Laplace's equation.

Boundary conditions: on a rigid wall, the normal velocity is zero: $\partial\phi/\partial n = 0$ (Neumann). On a free surface (with prescribed pressure), a Dirichlet-type condition on $\phi$ is imposed (via Bernoulli's equation).

**Complex potential (2D):** In two dimensions, the velocity potential $\phi$ and stream function $\psi$ (defined by $\mathbf{v} = (\phi_x, \phi_y) = (\psi_y, -\psi_x)$) satisfy the Cauchy-Riemann equations $\phi_x = \psi_y$, $\phi_y = -\psi_x$. The complex potential $w(z) = \phi + i\psi$ is an analytic function of $z = x+iy$. The power of complex analysis — conformal mappings, residue theorem, Cauchy integral formula — can be brought to bear on 2D potential problems.

## Summary: Common Mathematical Structure

All these physical problems share a common mathematical structure:

| Physics | Unknown $u$ | Equation | Typical BCs |
|---------|------------|----------|-------------|
| Electrostatics | Electric potential $V$ | $\Delta V = -\rho/\varepsilon_0$ | Dirichlet on conductors |
| Gravity | Gravitational potential $\Phi$ | $\Delta\Phi = 4\pi G\rho$ | Decay at $\infty$ |
| Heat (steady) | Temperature | $\Delta u = -Q/k$ | Dirichlet/Neumann |
| Fluid flow | Velocity potential $\phi$ | $\Delta\phi = 0$ | Neumann on walls |

The mathematical theory developed in this unit applies simultaneously to all these problems. A formula derived for the electrostatic potential in a sphere immediately gives the steady-state temperature distribution in a ball, the velocity potential for flow past a sphere, and the gravitational potential outside a spherical mass. This universality is one of the great strengths of the mathematical approach to physics.
