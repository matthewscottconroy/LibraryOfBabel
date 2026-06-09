# Chapter 3: The Fourier Transform Method for the Heat Equation

When the spatial domain is all of $\mathbb{R}$ (or $\mathbb{R}^n$), the boundary conditions disappear and eigenfunction expansions become Fourier integrals. The Fourier transform converts the heat equation from a PDE in $(x,t)$ to an ODE in $t$ — one for each frequency $\xi$ — which can be solved explicitly. The resulting formula expresses the solution as a convolution of the initial data with the **heat kernel**, the fundamental solution of the heat equation.

## The Fourier Transform

The Fourier transform of $f \in L^1(\mathbb{R})$ is defined as

$$\hat{f}(\xi) = \int_{-\infty}^\infty f(x)e^{-i\xi x}\,dx, \qquad \xi \in \mathbb{R},$$

with inverse transform $f(x) = \frac{1}{2\pi}\int_{-\infty}^\infty\hat{f}(\xi)e^{i\xi x}\,d\xi$.

The key properties:
- $\widehat{f'}(\xi) = i\xi\hat{f}(\xi)$ (differentiation becomes multiplication by $i\xi$).
- $\widehat{f''}(\xi) = -\xi^2\hat{f}(\xi)$.
- $\widehat{f*g}(\xi) = \hat{f}(\xi)\hat{g}(\xi)$ (convolution in $x$ becomes multiplication).

## Structure of This Chapter

**Section 1: Heat Equation on the Real Line** applies the Fourier transform to convert $u_t = \kappa u_{xx}$ to $\hat{u}_t = -\kappa\xi^2\hat{u}$, an ODE with solution $\hat{u}(\xi,t) = \hat{f}(\xi)e^{-\kappa\xi^2 t}$. Inverting gives the convolution formula $u(x,t) = f * K(\cdot,t)$, where $K$ is the heat kernel.

**Section 2: The Fundamental Solution (Heat Kernel)** derives and analyzes the heat kernel $K(x,t) = (4\pi\kappa t)^{-1/2}e^{-x^2/(4\kappa t)}$ in detail. This is the solution corresponding to a delta-function initial condition $u(x,0) = \delta(x)$. Its properties — Gaussian shape, preservation of total integral, self-similarity — are developed and interpreted physically.

**Section 3: The Maximum Principle** is one of the deepest results in parabolic theory. It states that the maximum and minimum of a solution of the heat equation on a bounded domain must be attained either at the initial time or on the boundary — never in the interior at a later time. Consequences include uniqueness, comparison theorems, and control of the solution by its initial and boundary data.

## Key Theme

The Fourier transform method reveals the spectral structure of the heat operator: different frequency components $e^{i\xi x}$ evolve independently, each decaying at rate $\kappa\xi^2$. High frequencies decay faster than low frequencies — this is the frequency-domain statement of the smoothing property. The heat kernel is the synthesis of all these exponentially decaying waves, and the convolution formula $u = f * K$ is the continuous analogue of the discrete eigenfunction expansion.
