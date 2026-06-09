# Legendre's Equation and Legendre Polynomials

Legendre's equation is

$$(1-x^2)y'' - 2xy' + n(n+1)y = 0, \qquad n \geq 0,$$

or equivalently $\frac{d}{dx}\left[(1-x^2)\frac{dy}{dx}\right] + n(n+1)y = 0$. It arises when solving the Laplace equation $\nabla^2 u = 0$ in spherical coordinates by separation of variables: the angular part gives this equation with $x = \cos\theta$.

## Series Solution and Termination

The singular points are at $x = \pm 1$; the origin is ordinary. Substituting $y = \sum a_k x^k$ gives the recurrence

$$a_{k+2} = \frac{k(k+1) - n(n+1)}{(k+2)(k+1)}a_k = \frac{(k-n)(k+n+1)}{(k+2)(k+1)}a_k.$$

When $n$ is a non-negative integer, the factor $(k - n)$ vanishes at $k = n$, and the recurrence gives $a_{n+2} = 0$, $a_{n+4} = 0$, etc. One solution terminates and becomes a polynomial of degree $n$. The other solution (starting with the opposite parity, $a_0$ or $a_1$) does not terminate and converges only for $|x| < 1$.

## The Legendre Polynomials $P_n(x)$

The polynomial solution of Legendre's equation, normalized by $P_n(1) = 1$, is the **Legendre polynomial** $P_n(x)$:

$$P_0(x) = 1, \quad P_1(x) = x, \quad P_2(x) = \frac{1}{2}(3x^2 - 1), \quad P_3(x) = \frac{1}{2}(5x^3 - 3x).$$

**Rodrigues' formula:** $P_n(x) = \frac{1}{2^n n!}\frac{d^n}{dx^n}(x^2 - 1)^n$.

**Three-term recurrence:** $(n+1)P_{n+1}(x) = (2n+1)xP_n(x) - nP_{n-1}(x)$.

**Generating function:** $\frac{1}{\sqrt{1 - 2xt + t^2}} = \sum_{n=0}^\infty P_n(x)t^n$, convergent for $|t| < 1$.

## Orthogonality

The Legendre polynomials are orthogonal on $[-1, 1]$ with weight function $w = 1$:

$$\int_{-1}^1 P_m(x)P_n(x)\,dx = \frac{2}{2n+1}\delta_{mn}.$$

This follows from the Sturm-Liouville structure (Unit 8). Orthogonality enables **Legendre series**: any piecewise smooth function $f$ on $[-1, 1]$ can be expanded as $f(x) = \sum_{n=0}^\infty c_n P_n(x)$ with $c_n = \frac{2n+1}{2}\int_{-1}^1 f(x)P_n(x)\,dx$.

## Physical Applications

In electrostatics, the potential of a charge distribution is expanded in Legendre polynomials (the multipole expansion). In quantum mechanics, the spherical harmonics $Y_\ell^m(\theta, \phi) = P_\ell^m(\cos\theta)e^{im\phi}$ (where $P_\ell^m$ are associated Legendre functions) are the eigenfunctions of the angular momentum operator. In numerical integration, Gauss-Legendre quadrature uses the zeros of $P_n(x)$ as quadrature points, giving maximum polynomial exactness.
