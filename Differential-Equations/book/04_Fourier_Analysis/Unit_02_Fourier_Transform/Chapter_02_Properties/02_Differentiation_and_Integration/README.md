# Differentiation and Integration

The most important property of the Fourier transform for differential equations is its behavior under differentiation. Differentiation in the time domain corresponds to multiplication by a frequency variable in the Fourier domain. This converts differential equations into algebraic ones, which can be solved by elementary methods, with the solution then recovered via the inverse transform.

## The Differentiation Property

**Theorem.** Let $f \in L^1(\mathbb{R})$ be differentiable with $f' \in L^1(\mathbb{R})$. Then
$$\mathcal{F}[f'](\xi) = 2\pi i\xi\,\hat{f}(\xi).$$

**Proof.** Integration by parts:
$$\int_{-\infty}^\infty f'(x)e^{-2\pi i\xi x}\,dx = \left[f(x)e^{-2\pi i\xi x}\right]_{-\infty}^\infty + 2\pi i\xi\int_{-\infty}^\infty f(x)e^{-2\pi i\xi x}\,dx.$$
The boundary term $[f(x)e^{-2\pi i\xi x}]_{-\infty}^\infty = 0$ because $f \in L^1(\mathbb{R})$ implies $f(x) \to 0$ as $|x| \to \infty$. The remaining integral is $2\pi i\xi\,\hat{f}(\xi)$.

**Iteration.** If $f$ is $n$ times differentiable with $f^{(k)} \in L^1$ for $0 \leq k \leq n$, then
$$\mathcal{F}[f^{(n)}](\xi) = (2\pi i\xi)^n\hat{f}(\xi).$$

**Interpretation.** Differentiation amplifies high frequencies. The factor $|2\pi i\xi| = 2\pi|\xi|$ grows with $|\xi|$, so differentiating a function magnifies its high-frequency components. Conversely, if $\hat{f}$ decays rapidly at high frequencies, the function is smooth (all its Fourier derivatives are integrable).

## The Frequency-Domain Differentiation Property

**Theorem.** For $f \in L^1(\mathbb{R})$ such that $xf(x) \in L^1(\mathbb{R})$:
$$\mathcal{F}[(-2\pi ix)f(x)](\xi) = \frac{d}{d\xi}\hat{f}(\xi).$$
Or equivalently: $\mathcal{F}[x^n f(x)](\xi) = \frac{1}{(-2\pi i)^n}\frac{d^n}{d\xi^n}\hat{f}(\xi)$.

**Proof.** Differentiate under the integral sign:
$$\frac{d}{d\xi}\hat{f}(\xi) = \frac{d}{d\xi}\int_{-\infty}^\infty f(x)e^{-2\pi i\xi x}\,dx = \int_{-\infty}^\infty f(x)(-2\pi ix)e^{-2\pi i\xi x}\,dx = \mathcal{F}[(-2\pi ix)f(x)](\xi).$$
The differentiation under the integral is justified by the dominated convergence theorem when $xf(x) \in L^1$.

## Application: Solving the Heat Equation

The **heat equation** on $\mathbb{R}$: find $u(x,t)$ satisfying $u_t = \alpha^2 u_{xx}$, $u(x,0) = f(x)$.

Take the Fourier transform in $x$ (treating $t$ as a parameter). By the differentiation property:
$$\frac{\partial}{\partial t}\hat{u}(\xi,t) = \alpha^2(2\pi i\xi)^2\hat{u}(\xi,t) = -4\pi^2\alpha^2\xi^2\hat{u}(\xi,t).$$
This is a first-order ODE in $t$ (for fixed $\xi$): $\hat{u}_t = -4\pi^2\alpha^2\xi^2\hat{u}$.

Solving: $\hat{u}(\xi,t) = \hat{u}(\xi,0)e^{-4\pi^2\alpha^2\xi^2 t} = \hat{f}(\xi)\,e^{-4\pi^2\alpha^2\xi^2 t}$.

The solution $u(x,t)$ is recovered by the inverse transform:
$$u(x,t) = \mathcal{F}^{-1}\!\left[\hat{f}(\xi)e^{-4\pi^2\alpha^2\xi^2 t}\right](x).$$
By the convolution theorem (Section 03), $u = f * G_t$ where $G_t$ is the **heat kernel**:
$$G_t(x) = \mathcal{F}^{-1}[e^{-4\pi^2\alpha^2\xi^2 t}](x) = \frac{1}{2\alpha\sqrt{\pi t}}e^{-x^2/(4\alpha^2 t)}.$$
This is a Gaussian of width $2\alpha\sqrt{t}$, spreading as $\sqrt{t}$.

## Application: Solving an ODE with Constant Coefficients

Consider $ay'' + by' + cy = f(x)$ on $\mathbb{R}$, where $f \in L^1(\mathbb{R})$.

Applying the Fourier transform:
$$a(2\pi i\xi)^2\hat{y} + b(2\pi i\xi)\hat{y} + c\hat{y} = \hat{f}(\xi),$$
$$[c + 2\pi ib\xi - 4\pi^2 a\xi^2]\hat{y}(\xi) = \hat{f}(\xi),$$
$$\hat{y}(\xi) = \frac{\hat{f}(\xi)}{P(2\pi i\xi)},$$
where $P(s) = as^2 + bs + c$ is the characteristic polynomial. The solution is $y = \mathcal{F}^{-1}[\hat{f}/P(2\pi i\xi)]$. If $P(2\pi i\xi)$ has no real zeros (no resonance), the solution is $y = f * h$ where $\hat{h} = 1/P(2\pi i\xi)$ is the transfer function.

## Integration Property

**Theorem.** Let $f \in L^1(\mathbb{R})$ with $\hat{f}(0) = \int_{-\infty}^\infty f(x)\,dx = 0$. Define $F(x) = \int_{-\infty}^x f(t)\,dt$. Then $F \in L^1(\mathbb{R})$ and
$$\hat{F}(\xi) = \frac{\hat{f}(\xi)}{2\pi i\xi}.$$

The condition $\hat{f}(0) = 0$ ensures that $F(x) \to 0$ as $x \to \pm\infty$ (if $\int f = 0$, the running integral returns to zero), so $F$ decays at infinity and is integrable.

**Interpretation.** Integration divides the Fourier transform by $2\pi i\xi$, enhancing low frequencies and smoothing out high-frequency variations. This is the frequency-domain counterpart of the fact that integrating a function makes it smoother.

## The Transfer Function and LTI Systems

A **linear, time-invariant (LTI) system** with input $u$ and output $y = h * u$ (where $h$ is the impulse response) has the property
$$\hat{y}(\xi) = \hat{h}(\xi)\cdot\hat{u}(\xi) =: H(\xi)\hat{u}(\xi),$$
where $H = \hat{h}$ is the **transfer function**. The transfer function characterizes the system completely:
- $|H(\xi)|$ is the **gain** at frequency $\xi$: how much the system amplifies or attenuates each frequency.
- $\arg(H(\xi))$ is the **phase shift** at frequency $\xi$: how much the system delays each frequency.

For the operator $L = a\frac{d^2}{dx^2} + b\frac{d}{dx} + c$, the transfer function is $H(\xi) = a(2\pi i\xi)^2 + b(2\pi i\xi) + c = P(2\pi i\xi)$. The equation $Lu = f$ becomes $H(\xi)\hat{u} = \hat{f}$ in frequency space — a pointwise algebraic equation.

## Worked Example: Decay Rate from Smoothness

Suppose $f$ is three times differentiable and $f, f', f'', f''' \in L^1(\mathbb{R})$. Then:
$$\mathcal{F}[f'''](\xi) = (2\pi i\xi)^3\hat{f}(\xi).$$
Since $f''' \in L^1$, its Fourier transform is bounded: $|\hat{f}'''(\xi)| \leq \|f'''\|_1$. Therefore
$$|(2\pi i\xi)^3\hat{f}(\xi)| \leq \|f'''\|_1 \implies |\hat{f}(\xi)| \leq \frac{\|f'''\|_1}{(2\pi)^3|\xi|^3}.$$
So $\hat{f}(\xi) = O(|\xi|^{-3})$ for large $|\xi|$. More generally, $k$ derivatives in $L^1$ implies $|\hat{f}(\xi)| = O(|\xi|^{-k})$. This confirms the heuristic: smoother functions have more rapidly decaying transforms.
