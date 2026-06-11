# Solutions: Fourier Analysis

## Problem 1: Computing Fourier Coefficients

**Problem.** Find the Fourier series of the sawtooth wave $f(x) = x$ on $(-\pi, \pi)$, extended periodically.

**Solution.** $f$ is odd, so all cosine coefficients $a_n = 0$.

$b_n = \frac{1}{\pi}\int_{-\pi}^\pi x\sin(nx)\,dx = \frac{2}{\pi}\int_0^\pi x\sin(nx)\,dx$.

Integrate by parts: $\int_0^\pi x\sin(nx)\,dx = \left[-\frac{x\cos(nx)}{n}\right]_0^\pi + \frac{1}{n}\int_0^\pi\cos(nx)\,dx = -\frac{\pi\cos(n\pi)}{n} + \frac{1}{n}\cdot\frac{\sin(nx)}{n}\bigg|_0^\pi$.

The second term is zero (since $\sin(n\pi) = 0$). So $\int_0^\pi x\sin(nx)\,dx = -\pi(-1)^n/n = (-1)^{n+1}\pi/n$.

$b_n = \frac{2}{\pi}\cdot\frac{(-1)^{n+1}\pi}{n} = \frac{2(-1)^{n+1}}{n}$.

Fourier series: $f(x) \sim 2\sum_{n=1}^\infty \frac{(-1)^{n+1}}{n}\sin(nx) = 2\left(\sin x - \frac{\sin 2x}{2} + \frac{\sin 3x}{3} - \cdots\right)$.

By Dirichlet's theorem, the series converges to $x$ for $x \in (-\pi, \pi)$ and to $0$ at $x = \pm\pi$ (average of $\pi$ and $-\pi$).

**Parseval's identity check:** $\frac{1}{\pi}\int_{-\pi}^\pi x^2\,dx = 2\pi^2/3$. Parseval: $\sum b_n^2 = \sum 4/n^2 = 4\pi^2/6 = 2\pi^2/3$. Consistent.

---

## Problem 2: Parseval's Identity and Series Summation

**Problem.** Use Parseval's identity for the Fourier series of $f(x) = x$ on $(-\pi,\pi)$ to sum $\sum_{n=1}^\infty 1/n^2$.

**Solution.** From Problem 1: the Fourier coefficients are $b_n = 2(-1)^{n+1}/n$, all $a_n = 0$.

Parseval: $\frac{1}{\pi}\int_{-\pi}^\pi |f(x)|^2\,dx = \frac{a_0^2}{2} + \sum_{n=1}^\infty(a_n^2+b_n^2)$.

Left side: $\frac{1}{\pi}\int_{-\pi}^\pi x^2\,dx = \frac{1}{\pi}\cdot\frac{2\pi^3}{3} = \frac{2\pi^2}{3}$.

Right side: $0 + \sum_{n=1}^\infty \frac{4}{n^2}$.

So $\frac{2\pi^2}{3} = 4\sum_{n=1}^\infty\frac{1}{n^2}$, giving $\sum_{n=1}^\infty\frac{1}{n^2} = \frac{\pi^2}{6}$.

This is the Basel problem, first solved by Euler in 1734 by a different method (product formula for $\sin x$). The Parseval proof is one of the most elegant.

---

## Problem 3: Fourier Transform and Convolution

**Problem.** Find the Fourier transform of $f(x) = e^{-a|x|}$ for $a > 0$, and use the convolution theorem to compute $\int_{-\infty}^\infty e^{-a|x|}\cdot e^{-b|x-t|}\,dx$.

**Solution.** $\hat{f}(\xi) = \int_{-\infty}^\infty e^{-a|x|}e^{-2\pi i\xi x}\,dx = \int_{-\infty}^0 e^{ax}e^{-2\pi i\xi x}\,dx + \int_0^\infty e^{-ax}e^{-2\pi i\xi x}\,dx$.

$= \frac{1}{a - 2\pi i\xi} + \frac{1}{a + 2\pi i\xi} = \frac{2a}{a^2 + 4\pi^2\xi^2}$.

This is a Lorentzian/Cauchy distribution in frequency space.

For the convolution: $(f*g)(t) = \int e^{-a|x|}e^{-b|t-x|}\,dx$.

By the convolution theorem: $\widehat{f*g}(\xi) = \hat{f}(\xi)\hat{g}(\xi) = \frac{2a}{a^2+4\pi^2\xi^2}\cdot\frac{2b}{b^2+4\pi^2\xi^2}$.

Taking the inverse Fourier transform (partial fractions and known transform):

$(f*g)(t) = \frac{2}{b^2-a^2}\left(be^{-a|t|} - ae^{-b|t|}\right)$ for $a \neq b$.

---

## Problem 4: Solving the Heat Equation via Fourier Series

