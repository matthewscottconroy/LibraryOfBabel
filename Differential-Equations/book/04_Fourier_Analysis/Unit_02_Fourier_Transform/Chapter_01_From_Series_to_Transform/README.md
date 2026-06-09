# Chapter 01: From Series to Transform

The Fourier series handles periodic functions by decomposing them into discrete frequencies. The Fourier transform handles non-periodic functions by allowing a continuous distribution of frequencies. This chapter bridges the two ideas by making precise the heuristic argument that increasing the period to infinity converts the sum into an integral.

## Chapter Overview

**Section 01: The Fourier Integral Theorem** motivates and states the fundamental representation formula for non-periodic functions. Starting from the complex Fourier series of a $2L$-periodic function and passing $L \to \infty$ heuristically, we arrive at the pair of formulas:
$$\hat{f}(\xi) = \int_{-\infty}^\infty f(x)e^{-2\pi i\xi x}\,dx, \qquad f(x) = \int_{-\infty}^\infty \hat{f}(\xi)e^{2\pi i\xi x}\,d\xi.$$
The theorem asserts that these formulas hold for functions in $L^1(\mathbb{R})$ that are piecewise smooth, with the inversion formula converging pointwise (to the midpoint of jumps, just as Dirichlet's theorem predicts for Fourier series).

**Section 02: Definition of the Fourier Transform** makes the definition precise, discusses the normalization conventions (the $2\pi$ can appear in the exponent, in a prefactor, or split between the two formulas), and computes the transform of fundamental examples. The Gaussian $f(x) = e^{-\pi x^2}$ is self-similar under the Fourier transform: $\hat{f}(\xi) = e^{-\pi\xi^2}$. The rectangular pulse gives a sinc function. The decaying exponential $e^{-a|x|}$ gives a Lorentzian $2a/(a^2 + 4\pi^2\xi^2)$.

## Key Ideas

The passage from series to transform is not just a formal limit; it represents a genuine conceptual transition. In the Fourier series, the "spectrum" of a periodic function is a sequence of numbers $\{c_n\}_{n \in \mathbb{Z}}$. In the Fourier transform, the "spectrum" of a non-periodic function is a function $\hat{f}(\xi)$ of a continuous variable. The series representation is exact and discrete; the transform representation is continuous and corresponds to an integral rather than a sum.

The symmetry between $f$ and $\hat{f}$ — the forward transform integrates $f(x)e^{-2\pi i\xi x}$ and the inverse integrates $\hat{f}(\xi)e^{2\pi i\xi x}$ — is one of the deep structural features of the Fourier transform. This symmetry is made precise by Plancherel's theorem (Chapter 03), which says the transform is an isometry on $L^2(\mathbb{R})$.
