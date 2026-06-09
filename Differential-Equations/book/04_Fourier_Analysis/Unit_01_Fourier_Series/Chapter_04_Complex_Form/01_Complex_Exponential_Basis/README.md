# The Complex Exponential Basis

Euler's formula $e^{i\theta} = \cos\theta + i\sin\theta$ is one of the most remarkable identities in mathematics. It transforms the real trigonometric functions into complex exponentials that are simultaneously simpler algebraically and more symmetric. For Fourier analysis, the key insight is that the functions $e^{inx}$ for $n \in \mathbb{Z}$ form an orthonormal basis for the space of square-integrable functions on $[-\pi, \pi]$, and this basis has a unity and elegance that the real trigonometric system lacks.

## The Functions $e^{inx}$ as Periodic Functions

For each integer $n \in \mathbb{Z}$, the function $e^{inx} = \cos(nx) + i\sin(nx)$ is $2\pi$-periodic: $e^{in(x+2\pi)} = e^{inx}e^{2\pi in} = e^{inx}$ (since $e^{2\pi in} = 1$ for integer $n$). For $n = 0$, the function is identically $1$. For $n \neq 0$, the function traces out the unit circle in $\mathbb{C}$ as $x$ runs over $[0, 2\pi]$, completing $|n|$ full rotations (counterclockwise for $n > 0$, clockwise for $n < 0$).

The function $e^{inx}$ carries both amplitude and phase information compactly: its modulus is $|e^{inx}| = 1$ for all $x$, and its argument $\arg(e^{inx}) = nx$ is a linear function of position. A linear combination $c_n e^{inx}$ has modulus $|c_n|$ (the amplitude of the $n$-th frequency component) and phase shifted by $\arg(c_n)$.

## The Complex Inner Product

To work with complex-valued functions, we need a modified inner product. For $f, g : [-\pi, \pi] \to \mathbb{C}$, define
$$\langle f, g \rangle = \frac{1}{2\pi}\int_{-\pi}^\pi f(x)\overline{g(x)}\,dx,$$
where $\overline{g(x)}$ is the complex conjugate. The conjugate in the second argument ensures positive definiteness: $\langle f, f\rangle = \frac{1}{2\pi}\int_{-\pi}^\pi |f(x)|^2\,dx \geq 0$, with equality only if $f = 0$ a.e.

This inner product is **sesquilinear**: linear in the first argument and conjugate-linear in the second. That is, $\langle \alpha f, g \rangle = \alpha\langle f,g\rangle$ but $\langle f, \alpha g\rangle = \bar{\alpha}\langle f,g\rangle$.

## Orthonormality of $\{e^{inx}\}_{n \in \mathbb{Z}}$

**Theorem.** The family $\{e^{inx}\}_{n \in \mathbb{Z}}$ is orthonormal in $L^2([-\pi,\pi], \mathbb{C})$ with respect to the inner product $\langle f, g\rangle = \frac{1}{2\pi}\int_{-\pi}^\pi f\bar{g}\,dx$.

**Proof.** For any integers $m$ and $n$:
$$\langle e^{imx}, e^{inx}\rangle = \frac{1}{2\pi}\int_{-\pi}^\pi e^{imx}\overline{e^{inx}}\,dx = \frac{1}{2\pi}\int_{-\pi}^\pi e^{imx}e^{-inx}\,dx = \frac{1}{2\pi}\int_{-\pi}^\pi e^{i(m-n)x}\,dx.$$

If $m = n$: $\frac{1}{2\pi}\int_{-\pi}^\pi 1\,dx = 1$.

If $m \neq n$: $\frac{1}{2\pi}\int_{-\pi}^\pi e^{i(m-n)x}\,dx = \frac{1}{2\pi}\left[\frac{e^{i(m-n)x}}{i(m-n)}\right]_{-\pi}^\pi = \frac{1}{2\pi i(m-n)}\left(e^{i(m-n)\pi} - e^{-i(m-n)\pi}\right) = \frac{\sin((m-n)\pi)}{\pi(m-n)} = 0,$
since $\sin(k\pi) = 0$ for any nonzero integer $k$.

Therefore $\langle e^{imx}, e^{inx}\rangle = \delta_{mn}$ for all $m, n \in \mathbb{Z}$.

This is a single formula — far more elegant than the three separate orthogonality cases needed for the real system.

## Connection to the Real System via Euler's Formula

