# Unit 02: The Fourier Transform

Fourier series decompose periodic functions into discrete frequency components: a function of period $2\pi$ contributes only to frequencies $n \in \mathbb{Z}$. But most physically interesting functions — a Gaussian pulse, a decaying exponential, an isolated wave packet — are not periodic. They are defined on all of $\mathbb{R}$ and possess a continuous spectrum rather than a discrete one. The Fourier transform is the extension of Fourier series to this non-periodic setting.

The transition from series to transform is conceptually natural. Take a function $f$ of period $2L$ and let $L \to \infty$. The discrete frequencies $n\pi/L$ accumulate to fill the real line $\mathbb{R}$, the Fourier coefficients $c_n$ become a function $\hat{f}(\xi)$ of a continuous frequency variable $\xi$, and the sum $\sum c_n e^{in\pi x/L}$ becomes an integral $\int \hat{f}(\xi) e^{i\xi x}\,d\xi$. Making this limit rigorous leads to the definition of the Fourier transform and the Fourier inversion formula.

## Chapter Overview

**Chapter 01: From Series to Transform** carries out the limit from Fourier series to the Fourier transform carefully. The **Fourier integral theorem** asserts that a sufficiently well-behaved function on $\mathbb{R}$ can be represented as an integral of complex exponentials:
$$f(x) = \int_{-\infty}^\infty \hat{f}(\xi) e^{2\pi i \xi x}\,d\xi, \quad \hat{f}(\xi) = \int_{-\infty}^\infty f(x)e^{-2\pi i \xi x}\,dx.$$
We then define the Fourier transform $\mathcal{F}$ formally and compute examples: Gaussians, rectangular pulses, decaying exponentials, and the Dirac delta.

**Chapter 02: Properties** develops the algebraic toolkit for working with the Fourier transform. The fundamental properties — linearity, shifting in time and frequency, scaling, differentiation in both domains, and the convolution theorem — make the Fourier transform a powerful tool for analyzing linear systems. The differentiation property $\widehat{f'}(\xi) = 2\pi i\xi\hat{f}(\xi)$ converts differential equations into algebraic equations in frequency space.

**Chapter 03: Inversion and Distributions** addresses the precise conditions under which the inversion formula $f = \mathcal{F}^{-1}[\hat{f}]$ holds, and what happens when $f$ is not in $L^1$ or $L^2$. The **Plancherel theorem** extends the Fourier transform to $L^2(\mathbb{R})$ as an isometric isomorphism, and Parseval's identity holds in $L^2$. **Tempered distributions** extend the theory further to objects like the Dirac delta, the Heaviside step function, and even constants and polynomials, which lie outside $L^1 \cup L^2$ but admit perfectly well-defined Fourier transforms in the distributional sense.

**Chapter 04: Discrete Fourier Transform** brings the theory back to the computational setting. When a signal is sampled at a finite number of equally spaced points, the natural analog is the **Discrete Fourier Transform (DFT)**, which maps a vector of $N$ samples to a vector of $N$ complex amplitudes. The **Fast Fourier Transform (FFT)** computes the DFT in $O(N\log N)$ operations rather than the naive $O(N^2)$, making Fourier analysis practical for large datasets. Applications to signal processing, spectral estimation, and efficient convolution are discussed.

## Prerequisites and Connections

This unit builds directly on Unit 01. The complex exponential basis and complex Fourier coefficients from Chapter 04 of Unit 01 are the immediate precursors. Comfort with improper integrals, basic complex analysis, and the $L^2$ inner product is assumed. The material connects forward to Unit 03 (where other integral transforms are understood via the Fourier framework) and to the study of PDEs on $\mathbb{R}^n$ (Unit 05).
