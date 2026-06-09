# Chapter 1: Derivation and Physical Interpretation of the Heat Equation

Understanding where a PDE comes from is as important as knowing how to solve it. The heat equation is not an abstract definition — it is a consequence of two physical laws: Fourier's law of heat conduction, which describes how heat flows in response to temperature gradients, and the principle of conservation of energy, which governs how much heat is stored and released in a material. Deriving the equation from first principles reveals both the assumptions built into the model and the meaning of each term.

## Structure of This Chapter

**Section 1: Fourier's Law of Heat Conduction** states and motivates the empirical law that the heat flux vector $\mathbf{q}$ (the rate of heat energy flowing per unit area per unit time) is proportional to the negative temperature gradient:

$$\mathbf{q} = -k\nabla u,$$

where $k > 0$ is the thermal conductivity and $u$ is temperature. Heat flows from hot to cold — down the gradient — at a rate proportional to the steepness of the temperature variation. This law is the thermal analogue of Fick's law of diffusion and Darcy's law of porous media flow.

**Section 2: Derivation of the Heat Equation** combines Fourier's law with conservation of energy. The rate of change of heat content in a region $\Omega$ equals the flux of heat through its boundary plus any internal heat generation:

$$\frac{d}{dt}\int_\Omega \rho c_p u\,dV = -\oint_{\partial\Omega}\mathbf{q}\cdot\hat{n}\,dS + \int_\Omega Q\,dV,$$

where $\rho$ is density, $c_p$ is specific heat at constant pressure, and $Q$ is the heat source density (watts per cubic meter). Applying the divergence theorem and using Fourier's law gives the heat equation:

$$\rho c_p\,u_t = \nabla\cdot(k\nabla u) + Q.$$

For a homogeneous isotropic medium (constant $k$, $\rho$, $c_p$) with no internal sources:

$$u_t = \kappa\,\Delta u, \qquad \kappa = \frac{k}{\rho c_p}.$$

The constant $\kappa$ is the **thermal diffusivity** (units: m²/s). Typical values range from $10^{-7}$ m²/s for water to $10^{-4}$ m²/s for copper.

**Section 3: Diffusion Interpretation** situates the heat equation in the broader context of diffusion processes. Fick's second law for the concentration $c$ of a chemical species is $c_t = D\Delta c$, identical in form to the heat equation with diffusivity $D$. The diffusion equation describes Brownian motion: if a particle performs a random walk, its probability density satisfies the diffusion equation. This connection between diffusion and random walks is the basis of stochastic analysis and underlies the Black-Scholes equation of mathematical finance.

## Key Dimensional Analysis

The heat equation $u_t = \kappa u_{xx}$ (in one dimension) has a characteristic time scale for equilibration over a length $L$:

$$t_{\text{diffuse}} \sim \frac{L^2}{\kappa}.$$

This $L^2$ scaling — diffusion time grows as the square of length scale — is a fundamental signature of parabolic equations. To halve the equilibration time, one must halve the length scale (or quadruple $\kappa$ by changing the material). This contrasts sharply with wave propagation, where time scales linearly with length.

Nondimensionalizing by setting $\bar{x} = x/L$, $\bar{t} = \kappa t/L^2$ reduces the heat equation to $\bar{u}_{\bar{t}} = \bar{u}_{\bar{x}\bar{x}}$ with $\kappa$ eliminated. All solutions are then related by this scaling, and qualitative properties depend only on the dimensionless parameters.
