# Half-Range Expansions

A function $f$ defined on $[0, L]$ can be expanded in either a Fourier sine series or a Fourier cosine series — two different representations, each valid on $(0, L)$ but extending the function differently outside. These **half-range expansions** are essential tools in solving boundary value problems, where the choice between sine and cosine is determined by the physical boundary conditions at $x = 0$ and $x = L$.

## The Half-Range Cosine Series

**Definition.** The **half-range Fourier cosine series** of $f : [0, L] \to \mathbb{R}$ is
$$f(x) \sim \frac{a_0}{2} + \sum_{n=1}^\infty a_n \cos\!\left(\frac{n\pi x}{L}\right),$$
where
$$a_0 = \frac{2}{L}\int_0^L f(x)\,dx, \quad a_n = \frac{2}{L}\int_0^L f(x)\cos\!\left(\frac{n\pi x}{L}\right)dx \quad (n \geq 1).$$
The factor $2/L$ (rather than $1/L$) arises because we integrate only over the half-interval $[0, L]$ instead of $[-L, L]$.

The cosine series converges to $f(x)$ for $x \in (0, L)$ (at points of continuity) and its periodic extension outside $[0, L]$ is an even, $2L$-periodic function.

**Boundary behavior:** The cosine functions $\cos(n\pi x/L)$ have zero derivative at $x = 0$ and $x = L$:
$$\frac{d}{dx}\cos\!\left(\frac{n\pi x}{L}\right)\Big|_{x=0} = 0, \quad \frac{d}{dx}\cos\!\left(\frac{n\pi x}{L}\right)\Big|_{x=L} = 0.$$
Therefore the cosine series is natural for problems with **Neumann boundary conditions**: $f'(0) = 0$ and/or $f'(L) = 0$.

## The Half-Range Sine Series

**Definition.** The **half-range Fourier sine series** of $f : [0, L] \to \mathbb{R}$ is
$$f(x) \sim \sum_{n=1}^\infty b_n \sin\!\left(\frac{n\pi x}{L}\right),$$
where
$$b_n = \frac{2}{L}\int_0^L f(x)\sin\!\left(\frac{n\pi x}{L}\right)dx.$$

The sine series converges to $f(x)$ for $x \in (0, L)$ (at points of continuity). Its periodic extension is an odd, $2L$-periodic function; outside $[0, L]$ it satisfies $f(-x) = -f(x)$.

**Boundary behavior:** The sine functions satisfy $\sin(0) = 0$ and $\sin(n\pi) = 0$:
$$\sin\!\left(\frac{n\pi \cdot 0}{L}\right) = 0, \quad \sin\!\left(\frac{n\pi \cdot L}{L}\right) = \sin(n\pi) = 0.$$
Therefore the sine series is natural for **Dirichlet boundary conditions**: $f(0) = 0$ and $f(L) = 0$.

## Worked Example: $f(x) = 1$ on $[0, \pi]$

**Cosine expansion:** $a_0 = \frac{2}{\pi}\int_0^\pi 1\,dx = 2$. For $n \geq 1$: $a_n = \frac{2}{\pi}\int_0^\pi\cos(nx)\,dx = \frac{2}{n\pi}\sin(n\pi) = 0$. So the cosine series is simply $f(x) = 1$ (the constant function needs only the $a_0/2$ term, which equals $1$). This makes sense: the even extension of $f \equiv 1$ is the constant function $1$, which already has a trivial Fourier series.

**Sine expansion:** $b_n = \frac{2}{\pi}\int_0^\pi \sin(nx)\,dx = \frac{2}{n\pi}[-\cos(nx)]_0^\pi = \frac{2}{n\pi}(1 - \cos(n\pi)) = \frac{2(1-(-1)^n)}{n\pi}$.

For even $n$: $b_n = 0$. For odd $n = 2k-1$: $b_{2k-1} = \frac{4}{(2k-1)\pi}$. So
$$1 = \frac{4}{\pi}\sum_{k=1}^\infty \frac{\sin((2k-1)x)}{2k-1}, \quad x \in (0, \pi).$$
At $x = \pi/2$: $1 = \frac{4}{\pi}(1 - \frac{1}{3} + \frac{1}{5} - \cdots) = \frac{4}{\pi}\cdot\frac{\pi}{4}$. Correct.

## Worked Example: $f(x) = x(L-x)$ on $[0, L]$

This function satisfies $f(0) = f(L) = 0$, making the sine expansion particularly natural.

$$b_n = \frac{2}{L}\int_0^L x(L-x)\sin\!\left(\frac{n\pi x}{L}\right)dx.$$

