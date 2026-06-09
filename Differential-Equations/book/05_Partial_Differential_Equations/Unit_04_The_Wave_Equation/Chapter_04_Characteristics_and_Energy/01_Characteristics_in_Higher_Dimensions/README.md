# Characteristics of the Wave Equation in Higher Dimensions

The characteristic surfaces of the wave equation in $\mathbb{R}^n$ are the geometric objects that encode the causal structure of the equation — the cones along which information propagates at speed $c$. Understanding characteristics in higher dimensions is essential for the theory of hyperbolic PDEs, for the design of stable numerical schemes, and for the physical interpretation of wave phenomena.

## Characteristic Surfaces for Second-Order PDEs

For a second-order linear PDE in $n+1$ variables $(x_1,\ldots,x_n,t)$ with principal symbol $\sum_{ij}a_{ij}\xi_i\xi_j$, a surface $\Sigma = \{S(x_1,\ldots,x_n,t) = 0\}$ is **characteristic** if the Cauchy problem with data on $\Sigma$ is not well-determined — i.e., if the normal to $\Sigma$ is in the null space of the principal symbol:

$$\sum_{i,j}a_{ij}\frac{\partial S}{\partial x_i}\frac{\partial S}{\partial x_j} = 0.$$

For the wave equation $u_{tt} - c^2\Delta u = 0$, the principal symbol in Fourier space $(\xi, \tau)$ is $-\tau^2 + c^2|\xi|^2$. So a surface $\Sigma$ with normal $(S_{x_1},\ldots,S_{x_n},S_t)$ is characteristic if:

$$-S_t^2 + c^2|\nabla S|^2 = 0, \qquad \text{i.e.,} \quad S_t^2 = c^2|\nabla S|^2. \tag{1}$$

This is the **eikonal equation**.

## The Light Cone as a Characteristic Surface

The forward cone through the origin: $S(x,t) = |x| - ct = 0$ (using $\mathbf{x} \in \mathbb{R}^n$). Then $S_t = -c$ and $|\nabla S| = 1$. Check: $S_t^2 = c^2 = c^2|\nabla S|^2$. The cone satisfies the eikonal equation — it is a characteristic surface.

More generally, any surface swept out by characteristics (rays) emanating from a source at speed $c$ is characteristic. These are the **wavefronts** of the wave equation.

## The Cauchy-Kovalevskaya Failure on Characteristics

On a characteristic surface, the Cauchy problem is degenerate: the PDE does not determine the second normal derivative of $u$ from data on $\Sigma$. Specifically, if $\Sigma$ is characteristic with unit normal $\mathbf{n}$, then $\sum_{ij}a_{ij}n_in_j = 0$, so the equation cannot be solved for $\partial^2 u/\partial n^2$ from data on $\Sigma$.

This means: on a characteristic surface, prescribing $u$ and $\partial u/\partial n$ (Cauchy data) is either overdetermined (data must satisfy a compatibility condition) or underdetermined (infinitely many solutions). This is why the correct initial surface for the wave equation is $t = 0$, which is non-characteristic ($S = t$, $S_t = 1 \neq 0 = c|\nabla S|$).

## Bicharacteristics and Rays

The characteristic surfaces foliate into curves called **bicharacteristics** (or **rays**). In the geometric optics approximation, rays are the paths along which energy propagates. For the wave equation, bicharacteristics satisfy the Hamiltonian system:

$$\dot{x}_i = \frac{\partial H}{\partial \xi_i} = 2c^2\xi_i, \qquad \dot{\xi}_i = -\frac{\partial H}{\partial x_i} = 0, \qquad \dot{t} = \frac{\partial H}{\partial\tau} = -2\tau, \qquad \dot{\tau} = 0,$$

where $H = -\tau^2 + c^2|\xi|^2$ is the principal symbol. Since $\dot{\xi} = 0$, the ray direction $\xi$ is constant; the ray is a straight line in $(\mathbf{x},t)$-space.

For a point source at $(\mathbf{0},0)$, the rays are straight lines $\mathbf{x} = \mathbf{v} t$ with $|\mathbf{v}| = c$ (all directions, speed $c$). The set of rays fills out the forward light cone $|\mathbf{x}| = ct$.

## The Propagation of Singularities

The most profound result about characteristics is the **propagation of singularities theorem**: if $u$ is a solution of the wave equation and has a singularity (e.g., a jump discontinuity) at a point $(\mathbf{x}_0, t_0)$, then this singularity propagates along the bicharacteristic through $(\mathbf{x}_0,t_0)$.

More precisely, the **wavefront set** of $u$ — the set of $(x,\xi)$ pairs where $u$ is not microlocally smooth — is invariant under the bicharacteristic flow. For the wave equation, this means: singularities travel along rays at speed $c$, in straight lines (for constant-coefficient equations), and cannot be created or destroyed in the interior of the domain.

This is the mathematical basis for geometric optics and the theory of diffractive phenomena (edges, creeping rays, etc.) in wave propagation.

## Domain of Dependence in $n$ Dimensions

The domain of dependence of $(\mathbf{x}_0,t_0)$ is the set of initial points $(\mathbf{y},0)$ such that $u(x_0,t_0)$ can be affected by data at $\mathbf{y}$. For the wave equation in $\mathbb{R}^n$:

$$\mathcal{D}(\mathbf{x}_0,t_0) = \{\mathbf{y}: |\mathbf{y}-\mathbf{x}_0| \leq ct_0\}.$$

This is the backward light cone restricted to $t=0$ — the ball of radius $ct_0$ centered at $\mathbf{x}_0$. In all dimensions, the domain of dependence is a ball (reflecting finite propagation speed). The question of whether the solution at $(\mathbf{x}_0,t_0)$ depends on data in the interior of this ball or only on its boundary is exactly the Huygens principle question, addressed in the next section.
