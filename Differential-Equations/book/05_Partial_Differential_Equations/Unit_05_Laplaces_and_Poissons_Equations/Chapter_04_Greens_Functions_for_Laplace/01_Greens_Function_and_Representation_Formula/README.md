# Green's Function and the Representation Formula

The Green's function for Laplace's equation on a domain $\Omega$ is the potential due to a unit point source at $\mathbf{y}$, subject to zero Dirichlet conditions on $\partial\Omega$. It encodes the full effect of both the source and the boundary in a single function, and provides the solution to any Dirichlet or Poisson problem as a convolution integral.

## Definition

The **Green's function** $G(\mathbf{x};\mathbf{y})$ for $-\Delta$ on $\Omega$ with Dirichlet boundary conditions is defined by:

1. $-\Delta_\mathbf{x}G(\mathbf{x};\mathbf{y}) = \delta(\mathbf{x}-\mathbf{y})$ for $\mathbf{x} \in \Omega$.
2. $G(\mathbf{x};\mathbf{y}) = 0$ for $\mathbf{x} \in \partial\Omega$.
3. $G(\mathbf{x};\mathbf{y}) = \Phi(\mathbf{x}-\mathbf{y}) - H(\mathbf{x};\mathbf{y})$, where $H$ is the correction (harmonic in $\mathbf{x}$) ensuring condition 2.

Here $\Phi$ is the fundamental solution and $H(\cdot;\mathbf{y})$ satisfies $\Delta H = 0$ in $\Omega$ with $H(\mathbf{x};\mathbf{y}) = \Phi(\mathbf{x}-\mathbf{y})$ on $\partial\Omega$.

## Derivation of the Representation Formula

Apply Green's second identity $\int_\Omega(u\Delta v - v\Delta u)\,d\mathbf{x} = \oint_{\partial\Omega}(u\partial v/\partial\nu - v\partial u/\partial\nu)\,dS$ with $v(\mathbf{x}) = G(\mathbf{x};\mathbf{y})$ and $u$ the solution of $-\Delta u = f$, $u|_{\partial\Omega} = g$:

$$\int_\Omega(u(-\Delta G) - G(-\Delta u))\,d\mathbf{x} = \oint_{\partial\Omega}\left(u\frac{\partial G}{\partial\nu} - G\frac{\partial u}{\partial\nu}\right)dS.$$

Since $-\Delta G = \delta(\mathbf{x}-\mathbf{y})$ and $-\Delta u = f$, the left side is $u(\mathbf{y}) - \int_\Omega Gf\,d\mathbf{x}$. Since $G|_{\partial\Omega}=0$:

$$u(\mathbf{y}) = \int_\Omega G(\mathbf{x};\mathbf{y})f(\mathbf{x})\,d\mathbf{x} + \oint_{\partial\Omega}g(\mathbf{x})\frac{\partial G}{\partial\nu_\mathbf{x}}(\mathbf{x};\mathbf{y})\,dS(\mathbf{x}). \tag{1}$$

**Interchanging $\mathbf{x}$ and $\mathbf{y}$** (using $G(\mathbf{x};\mathbf{y}) = G(\mathbf{y};\mathbf{x})$):

$$u(\mathbf{x}) = \int_\Omega G(\mathbf{x};\mathbf{y})f(\mathbf{y})\,d\mathbf{y} - \oint_{\partial\Omega}g(\mathbf{y})\frac{\partial G}{\partial\nu_\mathbf{y}}(\mathbf{x};\mathbf{y})\,dS(\mathbf{y}). \tag{2}$$

