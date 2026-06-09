# The Heat Equation in Cylindrical Coordinates

Many heat conduction problems arise in cylindrical domains: a circular rod, a cylindrical fuel rod in a nuclear reactor, a pipe carrying hot fluid. When the domain and boundary conditions have circular symmetry about the $z$-axis, the problem reduces to the heat equation in polar/cylindrical coordinates, and separation of variables leads naturally to Bessel's equation.

## Setup: Radially Symmetric Heat Equation on a Disk

Consider the heat equation on the disk $\Omega = \{(x,y): x^2+y^2 < a^2\}$ with Dirichlet boundary conditions and initial data depending only on the radial coordinate $r = \sqrt{x^2+y^2}$:

$$u_t = \kappa\!\left(u_{rr} + \frac{1}{r}u_r\right), \qquad 0 < r < a,\; t > 0,$$
$$u(a,t) = 0, \qquad u(r,0) = f(r).$$

The condition at $r=0$ is that $u$ remains bounded (and smooth): this replaces a formal boundary condition at the origin.

## Separation of Variables

Set $u(r,t) = R(r)T(t)$. Substituting:

$$R(r)T'(t) = \kappa\!\left(R''(r) + \frac{1}{r}R'(r)\right)T(t) \implies \frac{T'(t)}{\kappa T(t)} = \frac{R''(r) + R'(r)/r}{R(r)} = -\lambda^2.$$

The temporal equation gives $T(t) = e^{-\kappa\lambda^2 t}$ (only $\lambda^2 > 0$ gives decaying solutions).

The radial equation is:

$$R'' + \frac{1}{r}R' + \lambda^2 R = 0,$$

which is **Bessel's equation of order zero**. Multiplying by $r$: $\frac{d}{dr}(rR') + \lambda^2 rR = 0$.

## Bessel's Equation of Order Zero

The general solution of Bessel's equation of order zero is $R(r) = AJ_0(\lambda r) + BY_0(\lambda r)$, where:
- $J_0(\lambda r)$: Bessel function of the first kind, order zero — bounded at $r=0$.
- $Y_0(\lambda r)$: Bessel function of the second kind, order zero — diverges logarithmically as $r\to 0$.

For $u$ to remain bounded at $r=0$, we require $B = 0$. So $R(r) = J_0(\lambda r)$.

The Dirichlet boundary condition $R(a) = 0$ gives $J_0(\lambda a) = 0$. The equation $J_0(z) = 0$ has infinitely many positive zeros $j_{01} < j_{02} < j_{03} < \cdots$, which are tabulated:
$j_{01} \approx 2.4048$, $j_{02} \approx 5.5201$, $j_{03} \approx 8.6537$, $j_{04} \approx 11.7915$, ...

The eigenvalues are $\lambda_n = j_{0n}/a$ and the eigenfunctions are $R_n(r) = J_0(j_{0n}r/a)$.

## Orthogonality and Bessel-Fourier Series

The eigenfunctions $\{J_0(j_{0n}r/a)\}$ are orthogonal on $[0,a]$ with weight $r$:

$$\int_0^a J_0\!\left(\frac{j_{0n}r}{a}\right)J_0\!\left(\frac{j_{0m}r}{a}\right)r\,dr = \frac{a^2}{2}[J_1(j_{0n})]^2\,\delta_{mn},$$

where $J_1$ is the Bessel function of the first kind, order one. This is the Sturm-Liouville orthogonality for the Bessel equation (the weight $r$ comes from the Sturm-Liouville form $-(rR')' = \lambda^2 rR$).

## Solution by Superposition

$$u(r,t) = \sum_{n=1}^\infty c_n J_0\!\left(\frac{j_{0n}r}{a}\right)e^{-\kappa(j_{0n}/a)^2 t},$$

with coefficients determined by the initial condition:

$$c_n = \frac{2}{a^2[J_1(j_{0n})]^2}\int_0^a f(r)\,J_0\!\left(\frac{j_{0n}r}{a}\right)r\,dr.$$

This is the **Bessel-Fourier expansion** of $f(r)$ on $[0,a]$ with respect to the orthogonal functions $J_0(j_{0n}r/a)$.

## Non-Symmetric Case: Full Angular Dependence

If the initial data or boundary conditions depend on the angle $\theta$, we seek solutions of the form $u = R(r)\Theta(\theta)T(t)$. Separating the Laplacian:

$$\frac{T'}{\kappa T} = \frac{R'' + R'/r + \Theta''/(\Theta r^2)}{1} = -\lambda^2,$$

and separating $r$ from $\theta$:

$$\frac{r^2(R'' + R'/r) + \lambda^2 r^2 R}{R} = -\frac{\Theta''}{\Theta} = m^2.$$

The angular equation $\Theta'' + m^2\Theta = 0$ gives $\Theta_m(\theta) = A_m\cos(m\theta) + B_m\sin(m\theta)$ for $m = 0, 1, 2, \ldots$ (periodicity requires $m$ to be a non-negative integer).

The radial equation becomes **Bessel's equation of order $m$**:

$$R'' + \frac{1}{r}R' + \left(\lambda^2 - \frac{m^2}{r^2}\right)R = 0,$$

with bounded solution $R_{mn}(r) = J_m(j_{mn}r/a)$, where $j_{mn}$ is the $n$-th positive zero of $J_m$.

The full solution is:

$$u(r,\theta,t) = \sum_{m=0}^\infty\sum_{n=1}^\infty J_m\!\left(\frac{j_{mn}r}{a}\right)\left(A_{mn}\cos(m\theta) + B_{mn}\sin(m\theta)\right)e^{-\kappa(j_{mn}/a)^2 t},$$

with coefficients $A_{mn}$ and $B_{mn}$ determined by the initial condition.

## Physical Interpretation

The eigenvalues $\lambda_{mn} = j_{mn}/a$ determine the decay rates of the modes: $e^{-\kappa\lambda_{mn}^2 t}$. The smallest eigenvalue is $\lambda_{01} = j_{01}/a \approx 2.4048/a$, corresponding to the fundamental radially symmetric mode. All other modes decay faster. The fundamental mode has the spatial profile $J_0(j_{01}r/a)$ — a maximum at the center and zero at the boundary, decaying monotonically from center to edge.

For a disk cooling with its boundary held at zero temperature, the long-time behavior is dominated by this fundamental mode: $u(r,t) \approx c_1 J_0(j_{01}r/a)e^{-\kappa(j_{01}/a)^2 t}$. The disk cools uniformly, with the temperature remaining proportional to $J_0(j_{01}r/a)$ as $t\to\infty$.
