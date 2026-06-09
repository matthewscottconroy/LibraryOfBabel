# Legendre Polynomials: Properties

Legendre polynomials $P_\ell(t)$ are the orthogonal polynomials on $[-1,1]$ with weight function $w(t) = 1$. They arise as the bounded solutions of Legendre's equation when the azimuthal quantum number is $m=0$, and they encode the angular dependence of axially symmetric solutions of Laplace's equation in spherical coordinates. Beyond PDE applications, they appear as the kernel of the Funk-Hecke formula, the expansion basis for the Coulomb potential, the generating polynomials for 3-$j$ symbols in quantum mechanics, and the defining functions for Gauss-Legendre quadrature. This section collects their essential properties with complete derivations.

## Legendre's Equation and the Frobenius Method

Legendre's equation of degree $\ell$ is:

$$\frac{d}{dt}\!\left[(1-t^2)\frac{dP}{dt}\right] + \ell(\ell+1)P = 0, \quad -1 \leq t \leq 1,$$

or equivalently $(1-t^2)P'' - 2tP' + \ell(\ell+1)P = 0$.

The point $t=0$ is an ordinary point, so the Frobenius method gives power series solutions. Seeking $P = \sum_{k=0}^\infty a_k t^k$:

$$(1-t^2)\sum_{k=2}^\infty k(k-1)a_k t^{k-2} - 2t\sum_{k=1}^\infty ka_k t^{k-1} + \ell(\ell+1)\sum_{k=0}^\infty a_k t^k = 0.$$

Collecting the coefficient of $t^k$: $(k+2)(k+1)a_{k+2} - k(k-1)a_k - 2ka_k + \ell(\ell+1)a_k = 0$, giving the two-term recursion:

$$a_{k+2} = \frac{k(k+1) - \ell(\ell+1)}{(k+2)(k+1)}a_k = -\frac{(\ell-k)(\ell+k+1)}{(k+2)(k+1)}a_k.$$

**Two independent solutions** (one even, one odd): set $a_0 = 1, a_1 = 0$ for the even solution, and $a_0 = 0, a_1 = 1$ for the odd solution.

**Termination.** When $\ell = k$, the factor $(\ell-k) = 0$ terminates the recursion: the series reduces to a polynomial! For $\ell$ even, the even solution terminates at degree $\ell$; for $\ell$ odd, the odd solution terminates. The other series diverges at $t = \pm 1$ (ratio test: the terms grow like $1/(1-t^2)$ near $t=1$). The **Legendre polynomial** $P_\ell(t)$ is the terminating solution, normalized so that $P_\ell(1) = 1$.

## First Polynomials

From the recursion (normalized at $t=1$):

$$P_0(t) = 1, \quad P_1(t) = t, \quad P_2(t) = \tfrac{1}{2}(3t^2-1), \quad P_3(t) = \tfrac{1}{2}(5t^3-3t),$$

$$P_4(t) = \tfrac{1}{8}(35t^4-30t^2+3), \quad P_5(t) = \tfrac{1}{8}(63t^5-70t^3+15t).$$

**Quick check:** $P_2(1) = \frac{1}{2}(3-1) = 1$. $P_2(-1) = \frac{1}{2}(3-1) = 1 = (-1)^2$. $P_3(-1) = \frac{1}{2}(-5+3) = -1 = (-1)^3$. These illustrate the general formula $P_\ell(-1) = (-1)^\ell$.

## Rodrigues' Formula

$$P_\ell(t) = \frac{1}{2^\ell\ell!}\frac{d^\ell}{dt^\ell}(t^2-1)^\ell. \tag{Rodrigues}$$

**Verification for $\ell=2$:** $(t^2-1)^2 = t^4 - 2t^2 + 1$. $(d^2/dt^2)(t^4-2t^2+1) = 12t^2 - 4$. Dividing by $2^2\cdot 2! = 8$: $(12t^2-4)/8 = (3t^2-1)/2 = P_2(t)$. $\checkmark$

**Proof.** Let $v = (t^2-1)^\ell$ and $v_k = d^k v/dt^k$. Then $v_1 = 2\ell t(t^2-1)^{\ell-1}$, giving $(1-t^2)v_1 = -2\ell tv$, so $(1-t^2)v_1 + 2\ell tv = 0$. Differentiate this $\ell+1$ times using Leibniz's rule to get Legendre's equation for $v_\ell$. Setting $P_\ell = v_\ell/(2^\ell\ell!)$ normalizes $P_\ell(1) = 1$ (by the binomial theorem applied to $(t-1)^\ell(t+1)^\ell$ at $t=1$).

