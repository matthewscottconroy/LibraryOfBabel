# Complex Fourier Coefficients

With the complex exponential basis $\{e^{inx}\}_{n \in \mathbb{Z}}$ established as orthonormal and complete in $L^2([-\pi,\pi])$, the representation of a function as a sum of these basis elements and the formula for the corresponding coefficients follow immediately from the general theory. This section develops the complex coefficient formula, its relationship to the real coefficients, the complex Parseval identity, and the Hermitian symmetry property that makes the complex form especially powerful for real-valued functions.

## The Complex Fourier Coefficient Formula

**Definition.** For $f \in L^1([-\pi,\pi])$ (in particular for any $L^2$ function), the **$n$-th complex Fourier coefficient** of $f$ is
$$c_n = \langle f, e^{inx}\rangle = \frac{1}{2\pi}\int_{-\pi}^\pi f(x)\,e^{-inx}\,dx, \quad n \in \mathbb{Z}.$$

The **complex Fourier series** of $f$ is
$$f(x) \sim \sum_{n=-\infty}^\infty c_n\,e^{inx}.$$

The sum over $\mathbb{Z}$ is understood as $\lim_{N\to\infty}\sum_{n=-N}^N c_n e^{inx}$ (symmetric partial sums). This symmetric summation is important: taking the limit nonsymmetrically can produce a different answer.

## Relationship to Real Fourier Coefficients

If $f$ is real-valued, the real Fourier coefficients $a_n$ and $b_n$ are related to the complex coefficients by
$$a_n = c_n + c_{-n}, \quad b_n = i(c_n - c_{-n}) \quad (n \geq 1)$$
or equivalently
$$c_n = \frac{a_n - ib_n}{2} \quad (n \geq 1), \quad c_{-n} = \frac{a_n + ib_n}{2} = \overline{c_n} \quad (n \geq 1), \quad c_0 = \frac{a_0}{2}.$$

**Verification.** From the definitions:
$$c_n = \frac{1}{2\pi}\int_{-\pi}^\pi f(x)e^{-inx}\,dx = \frac{1}{2\pi}\int_{-\pi}^\pi f(x)[\cos(nx) - i\sin(nx)]\,dx = \frac{a_n}{2} - \frac{ib_n}{2}$$
for $n \geq 1$ (using $a_n = \frac{1}{\pi}\int f\cos(nx)$ and $b_n = \frac{1}{\pi}\int f\sin(nx)$).

**Hermitian symmetry.** For real $f$: $c_{-n} = \overline{c_n}$ for all $n$. This means the coefficients at positive and negative frequencies are complex conjugates. In terms of amplitudes and phases, $|c_n| = |c_{-n}|$ (equal amplitudes) and $\arg(c_{-n}) = -\arg(c_n)$ (opposite phases). The amplitude $|c_n|$ determines the magnitude of the $n$-th harmonic, and the phase $\arg(c_n)$ its timing.

The real series is recovered from the complex series by pairing $n$ and $-n$ terms:
$$c_n e^{inx} + c_{-n}e^{-inx} = c_n e^{inx} + \overline{c_n}e^{-inx} = 2\text{Re}(c_n e^{inx}) = a_n\cos(nx) + b_n\sin(nx).$$

## Parseval's Identity in Complex Form

**Theorem.** For $f \in L^2([-\pi,\pi])$ with complex Fourier coefficients $\{c_n\}_{n \in \mathbb{Z}}$:
$$\frac{1}{2\pi}\int_{-\pi}^\pi |f(x)|^2\,dx = \sum_{n=-\infty}^\infty |c_n|^2.$$

This is more symmetric than the real form. For real $f$, using $|c_n|^2 = (a_n^2 + b_n^2)/4$ for $n \neq 0$ and $|c_0|^2 = a_0^2/4$:
$$\frac{1}{2\pi}\int_{-\pi}^\pi f^2\,dx = \frac{a_0^2}{4} + \sum_{n=1}^\infty \frac{a_n^2 + b_n^2}{2},$$
which upon multiplying both sides by $2$ gives back the real Parseval identity $\frac{1}{\pi}\int_{-\pi}^\pi f^2 = \frac{a_0^2}{2} + \sum_{n=1}^\infty(a_n^2 + b_n^2)$.

