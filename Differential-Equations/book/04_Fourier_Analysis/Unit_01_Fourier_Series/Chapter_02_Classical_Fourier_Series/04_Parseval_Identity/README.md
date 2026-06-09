# Parseval's Identity

A fundamental principle in physics and engineering is that energy is conserved. When a periodic signal is decomposed into its Fourier components, the total energy in the signal should equal the total energy distributed across all frequency components. Parseval's identity is the mathematical expression of this principle. It says that the $L^2$ norm of a function equals the $\ell^2$ norm of its sequence of Fourier coefficients — a beautiful bridge between function space and sequence space.

## Statement of Parseval's Identity

**Theorem (Parseval's Identity).** Let $f$ be $2\pi$-periodic and square-integrable on $[-\pi, \pi]$. With Fourier coefficients
$$a_n = \frac{1}{\pi}\int_{-\pi}^\pi f(x)\cos(nx)\,dx, \quad b_n = \frac{1}{\pi}\int_{-\pi}^\pi f(x)\sin(nx)\,dx,$$
we have
$$\frac{1}{\pi}\int_{-\pi}^\pi |f(x)|^2\,dx = \frac{a_0^2}{2} + \sum_{n=1}^\infty (a_n^2 + b_n^2).$$

Equivalently, in terms of the $L^2$ inner product $\langle f, g\rangle = \frac{1}{\pi}\int_{-\pi}^\pi fg\,dx$:
$$\|f\|^2 = \frac{a_0^2}{2} + \sum_{n=1}^\infty (a_n^2 + b_n^2).$$

In the complex form (see Chapter 04), if $c_n = \frac{1}{2\pi}\int_{-\pi}^\pi f(x)e^{-inx}\,dx$ are the complex Fourier coefficients, Parseval's identity becomes the cleaner statement
$$\frac{1}{2\pi}\int_{-\pi}^\pi |f(x)|^2\,dx = \sum_{n=-\infty}^\infty |c_n|^2.$$

## Proof

One approach is through Bessel's inequality and the completeness of the trigonometric system. A more direct route uses the $L^2$ convergence of Fourier series.

Since the partial sums $S_N f \to f$ in $L^2$ (by completeness), we have $\|S_N f - f\|^2 \to 0$. Now compute $\|S_N f\|^2$ directly: since the trigonometric functions are orthogonal with $\|\cos(nx)\|^2 = \|\sin(nx)\|^2 = 1$ (with our normalization) and $\|1\|^2 = 2$,
$$\|S_N f\|^2 = \frac{a_0^2}{2} + \sum_{n=1}^N (a_n^2 + b_n^2).$$

From $\|f\|^2 = \|f - S_N f + S_N f\|^2$ and the expansion $\|f - S_N f\|^2 = \|f\|^2 - \|S_N f\|^2$ (which follows because $S_N f$ is the orthogonal projection of $f$ onto a subspace containing $S_N f$, so $f - S_N f \perp S_N f$), we get
$$\|f\|^2 - \|S_N f\|^2 = \|f - S_N f\|^2 \to 0.$$
Therefore $\|S_N f\|^2 \to \|f\|^2$, which is Parseval's identity.

## Applications: Summing Numerical Series

One of the most delightful applications of Parseval's identity is evaluating sums of series that would otherwise be very difficult to compute.

**Example 1: $\sum_{n=1}^\infty 1/n^2 = \pi^2/6$.**

Use $f(x) = x$ on $(-\pi, \pi)$. The Fourier series is $f(x) \sim 2\sum_{n=1}^\infty (-1)^{n+1}\sin(nx)/n$.

So $a_n = 0$ and $b_n = 2(-1)^{n+1}/n$. The left side of Parseval's identity:
$$\frac{1}{\pi}\int_{-\pi}^\pi x^2\,dx = \frac{1}{\pi}\cdot\frac{2\pi^3}{3} = \frac{2\pi^2}{3}.$$
The right side:
$$\sum_{n=1}^\infty b_n^2 = \sum_{n=1}^\infty \frac{4}{n^2}.$$
Setting them equal: $\frac{2\pi^2}{3} = 4\sum_{n=1}^\infty \frac{1}{n^2}$, so $\sum_{n=1}^\infty \frac{1}{n^2} = \frac{\pi^2}{6}$.

This is Euler's famous result from 1734, here obtained by a purely analytic method.

**Example 2: $\sum_{n=1}^\infty 1/n^4 = \pi^4/90$.**

Use $f(x) = x^2$ on $(-\pi, \pi)$. This is an even function, so $b_n = 0$. The Fourier coefficients are $a_0 = 2\pi^2/3$ and (by integration by parts) $a_n = 4(-1)^n/n^2$ for $n \geq 1$.

Left side of Parseval's:
$$\frac{1}{\pi}\int_{-\pi}^\pi x^4\,dx = \frac{2\pi^4}{5}.$$
Right side:
$$\frac{a_0^2}{2} + \sum_{n=1}^\infty a_n^2 = \frac{1}{2}\cdot\frac{4\pi^4}{9} + \sum_{n=1}^\infty \frac{16}{n^4} = \frac{2\pi^4}{9} + 16\sum_{n=1}^\infty \frac{1}{n^4}.$$
Setting equal: $\frac{2\pi^4}{5} = \frac{2\pi^4}{9} + 16\sum_{n=1}^\infty \frac{1}{n^4}$, so
$$16\sum_{n=1}^\infty \frac{1}{n^4} = \pi^4\left(\frac{2}{5} - \frac{2}{9}\right) = \pi^4\cdot\frac{8}{45},$$
giving $\sum_{n=1}^\infty \frac{1}{n^4} = \frac{\pi^4}{90}$.

**Example 3: $\sum_{k=0}^\infty 1/(2k+1)^2 = \pi^2/8$.**

From Example 1, $\pi^2/6 = \sum_{n=1}^\infty 1/n^2 = \sum_{\text{odd}} 1/n^2 + \sum_{\text{even}} 1/n^2$. The even sum is $\sum_{k=1}^\infty 1/(2k)^2 = \frac{1}{4}\cdot\frac{\pi^2}{6}$. So $\sum_{\text{odd}} 1/n^2 = \frac{\pi^2}{6} - \frac{\pi^2}{24} = \frac{\pi^2}{8}$.

## Plancherel's Theorem and Energy Interpretation

Parseval's identity can be written as an inner product formula too. If $f$ and $g$ are both in $L^2$ with Fourier coefficients $a_n, b_n$ and $\alpha_n, \beta_n$ respectively, then by the polarization identity $\langle f,g\rangle = \frac{1}{4}(\|f+g\|^2 - \|f-g\|^2)$:
$$\frac{1}{\pi}\int_{-\pi}^\pi f(x)g(x)\,dx = \frac{a_0\alpha_0}{2} + \sum_{n=1}^\infty (a_n\alpha_n + b_n\beta_n).$$

This is the **generalized Parseval formula** (or Plancherel-type identity for Fourier series). In the complex form with coefficients $c_n$ and $d_n$:
$$\frac{1}{2\pi}\int_{-\pi}^\pi f(x)\overline{g(x)}\,dx = \sum_{n=-\infty}^\infty c_n\overline{d_n}.$$

The energy interpretation: if $f(t)$ represents a signal in time, $|f(t)|^2$ is the instantaneous power and $\int |f|^2\,dt$ is the total energy. The Fourier coefficients give the amplitude at each frequency, $|c_n|^2$ is the energy at frequency $n$, and $\sum |c_n|^2$ is the total energy. Parseval says these are equal: no energy is created or destroyed by the Fourier decomposition.

## Completeness from Parseval

Parseval's identity is also equivalent to the completeness of the trigonometric system. If Parseval holds, then for any $f$ with all Fourier coefficients zero, $\|f\|^2 = 0$, so $f = 0$ a.e. Conversely, if the system is complete, the partial sums $S_N f \to f$ in $L^2$, and computing norms gives Parseval's identity. The identity is therefore a measure of how well the orthonormal system captures the full space.
