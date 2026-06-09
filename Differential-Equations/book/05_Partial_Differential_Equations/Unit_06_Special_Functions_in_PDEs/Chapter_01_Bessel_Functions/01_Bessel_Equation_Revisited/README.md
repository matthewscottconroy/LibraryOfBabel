# Bessel Equation Revisited

Bessel's equation $x^2 y'' + xy' + (x^2 - \nu^2)y = 0$ first appeared in connection with cylindrical heat flow and wave propagation, but it pervades mathematical physics: vibrating membranes, cylindrical waveguides, quantum mechanics in 2D, diffraction theory, Kelvin functions in AC circuit theory, and even the description of the shape of the universe in some cosmological models. The goal of this section is to derive Bessel's equation from first principles, construct its solutions via the Frobenius method, and understand the structure of the solution space.

## Derivation from the Cylindrical Laplacian

The Laplacian in cylindrical coordinates $(r,\theta,z)$ is:

$$\Delta u = \frac{1}{r}\frac{\partial}{\partial r}\left(r\frac{\partial u}{\partial r}\right) + \frac{1}{r^2}\frac{\partial^2 u}{\partial\theta^2} + \frac{\partial^2 u}{\partial z^2}.$$

**Heat equation on a cylinder.** Consider $u_t = \kappa\Delta u$ on a solid cylinder $\{r < R, 0 < z < L\}$ with homogeneous Dirichlet conditions. Separating $u = R(r)\Theta(\theta)Z(z)e^{-\lambda\kappa t}$:

