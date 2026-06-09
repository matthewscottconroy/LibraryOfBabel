# Fourier Coefficients

Suppose we believe that a $2\pi$-periodic function $f$ can be written as a sum of sines and cosines:
$$f(x) = \frac{a_0}{2} + \sum_{n=1}^\infty \left[a_n \cos(nx) + b_n\sin(nx)\right].$$
What must the numbers $a_n$ and $b_n$ be? The orthogonality relations from Chapter 01 make the answer immediate — and the fact that orthogonality forces a unique answer is exactly why these formulas are so powerful.

## Deriving the Coefficient Formulas

Assume the series converges in a way that allows term-by-term integration (this assumption will be justified by the convergence theory in subsequent sections). Multiply both sides of the assumed series by $\cos(mx)$ and integrate over $[-\pi, \pi]$:

$$\int_{-\pi}^\pi f(x)\cos(mx)\,dx = \frac{a_0}{2}\int_{-\pi}^\pi \cos(mx)\,dx + \sum_{n=1}^\infty \left[ a_n \int_{-\pi}^\pi \cos(nx)\cos(mx)\,dx + b_n \int_{-\pi}^\pi \sin(nx)\cos(mx)\,dx \right].$$

By the orthogonality relations:
- $\int_{-\pi}^\pi \cos(mx)\,dx = 0$ for $m \geq 1$.
- $\int_{-\pi}^\pi \cos(nx)\cos(mx)\,dx = \pi\,\delta_{mn}$ for $m, n \geq 1$.
- $\int_{-\pi}^\pi \sin(nx)\cos(mx)\,dx = 0$ for all $m, n$.

Every term in the sum is zero except when $n = m$ in the cosine-cosine integral, giving
$$\int_{-\pi}^\pi f(x)\cos(mx)\,dx = a_m \cdot \pi.$$

Therefore
$$a_m = \frac{1}{\pi}\int_{-\pi}^\pi f(x)\cos(mx)\,dx, \quad m \geq 1.$$

For $m = 0$: multiply by $\cos(0) = 1$ and integrate, noting $\int_{-\pi}^\pi \cos(nx)\,dx = 0$ for $n \geq 1$:
$$\int_{-\pi}^\pi f(x)\,dx = \frac{a_0}{2} \cdot 2\pi = a_0\pi,$$
so $a_0 = \frac{1}{\pi}\int_{-\pi}^\pi f(x)\,dx$. This is consistent with the formula for $a_m$ at $m=0$ (since $\cos(0) = 1$), which is why we wrote $a_0/2$ in the series rather than $a_0$ — it makes a single formula $a_n = \frac{1}{\pi}\int_{-\pi}^\pi f(x)\cos(nx)\,dx$ hold for all $n \geq 0$.

Similarly, multiplying by $\sin(mx)$ and integrating:
$$\int_{-\pi}^\pi f(x)\sin(mx)\,dx = b_m \cdot \pi,$$
so
$$b_m = \frac{1}{\pi}\int_{-\pi}^\pi f(x)\sin(mx)\,dx, \quad m \geq 1.$$

## The Fourier Coefficients as Inner Products

The formulas can be written compactly as:
$$a_n = \frac{\langle f, \cos(nx) \rangle}{\|\cos(nx)\|^2}, \quad b_n = \frac{\langle f, \sin(nx)\rangle}{\|\sin(nx)\|^2},$$
where $\langle f, g\rangle = \int_{-\pi}^\pi f g\,dx$ and $\|\cos(nx)\|^2 = \|\sin(nx)\|^2 = \pi$. This is exactly the formula for the coefficient of the projection of $f$ onto the direction of each basis function. The structure is purely geometric: $a_n$ measures how much of $f$ lies in the direction of $\cos(nx)$.

## General Period $2L$

For a function $f$ with period $2L$, the Fourier series is
$$f(x) = \frac{a_0}{2} + \sum_{n=1}^\infty \left[a_n\cos\!\left(\frac{n\pi x}{L}\right) + b_n\sin\!\left(\frac{n\pi x}{L}\right)\right],$$
with coefficients
$$a_n = \frac{1}{L}\int_{-L}^L f(x)\cos\!\left(\frac{n\pi x}{L}\right)dx, \quad b_n = \frac{1}{L}\int_{-L}^L f(x)\sin\!\left(\frac{n\pi x}{L}\right)dx.$$
These reduce to the $2\pi$-period formulas under the substitution $x \mapsto \pi x/L$.

## Worked Example 1: The Square Wave

Define $f$ on $[-\pi, \pi]$ by $f(x) = 1$ for $x \in (0, \pi)$ and $f(x) = -1$ for $x \in (-\pi, 0)$, extended periodically. This is an odd function.

