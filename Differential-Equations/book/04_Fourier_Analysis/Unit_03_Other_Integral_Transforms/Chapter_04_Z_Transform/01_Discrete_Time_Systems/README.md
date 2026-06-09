# Discrete-Time Systems and the Z-Transform

A **discrete-time system** takes a sequence of input values $(x_n)$ and produces a sequence of output values $(y_n)$ according to a rule. The most natural and important class are **linear, time-invariant (LTI) systems** governed by a linear recurrence with constant coefficients:
$$y_n + a_1 y_{n-1} + \cdots + a_p y_{n-p} = b_0 x_n + b_1 x_{n-1} + \cdots + b_q x_{n-q}.$$
The Z-transform converts this into an algebraic equation, making the analysis of such systems tractable.

## Definition of the Z-Transform

**Definition.** The **(unilateral) Z-transform** of a sequence $(x_n)_{n=0}^\infty$ is the formal power series
$$X(z) = \mathcal{Z}[(x_n)](z) = \sum_{n=0}^\infty x_n\,z^{-n},$$
for complex $z$ where the series converges absolutely. The series converges in an annulus $|z| > R$ for some $R \geq 0$ called the **radius of convergence**.

The **bilateral Z-transform** of $(x_n)_{n \in \mathbb{Z}}$ is $X(z) = \sum_{n=-\infty}^\infty x_n z^{-n}$, converging in a strip $R_1 < |z| < R_2$.

## Region of Convergence

The series $\sum_{n=0}^\infty x_n z^{-n}$ converges absolutely when $\sum |x_n||z|^{-n} < \infty$. This is a power series in $z^{-1}$, converging for $|z| > R$ where $R = \limsup_{n\to\infty}|x_n|^{1/n}$.

- If $x_n = a^n$ (geometric), then $R = |a|$ and $X(z) = \sum (a/z)^n = z/(z-a)$ for $|z| > |a|$.
- If $x_n = 0$ for $n < 0$ and $x_n$ is bounded, then $R \leq 1$.

The ROC is always of the form $|z| > R$ for the one-sided transform (or an annulus for the bilateral transform).

## Key Properties

**Linearity:** $\mathcal{Z}[\alpha x_n + \beta y_n] = \alpha X(z) + \beta Y(z)$.

**Time delay (shift right):** $\mathcal{Z}[x_{n-1}](z) = z^{-1}X(z) + x_{-1}$ (for unilateral; with zero initial condition $x_{-1} = 0$: $\mathcal{Z}[x_{n-1}] = z^{-1}X(z)$). More generally: $\mathcal{Z}[x_{n-k}] = z^{-k}X(z) + \sum_{j=1}^k x_{-j}z^{-k+j}$.

**Time advance (shift left):** $\mathcal{Z}[x_{n+1}](z) = zX(z) - zx_0$.

**Multiplication by $n$:** $\mathcal{Z}[nx_n](z) = -z\frac{d}{dz}X(z)$.

**Multiplication by $a^n$:** $\mathcal{Z}[a^n x_n](z) = X(z/a)$.

**Convolution:** $\mathcal{Z}[(x * y)_n](z) = X(z)\cdot Y(z)$, where $(x*y)_n = \sum_{k=0}^n x_k y_{n-k}$.

**Initial value theorem:** $\lim_{z\to\infty}X(z) = x_0$.

**Final value theorem:** If all poles of $(z-1)X(z)$ are inside the unit circle, then $\lim_{n\to\infty}x_n = \lim_{z\to 1}(z-1)X(z)$.

## Standard Transform Pairs

| Sequence $x_n$ ($n \geq 0$) | $X(z)$ | ROC |
|---|---|---|
| $\delta_n$ (impulse at $0$) | $1$ | All $z$ |
| $\delta_{n-k}$ | $z^{-k}$ | $z \neq 0$ |
| $u_n$ (unit step) | $z/(z-1)$ | $|z| > 1$ |
| $a^n$ | $z/(z-a)$ | $|z| > |a|$ |
| $na^n$ | $az/(z-a)^2$ | $|z| > |a|$ |
| $\cos(\omega_0 n)$ | $z(z - \cos\omega_0)/(z^2 - 2z\cos\omega_0 + 1)$ | $|z| > 1$ |
| $\sin(\omega_0 n)$ | $z\sin\omega_0/(z^2 - 2z\cos\omega_0 + 1)$ | $|z| > 1$ |

