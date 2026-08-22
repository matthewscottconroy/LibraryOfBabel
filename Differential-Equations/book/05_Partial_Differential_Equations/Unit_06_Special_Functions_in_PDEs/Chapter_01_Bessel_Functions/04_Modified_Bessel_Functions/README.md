# Modified Bessel Functions

When the Laplacian is separated in cylindrical coordinates and the separation constant appears with a sign opposite to the usual case — for instance, when $\lambda < 0$ rather than $\lambda > 0$, or when treating the radial equation in problems with real (rather than oscillatory) exponential behavior in the complementary direction — the radial ODE becomes the **modified Bessel equation**:

$$x^2 y'' + xy' - (x^2 + \nu^2)y = 0. \tag{Modified Bessel}$$

The solutions, the **modified Bessel functions** $I_\nu(x)$ and $K_\nu(x)$, are the real-variable counterparts of the oscillatory $J_\nu$ and $Y_\nu$: where $J_\nu$ oscillates and decays, $I_\nu$ grows monotonically; where $Y_\nu$ is singular at zero and oscillates, $K_\nu$ is singular at zero and decays to zero at infinity. This different qualitative behavior makes $I_\nu$ and $K_\nu$ the natural functions for problems with exponential (rather than wave-like) character.

## The Modified Bessel Equation

The modified Bessel equation arises in several ways:

1. **Sign flip.** Replace $x \to ix$ in Bessel's equation $x^2y'' + xy' + (x^2-\nu^2)y = 0$ to get the modified Bessel equation (up to factors of $i$).

2. **Separation with imaginary eigenvalue.** In the heat or wave equation, if the $z$-direction separation gives $Z'' = \mu^2 Z$ (positive sign, so $Z = e^{\pm\mu z}$), the radial equation for $R(r)$ has the sign $-\mu^2 r^2$ instead of $+\mu^2 r^2$, giving the modified Bessel equation.

3. **Fourier-Laplace transform.** Solving Laplace's equation $\Delta u = 0$ in 3D cylindrical coordinates with $z$-Fourier transform $U(r,k) = \int_{-\infty}^\infty u(r,z)e^{-ikz}\,dz$: the transformed equation $U_{rr} + U_r/r - (k^2 + m^2/r^2)U = 0$ is the modified Bessel equation with $\nu = m$ and $x = kr$.

**Physical examples:**
- Steady heat distribution in an infinite cylinder with heat source oscillating along $z$: the $z$-dependence is $e^{ikz}$, forcing the $r$-equation to be modified Bessel.
- Potential inside a cylinder driven by a nonuniform end condition: if $u(r,z=0) = f(r)$ and $u(r,z=L) = g(r)$, the $z$-eigenfunctions are $\sinh$ and $\cosh$ (growing/decaying), and the $r$-equation is modified Bessel.
- Capacitance between coaxial cylinders of different lengths.

## Definition and Series Representation

**Modified Bessel function of the first kind:**

$$I_\nu(x) = i^{-\nu}J_\nu(ix) = \sum_{k=0}^\infty \frac{1}{k!\,\Gamma(k+\nu+1)}\left(\frac{x}{2}\right)^{2k+\nu}.$$

Note the absence of the $(-1)^k$ factor compared to $J_\nu$: all terms are positive, explaining why $I_\nu$ is monotonically increasing. The series converges for all $x$.

**Modified Bessel function of the second kind:**

$$K_\nu(x) = \frac{\pi}{2}\frac{I_{-\nu}(x) - I_\nu(x)}{\sin(\nu\pi)} \quad (\nu \notin \mathbb{Z}),$$

with the integer-order case defined by continuity. Explicitly for $\nu = 0$:

$$K_0(x) = -\left[\ln\frac{x}{2} + \gamma\right]I_0(x) + \sum_{k=1}^\infty \frac{H_k}{(k!)^2}\left(\frac{x}{2}\right)^{2k},$$

where $\gamma \approx 0.5772$ is the Euler-Mascheroni constant and $H_k = \sum_{j=1}^k 1/j$.

**General solution.** The general solution of the modified Bessel equation on $(0,\infty)$ is $y = c_1 I_\nu(x) + c_2 K_\nu(x)$.

## Qualitative Behavior

The contrast between $I_\nu$, $K_\nu$ and $J_\nu$, $Y_\nu$ is stark:

**For small $x > 0$:**
- $I_\nu(x) \sim (x/2)^\nu/\Gamma(\nu+1)$ (same as $J_\nu$, since leading term is the same with $(-1)^0 = 1$).
- $K_0(x) \sim -\ln x$; $K_\nu(x) \sim \frac{\Gamma(\nu)}{2}(x/2)^{-\nu}$ for $\nu > 0$. (Singular at origin.)

**For large $x$:**
- $I_\nu(x) \sim e^x/\sqrt{2\pi x}$ (exponential growth).
- $K_\nu(x) \sim \sqrt{\pi/(2x)}\,e^{-x}$ (exponential decay).

More precisely, the asymptotic expansions are:

$$I_\nu(x) \sim \frac{e^x}{\sqrt{2\pi x}}\left(1 - \frac{4\nu^2-1}{8x} + \cdots\right), \qquad K_\nu(x) \sim \sqrt{\frac{\pi}{2x}}\,e^{-x}\left(1 + \frac{4\nu^2-1}{8x} + \cdots\right).$$