(Note the sign: $\partial G/\partial\nu_\mathbf{y}$ on $\partial\Omega$ is the outward normal derivative of $G$ with respect to $\mathbf{y}$, which is negative for the Dirichlet Green's function.)

## The Poisson Kernel

The normal derivative of the Green's function on the boundary, $-\partial G/\partial\nu_\mathbf{y}(\mathbf{x};\mathbf{y})|_{\mathbf{y}\in\partial\Omega}$, is called the **Poisson kernel** $P(\mathbf{x};\mathbf{y})$. Formula (2) for Laplace's equation ($f = 0$) becomes:

$$u(\mathbf{x}) = \oint_{\partial\Omega}P(\mathbf{x};\mathbf{y})g(\mathbf{y})\,dS(\mathbf{y}).$$

This is the abstract form of the Poisson integral formula: the solution is the convolution of the boundary data with the Poisson kernel.

Properties of the Poisson kernel:
- $P(\mathbf{x};\mathbf{y}) > 0$ for $\mathbf{x} \in \Omega$, $\mathbf{y} \in \partial\Omega$.
- $\int_{\partial\Omega}P(\mathbf{x};\mathbf{y})\,dS(\mathbf{y}) = 1$ for each $\mathbf{x} \in \Omega$.
- $P(\mathbf{x};\mathbf{y}) \to \delta_{\mathbf{x}_0}(\mathbf{y})$ as $\mathbf{x}\to\mathbf{x}_0 \in \partial\Omega$.

## Symmetry of the Green's Function

**Theorem.** $G(\mathbf{x};\mathbf{y}) = G(\mathbf{y};\mathbf{x})$ for all $\mathbf{x}\neq\mathbf{y}$ in $\Omega$.

**Proof.** Apply Green's second identity with $u = G(\cdot;\mathbf{x})$ and $v = G(\cdot;\mathbf{y})$ in $\Omega\setminus(B_\varepsilon(\mathbf{x})\cup B_\varepsilon(\mathbf{y}))$:

$$\int(u\Delta v - v\Delta u)\,d\mathbf{z} = \oint_{\partial\Omega}(\ldots)\,dS + \oint_{\partial B_\varepsilon(\mathbf{x})}(\ldots)\,dS + \oint_{\partial B_\varepsilon(\mathbf{y})}(\ldots)\,dS.$$

The boundary integral over $\partial\Omega$ vanishes (both $G$'s are zero there). The integrals over the small spheres converge to $G(\mathbf{x};\mathbf{y})$ and $-G(\mathbf{y};\mathbf{x})$ as $\varepsilon\to 0$ (using the singularity of the fundamental solution). The left side is $G(\mathbf{y};\mathbf{x}) - G(\mathbf{x};\mathbf{y})$ (using $-\Delta G(\cdot;\mathbf{x}) = \delta(\cdot-\mathbf{x})$). Setting this to zero: $G(\mathbf{x};\mathbf{y}) = G(\mathbf{y};\mathbf{x})$.

**Physical meaning:** The potential at $\mathbf{x}$ due to a unit source at $\mathbf{y}$ (with grounded boundary) equals the potential at $\mathbf{y}$ due to a unit source at $\mathbf{x}$. This is the principle of reciprocity in electrostatics.

## Example: The Disk in 2D

For the disk $B_R$ in $\mathbb{R}^2$, the Green's function is constructed by the method of images (next section):

$$G(\mathbf{x};\mathbf{y}) = \frac{1}{2\pi}\log\frac{|\mathbf{x}-\mathbf{y}^*||R/|\mathbf{y}||}{|\mathbf{x}-\mathbf{y}|} = -\frac{1}{2\pi}\log|\mathbf{x}-\mathbf{y}| + \frac{1}{2\pi}\log\left|\mathbf{x}-\frac{R^2\mathbf{y}}{|\mathbf{y}|^2}\right| + \frac{1}{2\pi}\log\frac{|\mathbf{y}|}{R},$$

where $\mathbf{y}^* = R^2\mathbf{y}/|\mathbf{y}|^2$ is the inversion of $\mathbf{y}$ in the circle $|\mathbf{y}|=R$. The Poisson kernel derived from this is exactly the classical Poisson kernel derived in the previous chapter.
