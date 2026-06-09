# Chapter 02: Properties of the Fourier Transform

The power of the Fourier transform as a tool lies not in computing individual transforms but in its systematic algebraic properties. These properties form a dictionary that allows one to compute the transform of a complicated function from the transform of a simpler related function, and to convert operations in the time domain (differentiation, convolution, shifting) into simpler operations in the frequency domain.

## Chapter Overview

**Section 01: Linearity, Shifting, and Scaling** covers the three most fundamental properties. Linearity is immediate from linearity of the integral. The **time-shifting property** says $\mathcal{F}[f(x - a)](\xi) = e^{-2\pi i a\xi}\hat{f}(\xi)$: shifting a function in time multiplies its transform by a complex exponential (phase shift) in frequency, with no change in amplitude. The **frequency-shifting property** (modulation) says $\mathcal{F}[e^{2\pi i\nu_0 x}f(x)](\xi) = \hat{f}(\xi - \nu_0)$: multiplying by a complex exponential shifts the spectrum. The **scaling property** says $\mathcal{F}[f(ax)](\xi) = \frac{1}{|a|}\hat{f}(\xi/a)$: compressing a function in time expands it in frequency, and vice versa.

**Section 02: Differentiation and Integration** are where the Fourier transform becomes a tool for solving differential equations. The key identity $\mathcal{F}[f'](\xi) = 2\pi i\xi\,\hat{f}(\xi)$ says that differentiation in the time domain becomes multiplication by $2\pi i\xi$ in the frequency domain. More generally, $\mathcal{F}[f^{(n)}](\xi) = (2\pi i\xi)^n\hat{f}(\xi)$. This converts an ODE or PDE into an algebraic equation in frequency space. The integration property is its counterpart: $\mathcal{F}\left[\int_{-\infty}^x f(t)\,dt\right](\xi) = \frac{1}{2\pi i\xi}\hat{f}(\xi)$ (under appropriate conditions).

**Section 03: The Convolution Theorem** is arguably the most important property. The convolution of $f$ and $g$ is $(f * g)(x) = \int_{-\infty}^\infty f(t)g(x-t)\,dt$. The convolution theorem states $\mathcal{F}[f * g] = \hat{f}\cdot\hat{g}$: convolution in the time domain becomes pointwise multiplication in the frequency domain. This transforms the computationally expensive convolution integral into a simple product, which is the principle behind FFT-based fast convolution.

## Significance

These properties make the Fourier transform a nearly complete algebraic toolkit for linear analysis. Every linear, time-invariant (LTI) system can be characterized by its **transfer function** $H(\xi) = \hat{h}(\xi)$, where $h$ is the impulse response. The output $y = h * u$ of the system to input $u$ has Fourier transform $\hat{y} = H\cdot\hat{u}$, which is simply a product. The system's behavior at each frequency is multiplicative and independent.

In PDE, the same principle applies: the heat equation $u_t = \alpha^2 u_{xx}$ transforms to $\hat{u}_t = -4\pi^2\alpha^2\xi^2\hat{u}$ in frequency space, which is an ODE in $t$ for each fixed $\xi$, with solution $\hat{u}(\xi, t) = \hat{u}(\xi, 0)e^{-4\pi^2\alpha^2\xi^2 t}$.
