# Fourier's Law of Heat Conduction

In 1822, Jean Baptiste Joseph Fourier published Théorie analytique de la chaleur, in which he proposed that the rate of heat flow through a material is proportional to the temperature gradient. This empirical law, now bearing his name, is the foundation of the mathematical theory of heat conduction and the starting point for deriving the heat equation.

## Statement of Fourier's Law

Let $u(\mathbf{x},t)$ denote the temperature (in Kelvin or degrees Celsius) at position $\mathbf{x} \in \mathbb{R}^3$ and time $t$. The **heat flux vector** $\mathbf{q}(\mathbf{x},t)$ (units: watts per square meter, W/m²) is defined so that the rate of heat energy flowing through an infinitesimal area element $dA$ with outward normal $\hat{n}$ is $\mathbf{q}\cdot\hat{n}\,dA$.

**Fourier's Law** states:

$$\mathbf{q} = -k\,\nabla u, \tag{1}$$

where $k > 0$ is the **thermal conductivity** of the material (units: W/(m·K)). The negative sign ensures that heat flows from hot to cold: if temperature increases in the direction $\hat{n}$, then $\nabla u\cdot\hat{n} > 0$, so $\mathbf{q}\cdot\hat{n} < 0$ — heat flows in the direction $-\hat{n}$, i.e., away from the hot region.

## Physical Motivation

Fourier's law is empirical, not derivable from first principles alone (at least not without microscopic kinetic theory). Its justification rests on:

1. **Dimensional analysis:** The only local quantity that can drive heat flow is the temperature gradient $\nabla u$ (global temperature differences drive flow over large distances, but locally only the gradient matters). For small gradients, linearity is the natural first approximation.

2. **Isotropy:** In an isotropic material (no preferred directions), $\mathbf{q}$ must be a scalar multiple of $\nabla u$. Fourier's law is the simplest such relation.

3. **Experimental verification:** For metals, polymers, gases (in the regime of small Knudsen number), and most solid materials under ordinary conditions, Fourier's law is experimentally accurate.

The law fails in highly non-equilibrium situations (very short time scales, very thin films, materials with long-range correlations) where memory effects or ballistic phonon transport become important. Modifications include the Cattaneo-Vernotte equation $\tau \mathbf{q}_t + \mathbf{q} = -k\nabla u$ (which leads to a damped wave equation for temperature rather than a parabolic equation).

## Thermal Conductivity Values

Thermal conductivity $k$ varies enormously across materials:

| Material | $k$ (W/m·K) |
|---------|------------|
| Air (at room temperature) | 0.026 |
| Water | 0.6 |
| Glass | 1.0 |
| Concrete | 1.7 |
| Steel | 50 |
| Aluminum | 200 |
| Copper | 400 |
| Diamond | 2000 |

A high $k$ means heat flows rapidly in response to a given temperature gradient — metals equilibrate quickly because of free electron heat transport. Insulators (air, foam) have low $k$ and resist heat flow.

## Anisotropic Media

In an anisotropic material (such as a crystal or a layered composite), the thermal conductivity depends on direction, and Fourier's law generalizes to

$$\mathbf{q} = -\mathbf{K}\nabla u,$$

where $\mathbf{K}$ is the **thermal conductivity tensor** (a symmetric positive definite $3\times 3$ matrix). The heat equation becomes $\rho c_p u_t = \nabla\cdot(\mathbf{K}\nabla u)$, which has the form of a second-order elliptic operator in space. If $\mathbf{K}$ is constant, this can be reduced to the standard Laplacian by a linear change of coordinates.

## Heat Flux and the Divergence Theorem

The total rate of heat flowing into a region $\Omega$ through its boundary is

$$\dot{Q}_{\text{in}} = -\oint_{\partial\Omega}\mathbf{q}\cdot\hat{n}\,dS = \oint_{\partial\Omega}k\nabla u\cdot\hat{n}\,dS = k\int_\Omega\Delta u\,d\mathbf{x},$$

by the divergence theorem and Fourier's law. This integral form connects the heat flux (a boundary phenomenon) to the Laplacian of temperature (an interior phenomenon). It is the key step in deriving the heat equation: the Laplacian measures how much hotter a point is than its immediate surroundings, and Fourier's law says that heat flows in to eliminate this discrepancy.

## Connection to Fick's and Darcy's Laws

Fourier's law belongs to a family of linear constitutive relations of the form "flux proportional to gradient of potential":

- **Fick's first law** (diffusion): $\mathbf{J} = -D\nabla c$, where $\mathbf{J}$ is the mass flux of a chemical species, $D$ is the diffusion coefficient, and $c$ is concentration.
- **Darcy's law** (porous media): $\mathbf{v} = -(k/\mu)\nabla p$, where $\mathbf{v}$ is the fluid velocity, $k$ is permeability, $\mu$ is viscosity, and $p$ is pressure.
- **Ohm's law** (electricity): $\mathbf{J} = \sigma\mathbf{E} = -\sigma\nabla V$, where $\mathbf{J}$ is current density, $\sigma$ is electrical conductivity, and $V$ is electric potential.

Each of these laws, combined with the appropriate conservation equation, leads to a diffusion equation for the relevant potential (temperature, concentration, pressure, voltage). The mathematical theory is identical in each case, which is why the heat equation and its solutions are relevant far beyond thermal physics.

## Steady-State Heat Conduction

In the steady state ($u_t = 0$), the heat equation reduces to Laplace's equation $\Delta u = 0$ (in the absence of sources). Fourier's law then says the heat flux satisfies $\nabla\cdot\mathbf{q} = 0$ — the divergence of the flux is zero, meaning heat is neither created nor destroyed in the interior. The study of steady heat conduction is therefore equivalent to potential theory, and all the tools of harmonic function theory (maximum principle, mean value property, Green's functions) apply directly.

This connection between the dynamic heat equation and the static Laplace equation motivates the study of Laplace's equation in Unit 5, where the full theory of harmonic functions is developed.
