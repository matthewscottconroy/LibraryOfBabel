# Chapter 04: The Complex Form of Fourier Series

The real Fourier series $\frac{a_0}{2} + \sum_{n=1}^\infty [a_n\cos(nx) + b_n\sin(nx)]$ works correctly but carries a notational awkwardness: positive and negative frequencies are treated separately, and the index $n$ runs only over non-negative integers. Reformulating the series using complex exponentials $e^{inx}$ removes this asymmetry, giving a single sum over all integers $n \in \mathbb{Z}$ with a single family of coefficients $c_n$. The result is algebraically cleaner, computationally more convenient, and the natural bridge to the Fourier transform.

## Chapter Overview

**Section 01: Complex Exponential Basis** establishes that the functions $\{e^{inx}\}_{n \in \mathbb{Z}}$ form a complete orthonormal basis for $L^2([-\pi, \pi])$ (with inner product $\frac{1}{2\pi}\int_{-\pi}^\pi f\bar{g}\,dx$). The orthogonality relation
$$\frac{1}{2\pi}\int_{-\pi}^\pi e^{imx}\overline{e^{inx}}\,dx = \frac{1}{2\pi}\int_{-\pi}^\pi e^{i(m-n)x}\,dx = \delta_{mn}$$
is a single formula covering all $m, n \in \mathbb{Z}$, replacing the three separate cases of the real theory. The completeness of this system follows from Euler's formula, which expresses each real trigonometric function as a linear combination of complex exponentials, and vice versa.

**Section 02: Complex Fourier Coefficients** derives the coefficient formula $c_n = \frac{1}{2\pi}\int_{-\pi}^\pi f(x)e^{-inx}\,dx$ and establishes its relationship to the real coefficients: $c_0 = a_0/2$, $c_n = (a_n - ib_n)/2$ for $n > 0$, and $c_{-n} = \overline{c_n}$ when $f$ is real-valued. The complex form of Parseval's identity becomes $\frac{1}{2\pi}\int_{-\pi}^\pi |f|^2\,dx = \sum_{n=-\infty}^\infty |c_n|^2$. For real $f$, the condition $c_{-n} = \overline{c_n}$ (Hermitian symmetry) means that negative and positive frequency components carry conjugate information, which is why the real form can get away with only non-negative frequencies.

## Why the Complex Form Matters

The complex form makes several things much cleaner:

1. **Algebraic manipulation**: products and compositions of exponentials are easier than products of sines and cosines. For example, $e^{imx}\cdot e^{inx} = e^{i(m+n)x}$, which immediately gives convolution identities.

2. **Frequency-domain thinking**: a single complex coefficient $c_n$ encodes both the amplitude $|c_n|$ and phase $\arg(c_n)$ of the $n$-th frequency component. In the real form, these are split between $a_n$ and $b_n$.

3. **Generalization to the Fourier transform**: the limit from period $2\pi \to \infty$ passes the discrete index $n$ to a continuous frequency variable $\xi$, and $c_n \to \hat{f}(\xi)\,d\xi$. This limit is conceptually clean in the complex form.

4. **Connection to physics**: quantum mechanics uses complex-valued wave functions, and the energy eigenstates of a particle in a periodic potential are exactly the complex Fourier modes.
