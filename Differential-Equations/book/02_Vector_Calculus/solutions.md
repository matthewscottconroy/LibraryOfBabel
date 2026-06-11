# Solutions: Vector Calculus

## Problem 1: Conservative Fields and Potential Functions

**Problem.** Determine if $\mathbf{F}(x,y) = (2xy + y^3, x^2 + 3xy^2)$ is conservative. If so, find a potential function.

**Solution.** Check: $\partial F_1/\partial y = 2x + 3y^2$ and $\partial F_2/\partial x = 2x + 3y^2$. Equal, so $\mathbf{F}$ is conservative on $\mathbb{R}^2$ (simply connected).

Find $\varphi$: $\partial\varphi/\partial x = 2xy + y^3$, so $\varphi = x^2y + xy^3 + g(y)$.

$\partial\varphi/\partial y = x^2 + 3xy^2 + g'(y) = x^2 + 3xy^2$, so $g'(y) = 0$, $g = C$.

Potential: $\varphi(x,y) = x^2y + xy^3$.

**Consequence.** For any curve from $(0,0)$ to $(2,1)$:
$\int_C \mathbf{F}\cdot d\mathbf{r} = \varphi(2,1) - \varphi(0,0) = (4)(1) + (2)(1) = 6$.

---

## Problem 2: Green's Theorem

**Problem.** Use Green's Theorem to compute $\oint_C (y^2 - x)\,dx + (x^2 + y)\,dy$ where $C$ is the boundary of the square $[0,1]^2$ traversed counterclockwise.

**Solution.** $P = y^2 - x$, $Q = x^2 + y$.

$\partial Q/\partial x - \partial P/\partial y = 2x - 2y$.

$\oint_C \mathbf{F}\cdot d\mathbf{r} = \iint_{[0,1]^2}(2x-2y)\,dA = 2\int_0^1\int_0^1(x-y)\,dy\,dx = 2\int_0^1\left[xy - y^2/2\right]_0^1\,dx = 2\int_0^1(x-1/2)\,dx = 2[x^2/2 - x/2]_0^1 = 2(1/2 - 1/2) = 0$.

**Geometric interpretation.** The integrand $2x - 2y$ has equal positive and negative contributions that cancel exactly over the unit square.

---

## Problem 3: Surface Integral and Divergence Theorem

**Problem.** Let $\mathbf{F} = (x^2, y^2, z^2)$. Find the flux of $\mathbf{F}$ outward through the closed surface bounding the cube $[0,1]^3$.

**Solution.** By the Divergence Theorem: $\oiint_S \mathbf{F}\cdot d\mathbf{S} = \iiint_V \nabla\cdot\mathbf{F}\,dV$.

$\nabla\cdot\mathbf{F} = 2x + 2y + 2z$.

$\iiint_{[0,1]^3}(2x+2y+2z)\,dV = 2\int_0^1\int_0^1\int_0^1(x+y+z)\,dx\,dy\,dz$.

By symmetry: $\iiint(x+y+z) = 3\iiint x = 3\cdot(1/2) = 3/2$.

Flux $= 2(3/2) = 3$.

**Direct check (top face only).** On the top face $z=1$, outward normal $\mathbf{n} = (0,0,1)$: flux $= \int_0^1\int_0^1 z^2|_{z=1}\,dx\,dy = 1$. By symmetry, bottom face ($z=0$, inward normal): flux $= 0$. Similarly for the four side faces: flux from right ($x=1$) is $1$, left ($x=0$) is $0$, front ($y=1$) is $1$, back ($y=0$) is $0$. Total: $3$. Agrees.

---

## Problem 4: Stokes' Theorem

**Problem.** Verify Stokes' theorem for $\mathbf{F} = (y, -x, z)$ and the hemisphere $S: x^2+y^2+z^2=1$, $z \geq 0$, with the upward normal, and $\partial S$ is the unit circle in the $xy$-plane.

**Solution.** **Left side (line integral):** $C: x = \cos t$, $y = \sin t$, $z=0$, $t \in [0,2\pi]$.

$\oint_C \mathbf{F}\cdot d\mathbf{r} = \int_0^{2\pi}[\sin t(-\sin t) + (-\cos t)(\cos t) + 0]\,dt = \int_0^{2\pi}(-\sin^2 t - \cos^2 t)\,dt = \int_0^{2\pi}(-1)\,dt = -2\pi$.

