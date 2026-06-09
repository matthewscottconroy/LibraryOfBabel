# The Green's Function for the Heat Equation

The Green's function for the heat equation encodes the complete response of the system to point sources. Once it is known, the solution to any initial-boundary value problem with any source term can be written as an explicit integral. The derivation and properties of the heat Green's function are central to the modern theory of parabolic PDEs.

## Definition on a Bounded Domain

Let $\Omega \subset \mathbb{R}^n$ be a bounded domain with smooth boundary. The **Green's function** $G(\mathbf{x},t;\mathbf{y},s)$ for the heat equation on $\Omega$ is defined by:

$$\frac{\partial G}{\partial t} = \kappa\Delta_\mathbf{x} G + \delta(\mathbf{x}-\mathbf{y})\delta(t-s), \qquad \mathbf{x} \in \Omega,\; t > 0,$$
$$G(\mathbf{x},t;\mathbf{y},s) = 0 \quad \text{for } \mathbf{x} \in \partial\Omega \text{ or } t < s,$$

where $(\mathbf{y},s)$ is the "source point" and $(\mathbf{x},t)$ is the "observation point."

## Eigenfunction Expansion of $G$

On a bounded domain, the Green's function can be expanded in the eigenfunctions of $-\Delta$ on $\Omega$:

$$-\Delta\phi_n = \lambda_n\phi_n \text{ in } \Omega, \qquad \phi_n|_{\partial\Omega} = 0, \qquad \langle\phi_m,\phi_n\rangle = \delta_{mn}.$$

The Green's function is:

$$G(\mathbf{x},t;\mathbf{y},s) = \sum_{n=1}^\infty e^{-\kappa\lambda_n(t-s)}\phi_n(\mathbf{x})\phi_n(\mathbf{y}) \qquad \text{for } t > s. \tag{1}$$

**Verification:** Apply $\partial_t - \kappa\Delta_\mathbf{x}$ to (1):

$$G_t - \kappa\Delta G = \sum_n (-\kappa\lambda_n + \kappa\lambda_n)e^{-\kappa\lambda_n(t-s)}\phi_n(\mathbf{x})\phi_n(\mathbf{y}) = 0 \quad \text{for } t \neq s.$$

As $t \to s^+$, $G(\mathbf{x},t;\mathbf{y},s) \to \sum_n\phi_n(\mathbf{x})\phi_n(\mathbf{y}) = \delta(\mathbf{x}-\mathbf{y})$ (completeness of the eigenfunctions).

So $G_t - \kappa\Delta G = \delta(\mathbf{x}-\mathbf{y})\delta(t-s)$ in the distributional sense.

## Representation Formula

Using the Green's function, the solution of the initial-boundary value problem

$$u_t = \kappa\Delta u + F(\mathbf{x},t) \text{ in } \Omega\times(0,T],$$
$$u = 0 \text{ on } \partial\Omega\times(0,T], \qquad u(\mathbf{x},0) = f(\mathbf{x}),$$

is given by:

$$u(\mathbf{x},t) = \int_\Omega G(\mathbf{x},t;\mathbf{y},0)\,f(\mathbf{y})\,d\mathbf{y} + \int_0^t\!\!\int_\Omega G(\mathbf{x},t;\mathbf{y},s)\,F(\mathbf{y},s)\,d\mathbf{y}\,ds. \tag{2}$$

**Derivation:** Multiply the eigenfunction expansion of $u(\mathbf{x},t) = \sum_n T_n(t)\phi_n(\mathbf{x})$ by the Green's function and use the mode equations $T_n' + \kappa\lambda_n T_n = f_n(t)$ (from the variation of parameters formula for each mode). The result is exactly (2).

## Symmetry and Self-Adjointness

The heat Green's function is symmetric in its space-time arguments in the following sense (a consequence of self-adjointness of $-\Delta$):

$$G(\mathbf{x},t;\mathbf{y},s) = G(\mathbf{y},t;\mathbf{x},s) \quad \text{(same time arguments)}$$

and satisfies the **adjoint symmetry**:

$$G(\mathbf{x},t;\mathbf{y},s) = G(\mathbf{y},-s;\mathbf{x},-t) \quad \text{(time reversal transforms the forward into the adjoint problem)}.$$

In practice, the eigenfunction expansion (1) shows the symmetry $G(\mathbf{x},t;\mathbf{y},s) = G(\mathbf{y},t;\mathbf{x},s)$ directly: the double sum is symmetric in $\mathbf{x}$ and $\mathbf{y}$.

## The Heat Trace

The **heat trace** is

$$Z(t) = \int_\Omega G(\mathbf{x},t;\mathbf{x},0)\,d\mathbf{x} = \sum_{n=1}^\infty e^{-\kappa\lambda_n t}.$$

This is the trace of the heat semigroup operator $e^{\kappa t\Delta}$. It encodes important geometric information about $\Omega$ via the Weyl asymptotic formula for eigenvalues and the heat kernel expansion:

$$Z(t) \sim \frac{|\Omega|}{(4\pi\kappa t)^{n/2}} - \frac{|\partial\Omega|}{4(4\pi\kappa t)^{(n-1)/2}} + O(t^{-(n-2)/2}) \quad \text{as } t \to 0^+,$$

where $|\Omega|$ is the volume of $\Omega$ and $|\partial\Omega|$ is the surface area of $\partial\Omega$. This expansion shows that the heat kernel can "hear" the shape of the domain — at least its volume, surface area, and higher-order geometric invariants (curvature, topology). The question of whether one can "hear the shape of a drum" (whether isospectral domains must be isometric) was famously posed by Mark Kac (1966) and answered negatively by Gordon, Webb, and Wolpert (1992).

## Connection to the Elliptic Green's Function

The elliptic Green's function $G_{\text{ell}}(\mathbf{x};\mathbf{y})$ for $-\Delta$ on $\Omega$ (the Green's function for Laplace's equation, developed in Unit 5) is related to the parabolic Green's function by:

$$G_{\text{ell}}(\mathbf{x};\mathbf{y}) = \int_0^\infty G(\mathbf{x},t;\mathbf{y},0)\,dt.$$

Formally, this says that the steady-state response to a continuous unit point source equals the integral over all time of the transient responses. In terms of eigenfunction expansions:

$$G_{\text{ell}}(\mathbf{x};\mathbf{y}) = \sum_n \frac{1}{\kappa\lambda_n}\phi_n(\mathbf{x})\phi_n(\mathbf{y}),$$

which is exactly the spectral expansion of the elliptic Green's function ($(-\Delta)^{-1}$ applied to $\delta(\mathbf{x}-\mathbf{y})$). This connection between parabolic and elliptic Green's functions is fundamental to potential theory and operator semigroup theory.
