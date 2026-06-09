# The Convolution Theorem

Of all the properties of the Fourier transform, the convolution theorem is arguably the most practically powerful. It converts the computationally expensive operation of convolution — an integral — into a simple pointwise product in the frequency domain. This is the theoretical basis for fast convolution algorithms, Fourier methods for solving PDEs, and the analysis of linear systems.

## Convolution: Definition and Motivation

**Definition.** The **convolution** of $f, g \in L^1(\mathbb{R})$ is the function
$$(f * g)(x) = \int_{-\infty}^\infty f(t)\,g(x - t)\,dt.$$

This integral can be thought of as a weighted average of $f$, with weights given by $g$ translated and reflected. Convolution arises naturally in:

- **Signal processing:** if $h$ is the impulse response of an LTI system and $u$ is the input, the output is $y = h * u$.
- **Probability:** if $X$ and $Y$ are independent random variables with densities $f$ and $g$, the density of $X + Y$ is $f * g$.
- **PDEs:** the heat equation solution $u(x,t) = (G_t * f)(x)$ is a convolution with the heat kernel.
- **Smoothing:** convolving a rough function with a smooth "kernel" $g$ produces a smoother function.

## The Convolution Theorem

**Theorem.** For $f, g \in L^1(\mathbb{R})$:
$$\mathcal{F}[f * g](\xi) = \hat{f}(\xi)\cdot\hat{g}(\xi).$$
Convolution in the time domain corresponds to multiplication in the frequency domain.

**Proof.** By Fubini's theorem (justified since $f, g \in L^1$ implies $f * g \in L^1$ with $\|f*g\|_1 \leq \|f\|_1\|g\|_1$):
$$\mathcal{F}[f*g](\xi) = \int_{-\infty}^\infty \left(\int_{-\infty}^\infty f(t)g(x-t)\,dt\right)e^{-2\pi i\xi x}\,dx$$
$$= \int_{-\infty}^\infty f(t)\left(\int_{-\infty}^\infty g(x-t)e^{-2\pi i\xi x}\,dx\right)dt.$$
Substitute $u = x - t$ in the inner integral:
$$= \int_{-\infty}^\infty f(t)\left(\int_{-\infty}^\infty g(u)e^{-2\pi i\xi(u+t)}\,du\right)dt = \int_{-\infty}^\infty f(t)e^{-2\pi i\xi t}\,dt\cdot\int_{-\infty}^\infty g(u)e^{-2\pi i\xi u}\,du = \hat{f}(\xi)\cdot\hat{g}(\xi).$$

**Corollary.** The inverse statement also holds: multiplication in the time domain corresponds to convolution in the frequency domain:
$$\mathcal{F}[f\cdot g](\xi) = (\hat{f} * \hat{g})(\xi).$$

## The Heat Kernel via Convolution

The solution to the heat equation $u_t = \alpha^2 u_{xx}$ with initial condition $u(x,0) = f(x)$ is, as derived in Section 02:
$$\hat{u}(\xi,t) = \hat{f}(\xi)\cdot e^{-4\pi^2\alpha^2\xi^2 t}.$$
By the convolution theorem, $u(x,t) = (f * G_t)(x)$ where $G_t$ is the heat kernel:
$$G_t(x) = \mathcal{F}^{-1}[e^{-4\pi^2\alpha^2\xi^2 t}](x) = \frac{1}{2\alpha\sqrt{\pi t}}e^{-x^2/(4\alpha^2 t)}.$$
This uses the Gaussian transform formula: $\mathcal{F}[e^{-\pi\alpha x^2}] = \alpha^{-1/2}e^{-\pi\xi^2/\alpha}$, with $\alpha = 4\pi\alpha^2 t$, giving $G_t(\xi) = e^{-\pi\xi^2/(4\pi\alpha^2 t)} = e^{-\xi^2/(4\alpha^2 t)}$... let me be more careful:

For the convention $\mathcal{F}[e^{-\pi x^2}] = e^{-\pi\xi^2}$, rescaling gives $\mathcal{F}[e^{-ax^2}] = (\pi/a)^{1/2}e^{-\pi^2\xi^2/a}$ (after adjusting to the $2\pi$ convention). The key result is that the inverse transform of $e^{-4\pi^2\alpha^2\xi^2 t}$ is a Gaussian in $x$ with variance proportional to $\alpha^2 t$, spreading as $\sqrt{t}$.

## Properties of Convolution

Convolution is:
- **Commutative:** $f * g = g * f$ (change variables $u = x - t$).
- **Associative:** $(f * g) * h = f * (g * h)$ (Fubini).
- **Distributive over addition:** $f * (g + h) = f*g + f*h$.
- **Identity element:** the Dirac delta acts as the identity: $f * \delta = f$ (proved rigorously in the distributional setting of Chapter 03).
- **Differentiable:** $(f * g)' = f' * g = f * g'$, provided the derivatives exist.

## Worked Example: Characteristic Function of a Sum

The **characteristic function** $\phi_X(\xi) = \mathbb{E}[e^{2\pi i\xi X}]$ of a random variable $X$ with density $f_X$ is $\phi_X(\xi) = \hat{f}_X(-\xi)$ (essentially the Fourier transform of the density). If $X$ and $Y$ are independent, the density of $Z = X + Y$ is $f_Z = f_X * f_Y$. By the convolution theorem:
$$\hat{f}_Z(\xi) = \hat{f}_X(\xi)\cdot\hat{f}_Y(\xi),$$
so $\phi_Z = \phi_X \cdot \phi_Y$. This multiplicative property of characteristic functions is the standard tool for proving central limit theorems and computing distributions of sums.

**Concrete example.** Let $X_1, \ldots, X_n$ be i.i.d. uniform on $[-1/2, 1/2]$, so $f_{X_k}(x) = \mathbf{1}_{[-1/2,1/2]}(x)$ and $\hat{f}_{X_k}(\xi) = \text{sinc}(\xi)$. The density of $S_n = X_1 + \cdots + X_n$ has transform $\hat{f}_{S_n}(\xi) = \text{sinc}^n(\xi)$. As $n \to \infty$, $\text{sinc}^n(\xi)$ concentrates near $\xi = 0$ and the density $f_{S_n}$ approaches a Gaussian — the central limit theorem.

## Autocorrelation and Power Spectral Density

The **autocorrelation** of $f$ is $R_f(t) = (f * f^*)(t)$ where $f^*(x) = \overline{f(-x)}$. By the convolution theorem and the transform property $\mathcal{F}[f^*](\xi) = \overline{\hat{f}(\xi)}$:
$$\hat{R}_f(\xi) = \hat{f}(\xi)\cdot\overline{\hat{f}(\xi)} = |\hat{f}(\xi)|^2.$$
This is the **Wiener-Khinchin theorem**: the Fourier transform of the autocorrelation equals the power spectral density $|\hat{f}|^2$. It is the foundation of spectral estimation from time-series data.

## The Deconvolution Problem

Given the output $y = h * u$ of a system and the system's impulse response $h$, recovering the input $u$ is **deconvolution**. In frequency space, $\hat{y} = \hat{h}\cdot\hat{u}$, so $\hat{u} = \hat{y}/\hat{h}$ — simple division. The difficulty is that $\hat{h}$ may be zero or very small at some frequencies, making $\hat{y}/\hat{h}$ numerically unstable or infinite. This is the **ill-posedness** of deconvolution, addressed in practice by regularization (e.g., Tikhonov regularization, which adds a small constant to the denominator: $\hat{u} \approx \hat{y}\overline{\hat{h}}/(|\hat{h}|^2 + \epsilon)$).
