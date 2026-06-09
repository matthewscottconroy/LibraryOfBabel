# The Heat Equation in Spherical Coordinates

Spherical geometry arises naturally when studying heat conduction in a ball, the temperature of a planet or star, or the diffusion of a substance from a spherical source. The heat equation in spherical coordinates can be solved by a remarkable algebraic trick for radially symmetric problems, and by separation into spherical harmonics for the general case.

## The Radially Symmetric Case

For temperature $u = u(r,t)$ depending only on the radial coordinate $r$ and time $t$, the Laplacian reduces to

$$\Delta u = u_{rr} + \frac{2}{r}u_r = \frac{1}{r^2}\frac{\partial}{\partial r}\!\left(r^2\frac{\partial u}{\partial r}\right),$$

and the heat equation becomes:

$$u_t = \kappa\!\left(u_{rr} + \frac{2}{r}u_r\right), \qquad 0 < r < a,\; t > 0.$$

**The key substitution:** set $v(r,t) = r\,u(r,t)$. Then:

$$v_t = r\,u_t = r\kappa\!\left(u_{rr} + \frac{2}{r}u_r\right) = \kappa(ru_{rr} + 2u_r).$$

And $v_{rr} = (ru)_{rr} = r u_{rr} + 2u_r$. Therefore $v_t = \kappa v_{rr}$ — the one-dimensional heat equation for $v$.

This is a powerful dimensional reduction: the radially symmetric 3D heat equation reduces exactly to the 1D heat equation for $v = ru$. All the theory developed for the 1D problem applies directly.

## Boundary and Initial Conditions for $v$

If $u$ is regular at $r=0$ (temperature is finite and smooth at the center), then $v = ru \to 0$ as $r\to 0$, giving the Dirichlet condition $v(0,t) = 0$.

For a ball of radius $a$ with Dirichlet condition $u(a,t) = 0$: $v(a,t) = au(a,t) = 0$.

Initial condition: $v(r,0) = rf(r)$ where $u(r,0) = f(r)$.

## Solution for the Ball

The problem for $v$ is:

$$v_t = \kappa v_{rr}, \quad 0 < r < a, \qquad v(0,t) = v(a,t) = 0, \qquad v(r,0) = rf(r).$$

This is the standard 1D heat equation on $[0,a]$ with Dirichlet conditions, solved by:

$$v(r,t) = \sum_{n=1}^\infty b_n\sin\!\left(\frac{n\pi r}{a}\right)e^{-\kappa(n\pi/a)^2 t},$$

where $b_n = \frac{2}{a}\int_0^a rf(r)\sin(n\pi r/a)\,dr$.

The temperature is $u(r,t) = v(r,t)/r$:

$$u(r,t) = \sum_{n=1}^\infty \frac{b_n}{r}\sin\!\left(\frac{n\pi r}{a}\right)e^{-\kappa(n\pi/a)^2 t} = \sum_{n=1}^\infty b_n\,\frac{\sin(n\pi r/a)}{r}\,e^{-\kappa(n\pi/a)^2 t}.$$

Note that $\sin(n\pi r/a)/r$ is well-defined and smooth at $r=0$ (its limit as $r\to 0$ is $n\pi/a$).

## Example: Uniform Initial Temperature

If $f(r) = T_0$ (constant), then $b_n = \frac{2}{a}\int_0^a r T_0\sin(n\pi r/a)\,dr = \frac{2T_0}{a}\cdot\frac{(-1)^{n+1}a^2}{n\pi} = \frac{2T_0 a(-1)^{n+1}}{n\pi}$.

The solution:

$$u(r,t) = \frac{2T_0 a}{\pi r}\sum_{n=1}^\infty \frac{(-1)^{n+1}}{n}\sin\!\left(\frac{n\pi r}{a}\right)e^{-\kappa(n\pi/a)^2 t}.$$

At $t=0$: the series $\frac{2a}{\pi r}\sum_n \frac{(-1)^{n+1}}{n}\sin(n\pi r/a) = T_0$ (Fourier series of $T_0$ on $[0,a]$, after dividing by $r$). The temperature is initially uniform, then decays to zero as heat escapes through the boundary.

## The Full Angular Case

For temperature $u(r,\theta,\phi,t)$ depending on all spatial variables in spherical coordinates, separation of variables leads to the expansion:

$$u(r,\theta,\phi,t) = \sum_{\ell=0}^\infty\sum_{m=-\ell}^\ell R_\ell(r,t)\,Y_\ell^m(\theta,\phi),$$

where $Y_\ell^m$ are the **spherical harmonics** — the eigenfunctions of the angular part of the Laplacian. The radial equation for each $(\ell,m)$-mode involves spherical Bessel functions. This is developed in Unit 5 (Laplace's equation in spherical coordinates) and Unit 6 (spherical harmonics).

## The Long-Time Behavior

For the ball with Dirichlet conditions and any continuous initial data, the long-time behavior is dominated by the $n=1$ mode (smallest eigenvalue $\lambda_1 = \pi^2/a^2$):

$$u(r,t) \approx \frac{b_1}{r}\sin\!\left(\frac{\pi r}{a}\right)e^{-\kappa\pi^2 t/a^2} \quad \text{as } t \to \infty.$$

The temperature profile approaches $\sin(\pi r/a)/r$ — a spherically symmetric distribution that is maximum at the center and zero at the boundary, decaying uniformly with characteristic time $a^2/(\kappa\pi^2)$.

## Heat Flow in a Spherical Shell

For a spherical shell $a < r < b$ with Dirichlet conditions $u(a,t) = T_1$ and $u(b,t) = T_2$, the steady state is $u_s(r) = (T_2 b(r-a) - T_1 a(r-b)) / (r(b-a))$ (found by solving $(r^2 u_s')' = 0$, which gives $u_s = A/r + B$). Setting $w = u - u_s$ reduces to the homogeneous problem, and the solution is expressed in terms of $v = rw$ and solved exactly as before with the 1D heat equation on $[a,b]$ with Dirichlet conditions.
