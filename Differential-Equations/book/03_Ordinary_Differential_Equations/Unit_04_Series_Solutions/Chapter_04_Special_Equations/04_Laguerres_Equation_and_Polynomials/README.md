# Laguerre's Equation and Laguerre Polynomials

Laguerre's equation is

$$xy'' + (1-x)y' + ny = 0, \qquad n \geq 0.$$

It arises in quantum mechanics as the equation governing the radial wave function of the hydrogen atom after separating variables in the Schrodinger equation in spherical coordinates.

## Analysis at the Singular Point

The point $x = 0$ is a regular singular point ($P_0 = \lim_{x\to 0} xp(x) = 1$, $Q_0 = \lim_{x\to 0} x^2 q(x) = 0$). Indicial equation: $r(r-1) + r + 0 = r^2 = 0$, repeated root $r = 0$. The first Frobenius solution is a standard power series; the second involves $\ln x$.

Substituting $y = \sum_{k=0}^\infty a_k x^k$ into the equation (which has $r = 0$, so the series starts as a standard power series):

$$k^2 a_k - (k-1-n)a_{k-1} = 0 \implies a_k = \frac{k-1-n}{k^2}a_{k-1}.$$

For non-negative integer $n$, $a_{n+1} = 0$ and all subsequent terms vanish: the series terminates.

## The Laguerre Polynomials $L_n(x)$

The polynomial solution, normalized so $L_n(0) = 1$, is:

$$L_n(x) = \sum_{k=0}^n \binom{n}{k}\frac{(-x)^k}{k!}.$$

**Rodrigues' formula:** $L_n(x) = \frac{e^x}{n!}\frac{d^n}{dx^n}(e^{-x}x^n)$.

**Three-term recurrence:** $(n+1)L_{n+1}(x) = (2n+1-x)L_n(x) - nL_{n-1}(x)$.

**First few polynomials:** $L_0 = 1$, $L_1 = 1 - x$, $L_2 = 1 - 2x + x^2/2$, $L_3 = 1 - 3x + \frac{3}{2}x^2 - \frac{1}{6}x^3$.

## Orthogonality

The Laguerre polynomials are orthogonal on $[0, \infty)$ with weight $e^{-x}$:

$$\int_0^\infty L_m(x)L_n(x)e^{-x}\,dx = \delta_{mn}.$$

## Hydrogen Atom Connection

The radial wave functions of hydrogen are $R_{n\ell}(r) = N_{n\ell}e^{-r/(na_0)}(2r/(na_0))^\ell L_{n-\ell-1}^{(2\ell+1)}(2r/(na_0))$, where $L_k^{(\alpha)}$ are the associated Laguerre polynomials (solutions of $xy'' + (\alpha+1-x)y' + ny = 0$). The principal quantum number $n$ and the orbital quantum number $\ell$ appear in the polynomial index, with the polynomial degree $n - \ell - 1$ determining the number of radial nodes.
