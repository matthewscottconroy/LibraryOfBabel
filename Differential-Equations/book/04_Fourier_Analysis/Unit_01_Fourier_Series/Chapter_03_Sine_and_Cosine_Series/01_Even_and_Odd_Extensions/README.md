# Even and Odd Extensions

Given a function $f$ defined only on $[0, L]$, we have considerable freedom in how to extend it to the full interval $[-L, L]$ and then periodically to all of $\mathbb{R}$. Two canonical choices — the even and odd extensions — produce Fourier series with only cosine or only sine terms, respectively. These extensions are not merely mathematical conveniences; they encode the physical boundary conditions of the problem.

## The Even Extension

**Definition.** The **even extension** of $f : [0, L] \to \mathbb{R}$ to $[-L, L]$ is
$$f_e(x) = \begin{cases} f(x) & 0 \leq x \leq L \\ f(-x) & -L \leq x < 0. \end{cases}$$
By construction, $f_e(-x) = f_e(x)$, so $f_e$ is an even function of $x$.

Since $f_e$ is even, its Fourier series on $[-L, L]$ contains only cosine terms:
$$\int_{-L}^L f_e(x)\sin\!\left(\frac{n\pi x}{L}\right)dx = 0 \quad \text{(odd integrand on symmetric interval)}.$$
The cosine coefficients are
$$a_n = \frac{1}{L}\int_{-L}^L f_e(x)\cos\!\left(\frac{n\pi x}{L}\right)dx = \frac{2}{L}\int_0^L f(x)\cos\!\left(\frac{n\pi x}{L}\right)dx.$$

At the boundaries: $f_e(0) = f(0)$ and $f_e(L) = f(L)$, with no special constraint imposed. The graph of $f_e$ on $[-L, L]$ is the reflection of the graph of $f$ on $[0, L]$ about the $y$-axis. When extended periodically, $f_e$ is a $2L$-periodic, even function with the same values as $f$ on $[0, L]$.

## The Odd Extension

**Definition.** The **odd extension** of $f : [0, L] \to \mathbb{R}$ to $[-L, L]$ is
$$f_o(x) = \begin{cases} f(x) & 0 < x \leq L \\ 0 & x = 0 \\ -f(-x) & -L \leq x < 0. \end{cases}$$
By construction, $f_o(-x) = -f_o(x)$, so $f_o$ is an odd function. Note that $f_o(0) = 0$ regardless of $f(0)$.

Since $f_o$ is odd, its Fourier series on $[-L, L]$ contains only sine terms:
$$a_n = \frac{1}{L}\int_{-L}^L f_o(x)\cos\!\left(\frac{n\pi x}{L}\right)dx = 0 \quad \text{(odd integrand)}.$$
The sine coefficients are
$$b_n = \frac{1}{L}\int_{-L}^L f_o(x)\sin\!\left(\frac{n\pi x}{L}\right)dx = \frac{2}{L}\int_0^L f(x)\sin\!\left(\frac{n\pi x}{L}\right)dx.$$

At $x = L$: if $f(L) \neq 0$, then the periodic odd extension has $f_o(L^+) = -f(L)$ from the next period (since $f_o(-L) = -f_o(L) = -f(L)$, and the periodic extension identifies $L$ and $-L$). Thus the periodic odd extension has a jump discontinuity of magnitude $2f(L)$ at $x = L$ (and $x = -L$, $x = 3L$, etc.) unless $f(L) = 0$.

## Symmetry Properties and Their Consequences

The even/odd decomposition has a useful consequence for arbitrary functions: any function $f$ on $[-L, L]$ can be written as a sum of its even and odd parts:
$$f = \frac{f(x) + f(-x)}{2} + \frac{f(x) - f(-x)}{2} = f_e(x) + f_o(x).$$
The even part gives the cosine coefficients and the odd part gives the sine coefficients. The full Fourier series is the sum of the two half-range series.

For a function defined only on $[0, L]$, the "even part" and "odd part" interpretations are:
- Even extension: the function is "reflected" at $x = 0$ and at $x = L$.
- Odd extension: the function is "anti-reflected" at $x = 0$ (forced to zero there).