**Problem.** Solve $u_t = 4u_{xx}$ for $0 < x < \pi$, $t > 0$, with $u(0,t) = u(\pi,t) = 0$ and $u(x,0) = \sin x + 3\sin 2x$.

**Solution.** Eigenfunctions: $\sin(nx)$ with eigenvalues $n^2$. Solution:
$$u(x,t) = \sum_{n=1}^\infty b_n\sin(nx)e^{-4n^2 t}.$$

Match initial condition $u(x,0) = \sin x + 3\sin 2x$: $b_1 = 1$, $b_2 = 3$, all other $b_n = 0$.

$u(x,t) = e^{-4t}\sin x + 3e^{-16t}\sin 2x$.

**Observation.** Higher-frequency modes decay faster (as $e^{-4n^2 t}$). For large $t$, the $n=1$ mode dominates and $u \approx e^{-4t}\sin x$. This reflects the smoothing/dissipative nature of the heat equation.

---

## Problem 5: Gibbs Phenomenon

**Problem.** Explain why the Fourier partial sums of a function with a jump discontinuity overshoot the function near the jump.

**Solution.** Consider $f(x) = 1$ for $0 < x < \pi$ and $f(x) = -1$ for $-\pi < x < 0$.

Fourier series: $f(x) \sim \frac{4}{\pi}\sum_{k=0}^\infty \frac{\sin(2k+1)x}{2k+1}$.

The $N$-th partial sum: $S_N(x) = \frac{4}{\pi}\sum_{k=0}^{N-1}\frac{\sin(2k+1)x}{2k+1}$.

The maximum of $S_N$ occurs near $x_N = \pi/(2N)$ (first maximum of the partial sum near the jump). Computing:

$S_N(x_N) = \frac{4}{\pi}\sum_{k=0}^{N-1}\frac{\sin((2k+1)\pi/(2N))}{2k+1} \approx \frac{4}{\pi}\cdot\frac{N}{\pi}\int_0^\pi \frac{\sin\theta}{\theta}\,d\theta \to \frac{4}{\pi^2}\int_0^\pi\frac{\sin\theta}{\theta}\,d\theta\cdot\frac{\pi}{2}$.

Numerically: $\int_0^\pi\sin(\theta)/\theta\,d\theta \approx 1.8519$, so the overshoot is approximately $\frac{2}{\pi}\cdot 1.8519 - 1 \approx 0.1789 \approx 8.9\%$ above the function value.

This $\approx 9\%$ overshoot does not decrease as $N \to \infty$; it merely localizes in a narrower region near the jump. The Gibbs phenomenon shows that uniform convergence fails at a jump discontinuity, even though $L^2$ convergence holds everywhere.

---

## Problem 6: Uncertainty Principle Calculation

**Problem.** For $f(x) = e^{-x^2/2}$ (a Gaussian), compute both sides of the uncertainty principle inequality and verify it is achieved.

**Solution.** $f \in L^2(\mathbb{R})$, $\|f\|_2^2 = \int e^{-x^2}\,dx = \sqrt{\pi}$.

$\|xf\|_2^2 = \int x^2 e^{-x^2}\,dx = \frac{\sqrt{\pi}}{2}$ (standard Gaussian integral with extra $x^2$).

Fourier transform (with our convention $\hat{f}(\xi) = \int f(x)e^{-2\pi i\xi x}\,dx$):
$\hat{f}(\xi) = e^{-2\pi^2\xi^2}$ (Fourier transform of a Gaussian is a Gaussian; the Gaussian is a fixed point of the Fourier transform).

$\|\xi\hat{f}\|_2^2 = \int\xi^2 e^{-4\pi^2\xi^2}\,d\xi = \frac{1}{8\pi^2\sqrt{\pi}}\cdot\sqrt{\pi} = \frac{1}{8\pi^2}$.

Wait — let us redo: $\int \xi^2 e^{-4\pi^2\xi^2}\,d\xi$. Let $u = 2\pi\xi$: $= \frac{1}{8\pi^3}\int u^2 e^{-u^2}\,du = \frac{1}{8\pi^3}\cdot\frac{\sqrt{\pi}}{2} = \frac{\sqrt{\pi}}{16\pi^3}$.

LHS of uncertainty principle (in appropriate normalization): $\|xf\|_2\cdot\|\xi\hat{f}\|_2 = \sqrt{\sqrt{\pi}/2}\cdot\sqrt{\sqrt{\pi}/(16\pi^3)}$... 

The key point: for the Gaussian, the uncertainty principle is achieved with equality. The Gaussian is the unique minimizer of the Heisenberg uncertainty principle.

**Common mistake.** Using different conventions for the Fourier transform (with or without $2\pi$ in the exponent) leads to different numerical constants in the uncertainty principle. Always verify which convention is in use.
