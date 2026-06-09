# Chapter 02: Classical Fourier Series

With the language of inner products and orthogonality established, we now derive the central object of this unit: the Fourier series of a periodic function. The coefficient formulas emerge directly from orthogonality, but the deeper questions — when does the series converge, what does it converge to, and in what sense — require more careful analysis.

## Chapter Overview

**Section 01: Fourier Coefficients** derives the formulas for the Fourier coefficients of a $2\pi$-periodic function $f$. The coefficient $a_n = \frac{1}{\pi}\int_{-\pi}^\pi f(x)\cos(nx)\,dx$ is the inner product of $f$ with the $n$-th cosine, divided by the squared norm of that cosine. The formula is forced by orthogonality: if $f = \frac{a_0}{2} + \sum (a_n\cos(nx) + b_n\sin(nx))$ and we multiply both sides by $\cos(mx)$ and integrate, all cross terms vanish and we isolate $a_m$.

**Section 02: Pointwise Convergence** addresses the most natural convergence question: for a given $x$, does $S_N f(x) \to f(x)$ as $N \to \infty$? The answer depends on the local behavior of $f$ near $x$. **Dirichlet's theorem** gives a clean sufficient condition: if $f$ is piecewise smooth, then the partial sums converge at every point, with the limit being $f(x)$ at continuity points and the average $\frac{1}{2}[f(x^+) + f(x^-)]$ at jump discontinuities. The proof proceeds via the Dirichlet kernel $D_N(t) = \frac{1}{2} + \sum_{n=1}^N \cos(nt) = \frac{\sin((N+\frac{1}{2})t)}{2\sin(t/2)}$ and requires estimating an oscillatory integral.

**Section 03: Gibbs Phenomenon** examines what happens near a jump discontinuity in more detail. Although the partial sums converge to the correct average value at the jump, the maximum overshoot does not go to zero as $N \to \infty$. Instead, it approaches approximately $8.9\%$ of the jump height, regardless of $N$. This overshoot is not a defect of particular functions but an intrinsic feature of truncated Fourier series near discontinuities.

**Section 04: Parseval's Identity** relates the $L^2$ norm of $f$ to the sum of squares of its Fourier coefficients:
$$\frac{1}{\pi}\int_{-\pi}^\pi |f(x)|^2\,dx = \frac{a_0^2}{2} + \sum_{n=1}^\infty (a_n^2 + b_n^2).$$
This is a statement of conservation of energy: the energy in the function equals the energy distributed across its frequency components. It also has the algebraic consequence of allowing the computation of numerical series: for instance, evaluating Parseval's identity for $f(x) = x$ yields $\sum_{n=1}^\infty 1/n^2 = \pi^2/6$.

**Section 05: Uniform Convergence** identifies conditions under which the Fourier series converges uniformly — meaning the partial sums $S_N f$ converge to $f$ in the sup norm. The key result is that if $f$ is continuous and $2\pi$-periodic, and if $f'$ is piecewise continuous, then the Fourier series of $f$ converges to $f$ uniformly on $\mathbb{R}$. Uniform convergence is strictly stronger than pointwise convergence and allows term-by-term integration and differentiation.

## Key Theorems at a Glance

**Dirichlet's Theorem:** If $f$ is piecewise smooth and $2\pi$-periodic, then at each $x$,
$$S_N f(x) \to \frac{f(x^+) + f(x^-)}{2}.$$

**Parseval's Identity:** For $f \in L^2([-\pi, \pi])$,
$$\|f\|_2^2 = \pi\left[\frac{a_0^2}{2} + \sum_{n=1}^\infty (a_n^2 + b_n^2)\right].$$

**Uniform Convergence Criterion:** If $f$ is continuous, $2\pi$-periodic, and $f'$ is piecewise continuous, then the Fourier series converges uniformly.

## Connections

The material in this chapter provides the technical core of classical Fourier analysis. The Gibbs phenomenon informs the design of digital filters (Lanczos windowing mitigates it). Parseval's identity is the prototype for spectral methods in PDEs, where energy estimates in frequency space control the solution. Uniform convergence is needed to justify term-by-term differentiation of Fourier series solutions to PDEs.