## Worked Example: Extending $f(x) = x$ on $[0, \pi]$

**Even extension** of $f(x) = x$ to $[-\pi, \pi]$: $f_e(x) = |x|$.

The Fourier cosine series of $f_e$ is (from Chapter 02):
$$|x| = \frac{\pi}{2} - \frac{4}{\pi}\sum_{k=0}^\infty \frac{\cos((2k+1)x)}{(2k+1)^2}.$$
This converges uniformly on all of $\mathbb{R}$, since $f_e$ is continuous and piecewise smooth.

**Odd extension** of $f(x) = x$ to $[-\pi, \pi]$: $f_o(x) = x$ for $x \in (-\pi, \pi)$, with jump at $\pm\pi$.

The Fourier sine series (from Chapter 02):
$$x = 2\sum_{n=1}^\infty \frac{(-1)^{n+1}}{n}\sin(nx), \quad x \in (-\pi, \pi).$$
This converges pointwise on $(-\pi, \pi)$ but not at $x = \pm\pi$, where the series gives $0$ (the midpoint of the jump from $-\pi$ to $\pi$).

## Continuity of the Extended Function

The behavior of the extension at the endpoints determines whether the periodic extension is continuous:

- **Even extension at $x = 0$**: always continuous (the reflection is smooth there).
- **Even extension at $x = L$**: the periodic even extension has two copies of $f$ meeting at $x = L$. If $f$ is differentiable, the derivative from the left is $-f'(L)$ (from the reflected copy) and from the right is $+f'(L)$, so there is a kink (corner) unless $f'(L) = 0$.
- **Odd extension at $x = 0$**: continuous at $0$ only if $f(0) = 0$ (since $f_o(0) = 0$).
- **Odd extension at $x = L$**: continuous at $L$ only if $f(L) = 0$.

These observations have direct consequences for convergence:
- If $f(0) = 0$ and $f(L) = 0$, the odd extension is continuous at the endpoints, and the periodic odd extension is everywhere continuous if $f$ is continuous on $[0, L]$.
- If $f'(0) = 0$ and $f'(L) = 0$, the periodic even extension is $C^1$ at the endpoints, improving convergence.

## Worked Example: Extension of $f(x) = 1 - x/\pi$ on $[0, \pi]$

This function satisfies $f(0) = 1$ and $f(\pi) = 0$.

**Even extension**: $f_e(0) = 1$, $f_e(\pi) = 0$. The periodic even extension has $f_e(-\pi) = 0$ meeting $f_e(\pi) = 0$ — continuous at $\pm\pi$. At $0$: $f_e$ is the reflection of $1 - x/\pi$, which has a corner (kink) at $0$ since $f'(0^+) = -1/\pi \neq 0$.

Cosine coefficients: $a_0 = \frac{2}{\pi}\int_0^\pi (1 - x/\pi)\,dx = \frac{2}{\pi}\cdot\frac{\pi}{2} = 1$. For $n \geq 1$:
$$a_n = \frac{2}{\pi}\int_0^\pi \left(1 - \frac{x}{\pi}\right)\cos(nx)\,dx.$$
The $\int_0^\pi \cos(nx)\,dx = 0$ term, and $\int_0^\pi x\cos(nx)\,dx = (\cos(n\pi)-1)/n^2 = ((-1)^n-1)/n^2$. So $a_n = -\frac{2}{\pi} \cdot \frac{(-1)^n - 1}{\pi n^2} = \frac{2(1-(-1)^n)}{\pi^2 n^2}$. For even $n$, $a_n = 0$; for odd $n = 2k-1$, $a_{2k-1} = \frac{4}{\pi^2(2k-1)^2}$.

**Odd extension**: $f_o(0) = 0$ but $f_o(0^+) = f(0^+) = 1 \neq 0$, so there is a jump discontinuity at $0$. Also $f_o(\pi) = 0 = f(\pi)$, so no jump at $\pi$. The Gibbs phenomenon will appear near $x = 0$ in the sine series.

These contrasting behaviors illustrate how the choice of extension affects both the nature of the series and the quality of convergence.
