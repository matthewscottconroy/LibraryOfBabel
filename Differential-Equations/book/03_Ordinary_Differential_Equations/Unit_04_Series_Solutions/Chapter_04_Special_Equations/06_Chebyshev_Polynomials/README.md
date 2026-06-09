# Chebyshev Polynomials

Chebyshev's equation is

$$(1-x^2)y'' - xy' + n^2 y = 0, \qquad n \geq 0.$$

The substitution $x = \cos\theta$ transforms it to $d^2y/d\theta^2 + n^2 y = 0$, with solutions $\cos(n\theta)$ and $\sin(n\theta)$. The Chebyshev polynomials $T_n(x) = \cos(n\arccos x)$ are thus precisely the oscillatory solutions of the equation, expressed as polynomials in $x$ via trigonometric identities.

## Definition and Explicit Formulae

$$T_0(x) = 1,\quad T_1(x) = x,\quad T_2(x) = 2x^2-1,\quad T_3(x) = 4x^3-3x,\quad T_4(x) = 8x^4 - 8x^2 + 1.$$

**Three-term recurrence:** $T_{n+1}(x) = 2xT_n(x) - T_{n-1}(x)$.

**Rodrigues' formula:** $T_n(x) = \frac{(-1)^n\sqrt{\pi}(1-x^2)^{1/2}}{(2n-1)!!}\frac{d^n}{dx^n}(1-x^2)^{n-1/2}$.

## Orthogonality

The Chebyshev polynomials are orthogonal on $[-1, 1]$ with weight $w(x) = 1/\sqrt{1-x^2}$:

$$\int_{-1}^1 T_m(x)T_n(x)\frac{dx}{\sqrt{1-x^2}} = \begin{cases}0 & m \neq n,\\ \pi & m = n = 0,\\ \pi/2 & m = n \geq 1.\end{cases}$$

This is the $L^2$ orthogonality with the arcsine measure $d\theta$ (after $x = \cos\theta$).

## Minimax Approximation

The most important property of Chebyshev polynomials for practical applications is the **minimax property**: among all monic polynomials of degree $n$, $T_n(x)/2^{n-1}$ has the smallest maximum absolute value on $[-1, 1]$:

$$\max_{x \in [-1,1]}\left|\frac{T_n(x)}{2^{n-1}}\right| = \frac{1}{2^{n-1}} \leq \max_{x\in[-1,1]}|p(x)|$$

for any monic polynomial $p$ of degree $n$. This makes Chebyshev polynomials the optimal basis for polynomial approximation in the infinity norm: expanding a function in Chebyshev series and truncating gives the best possible polynomial approximation of each degree.

Chebyshev nodes (the zeros of $T_n$: $x_k = \cos((2k-1)\pi/(2n))$) are optimal for polynomial interpolation, minimizing the Runge phenomenon that plagues equally spaced nodes. This makes Chebyshev polynomials indispensable in scientific computing, spectral methods for ODEs and PDEs, and numerical integration.