Since $f$ is odd and $\cos(nx)$ is even, $f(x)\cos(nx)$ is odd, so $a_n = 0$ for all $n \geq 0$.

For $b_n$:
$$b_n = \frac{1}{\pi}\int_{-\pi}^\pi f(x)\sin(nx)\,dx = \frac{2}{\pi}\int_0^\pi \sin(nx)\,dx = \frac{2}{\pi}\left[-\frac{\cos(nx)}{n}\right]_0^\pi = \frac{2}{n\pi}(1 - \cos(n\pi)) = \frac{2}{n\pi}(1 - (-1)^n).$$

For even $n$, $b_n = 0$. For odd $n = 2k-1$:
$$b_{2k-1} = \frac{2}{(2k-1)\pi} \cdot 2 = \frac{4}{(2k-1)\pi}.$$

The Fourier series is
$$f(x) \sim \frac{4}{\pi}\left[\sin x + \frac{\sin 3x}{3} + \frac{\sin 5x}{5} + \cdots\right] = \frac{4}{\pi}\sum_{k=1}^\infty \frac{\sin((2k-1)x)}{2k-1}.$$

## Worked Example 2: The Sawtooth Wave

Let $f(x) = x$ on $(-\pi, \pi)$, extended $2\pi$-periodically. This is also an odd function, so $a_n = 0$.

$$b_n = \frac{1}{\pi}\int_{-\pi}^\pi x\sin(nx)\,dx = \frac{2}{\pi}\int_0^\pi x\sin(nx)\,dx.$$

Integrate by parts with $u = x$, $dv = \sin(nx)\,dx$:
$$\int_0^\pi x\sin(nx)\,dx = \left[-\frac{x\cos(nx)}{n}\right]_0^\pi + \frac{1}{n}\int_0^\pi \cos(nx)\,dx = -\frac{\pi\cos(n\pi)}{n} + \frac{\sin(n\pi)}{n^2} = -\frac{\pi(-1)^n}{n}.$$

So $b_n = \frac{2}{\pi} \cdot \left(-\frac{\pi(-1)^n}{n}\right) = \frac{2(-1)^{n+1}}{n}$.

The Fourier series is
$$f(x) \sim 2\sum_{n=1}^\infty \frac{(-1)^{n+1}}{n}\sin(nx) = 2\left[\sin x - \frac{\sin 2x}{2} + \frac{\sin 3x}{3} - \cdots\right].$$

## Worked Example 3: A Cosine Expansion

Let $f(x) = |x|$ on $[-\pi, \pi]$, extended periodically. This is an even function, so $b_n = 0$ and
$$a_0 = \frac{1}{\pi}\int_{-\pi}^\pi |x|\,dx = \frac{2}{\pi}\int_0^\pi x\,dx = \pi.$$
For $n \geq 1$:
$$a_n = \frac{1}{\pi}\int_{-\pi}^\pi |x|\cos(nx)\,dx = \frac{2}{\pi}\int_0^\pi x\cos(nx)\,dx.$$
Integrate by parts: $\int_0^\pi x\cos(nx)\,dx = \left[\frac{x\sin(nx)}{n}\right]_0^\pi - \frac{1}{n}\int_0^\pi \sin(nx)\,dx = 0 + \frac{1}{n}\left[\frac{\cos(nx)}{n}\right]_0^\pi = \frac{(-1)^n - 1}{n^2}.$

So $a_n = \frac{2((-1)^n - 1)}{n^2\pi}$. For even $n$, $a_n = 0$. For odd $n = 2k-1$, $a_{2k-1} = -4/((2k-1)^2\pi)$.

$$|x| = \frac{\pi}{2} - \frac{4}{\pi}\sum_{k=1}^\infty \frac{\cos((2k-1)x)}{(2k-1)^2}.$$

Note the $1/n^2$ decay of coefficients, reflecting the continuity of $|x|$ (compared to the $1/n$ decay for the discontinuous square wave).

## Properties of Fourier Coefficients

The rate of decay of the coefficients encodes smoothness: if $f$ has $k$ continuous derivatives, then $|a_n|, |b_n| = O(n^{-k-1})$. This is proved by repeated integration by parts. Conversely, if $\sum n^k (|a_n| + |b_n|) < \infty$, then the Fourier series can be differentiated term by term $k$ times. The smoother the function, the faster its Fourier coefficients decay and the fewer terms suffice for a good approximation.

Symmetry also simplifies computation:
- If $f$ is even, then $b_n = 0$ and $a_n = \frac{2}{\pi}\int_0^\pi f(x)\cos(nx)\,dx$.
- If $f$ is odd, then $a_n = 0$ and $b_n = \frac{2}{\pi}\int_0^\pi f(x)\sin(nx)\,dx$.
