# The Discrete Fourier Transform: Definition and Properties

A sequence $(x_0, x_1, \ldots, x_{N-1})$ of $N$ complex numbers represents a sampled signal. Just as the continuous Fourier transform decomposes a function on $\mathbb{R}$ into sinusoidal components, the Discrete Fourier Transform (DFT) decomposes a finite sequence into $N$ discrete sinusoidal components at uniformly spaced frequencies.

## Definition

Let $\omega_N = e^{2\pi i/N}$, the primitive $N$-th root of unity. The **Discrete Fourier Transform** of $(x_0, \ldots, x_{N-1}) \in \mathbb{C}^N$ is the sequence $(X_0, \ldots, X_{N-1}) \in \mathbb{C}^N$ defined by
$$X_k = \sum_{n=0}^{N-1} x_n\,\omega_N^{-kn} = \sum_{n=0}^{N-1} x_n\,e^{-2\pi ikn/N}, \quad k = 0, 1, \ldots, N-1.$$

The **inverse DFT (IDFT)** is
$$x_n = \frac{1}{N}\sum_{k=0}^{N-1} X_k\,\omega_N^{kn} = \frac{1}{N}\sum_{k=0}^{N-1} X_k\,e^{2\pi ikn/N}, \quad n = 0, 1, \ldots, N-1.$$

The factor $1/N$ in the inverse (rather than $1/\sqrt{N}$ in both) is the standard engineering convention. Some texts use $1/\sqrt{N}$ in both, making the DFT unitary.

## The DFT Matrix

The DFT can be written as a matrix multiplication: $\mathbf{X} = F_N \mathbf{x}$, where
$$(F_N)_{kn} = \omega_N^{-kn} = e^{-2\pi ikn/N}, \quad k, n = 0, 1, \ldots, N-1.$$

For $N = 4$, with $\omega_4 = i$:
$$F_4 = \begin{pmatrix} 1 & 1 & 1 & 1 \\ 1 & -i & -1 & i \\ 1 & -1 & 1 & -1 \\ 1 & i & -1 & -i \end{pmatrix}.$$

The matrix $F_N$ is a **Vandermonde matrix** in the roots of unity. Its rows and columns are orthogonal: $(F_N)_{k\cdot}(F_N)^*_{m\cdot} = N\delta_{km}$, so $F_N F_N^* = N I_N$, making $(1/\sqrt{N})F_N$ a unitary matrix.

## Orthogonality of the DFT Basis

The DFT is grounded in the orthogonality of the discrete exponential sequences $e_k = (\omega_N^{kn})_{n=0}^{N-1}$:
$$\sum_{n=0}^{N-1} \omega_N^{kn}\overline{\omega_N^{mn}} = \sum_{n=0}^{N-1} \omega_N^{(k-m)n} = \begin{cases} N & k \equiv m \pmod{N} \\ 0 & k \not\equiv m \pmod{N}. \end{cases}$$

The last equality follows from the geometric series: $\sum_{n=0}^{N-1} r^n = (r^N - 1)/(r-1)$ for $r \neq 1$. If $r = \omega_N^{k-m}$ with $k \neq m$, then $r^N = \omega_N^{N(k-m)} = 1$, so the sum is $0$.

## Frequency Interpretation

The sequence $x_n$ is thought of as $N$ samples of a periodic function, with sample $x_n$ taken at time $t_n = n/N$ (normalizing the period to $1$). The DFT output $X_k$ represents the amplitude and phase at **discrete frequency** $k$ cycles per period (or $k/N$ cycles per sample). Specifically:

- $X_0 = \sum x_n$: the sum (DC component, zero frequency).
- $X_1$: amplitude at frequency $1/N$ (one cycle per period).
- $X_{N/2}$: amplitude at the Nyquist frequency $1/2$ (for even $N$).
- $X_k$ for $k > N/2$: frequencies above Nyquist, which alias back to negative frequencies. Specifically, $X_{N-k}$ corresponds to frequency $-k/N$.

**Aliasing:** Because we sample at $N$ points per period, we can only represent frequencies up to $N/2$ cycles per period. Higher frequencies "fold" into lower ones — the frequency $k + N$ is indistinguishable from frequency $k$ in the sampled signal.

## Worked Example: $N = 4$

Let $\mathbf{x} = (1, 0, -1, 0)$.

$$X_0 = 1 + 0 + (-1) + 0 = 0.$$
$$X_1 = 1 + 0\cdot(-i) + (-1)(-1) + 0\cdot i = 1 + 0 + 1 + 0 = 2.$$
$$X_2 = 1 + 0\cdot(-1) + (-1)(1) + 0\cdot(-1) = 1 + 0 - 1 + 0 = 0.$$
$$X_3 = 1 + 0\cdot i + (-1)(-1) + 0\cdot(-i) = 1 + 0 + 1 + 0 = 2.$$

So $\mathbf{X} = (0, 2, 0, 2)$. The nonzero components at $k = 1$ and $k = 3$ reflect the fact that $x_n = -\sin(2\pi n/4 - \pi/2) = \cos(2\pi n/4)$... more precisely, $x_n = 1, 0, -1, 0$ is $(x_0, x_1, x_2, x_3)$ which matches $\cos(2\pi n/4)\cdot 1$ evaluated at $n = 0,1,2,3$: $\cos(0) = 1$, $\cos(\pi/2) = 0$, $\cos(\pi) = -1$, $\cos(3\pi/2) = 0$. So $\mathbf{x}$ is a cosine at frequency $k=1$, reflected in $X_1 = X_3 = 2$ (Hermitian symmetry).

**Verify IDFT:** $x_0 = \frac{1}{4}(0 + 2 + 0 + 2) = 1$. $x_1 = \frac{1}{4}(0 + 2i + 0 + 2(-i)) = 0$. Correct.

## Properties of the DFT

**Linearity:** $\text{DFT}[\alpha\mathbf{x} + \beta\mathbf{y}] = \alpha\text{DFT}[\mathbf{x}] + \beta\text{DFT}[\mathbf{y}]$.

**Circular shift:** If $\mathbf{y} = (x_{n-m \bmod N})_{n=0}^{N-1}$ (shift by $m$), then $Y_k = \omega_N^{-km}X_k$.

**Circular convolution:** Define $(x \circledast y)_n = \sum_{m=0}^{N-1} x_m y_{(n-m)\bmod N}$. Then $\text{DFT}[x \circledast y]_k = X_k Y_k$ (pointwise product). This is the discrete analog of the convolution theorem.

**Parseval's theorem:** $\sum_{n=0}^{N-1}|x_n|^2 = \frac{1}{N}\sum_{k=0}^{N-1}|X_k|^2$.

**Hermitian symmetry:** If $\mathbf{x}$ is real, then $X_{N-k} = \overline{X_k}$ (equivalently, $X_k = \overline{X_{N-k}}$). Only the first $\lfloor N/2\rfloor + 1$ outputs are independent.

## Relationship to the Continuous Fourier Transform

Suppose $f : [0,1] \to \mathbb{C}$ is sampled at $N$ equally spaced points: $x_n = f(n/N)$. The DFT output $X_k = \sum_{n=0}^{N-1} f(n/N)e^{-2\pi ikn/N}$ is a Riemann sum approximation to
$$N\int_0^1 f(t)e^{-2\pi ikt}\,dt = N\hat{f}(k),$$
where $\hat{f}(k)$ is the $k$-th Fourier coefficient of $f$ (in the period-$1$ convention). Thus $X_k \approx N\hat{f}(k)$ for large $N$, and the DFT approximates the Fourier series coefficients by Riemann sums.
