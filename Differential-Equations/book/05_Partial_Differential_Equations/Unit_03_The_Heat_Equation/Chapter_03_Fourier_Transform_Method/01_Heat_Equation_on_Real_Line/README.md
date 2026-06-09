# The Heat Equation on the Real Line

The Cauchy problem for the heat equation on all of $\mathbb{R}$ is:

$$u_t = \kappa u_{xx}, \qquad x \in \mathbb{R},\; t > 0,$$
$$u(x,0) = f(x), \qquad x \in \mathbb{R}.$$

Without boundary conditions at $\pm\infty$, the problem requires a different approach from separation of variables on a bounded interval. The Fourier transform is the natural tool: it replaces the spatial variable with a frequency variable, converting the PDE into a family of ODEs parametrized by frequency, each of which is solved explicitly.

## Applying the Fourier Transform

Take the Fourier transform of both sides of the heat equation in the $x$-variable (treating $t$ as a parameter):

$$\frac{\partial}{\partial t}\hat{u}(\xi,t) = \kappa\,\widehat{u_{xx}}(\xi,t) = \kappa(i\xi)^2\hat{u}(\xi,t) = -\kappa\xi^2\hat{u}(\xi,t).$$

This is an ODE in $t$ for each fixed $\xi$:

$$\frac{d\hat{u}}{dt} = -\kappa\xi^2\hat{u}, \qquad \hat{u}(\xi,0) = \hat{f}(\xi).$$

The solution is:

$$\hat{u}(\xi,t) = \hat{f}(\xi)\,e^{-\kappa\xi^2 t}.$$

## Inverting the Transform

Taking the inverse Fourier transform:

$$u(x,t) = \frac{1}{2\pi}\int_{-\infty}^\infty\hat{f}(\xi)e^{-\kappa\xi^2 t}e^{i\xi x}\,d\xi.$$

Since $e^{-\kappa\xi^2 t}$ is itself the Fourier transform of a Gaussian (this is the key computation), we can write $\hat{u}(\xi,t) = \hat{f}(\xi)\cdot\widehat{K}(\xi,t)$, where $\widehat{K}(\xi,t) = e^{-\kappa\xi^2 t}$. By the convolution theorem:

$$u(x,t) = (f * K)(x,t) = \int_{-\infty}^\infty f(y)K(x-y,t)\,dy,$$

where $K$ is the inverse Fourier transform of $e^{-\kappa\xi^2 t}$:

$$K(x,t) = \frac{1}{2\pi}\int_{-\infty}^\infty e^{-\kappa\xi^2 t}e^{i\xi x}\,d\xi.$$

## Computing the Heat Kernel

The key integral is $\int_{-\infty}^\infty e^{-\kappa\xi^2 t + i\xi x}\,d\xi$. Complete the square in the exponent:

$$-\kappa\xi^2 t + i\xi x = -\kappa t\left(\xi - \frac{ix}{2\kappa t}\right)^2 - \frac{x^2}{4\kappa t}.$$

Substitute $\eta = \xi - ix/(2\kappa t)$ (shift contour, justified by Cauchy's theorem since $e^{-\kappa t\eta^2}$ decays rapidly):

$$\int_{-\infty}^\infty e^{-\kappa\xi^2 t + i\xi x}\,d\xi = e^{-x^2/(4\kappa t)}\int_{-\infty}^\infty e^{-\kappa t\eta^2}\,d\eta = e^{-x^2/(4\kappa t)}\sqrt{\frac{\pi}{\kappa t}}.$$

Therefore:

$$K(x,t) = \frac{1}{2\pi}\sqrt{\frac{\pi}{\kappa t}}\,e^{-x^2/(4\kappa t)} = \frac{1}{\sqrt{4\pi\kappa t}}\,e^{-x^2/(4\kappa t)}. \tag{1}$$

This is the **heat kernel** (or fundamental solution, or Gaussian kernel).

## The Solution Formula

The solution to the Cauchy problem for the heat equation on $\mathbb{R}$ is:

$$\boxed{u(x,t) = \frac{1}{\sqrt{4\pi\kappa t}}\int_{-\infty}^\infty f(y)\,e^{-(x-y)^2/(4\kappa t)}\,dy.} \tag{2}$$

**Interpretation:** $u(x,t)$ is the weighted average of the initial data $f$, with Gaussian weights centered at $x$ and width $\sqrt{2\kappa t}$. As $t$ increases, the Gaussian widens and averages over more of the initial data, smoothing out local features.

## Properties of the Solution

**Existence.** For $f \in L^\infty(\mathbb{R})$ (or even for $f$ with polynomial growth $|f(y)| \leq Ce^{ay^2}$ for some $a < 1/(4\kappa T)$ for $0 < t < T$), the integral (2) converges and defines a smooth function for $t > 0$.

**Infinite propagation speed.** For any $t > 0$ and any point $x$, the integral (2) depends on $f(y)$ for all $y$ — the support of $K(x-y, t)$ (as a function of $y$) is all of $\mathbb{R}$. This means that a compactly supported initial datum instantly affects $u$ at every point in space. Contrast with the wave equation, where signals propagate at finite speed $c$.

**Positivity.** If $f \geq 0$, then $u(x,t) \geq 0$ for all $x$ and $t > 0$. In fact, if $f \geq 0$ and $f \not\equiv 0$, then $u(x,t) > 0$ for all $x \in \mathbb{R}$ and $t > 0$ — the solution is strictly positive everywhere, even if $f$ vanishes on a large set.

**Mass conservation.** $\int_{-\infty}^\infty u(x,t)\,dx = \int_{-\infty}^\infty f(x)\,dx$, because $\int K(x,t)\,dx = 1$ (the heat kernel is a probability density integrating to $1$).

**Smoothing.** For any $t > 0$, the solution $u(\cdot,t)$ is infinitely differentiable, regardless of the regularity of $f$. Differentiating under the integral is justified by the rapid decay of $e^{-(x-y)^2/(4\kappa t)}$.

## Connection to the Error Function

For the specific initial data $f(x) = \mathbf{1}_{x > 0}$ (step function), the solution is:

$$u(x,t) = \frac{1}{\sqrt{4\pi\kappa t}}\int_0^\infty e^{-(x-y)^2/(4\kappa t)}\,dy = \frac{1}{2}\mathrm{erfc}\!\left(\frac{-x}{\sqrt{4\kappa t}}\right) = \frac{1}{2}\left(1 + \mathrm{erf}\!\left(\frac{x}{\sqrt{4\kappa t}}\right)\right),$$

where $\mathrm{erf}(z) = \frac{2}{\sqrt{\pi}}\int_0^z e^{-s^2}\,ds$ is the error function. This solution represents the diffusion of a sharp interface initially at $x=0$ and is important in metallurgy (the diffusion of alloy components) and semiconductor physics.

## Well-Posedness

For $f \in L^2(\mathbb{R})$, the solution formula (2) satisfies:

$$\|u(\cdot,t)\|_{L^2(\mathbb{R})} \leq \|f\|_{L^2(\mathbb{R})} \quad \text{for all } t > 0.$$

This follows from Young's convolution inequality: $\|f*g\|_{L^2} \leq \|f\|_{L^2}\|g\|_{L^1}$ and $\|K(\cdot,t)\|_{L^1} = 1$. The solution map $f \mapsto u(\cdot,t)$ is a bounded operator on $L^2(\mathbb{R})$ for each $t > 0$, with operator norm $\leq 1$. This is well-posedness in $L^2$.