**Right side (surface integral):** $\nabla\times\mathbf{F} = \begin{vmatrix}\mathbf{i}&\mathbf{j}&\mathbf{k}\\\partial_x&\partial_y&\partial_z\\y&-x&z\end{vmatrix} = (0-0, 0-0, -1-1) = (0,0,-2)$.

$\iint_S(0,0,-2)\cdot\mathbf{n}\,dS$. The upward unit normal on the hemisphere is $\mathbf{n} = (x,y,z)$.

$\iint_S(0,0,-2)\cdot(x,y,z)\,dS = \iint_S -2z\,dS$.

In spherical coordinates: $x = \sin\phi\cos\theta$, $y=\sin\phi\sin\theta$, $z = \cos\phi$, $dS = \sin\phi\,d\phi\,d\theta$, $\phi\in[0,\pi/2]$, $\theta\in[0,2\pi]$:

$\int_0^{2\pi}\int_0^{\pi/2}(-2\cos\phi)\sin\phi\,d\phi\,d\theta = 2\pi\cdot(-2)\int_0^{\pi/2}\cos\phi\sin\phi\,d\phi = -4\pi\cdot[\sin^2\phi/2]_0^{\pi/2} = -4\pi\cdot(1/2) = -2\pi$.

Both sides equal $-2\pi$. Stokes' theorem verified.

---

## Problem 5: Deriving the Wave Equation

**Problem.** Derive the 1D wave equation $\rho u_{tt} = T u_{xx}$ for a vibrating string of linear mass density $\rho$ under tension $T$.

**Solution.** Let $u(x,t)$ be the transverse displacement. Consider a small segment $[x, x+\Delta x]$. The mass of this segment is $\rho\Delta x$.

Vertical forces: the tension $T$ acts tangentially at both ends. Under the small-angle approximation ($|u_x| \ll 1$), the vertical component of tension at $x+\Delta x$ is approximately $Tu_x(x+\Delta x, t)$, and at $x$ is $-Tu_x(x,t)$ (acting downward).

Net vertical force: $T u_x(x+\Delta x,t) - Tu_x(x,t) \approx T u_{xx}(x,t)\Delta x$.

Newton's second law: $\rho\Delta x \cdot u_{tt} = T u_{xx}\Delta x$.

Dividing by $\Delta x$: $\rho u_{tt} = T u_{xx}$, i.e., $u_{tt} = c^2 u_{xx}$ where $c = \sqrt{T/\rho}$ is the wave speed.

**Connection to vector calculus.** This is a 1D version of the derivation that uses the Divergence Theorem in 2D or 3D. The key step — converting a "balance over a region" to a local PDE — follows the same pattern as deriving the heat equation.

---

## Problem 6: Green's Identities and a Uniqueness Proof

**Problem.** Use Green's first identity to prove that the Dirichlet problem $\Delta u = 0$ on $\Omega$, $u = f$ on $\partial\Omega$, has at most one solution.

**Solution.** Green's first identity: $\int_\Omega v\Delta u\,dV = \oint_{\partial\Omega} v\frac{\partial u}{\partial n}\,dS - \int_\Omega \nabla u\cdot\nabla v\,dV$.

Suppose $u_1$ and $u_2$ are two solutions; let $w = u_1 - u_2$. Then $\Delta w = 0$ on $\Omega$ and $w = 0$ on $\partial\Omega$.

Apply Green's first identity with $v = w$, $u = w$:
$\int_\Omega w\Delta w\,dV = \oint_{\partial\Omega}w\frac{\partial w}{\partial n}\,dS - \int_\Omega|\nabla w|^2\,dV$.

Left side: $\int_\Omega w \cdot 0 = 0$. Boundary term: $\oint w(\ldots)dS = 0$ since $w = 0$ on $\partial\Omega$.

So $0 = 0 - \int_\Omega|\nabla w|^2\,dV$, giving $\int_\Omega|\nabla w|^2 = 0$. Since $|\nabla w|^2 \geq 0$, we get $\nabla w = 0$ on $\Omega$, hence $w$ is constant on $\Omega$. Since $w = 0$ on $\partial\Omega$, $w \equiv 0$.