Let $u = \pi x / L$, so $x = Lu/\pi$, $L - x = L(1 - u/\pi) = L(\pi - u)/\pi$, $dx = L\,du/\pi$:
$$b_n = \frac{2}{L}\int_0^\pi \frac{Lu}{\pi}\cdot\frac{L(\pi - u)}{\pi}\sin(nu)\frac{L\,du}{\pi} = \frac{2L^2}{\pi^3}\int_0^\pi u(\pi - u)\sin(nu)\,du.$$

Compute $I = \int_0^\pi u(\pi - u)\sin(nu)\,du$ by parts twice:

$\int_0^\pi u\sin(nu)\,du = [-u\cos(nu)/n]_0^\pi + \int_0^\pi \cos(nu)/n\,du = (-1)^{n+1}\pi/n$.

$\int_0^\pi u^2\sin(nu)\,du = [-u^2\cos(nu)/n]_0^\pi + 2\int_0^\pi u\cos(nu)/n\,du$.
$= -\pi^2(-1)^n/n + \frac{2}{n}\int_0^\pi u\cos(nu)\,du$.
$\int_0^\pi u\cos(nu)\,du = [u\sin(nu)/n]_0^\pi - \int_0^\pi \sin(nu)/n\,du = 0 + [cos(nu)/n^2]_0^\pi = ((-1)^n - 1)/n^2$.
So $\int_0^\pi u^2\sin(nu)\,du = -\pi^2(-1)^n/n + 2((-1)^n - 1)/n^3$.

Therefore:
$$I = \pi\int_0^\pi u\sin(nu)\,du - \int_0^\pi u^2\sin(nu)\,du = \pi\cdot\frac{(-1)^{n+1}\pi}{n} - \left(\frac{-\pi^2(-1)^n}{n} + \frac{2((-1)^n-1)}{n^3}\right)$$
$$= \frac{-\pi^2(-1)^n}{n} + \frac{\pi^2(-1)^n}{n} + \frac{2(1-(-1)^n)}{n^3} = \frac{2(1-(-1)^n)}{n^3}.$$

So $b_n = \frac{2L^2}{\pi^3}\cdot\frac{2(1-(-1)^n)}{n^3}$. For even $n$: $b_n = 0$. For odd $n = 2k-1$:
$$b_{2k-1} = \frac{8L^2}{\pi^3(2k-1)^3}.$$

The sine series is
$$x(L-x) = \frac{8L^2}{\pi^3}\sum_{k=1}^\infty \frac{\sin\!\left(\frac{(2k-1)\pi x}{L}\right)}{(2k-1)^3}, \quad x \in [0, L].$$

The $1/n^3$ decay reflects the continuity of $f$ and its first derivative, with $f'$ being piecewise $C^1$.

## Application to the Heat Equation

Consider the heat equation $u_t = \alpha^2 u_{xx}$ on $0 < x < L$, $t > 0$, with Dirichlet conditions $u(0,t) = u(L,t) = 0$ and initial condition $u(x,0) = f(x)$.

Separation of variables gives $u(x,t) = \sum_{n=1}^\infty B_n e^{-\alpha^2(n\pi/L)^2 t}\sin(n\pi x/L)$, where the coefficients $B_n$ are the sine series coefficients of the initial temperature $f$:
$$B_n = b_n = \frac{2}{L}\int_0^L f(x)\sin\!\left(\frac{n\pi x}{L}\right)dx.$$

For $f(x) = x(L-x)$, we computed $B_{2k-1} = 8L^2/[\pi^3(2k-1)^3]$ and $B_{2k} = 0$. The solution is
$$u(x,t) = \frac{8L^2}{\pi^3}\sum_{k=1}^\infty \frac{1}{(2k-1)^3}e^{-\alpha^2(2k-1)^2\pi^2 t/L^2}\sin\!\left(\frac{(2k-1)\pi x}{L}\right).$$

As $t \to \infty$, all terms decay exponentially, with the fundamental mode $(k=1)$ decaying slowest, governing the long-time behavior.

## Convergence Comparison

For the same function $f$ on $[0, L]$:

- The cosine series typically converges faster if $f$ is smooth and $f'(0) = f'(L) = 0$, because the even extension is $C^1$ at the endpoints.
- The sine series typically converges faster if $f(0) = f(L) = 0$, because the odd extension is continuous at the endpoints.
- If both $f(0) = f(L) = 0$ and $f'(0) = f'(L) = 0$, both series converge quickly ($1/n^3$ or better).
- If $f(0) \neq 0$, the sine series must produce large oscillations near $0$ to achieve the zero boundary value, leading to slower convergence and Gibbs-like behavior.
