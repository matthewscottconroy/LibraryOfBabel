# Chapter 4: Green's Functions for Laplace's Equation

The Green's function for Laplace's equation is the central object of classical potential theory. It encodes the response of the potential to a point source in the presence of boundary conditions, and via Green's second identity it provides an explicit representation formula for the solution of any Dirichlet or Neumann problem. The Green's function connects the abstract theory (existence and uniqueness from the maximum principle) to explicit computation (the Poisson formula for the disk arises from the Green's function for the disk).

## The Fundamental Solution

The **fundamental solution** of $-\Delta$ in $\mathbb{R}^n$ is the radially symmetric function satisfying $-\Delta\Phi = \delta(\mathbf{x})$:

$$\Phi(\mathbf{x}) = \begin{cases} -\dfrac{1}{2\pi}\log|\mathbf{x}| & n=2 \\ \dfrac{1}{n(n-2)\omega_n|\mathbf{x}|^{n-2}} & n\geq 3 \end{cases}.$$

In 3D: $\Phi(\mathbf{x}) = 1/(4\pi|\mathbf{x}|)$. This is the Coulomb potential of a unit point charge at the origin.

## Structure of This Chapter

**Section 1: Green's Function and Representation Formula** defines the Green's function $G(\mathbf{x};\mathbf{y})$ for $-\Delta$ on a domain $\Omega$ as the fundamental solution with a correction term that enforces the Dirichlet boundary condition:

$$G(\mathbf{x};\mathbf{y}) = \Phi(\mathbf{x}-\mathbf{y}) - H(\mathbf{x};\mathbf{y}),$$

where $H(\cdot;\mathbf{y})$ is harmonic in $\Omega$ and equals $\Phi(\cdot-\mathbf{y})$ on $\partial\Omega$. Using Green's second identity, the solution of $-\Delta u = f$ with $u = g$ on $\partial\Omega$ is:

$$u(\mathbf{x}) = \int_\Omega G(\mathbf{x};\mathbf{y})f(\mathbf{y})\,d\mathbf{y} + \int_{\partial\Omega}g(\mathbf{y})\frac{\partial G}{\partial\nu_\mathbf{y}}(\mathbf{x};\mathbf{y})\,dS(\mathbf{y}).$$

The normal derivative of $G$ on $\partial\Omega$ is the Poisson kernel.

**Section 2: Method of Images** constructs Green's functions for simple domains by "reflecting" the fundamental solution across the boundary — the image charge method of electrostatics. For the upper half-space $\{x_n > 0\}$, the Green's function is $G(\mathbf{x};\mathbf{y}) = \Phi(\mathbf{x}-\mathbf{y}) - \Phi(\mathbf{x}-\mathbf{y}^*)$ where $\mathbf{y}^* = (y_1,\ldots,y_{n-1},-y_n)$ is the reflection of $\mathbf{y}$ across $\{x_n=0\}$.

**Section 3: Green's Function for the Half-Space** derives the Poisson formula for the upper half-plane and upper half-space from the method of images.

**Section 4: Green's Function for the Sphere** uses the Kelvin transform (inversion in the sphere) to construct the Green's function for a ball in $\mathbb{R}^n$, recovering the Poisson formula.

## Symmetry and Self-Adjointness

A key property: $G(\mathbf{x};\mathbf{y}) = G(\mathbf{y};\mathbf{x})$ (symmetry). This is equivalent to the self-adjointness of $-\Delta$ with Dirichlet boundary conditions and reflects the physical reciprocity principle: the potential at $\mathbf{x}$ due to a unit source at $\mathbf{y}$ equals the potential at $\mathbf{y}$ due to a unit source at $\mathbf{x}$.