**Consequence.** Since $\frac{d^\ell}{dt^\ell}[(t-1)^\ell(t+1)^\ell]$ is a polynomial of degree $\ell$, and $P_\ell$ is obtained by differentiating $\ell$ times, $P_\ell$ is indeed a polynomial of degree $\ell$, with leading coefficient $\frac{(2\ell)!}{2^\ell(\ell!)^2}$.

## Generating Function

$$\sum_{\ell=0}^\infty P_\ell(t)\,s^\ell = \frac{1}{\sqrt{1-2ts+s^2}}, \qquad |s| < 1, \; |t| \leq 1. \tag{Generating function}$$

**Derivation.** The function $G(t,s) = (1-2ts+s^2)^{-1/2}$ satisfies $\frac{\partial}{\partial s}[(1-2ts+s^2)G] = (t-s)G$. Expanding $G = \sum a_\ell s^\ell$ and differentiating with respect to $s$ yields the Legendre recursion relation for $a_\ell$, with initial conditions $a_0 = 1$ and $a_1 = t$ matching $P_0 = 1$ and $P_1 = t$.

**Applications:**
1. **Special values.** Setting $t=1$: $\sum P_\ell(1)s^\ell = (1-s)^{-1} = \sum s^\ell$, so $P_\ell(1) = 1$. Setting $t=-1$: $\sum P_\ell(-1)s^\ell = (1+s)^{-1} = \sum(-s)^\ell$, giving $P_\ell(-1) = (-1)^\ell$.
2. **Coulomb potential.** Setting $t = \cos\gamma$ and $s = r_</r_>$: $1/|\mathbf{x}-\mathbf{y}| = (r_>^2 - 2r_<r_>\cos\gamma + r_<^2)^{-1/2}/r_> = \sum_\ell(r_<^\ell/r_>^{\ell+1})P_\ell(\cos\gamma)$.

## Orthogonality

**Theorem.**

$$\int_{-1}^1 P_\ell(t)P_k(t)\,dt = \frac{2}{2\ell+1}\delta_{\ell k}. \tag{Orthogonality}$$

**Proof.** Write both $P_\ell$ and $P_k$ in Rodrigues form. If $\ell > k$, integrate by parts $k$ times (each time differentiating $(t^2-1)^k$ and integrating $(d/dt)^{\ell+1}[\cdots]$); the boundary terms vanish at $\pm 1$ since $(t^2-1)^k$ has a zero of order $k$ at each endpoint. After $k$ integrations by parts, the integrand contains $(d/dt)^{\ell-k}(t^2-1)^\ell$ times $(t^2-1)^k$, and since $\ell > k$, $(d/dt)^{\ell-k}(t^2-1)^\ell$ is a polynomial of degree $\ell + k$ that vanishes at $\pm 1$ — further integration by parts reduces to zero.

For $\ell = k$: use the Rodrigues formula to compute $\int_{-1}^1[P_\ell(t)]^2\,dt = \frac{1}{2^{2\ell}(\ell!)^2}\int_{-1}^1\left[\frac{d^\ell}{dt^\ell}(t^2-1)^\ell\right]^2dt$. Integrate by parts $\ell$ times (boundary terms vanish) to get $\frac{(-1)^\ell}{2^{2\ell}(\ell!)^2}\int_{-1}^1(t^2-1)^\ell\frac{d^{2\ell}}{dt^{2\ell}}(t^2-1)^\ell\,dt$. The $2\ell$-th derivative of $(t^2-1)^\ell$ is $(2\ell)!$ (constant). The remaining integral $\int_{-1}^1(1-t^2)^\ell\,dt = 2^{2\ell+1}(\ell!)^2/(2\ell+1)!$ (by the beta function formula). Combining: $\int_{-1}^1[P_\ell]^2\,dt = 2/(2\ell+1)$. $\square$

## Recursion Relations

The fundamental three-term recursion is:

$$(n+1)P_{n+1}(t) = (2n+1)tP_n(t) - nP_{n-1}(t). \tag{3-term recursion}$$

**Proof.** Differentiate the generating function identity $G = \sum_\ell P_\ell s^\ell$ with respect to $s$: $(s-t)G + (1-2ts+s^2)G_s = 0$. Substituting the generating function series and comparing coefficients of $s^n$ gives the recursion.

