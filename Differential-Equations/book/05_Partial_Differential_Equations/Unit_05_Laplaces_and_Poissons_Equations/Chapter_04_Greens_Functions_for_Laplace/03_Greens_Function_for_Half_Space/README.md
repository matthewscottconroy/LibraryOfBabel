# Green's Function for the Half-Space

The upper half-space $\Omega = \mathbb{R}^n_+ = \{(\mathbf{x}', x_n): \mathbf{x}'\in\mathbb{R}^{n-1}, x_n > 0\}$ is the prototypical unbounded domain for Laplace's equation. Its Green's function, constructed by the method of images, yields the Poisson formula for the half-space and describes the electrostatic potential above a grounded conducting plane, the gravitational potential above a flat earth, and the velocity potential of ideal flow above a flat surface.

## Green's Function

By the method of images (the image of $\mathbf{y} = (\mathbf{y}',y_n)$ is $\mathbf{y}^* = (\mathbf{y}',-y_n)$):

$$G(\mathbf{x};\mathbf{y}) = \Phi(\mathbf{x}-\mathbf{y}) - \Phi(\mathbf{x}-\mathbf{y}^*).$$

In $\mathbb{R}^3$ ($n=3$):

$$G(\mathbf{x};\mathbf{y}) = \frac{1}{4\pi}\left[\frac{1}{|\mathbf{x}-\mathbf{y}|} - \frac{1}{|\mathbf{x}-\mathbf{y}^*|}\right].$$

In $\mathbb{R}^2$ ($n=2$):

$$G(\mathbf{x};\mathbf{y}) = \frac{1}{2\pi}\left[\log|\mathbf{x}-\mathbf{y}^*| - \log|\mathbf{x}-\mathbf{y}|\right] = \frac{1}{2\pi}\log\frac{|\mathbf{x}-\mathbf{y}^*|}{|\mathbf{x}-\mathbf{y}|}.$$

**Verification:** On $\partial\Omega = \{x_n=0\}$: $|\mathbf{x}-\mathbf{y}| = |\mathbf{x}-\mathbf{y}^*|$ (since $\mathbf{x}^* = \mathbf{x}$ for $x_n=0$), so $G=0$. In $\Omega$: harmonic except at $\mathbf{x}=\mathbf{y}$, with the correct singularity.

## Poisson Formula for the Half-Space

The Poisson kernel is:

$$-\frac{\partial G}{\partial\nu_\mathbf{y}}(\mathbf{x};\mathbf{y})\Big|_{y_n=0} = \frac{\partial G}{\partial y_n}(\mathbf{x};\mathbf{y})\Big|_{y_n=0} = \frac{2x_n}{n\omega_n|\mathbf{x}-\mathbf{y}'|^n}\Big|_{y_n=0},$$

where $\mathbf{y}' = (\mathbf{y}',0)$ and $|\mathbf{x}-\mathbf{y}'|^2 = |\mathbf{x}'-\mathbf{y}'|^2 + x_n^2$.

**Poisson formula for the half-space:**

$$u(\mathbf{x}) = \frac{2x_n}{n\omega_n}\int_{\mathbb{R}^{n-1}}\frac{g(\mathbf{y}')}{(|\mathbf{x}'-\mathbf{y}'|^2+x_n^2)^{n/2}}\,d\mathbf{y}'. \tag{1}$$

**Verification in 3D:** The Poisson kernel in $\mathbb{R}^3$ is $P(\mathbf{x};\mathbf{y}') = x_3/[2\pi(|\mathbf{x}'-\mathbf{y}'|^2+x_3^2)^{3/2}]$.

This is a Cauchy-type kernel: it decays as $x_3|\mathbf{x}'-\mathbf{y}'|^{-3}$ for large $|\mathbf{x}'-\mathbf{y}'|$ and concentrates at $\mathbf{y}' = \mathbf{x}'$ as $x_3\to 0^+$, giving $P\to\delta(\mathbf{x}'-\mathbf{y}')$.

## Physical Interpretation

**Electrostatics:** A grounded conducting plane at $\{x_3=0\}$ (held at potential $g=0$) with a point charge $q$ at $\mathbf{y}\in\mathbb{R}^3_+$ induces a surface charge distribution on the plane. The potential above the plane is $G(\mathbf{x};\mathbf{y})\cdot q$, and the image charge $-q$ at $\mathbf{y}^*$ represents the effect of the induced surface charge.

For Poisson's equation $-\Delta u = \rho/\varepsilon_0$ in the half-space with $u=g$ on the boundary, the representation formula gives:

$$u(\mathbf{x}) = \frac{1}{\varepsilon_0}\int_\Omega G(\mathbf{x};\mathbf{y})\rho(\mathbf{y})\,d\mathbf{y} + \int_{\partial\Omega}g(\mathbf{y})P(\mathbf{x};\mathbf{y})\,dS(\mathbf{y}).$$

## Connection to the Heat Kernel

The Poisson kernel for the half-space is related to the heat kernel by a Laplace transform. Specifically:

$$P(\mathbf{x};\mathbf{y}') = \frac{2}{\sqrt{\pi}}\int_0^\infty e^{-t^2}K_t(\mathbf{x}'-\mathbf{y}')\,dt,$$

where $K_t$ is the heat kernel. This connection (Poisson semigroup = subordination of heat semigroup) is the spectral-theoretic statement that $(-\Delta)^{1/2}$ (the generator of the Poisson semigroup) is the square root of $-\Delta$ (the generator of the heat semigroup). It plays a fundamental role in harmonic analysis and the theory of singular integrals.

## The Schwarz Reflection Principle

The method of images is the PDE expression of the Schwarz reflection principle in complex analysis: a harmonic function in the upper half-plane that vanishes on the real axis extends to an odd harmonic function in all of $\mathbb{C}$ by reflection: $u(x,-y) = -u(x,y)$. The Green's function construction $G = \Phi(\mathbf{x}-\mathbf{y}) - \Phi(\mathbf{x}-\mathbf{y}^*)$ is precisely this odd extension.