The complex Parseval identity can also be written as an isometry: the map $f \mapsto \{c_n\}$ is an isometric isomorphism from $L^2([-\pi,\pi])$ (with norm $\frac{1}{\sqrt{2\pi}}\|f\|_2$) to $\ell^2(\mathbb{Z})$ (the space of square-summable sequences).

## The Spectrum of a Function

For a real-valued periodic function $f$, its **spectrum** is the function $n \mapsto |c_n|^2$, which gives the "power" at each integer frequency $n$. By Hermitian symmetry, $|c_n|^2 = |c_{-n}|^2$, so the spectrum is symmetric in $n$. It is conventional to display only the non-negative frequencies (the **one-sided spectrum**), noting that each positive frequency $n > 0$ contributes $|c_n|^2 + |c_{-n}|^2 = 2|c_n|^2$ to the total power.

Parseval's identity says the total power equals $\int_{-\pi}^\pi |f|^2\,dx / (2\pi)$.

## Worked Example: Square Wave

The square wave $f(x) = \text{sgn}(x)$ on $(-\pi, \pi)$ has real Fourier coefficients $a_n = 0$ and $b_{2k-1} = 4/((2k-1)\pi)$. Therefore the complex coefficients are:
- $c_0 = 0$ (since $a_0 = 0$).
- For $n = 2k-1 > 0$ (odd positive $n$): $c_n = (a_n - ib_n)/2 = -i \cdot 2/((n)\pi) = -2i/(n\pi)$.
- For $n = -(2k-1) < 0$ (odd negative $n$): $c_n = \overline{c_{-n}} = 2i/((-n)\pi) = 2i/(|n|\pi)$.
- For even $n \neq 0$: $c_n = 0$.

Check: $c_n e^{inx} + c_{-n}e^{-inx} = \frac{-2i}{n\pi}e^{inx} + \frac{2i}{n\pi}e^{-inx} = \frac{2}{n\pi}\cdot 2\sin(nx) = \frac{4\sin(nx)}{n\pi}$, which matches $b_n\sin(nx) = \frac{4\sin(nx)}{n\pi}$ for odd $n$. Correct.

Parseval's identity check: $\frac{1}{2\pi}\int_{-\pi}^\pi 1^2\,dx = 1$. And $\sum_{n=-\infty}^\infty |c_n|^2 = \sum_{k=1}^\infty \frac{4}{(2k-1)^2\pi^2} + \sum_{k=1}^\infty \frac{4}{(2k-1)^2\pi^2} = \frac{8}{\pi^2}\cdot\frac{\pi^2}{8} = 1$ (using $\sum_{k=1}^\infty 1/(2k-1)^2 = \pi^2/8$). Checks out.

## Period $2L$ and General Conventions

For $f$ with period $2L$, the complex Fourier series is
$$f(x) = \sum_{n=-\infty}^\infty c_n e^{in\pi x/L}, \quad c_n = \frac{1}{2L}\int_{-L}^L f(x)e^{-in\pi x/L}\,dx.$$

Different conventions for the normalization factor are common in the literature. Some texts put $\frac{1}{2\pi}$ in the coefficient and $\frac{1}{1}$ in the series. Others use $\frac{1}{\sqrt{2\pi}}$ in both. The form here is standard in analysis; engineering texts often prefer the $1/(2\pi)$ in the series instead. These conventions lead to Parseval's identity in slightly different forms, but the content is the same.

## Connection to the Fourier Transform

Taking the limit as $L \to \infty$ in the complex series formally converts the sum to an integral. As $L \to \infty$, the frequency spacing $\pi/L \to 0$, the discrete frequencies $n\pi/L$ become a continuous variable $\xi$, and $c_n \to \hat{f}(\xi) d\xi / (2L)$ in an appropriate sense. The sum $\sum c_n e^{in\pi x/L}$ becomes $\int \hat{f}(\xi) e^{i\xi x}\,d\xi$, which is the Fourier inversion formula. This heuristic argument motivates the definition of the Fourier transform, which is made rigorous in Unit 02.
