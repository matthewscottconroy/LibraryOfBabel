# Hermite's Equation and Hermite Polynomials

Hermite's equation is

$$y'' - 2xy' + 2ny = 0, \qquad n \geq 0.$$

It arises in quantum mechanics as the equation for the energy eigenfunctions of the one-dimensional harmonic oscillator: writing the time-independent Schrodinger equation $-\psi'' + x^2\psi = E\psi$ and substituting $\psi(x) = e^{-x^2/2}H(x)$ gives Hermite's equation for $H(x)$ with $2n = E - 1$.

## Series Solution and Polynomial Termination

The origin is an ordinary point; all singular points are at infinity. Substituting $y = \sum a_k x^k$ gives the recurrence

$$a_{k+2} = \frac{2(k-n)}{(k+2)(k+1)}a_k.$$

For non-negative integer $n$, the recurrence gives $a_{n+2} = 0$ when the factor $(k - n)$ vanishes at $k = n$. The corresponding series solution terminates at $x^n$, giving a polynomial.

## The Hermite Polynomials $H_n(x)$

By convention (physicists' convention), the Hermite polynomials are normalized so that the coefficient of $x^n$ is $2^n$:

$$H_0 = 1,\quad H_1 = 2x,\quad H_2 = 4x^2 - 2,\quad H_3 = 8x^3 - 12x,\quad H_4 = 16x^4 - 48x^2 + 12.$$

**Rodrigues' formula:** $H_n(x) = (-1)^n e^{x^2}\frac{d^n}{dx^n}e^{-x^2}$.

**Three-term recurrence:** $H_{n+1}(x) = 2xH_n(x) - 2nH_{n-1}(x)$.

**Generating function:** $e^{2xt - t^2} = \sum_{n=0}^\infty H_n(x)\frac{t^n}{n!}$.

## Orthogonality

The Hermite polynomials are orthogonal on $(-\infty, \infty)$ with weight $e^{-x^2}$:

$$\int_{-\infty}^\infty H_m(x)H_n(x)e^{-x^2}\,dx = 2^n n!\sqrt{\pi}\,\delta_{mn}.$$

## Quantum Harmonic Oscillator

The energy eigenfunctions of the harmonic oscillator are

$$\psi_n(x) = \left(\frac{1}{2^n n!\sqrt{\pi}}\right)^{1/2}e^{-x^2/2}H_n(x), \qquad n = 0, 1, 2, \ldots$$

These form a complete orthonormal basis for $L^2(\mathbb{R})$. The energy eigenvalues are $E_n = \hbar\omega(n + 1/2)$, equally spaced by $\hbar\omega$.

The probabilists' Hermite polynomials $\text{He}_n(x) = 2^{-n/2}H_n(x/\sqrt{2})$ appear in probability theory, where the weight function $e^{-x^2/2}/\sqrt{2\pi}$ is the standard normal density. This connection underlies the central role of Hermite polynomials in stochastic analysis and the Malliavin calculus.