The real trigonometric system and the complex exponential system contain the same information, related by
$$\cos(nx) = \frac{e^{inx} + e^{-inx}}{2}, \quad \sin(nx) = \frac{e^{inx} - e^{-inx}}{2i}.$$
Conversely, $e^{inx} = \cos(nx) + i\sin(nx)$ and $e^{-inx} = \cos(nx) - i\sin(nx)$.

The real system $\{1, \cos(x), \sin(x), \cos(2x), \sin(2x), \ldots\}$ has one function for each non-negative integer, organized by frequency. The complex system $\{\ldots, e^{-2ix}, e^{-ix}, e^{0}, e^{ix}, e^{2ix}, \ldots\}$ has one function for each integer, with positive and negative frequencies both present. Each real basis element is a linear combination of two complex basis elements at $\pm n$, and each complex basis element is a linear combination of two real basis elements at frequency $|n|$.

## Completeness of the Complex Exponential System

**Theorem.** The orthonormal set $\{e^{inx}\}_{n \in \mathbb{Z}}$ is complete in $L^2([-\pi,\pi])$.

This means: if $f \in L^2([-\pi,\pi])$ satisfies $\langle f, e^{inx}\rangle = 0$ for all $n \in \mathbb{Z}$, then $f = 0$ a.e.

**Proof outline.** By Euler's formula, $\langle f, e^{inx}\rangle = 0$ for all $n \in \mathbb{Z}$ implies both $\int f\cos(nx) = 0$ and $\int f\sin(nx) = 0$ for all $n \geq 0$. By the completeness of the real trigonometric system (which follows from the Weierstrass approximation theorem), $f = 0$ a.e.

Completeness, combined with orthonormality, means that every $f \in L^2([-\pi,\pi])$ has the expansion
$$f(x) = \sum_{n=-\infty}^\infty c_n e^{inx}$$
where $c_n = \langle f, e^{inx}\rangle = \frac{1}{2\pi}\int_{-\pi}^\pi f(x)e^{-inx}\,dx$, and the series converges in $L^2$.

## Geometric Interpretation

In the finite-dimensional analogy, an orthonormal basis $\{\mathbf{e}_1, \ldots, \mathbf{e}_n\}$ of $\mathbb{C}^n$ allows every vector to be decomposed as $\mathbf{v} = \sum_k (\mathbf{v}\cdot\mathbf{e}_k)\mathbf{e}_k$. The complex Fourier basis $\{e^{inx}\}$ plays exactly this role in the infinite-dimensional Hilbert space $L^2([-\pi,\pi])$. The "coordinates" of $f$ in this basis are the Fourier coefficients $c_n$, and just as coordinates in $\mathbb{C}^n$ are determined by inner products with basis vectors, so are the Fourier coefficients.

The Fourier transform (Unit 02) extends this picture from $[-\pi, \pi]$ to all of $\mathbb{R}$, where the discrete spectrum $\{n : n \in \mathbb{Z}\}$ becomes a continuous one.

## Example: Expansion of $e^{ax}$ in $\{e^{inx}\}$

Let $f(x) = e^{ax}$ on $[-\pi, \pi]$ for real $a \neq 0$. The complex Fourier coefficients are:
$$c_n = \frac{1}{2\pi}\int_{-\pi}^\pi e^{ax}e^{-inx}\,dx = \frac{1}{2\pi}\int_{-\pi}^\pi e^{(a-in)x}\,dx = \frac{1}{2\pi}\cdot\frac{e^{(a-in)\pi} - e^{-(a-in)\pi}}{a - in}$$
$$= \frac{e^{a\pi}(-1)^n - e^{-a\pi}(-1)^n}{2\pi(a-in)} = \frac{(-1)^n(e^{a\pi} - e^{-a\pi})}{2\pi(a-in)} = \frac{(-1)^n\sinh(a\pi)}{\pi(a-in)}.$$

So the complex Fourier series is
$$e^{ax} = \frac{\sinh(a\pi)}{\pi}\sum_{n=-\infty}^\infty \frac{(-1)^n}{a - in}e^{inx}.$$

The coefficients decay as $|c_n| \sim \frac{\sinh(a\pi)}{\pi|n|}$ for large $|n|$, consistent with $e^{ax}$ being analytic (if the coefficients decayed exponentially, that would indicate a function with no continuation to a larger region — but here $e^{ax}$ does have a continuation, and the decay is only polynomial due to the jump discontinuity when we periodically extend $e^{ax}$).