## Solving a Difference Equation

**Example.** Solve $y_n - 0.5y_{n-1} = x_n$ with $y_{-1} = 0$ and $x_n = u_n$ (unit step: $x_n = 1$ for $n \geq 0$).

Take the Z-transform: $Y(z) - 0.5z^{-1}Y(z) = X(z) = z/(z-1)$.

$(1 - 0.5z^{-1})Y(z) = \frac{z}{z-1} \implies Y(z) = \frac{z}{z-1}\cdot\frac{1}{1-0.5z^{-1}} = \frac{z}{z-1}\cdot\frac{z}{z-0.5} = \frac{z^2}{(z-1)(z-0.5)}.$

Partial fractions (dividing by $z$ to find $Y(z)/z$):
$$\frac{Y(z)}{z} = \frac{z}{(z-1)(z-0.5)} = \frac{A}{z-1} + \frac{B}{z-0.5}.$$
$A = \frac{z}{z-0.5}\big|_{z=1} = 2$, $B = \frac{z}{z-1}\big|_{z=0.5} = -1$.

So $Y(z) = \frac{2z}{z-1} - \frac{z}{z-0.5}$, giving $y_n = 2\cdot 1^n - (0.5)^n = 2 - (0.5)^n$ for $n \geq 0$.

**Verification:** $y_0 = 2 - 1 = 1$, $x_0 = 1$, $y_{-1} = 0$, so $y_0 - 0.5\cdot 0 = 1 = x_0$. Check. $y_1 = 2 - 0.5 = 1.5$, $y_1 - 0.5y_0 = 1.5 - 0.5 = 1 = x_1$. Check.

## Transfer Function and Stability

The **transfer function** of an LTI system with difference equation $\sum_{k=0}^p a_k y_{n-k} = \sum_{k=0}^q b_k x_{n-k}$ is
$$H(z) = \frac{Y(z)}{X(z)}\bigg|_{\text{zero ICs}} = \frac{\sum_{k=0}^q b_k z^{-k}}{\sum_{k=0}^p a_k z^{-k}} = \frac{B(z)}{A(z)}.$$

**Stability:** The system is **BIBO stable** (bounded-input-bounded-output stable) if and only if all poles of $H(z)$ lie strictly inside the unit circle $|z| < 1$. A pole at $z = p$ contributes a term $\sim p^n$ to the impulse response; this decays if $|p| < 1$ and grows if $|p| > 1$.

**Frequency response:** On the unit circle $z = e^{i\omega}$, the transfer function becomes $H(e^{i\omega})$, the **frequency response**. This is related to the DFT: evaluating $H$ at the $N$-th roots of unity gives the DFT of the impulse response.

## Relationship to the Laplace Transform and DFT

**Laplace $\to$ Z.** Sampling at rate $1/T$ seconds and mapping $z = e^{sT}$ converts the (bilateral) Laplace transform to the Z-transform. A pole at $s = -\alpha$ (with $\alpha > 0$, stable in continuous time) maps to $z = e^{-\alpha T}$ with $|z| = e^{-\alpha T} < 1$ (stable in discrete time). This is the basis for the **bilinear transform** and other continuous-to-discrete design methods for digital filters.

**DFT $\to$ Z.** Evaluating the Z-transform on the unit circle at $z = e^{2\pi ik/N}$ gives the DFT: $X(e^{2\pi ik/N}) = \sum_{n=0}^{N-1} x_n e^{-2\pi ikn/N} = X_k$. The Z-transform is the analytic continuation of the discrete-time Fourier transform from the unit circle to the complex plane.
