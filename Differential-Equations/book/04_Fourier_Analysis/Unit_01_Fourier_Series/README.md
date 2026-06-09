# Unit 01: Fourier Series

A plucked guitar string vibrates in a superposition of modes: the fundamental frequency and its integer multiples, called harmonics. Each mode contributes a sinusoidal oscillation, and the perceived timbre of the note is determined by how much energy sits in each harmonic. This physical reality motivates one of the central questions of classical analysis: given a periodic function $f$, can we express it as an infinite sum of sines and cosines, and if so, how?

Fourier series answer this question affirmatively for an enormous class of functions. This unit builds the theory from the ground up, moving from a careful treatment of periodicity and orthogonality to the full convergence theory of classical Fourier series, specialized expansions, and the complex exponential formulation.

## Chapter Overview

**Chapter 01: Periodic Functions and Orthogonality** lays the conceptual foundations. We begin by making precise what it means for a function to be periodic and how different periods relate to one another through the notion of harmonics. The key structural insight is that the trigonometric functions $\{1, \cos(nx), \sin(nx)\}$ form an orthogonal set under the $L^2$ inner product on $[-\pi, \pi]$. The inner product $\langle f, g \rangle = \int_{-\pi}^\pi f(x)g(x)\,dx$ gives function space the same geometric structure as Euclidean space, allowing us to think of Fourier coefficients as orthogonal projections.

**Chapter 02: Classical Fourier Series** develops the core formulas and convergence theory. Given a $2\pi$-periodic function $f$, its Fourier series is
$$f(x) \sim \frac{a_0}{2} + \sum_{n=1}^\infty \left[ a_n \cos(nx) + b_n \sin(nx) \right],$$
where the coefficients are computed by integration against the basis functions. We prove Dirichlet's theorem on pointwise convergence: at a point where $f$ has left and right limits and left and right derivatives, the series converges to the average of the left and right limits. We examine the Gibbs phenomenon, the persistent overshoot near jump discontinuities that no amount of truncation eliminates. We establish Parseval's identity, which equates the $L^2$ norm of $f$ to the sum of squares of its Fourier coefficients, and discuss conditions under which convergence is uniform.

**Chapter 03: Sine and Cosine Series** addresses functions defined on a half-interval $[0, L]$. By extending such a function to $[-L, L]$ as an even or odd function, we obtain expansions purely in cosines or sines, respectively. These half-range expansions are essential in boundary value problems, where physical conditions at an endpoint force either the function or its derivative to vanish there.

**Chapter 04: Complex Form** reformulates the Fourier series using complex exponentials. Euler's formula $e^{inx} = \cos(nx) + i\sin(nx)$ allows us to write the series as $\sum_{n=-\infty}^\infty c_n e^{inx}$, where the complex coefficients $c_n = \frac{1}{2\pi}\int_{-\pi}^\pi f(x)e^{-inx}\,dx$ encode both amplitude and phase information. This form is algebraically cleaner, computationally convenient, and the natural bridge to the Fourier transform on $\mathbb{R}$.

## Learning Goals

By the end of this unit, you should be able to compute the Fourier series of a given periodic function and determine what values the series converges to at points of continuity and discontinuity. You should understand why orthogonality is the key structural property making the coefficient formulas work, and be able to use Parseval's identity to sum numerical series. You should be fluent in both the real and complex forms of the series, and comfortable choosing sine or cosine expansions when boundary conditions dictate.