- Azimuthal: $\Theta'' = -m^2\Theta$ gives $\Theta = e^{\pm im\theta}$ with $m \in \mathbb{Z}$.
- Axial: $Z'' = -\mu^2 Z$ gives $Z = \sin(n\pi z/L)$ for Dirichlet conditions, with $\mu = n\pi/L$.
- Radial: $\frac{1}{r}(rR')' - m^2r^{-2}R = -(\lambda - \mu^2)R$. Setting $\alpha^2 = \lambda - \mu^2 > 0$ and $x = \alpha r$:

$$x^2 R'' + xR' + (x^2 - m^2)R = 0. \tag{Bessel, $\nu=m$}$$

The boundary condition $R(R) = 0$ then forces $\alpha R = j_{m,k}$ (a zero of $J_m$), determining the radial eigenvalues.

**Vibrating circular membrane.** The wave equation $u_{tt} = c^2\Delta u$ on a disk with $u|_{r=a} = 0$ leads, after setting $u = R(r)\Theta(\theta)\cos(\omega t + \phi)$, to the same Bessel equation with $\alpha = \omega/c$.

**Schrödinger equation.** The 2D quantum particle in a cylindrical box satisfies $-\frac{\hbar^2}{2m}\Delta\psi = E\psi$ on $r < R$, giving Bessel's equation with $\alpha^2 = 2mE/\hbar^2$.

## The Frobenius Method

Since $x = 0$ is a **regular singular point** of Bessel's equation (the coefficient of $y''$ is $x^2$, vanishing at $x=0$, but $xP(x) = 1$ and $x^2Q(x) = x^2 - \nu^2$ are analytic), the Frobenius method guarantees solutions of the form $y = x^\rho\sum_{k=0}^\infty a_k x^k$.

**Indicial equation.** Substituting $y = x^\rho\sum a_k x^k$ into Bessel's equation and collecting the lowest-order term ($k=0$):

$$a_0[\rho(\rho-1) + \rho - \nu^2] = 0 \implies \rho^2 = \nu^2 \implies \rho = \pm\nu.$$

**First solution ($\rho = \nu \geq 0$).** The recurrence relation for $a_k$ is:

$$[(k+\nu)^2 - \nu^2]a_k = -a_{k-2}, \quad k \geq 2; \qquad a_1 = 0.$$

So $a_k = 0$ for odd $k$, and setting $k = 2j$: $a_{2j} = \frac{-1}{4j(j+\nu)}a_{2j-2}$. With the standard normalization $a_0 = 1/(2^\nu\Gamma(\nu+1))$:

$$J_\nu(x) = \sum_{k=0}^\infty \frac{(-1)^k}{k!\,\Gamma(k+\nu+1)}\left(\frac{x}{2}\right)^{2k+\nu}.$$

This is the **Bessel function of the first kind** of order $\nu$. The series converges for all $x$ (ratio test: $|a_{2k+2}/a_{2k}| = x^2/(4(k+1)(k+1+\nu)) \to 0$).

**Properties from the series:**
- $J_\nu(x) \sim (x/2)^\nu/\Gamma(\nu+1)$ as $x \to 0^+$ (the leading term).
- $J_0(0) = 1$, $J_\nu(0) = 0$ for $\nu > 0$.
- $J_n(x) = (-1)^n J_n(-x)$ for integer $n$ (parity).
- $J_{-n}(x) = (-1)^n J_n(x)$ for integer $n$ — so $J_{-n}$ is not independent of $J_n$.

## The Second Solution $Y_\nu$

**Non-integer $\nu$.** When $\nu \notin \mathbb{Z}$, the two indicial roots $\rho = \nu$ and $\rho = -\nu$ differ by $2\nu \notin \mathbb{Z}$, so the Frobenius method gives two independent solutions:

$$J_{-\nu}(x) = \sum_{k=0}^\infty \frac{(-1)^k}{k!\,\Gamma(k-\nu+1)}\left(\frac{x}{2}\right)^{2k-\nu}.$$

The general solution is $y = c_1 J_\nu(x) + c_2 J_{-\nu}(x)$. The **Neumann function** (second kind) is defined as:

$$Y_\nu(x) = \frac{J_\nu(x)\cos(\nu\pi) - J_{-\nu}(x)}{\sin(\nu\pi)},$$

which has a well-defined limit as $\nu \to n \in \mathbb{Z}$ (by L'Hopital's rule, since $J_{-n} = (-1)^n J_n$ makes the numerator vanish).

**Integer order.** For integer $n$, the second solution $Y_n(x)$ has a logarithmic singularity at the origin:

$$Y_0(x) = \frac{2}{\pi}\left[J_0(x)\ln\frac{x}{2} + \sum_{k=1}^\infty \frac{(-1)^{k+1}H_k}{(k!)^2}\left(\frac{x}{2}\right)^{2k}\right],$$

where $H_k = 1 + 1/2 + \cdots + 1/k$ are the harmonic numbers. For general $n$:

$$Y_n(x) \sim -\frac{(n-1)!}{\pi}\left(\frac{2}{x}\right)^n \quad \text{as } x \to 0^+.$$

**Physical role.** In problems on a complete cylinder (including the axis $r=0$), regularity forces $c_2 = 0$, so only $J_\nu$ is used. $Y_\nu$ is retained in problems on annular regions $R_1 < r < R_2$ (where the singularity at $r=0$ does not arise), in exterior problems $r > R$, or in problems with a cylindrical hole.

## Hankel Functions

The **Hankel functions** (Bessel functions of the third kind) are defined by:

$$H_\nu^{(1)}(x) = J_\nu(x) + iY_\nu(x), \qquad H_\nu^{(2)}(x) = J_\nu(x) - iY_\nu(x).$$

Their large-$x$ asymptotics are:

$$H_\nu^{(1)}(x) \sim \sqrt{\frac{2}{\pi x}}\,e^{i(x - \nu\pi/2 - \pi/4)}, \qquad H_\nu^{(2)}(x) \sim \sqrt{\frac{2}{\pi x}}\,e^{-i(x - \nu\pi/2 - \pi/4)}.$$

These represent **outgoing** and **incoming** cylindrical waves, respectively. In scattering problems, the radiation condition (outgoing waves at infinity) selects $H_\nu^{(1)}$.

## Asymptotic Behavior and Wronskian

For large $x$, the WKBJ approximation applied to Bessel's equation gives:

$$J_\nu(x) \sim \sqrt{\frac{2}{\pi x}}\cos\!\left(x - \frac{\nu\pi}{2} - \frac{\pi}{4}\right), \qquad Y_\nu(x) \sim \sqrt{\frac{2}{\pi x}}\sin\!\left(x - \frac{\nu\pi}{2} - \frac{\pi}{4}\right).$$

The Wronskian (computed from the differential equation or directly from asymptotics):

$$W[J_\nu, Y_\nu](x) = J_\nu(x)Y_\nu'(x) - J_\nu'(x)Y_\nu(x) = \frac{2}{\pi x}.$$

This is everywhere nonzero, confirming that $J_\nu$ and $Y_\nu$ are linearly independent for all $x > 0$.

## Worked Example: Radial Modes of a Cylinder

**Problem.** Find the natural frequencies of radial ($m=0$) vibrations of a cylindrical drum of radius $a$ and height $L$, with fixed boundary (Dirichlet on all faces).

**Solution.** Separate $u = R(r)Z(z)\cos(\omega t)$ with $\Delta u = -(\omega/c)^2 u$. With $m=0$: $R$ satisfies Bessel's equation of order zero, and $Z$ satisfies $Z'' + \mu^2 Z = 0$ with $Z(0) = Z(L) = 0$, giving $\mu_n = n\pi/L$.

The radial equation is $(xR')'/x = \alpha^2 R$ with $\alpha^2 = (\omega/c)^2 - \mu_n^2$. For modes with $\alpha > 0$, $R = J_0(\alpha r)$ (regularity at $r=0$). The condition $R(a) = 0$ requires $\alpha_{kn} a = j_{0,k}$, so $\alpha_{kn} = j_{0,k}/a$.

The natural frequencies are:

$$\omega_{kn} = c\sqrt{\alpha_{kn}^2 + \mu_n^2} = c\sqrt{\left(\frac{j_{0,k}}{a}\right)^2 + \left(\frac{n\pi}{L}\right)^2}.$$

With $j_{0,1} \approx 2.405$, the fundamental mode ($k=1$, $n=1$) has $\omega_{11} = c\sqrt{(j_{0,1}/a)^2 + (\pi/L)^2}$.

## Recursion Relations

Differentiation of the series definition yields the fundamental recursion relations:

$$J_{\nu-1}(x) + J_{\nu+1}(x) = \frac{2\nu}{x}J_\nu(x), \qquad J_{\nu-1}(x) - J_{\nu+1}(x) = 2J_\nu'(x).$$

Equivalently:

$$\frac{d}{dx}[x^\nu J_\nu(x)] = x^\nu J_{\nu-1}(x), \qquad \frac{d}{dx}[x^{-\nu}J_\nu(x)] = -x^{-\nu}J_{\nu+1}(x).$$

**Consequence:** $J_0'(x) = -J_1(x)$, $J_1'(x) = J_0(x) - J_1(x)/x$. The recursion reduces integrals and derivatives of Bessel functions to combinations of $J_\nu$ for different $\nu$ — essential for both analytical and numerical work.

## Generating Function and Addition Theorem

For integer $n$, the generating function is:

$$e^{x(t-1/t)/2} = \sum_{n=-\infty}^\infty J_n(x)\, t^n,$$

which immediately gives the integral representation $J_n(x) = \frac{1}{2\pi}\int_0^{2\pi}e^{ix\sin\theta}e^{-in\theta}\,d\theta$. Setting $t = e^{i\theta}$ and taking real part yields $J_n(x) = \frac{1}{\pi}\int_0^\pi\cos(n\theta - x\sin\theta)\,d\theta$ — Bessel's original integral formula. This integral representation is useful for asymptotic analysis via the method of stationary phase, yielding the large-$x$ asymptotics rigorously.
