# Chapter 03: Inversion and Distributions

The Fourier integral theorem guarantees inversion under conditions ($f \in L^1$, piecewise smooth) that are somewhat restrictive. Many important functions do not satisfy these conditions: the Dirac delta is not a function at all in the classical sense; the constant function $f(x) = 1$ is not in $L^1$; and the Heaviside step function $H(x)$ has a non-integrable transform. Yet these objects have meaningful Fourier transforms, and the theory would be impoverished without them.

This chapter addresses inversion more carefully and extends the Fourier transform to a larger class of mathematical objects.

## Chapter Overview

**Section 01: Inverse Fourier Transform** revisits the inversion problem systematically. For $f \in L^1(\mathbb{R})$ with $\hat{f} \in L^1(\mathbb{R})$ (a stronger condition), the inversion formula holds everywhere. The chapter examines what happens when $\hat{f} \notin L^1$ (as with the sinc function) and introduces principal value integrals and convergence in mean.

**Section 02: Plancherel's Theorem** is the central result of $L^2$ Fourier analysis. For $f \in L^1 \cap L^2$, the Fourier transform satisfies $\|\hat{f}\|_2 = \|f\|_2$ (isometry). By density of $L^1 \cap L^2$ in $L^2$, the Fourier transform extends uniquely to an isometric isomorphism on all of $L^2(\mathbb{R})$. The Parseval identity $\int |\hat{f}|^2 = \int |f|^2$ and the more general $\int \hat{f}\overline{\hat{g}} = \int f\bar{g}$ hold. This makes the Fourier transform not just an analytic tool but a unitary operator on the Hilbert space $L^2$.

**Section 03: Tempered Distributions** provides the framework for Fourier transforms of objects that are neither in $L^1$ nor in $L^2$. A tempered distribution is a continuous linear functional on the Schwartz space $\mathcal{S}(\mathbb{R})$. The Fourier transform extends to tempered distributions by duality: $\langle\hat{T}, \phi\rangle = \langle T, \hat{\phi}\rangle$ for test functions $\phi \in \mathcal{S}$. The Dirac delta $\delta$ has Fourier transform $\hat{\delta} = 1$ (the constant function), and conversely $\hat{1} = \delta$. Polynomials, exponentials $e^{2\pi i\nu_0 x}$, and trigonometric functions all have well-defined distributional Fourier transforms as sums of delta functions.

## Key Themes

The progression from $L^1$ to $L^2$ to distributions reflects increasing generality at the cost of increasing abstraction. The $L^1$ theory is the most elementary but least symmetric: the transform of an $L^1$ function need not be in $L^1$. The $L^2$ theory is symmetric and elegant but loses pointwise interpretation. The distributional theory is the most general and powerful, handling objects that appear throughout mathematics and physics, but requires the framework of topological vector spaces and duality.

For applications to ODEs and PDEs, the distributional framework is essential: Green's functions are often distributions (the Green's function for $-d^2/dx^2$ involves Heaviside functions), and source terms like point masses or instantaneous impulses are modeled by delta functions.
