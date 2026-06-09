# The Method of Images

The method of images is a technique for constructing Green's functions — and hence explicit solutions of Laplace's equation — for domains with simple geometric symmetry. The idea: to satisfy the zero Dirichlet condition on a flat boundary, place an "image" source of opposite sign at the mirror-reflected position outside the domain. The combined potential (true source plus image source) is zero on the boundary and satisfies Laplace's equation everywhere inside the domain.

## The Upper Half-Plane

Consider the upper half-plane $\Omega = \{(x,y): y > 0\}$ with Dirichlet boundary condition $u = 0$ on $\{y=0\}$.

The fundamental solution at $\mathbf{y} = (y_1, y_2)$ (with $y_2 > 0$) is $\Phi(\mathbf{x}-\mathbf{y}) = -\frac{1}{2\pi}\log|\mathbf{x}-\mathbf{y}|$. This is nonzero on $\{x_2=0\}$.

The **image source** is placed at $\mathbf{y}^* = (y_1, -y_2)$, the reflection of $\mathbf{y}$ across $\{x_2=0\}$. The image fundamental solution is $-\Phi(\mathbf{x}-\mathbf{y}^*) = \frac{1}{2\pi}\log|\mathbf{x}-\mathbf{y}^*|$.

The **Green's function**:

$$G(\mathbf{x};\mathbf{y}) = \Phi(\mathbf{x}-\mathbf{y}) - \Phi(\mathbf{x}-\mathbf{y}^*) = -\frac{1}{2\pi}\log\frac{|\mathbf{x}-\mathbf{y}|}{|\mathbf{x}-\mathbf{y}^*|}.$$

**Verification:** On $\{x_2=0\}$: $|\mathbf{x}-\mathbf{y}| = |\mathbf{x}-\mathbf{y}^*|$ (by symmetry, since $\mathbf{x}$ is on the line of symmetry between $\mathbf{y}$ and $\mathbf{y}^*$), so $G = 0$. In $\Omega$: $G$ is harmonic except at $\mathbf{x}=\mathbf{y}$ (where it has the correct singularity $\Phi(\mathbf{x}-\mathbf{y})$), since $\mathbf{y}^* \notin \overline\Omega$.

**Poisson kernel** (outward normal on $\{x_2=0\}$ points in $-x_2$ direction):

$$-\frac{\partial G}{\partial\nu_\mathbf{y}}\Big|_{y_2=0} = \frac{1}{2\pi}\frac{\partial}{\partial y_2}\log\frac{|\mathbf{x}-\mathbf{y}|}{|\mathbf{x}-\mathbf{y}^*|}\Big|_{y_2=0} = \frac{x_2}{\pi(|\mathbf{x}-\mathbf{y}|^2)}\Big|_{y_2=0} = \frac{x_2}{\pi((x_1-y_1)^2+x_2^2)}.$$

This gives the Poisson formula for the upper half-plane:

$$u(x_1,x_2) = \frac{x_2}{\pi}\int_{-\infty}^\infty\frac{g(t)}{(x_1-t)^2+x_2^2}\,dt.$$

## The Method in 3D

For the upper half-space $\{x_3 > 0\}$ in $\mathbb{R}^3$, the fundamental solution is $\Phi(\mathbf{x}-\mathbf{y}) = 1/(4\pi|\mathbf{x}-\mathbf{y}|)$. The image at $\mathbf{y}^* = (y_1,y_2,-y_3)$:

$$G(\mathbf{x};\mathbf{y}) = \frac{1}{4\pi|\mathbf{x}-\mathbf{y}|} - \frac{1}{4\pi|\mathbf{x}-\mathbf{y}^*|}.$$

The Poisson kernel for the half-space:

$$P(\mathbf{x};\mathbf{y})|_{y_3=0} = \frac{2x_3}{4\pi(|\mathbf{x}'-\mathbf{y}'|^2+x_3^2)^{3/2}} = \frac{x_3}{2\pi(|\mathbf{x}-\mathbf{y}|^3)}\Big|_{y_3=0},$$

where $\mathbf{x}' = (x_1,x_2)$ and $\mathbf{y}' = (y_1,y_2)$ are the horizontal components.

## Multiple Images

For a strip $0 < x < a$ or a corner (intersection of two half-planes), the method of images requires multiple image sources — in fact, infinitely many for some geometries. For the strip $\{0 < x < a\}$, reflecting in both walls generates an infinite sequence of images at positions $x = \pm 2na \pm y_1$ for $n = 0, \pm 1, \pm 2, \ldots$ This infinite sum converges and gives the Green's function for the strip. For a quarter-plane (two reflections, finite images), only 3 image sources are needed.

## Method of Images for the Disk: Kelvin Transform

For the disk $B_R$ in $\mathbb{R}^2$ (or ball in $\mathbb{R}^n$), the image of $\mathbf{y}$ is its **Kelvin inverse** $\mathbf{y}^* = R^2\mathbf{y}/|\mathbf{y}|^2$ (inversion in the circle/sphere). The Green's function:

$$G(\mathbf{x};\mathbf{y}) = \Phi(\mathbf{x}-\mathbf{y}) - \frac{|\mathbf{y}|^{n-2}}{R^{n-2}}\Phi\!\left(\mathbf{x}-\mathbf{y}^*\right) \qquad (n \geq 3),$$

$$G(\mathbf{x};\mathbf{y}) = -\frac{1}{2\pi}\log|\mathbf{x}-\mathbf{y}| + \frac{1}{2\pi}\log\!\left(\frac{|\mathbf{y}|}{R}|\mathbf{x}-\mathbf{y}^*|\right) \qquad (n=2).$$

The correction factor $|\mathbf{y}|^{n-2}/R^{n-2}$ (or $|\mathbf{y}|/R$ in 2D) ensures that $G = 0$ on $\partial B_R$ — not obvious by symmetry alone, but verified by direct computation using the identity $|\mathbf{x}-\mathbf{y}^*|/|\mathbf{x}-\mathbf{y}| = R/|\mathbf{y}|$ for $|\mathbf{x}|=R$.

Taking the normal derivative of $G$ on $\partial B_R$ recovers the Poisson kernel for the disk, confirming consistency with the earlier derivation.