Additional useful recursion relations (derivable by differentiating the generating function):

$$P_n'(t) = tP_{n-1}'(t) + nP_{n-1}(t) = tP_{n+1}'(t) - (n+1)P_{n+1}(t)/1 \cdot P_n$$

and:

$$P_n'(t) - P_{n-2}'(t) = (2n-1)P_{n-1}(t), \qquad (1-t^2)P_n'(t) = n[P_{n-1}(t) - tP_n(t)].$$

**Practical use.** The three-term recursion is the standard algorithm for evaluating $P_\ell(t)$ numerically: given $P_0 = 1$ and $P_1 = t$, compute $P_2, P_3, \ldots$ iteratively. This is faster and more numerically stable than using the explicit formula.

## Special Values and Symmetry

- $P_\ell(1) = 1$ (from generating function at $t=1$).
- $P_\ell(-1) = (-1)^\ell$ (from generating function at $t=-1$).
- $P_\ell(0) = 0$ for $\ell$ odd; $P_{2k}(0) = (-1)^k\frac{(2k)!}{2^{2k}(k!)^2} = (-1)^k\binom{2k}{k}/4^k$.
- **Parity:** $P_\ell(-t) = (-1)^\ell P_\ell(t)$ (even/odd depending on $\ell$).
- **Bound:** $|P_\ell(t)| \leq 1$ for all $t \in [-1,1]$, with $P_\ell(\pm 1) = \pm 1$.

## Completeness and the Legendre Series

**Theorem (completeness).** The Legendre polynomials $\{P_\ell\}_{\ell=0}^\infty$ form a complete orthogonal system in $L^2([-1,1])$. Every $f \in L^2([-1,1])$ has the expansion:

$$f(t) = \sum_{\ell=0}^\infty \hat{f}_\ell P_\ell(t), \qquad \hat{f}_\ell = \frac{2\ell+1}{2}\int_{-1}^1 f(t)P_\ell(t)\,dt,$$

with convergence in $L^2$. Parseval's identity: $\int_{-1}^1|f|^2\,dt = \sum_\ell \frac{2}{2\ell+1}|\hat{f}_\ell|^2$.

**Proof of completeness.** The Weierstrass approximation theorem guarantees that polynomials are dense in $C([-1,1])$ and hence in $L^2([-1,1])$. Since $\text{span}\{P_0, P_1, \ldots, P_N\} = \text{span}\{1,t,\ldots,t^N\}$ (the span of the first $N+1$ Legendre polynomials equals the polynomials of degree $\leq N$), the Legendre polynomials span a dense set.

**Worked example.** Expand $f(t) = |t|$ in Legendre series. By parity, only even-$\ell$ terms contribute. Using $\int_{-1}^1 |t| P_{2k}(t)\,dt = 2\int_0^1 t P_{2k}(t)\,dt$:

$$\hat{f}_0 = \frac{1}{2}\int_{-1}^1|t|\,dt = \frac{1}{2}, \qquad \hat{f}_{2k} = \frac{4k+1}{2}\cdot 2\int_0^1 t P_{2k}(t)\,dt.$$

Using the recursion $tP_{2k}(t) = \frac{1}{4k+1}[(2k+1)P_{2k+1}(t) + 2kP_{2k-1}(t)]$ and orthogonality: the coefficients are $\hat{f}_0 = 1/2$, $\hat{f}_2 = -5/8$, $\hat{f}_4 = 3/16$, ... and in general $\hat{f}_{2k} = (-1)^{k+1}\frac{(4k+1)(2k-2)!}{2^{2k+1}k!(k+1)!}$ for $k \geq 1$.

## Connection to Gauss-Legendre Quadrature

The zeros $t_1, \ldots, t_\ell$ of $P_\ell(t)$ are the nodes of **Gauss-Legendre quadrature**: the formula $\int_{-1}^1 f(t)\,dt \approx \sum_{k=1}^\ell w_k f(t_k)$ (with appropriately chosen weights $w_k$) integrates all polynomials of degree $\leq 2\ell-1$ exactly. This is optimal: no $\ell$-point quadrature rule can exactly integrate polynomials of degree higher than $2\ell-1$. The interlacing property of zeros ensures the nodes are distinct and lie in $(-1,1)$, and the weights $w_k = 2/[(1-t_k^2)(P_\ell'(t_k))^2]$ are all positive — making Gauss-Legendre quadrature numerically stable.
