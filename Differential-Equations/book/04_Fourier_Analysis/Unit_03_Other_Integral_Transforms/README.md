# Unit 03: Other Integral Transforms

The Fourier transform is the natural integral transform associated with the additive group structure of the real line: the complex exponentials $e^{i\xi x}$ are the characters (multiplicative homomorphisms) of $(\mathbb{R}, +)$, and the Fourier transform decomposes functions into components corresponding to these characters. But the real line has other structures, other geometries generate other differential operators, and different physical problems are best analyzed by transforms adapted to their own symmetry.

This unit surveys the principal integral transforms beyond the Fourier transform, emphasizing their relationships to one another and to the Fourier transform itself, and demonstrating their applications.

## Chapter Overview

**Chapter 01: Laplace as an Integral Transform** revisits the Laplace transform from the perspective of this unit. The one-sided Laplace transform $\mathcal{L}[f](s) = \int_0^\infty f(t)e^{-st}\,dt$ was introduced earlier in this course as a tool for solving initial value problems. Here we see it as a specialization of the Fourier transform: replacing $\xi$ by $-is/(2\pi)$ in the Fourier transform of $f(t)e^{-\sigma t}$ (for appropriate $\sigma$) gives the Laplace transform. The bilateral Laplace transform $\int_{-\infty}^\infty f(t)e^{-st}\,dt$ is the two-sided version, whose inversion formula involves integration along a vertical line in the complex $s$-plane.

**Chapter 02: Mellin Transform** introduces the transform $\mathcal{M}[f](s) = \int_0^\infty f(x)x^{s-1}\,dx$, which is the natural transform associated with the multiplicative group $(\mathbb{R}^+, \times)$. A simple substitution $x = e^t$ converts the Mellin transform to a two-sided Laplace transform. The Mellin transform appears in analytic number theory (the Dirichlet series and the Riemann zeta function are closely related to Mellin transforms of arithmetic functions), in the theory of special functions, and in the computation of definite integrals.

**Chapter 03: Hankel Transform** is the transform appropriate for radially symmetric functions in two or three dimensions. When a PDE has cylindrical symmetry, expanding in Bessel functions plays the role that expanding in trigonometric functions plays in the Cartesian case. The Hankel transform $\mathcal{H}_\nu[f](\rho) = \int_0^\infty f(r)J_\nu(\rho r)r\,dr$ uses the Bessel functions $J_\nu$ as its kernel. The 2D Fourier transform of a radially symmetric function is a Hankel transform, establishing the connection.

**Chapter 04: Z-Transform** is the discrete-time analog of the Laplace transform, defined by $\mathcal{Z}[x](z) = \sum_{n=0}^\infty x_n z^{-n}$ for a sequence $(x_n)$. Just as the Laplace transform converts constant-coefficient ODEs to algebraic equations, the Z-transform converts linear recurrence relations (difference equations) to algebraic equations in $z$. The DFT is the restriction of the Z-transform to the unit circle $|z| = 1$.

## Unifying Theme

The transforms in this unit are all special cases of a general pattern: an integral transform $T[f](s) = \int_X f(x) K(x, s)\,d\mu(x)$ converts a function on one space to a function on another, using a kernel $K$ and a measure $\mu$. The choice of kernel and measure is dictated by the symmetry of the problem:

| Symmetry | Transform | Kernel |
|---|---|---|
| Translation (additive) | Fourier | $e^{-2\pi i\xi x}$ |
| Scaling (multiplicative) | Mellin | $x^{s-1}$ |
| Causal/exponential growth | Laplace | $e^{-st}$ |
| Cylindrical | Hankel | $J_\nu(\rho r)$ |
| Discrete shifts | Z-transform | $z^{-n}$ |

Understanding this unifying structure allows the techniques of one transform to inform the others.
