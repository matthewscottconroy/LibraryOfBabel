# Chapter 02: The Mellin Transform

The Fourier transform is adapted to the additive structure of the real line. The **Mellin transform** is the natural analog for the multiplicative structure of the positive real line: instead of decomposing into additive waves $e^{2\pi i\xi x}$, it decomposes into power functions $x^{s-1}$ (the multiplicative characters of $(\mathbb{R}^+, \times)$).

The Mellin transform appears in contexts where multiplicative scaling is the natural symmetry: special functions, Dirichlet series in number theory, the analysis of algorithms (particularly divide-and-conquer recurrences), and the computation of certain integrals that are otherwise difficult.

## Chapter Overview

**Section 01: Definition and Properties** introduces the Mellin transform $\mathcal{M}[f](s) = \int_0^\infty f(x)x^{s-1}\,dx$ and establishes its fundamental properties. The key relationship to the Fourier and Laplace transforms is made explicit: the substitution $x = e^t$ converts the Mellin transform to the bilateral Laplace transform. The Mellin convolution $(f \star g)(x) = \int_0^\infty f(y)g(x/y)\,dy/y$ (multiplicative convolution) corresponds to multiplication of Mellin transforms: $\mathcal{M}[f \star g] = \mathcal{M}[f]\cdot\mathcal{M}[g]$.

**Section 02: Applications** demonstrates the Mellin transform in action. In analytic number theory, the Mellin transform of the Jacobi theta function yields the functional equation for the Riemann zeta function. In the computation of definite integrals, the Mellin transform of elementary functions gives closed-form results for Euler integrals and related expressions. In the analysis of the behavior of algorithms (e.g., the runtime of a divide-and-conquer algorithm satisfying $T(n) = 2T(n/2) + n$), the Mellin transform solves the recurrence in a systematic way.

## The Core Idea

The Mellin transform detects behavior at $x = 0$ and $x = \infty$ in terms of power laws. If $f(x) \sim Cx^\alpha$ as $x \to 0^+$, the transform $\mathcal{M}[f](s)$ has a pole at $s = -\alpha$. If $f(x) \sim Cx^\beta$ as $x \to \infty$, there is a pole at $s = -\beta$. The strip of analyticity of $\mathcal{M}[f]$ lies between these poles, encoding the complete power-law behavior of $f$.