**Physical interpretation.** In problems on a cylinder $0 < r < R$, regularity at $r=0$ forces $c_2 = 0$ (no $K_\nu$); in exterior problems $r > R$ with decay at infinity, the condition $u \to 0$ forces $c_1 = 0$ (no $I_\nu$). In problems on an annulus $R_1 < r < R_2$, both $I_\nu$ and $K_\nu$ are present.

## Wronskian and Independence

$$W[I_\nu, K_\nu](x) = I_\nu(x)K_\nu'(x) - I_\nu'(x)K_\nu(x) = -\frac{1}{x}.$$

The nonvanishing Wronskian confirms linear independence for all $x > 0$.

## Recursion Relations

Analogous to the Bessel function recursions:

$$I_{\nu-1}(x) - I_{\nu+1}(x) = \frac{2\nu}{x}I_\nu(x), \qquad I_{\nu-1}(x) + I_{\nu+1}(x) = 2I_\nu'(x),$$

$$K_{\nu-1}(x) - K_{\nu+1}(x) = -\frac{2\nu}{x}K_\nu(x), \qquad K_{\nu-1}(x) + K_{\nu+1}(x) = -2K_\nu'(x).$$

In integral form:

$$\frac{d}{dx}[x^\nu I_\nu(x)] = x^\nu I_{\nu-1}(x), \qquad \frac{d}{dx}[x^{-\nu}I_\nu(x)] = x^{-\nu}I_{\nu+1}(x).$$

## Worked Example: Steady Heat in a Cylinder

**Problem.** A solid cylinder $\{r < R, 0 < z < L\}$ has insulated top and bottom ($u_z = 0$ at $z=0$ and $z=L$) and lateral surface temperature $u(R,z) = f(z)$. Find the steady-state temperature distribution.

**Setup.** We seek $\Delta u = 0$ with Neumann BCs in $z$ and Dirichlet BC on $r = R$. Separate $u = R(r)Z(z)$:

$$\frac{Z''}{Z} = -\frac{(rR')'}{rR} = \mu^2.$$

**$z$-equation:** $Z'' = \mu^2 Z$ with $Z'(0) = Z'(L) = 0$ gives $Z_n = \cos(n\pi z/L)$ with $\mu_n = n\pi/L$ for $n = 0,1,2,\ldots$. (The $n=0$ case $Z = \text{const}$ gives the azimuthally uniform mode.)

**$r$-equation:** $(rR')' - \mu_n^2 rR = 0$, or equivalently $r^2R'' + rR' - \mu_n^2 r^2 R = 0$, which is the modified Bessel equation with $\nu = 0$ and $x = \mu_n r$. Regularity at $r=0$ forces $R_n(r) = I_0(\mu_n r)$ (for $n \geq 1$); for $n=0$: $R_0(r) =$ const.

**Solution:**

$$u(r,z) = A_0 + \sum_{n=1}^\infty A_n \frac{I_0(\mu_n r)}{I_0(\mu_n R)}\cos\!\left(\frac{n\pi z}{L}\right),$$

where the factor $I_0(\mu_n R)$ normalizes the radial part to have value $1$ at $r=R$.

**Coefficients.** Matching $u(R,z) = f(z)$ and using Fourier cosine orthogonality:

$$A_0 = \frac{1}{L}\int_0^L f(z)\,dz, \qquad A_n = \frac{2}{L}\int_0^L f(z)\cos\!\left(\frac{n\pi z}{L}\right)dz.$$

The factors $I_0(\mu_n r)/I_0(\mu_n R) < 1$ for $r < R$ confirm that the temperature is bounded by the boundary values — consistent with the maximum principle.

## Half-Integer Orders: Elementary Functions

For $\nu = 1/2$:

$$I_{1/2}(x) = \sqrt{\frac{2}{\pi x}}\sinh x, \qquad K_{1/2}(x) = \sqrt{\frac{\pi}{2x}}\,e^{-x}.$$

More generally, for half-integer $\nu = n + 1/2$ ($n \geq 0$), $I_{n+1/2}(x)$ and $K_{n+1/2}(x)$ can be expressed in terms of $\sinh$, $\cosh$, and polynomials in $1/x$. This is the "spherical Bessel function" case: the radial equations for the wave equation in spherical coordinates lead to $I_{n+1/2}$ and $K_{n+1/2}$, or equivalently to **spherical modified Bessel functions** $i_n(x) = \sqrt{\pi/(2x)}I_{n+1/2}(x)$ and $k_n(x) = \sqrt{2/(\pi x)}K_{n+1/2}(x)$.

## Connection to Other Functions

**Kelvin functions.** The functions $\text{ber}(x) = \text{Re}[J_0(x\sqrt{i})]$ and $\text{bei}(x) = \text{Im}[J_0(x\sqrt{i})]$ (real and imaginary parts of $J_0$ at complex argument) are related to modified Bessel functions and appear in AC circuit theory (skin effect in cylindrical conductors).

**Hankel transform.** The Hankel (or Bessel) transform $F_\nu(\lambda) = \int_0^\infty f(r)J_\nu(\lambda r)\sqrt{\lambda r}\,dr$ is its own inverse (Parseval theorem). The modified Bessel functions appear as the kernels of the corresponding Laplace-Hankel transform pairs.

**Macdonald function.** $K_\nu$ is also called the Macdonald function or the Basset function. It has an integral representation: $K_\nu(x) = \frac{\sqrt{\pi}(x/2)^\nu}{\Gamma(\nu+1/2)}\int_1^\infty e^{-xt}(t^2-1)^{\nu-1/2}\,dt$ for $\nu > -1/2$, $x > 0$.
