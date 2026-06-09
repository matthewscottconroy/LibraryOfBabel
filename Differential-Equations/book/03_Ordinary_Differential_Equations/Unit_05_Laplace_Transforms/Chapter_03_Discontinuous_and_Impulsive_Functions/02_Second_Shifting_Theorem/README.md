# The Second Shifting Theorem

The second shifting theorem (time-domain shifting or $t$-shifting) states: if $F(s) = \mathcal{L}\{f(t)\}$, then

$$\mathcal{L}\{u(t-a)f(t-a)\} = e^{-as}F(s), \qquad a \geq 0.$$

**Proof.** Substitute $\tau = t - a$:

$$\int_0^\infty e^{-st}u(t-a)f(t-a)\,dt = \int_a^\infty e^{-st}f(t-a)\,dt = \int_0^\infty e^{-s(\tau+a)}f(\tau)\,d\tau = e^{-as}F(s).$$

## Inverse Form

The inverse: $\mathcal{L}^{-1}\{e^{-as}F(s)\} = u(t-a)f(t-a)$. To use this, given $e^{-as}F(s)$: first find $f(t) = \mathcal{L}^{-1}\{F(s)\}$ by ignoring the exponential factor, then replace $t$ by $t - a$ and multiply by $u(t-a)$.

## Worked Example

Solve $y'' + 4y = u(t-\pi)$, $y(0) = 0$, $y'(0) = 0$.

Taking the Laplace transform: $(s^2 + 4)Y = e^{-\pi s}/s$, so $Y = \frac{e^{-\pi s}}{s(s^2+4)}$.

Ignore the $e^{-\pi s}$ factor: $G(s) = 1/(s(s^2+4))$. Partial fractions: $1/(s(s^2+4)) = 1/(4s) - s/(4(s^2+4))$. So $g(t) = \mathcal{L}^{-1}\{G(s)\} = \frac{1}{4} - \frac{1}{4}\cos 2t$.

By the second shifting theorem: $y(t) = u(t-\pi)g(t-\pi) = u(t-\pi)\left[\frac{1}{4} - \frac{1}{4}\cos(2(t-\pi))\right] = u(t-\pi)\cdot\frac{1-\cos 2t}{4}$.

(Since $\cos(2(t-\pi)) = \cos(2t - 2\pi) = \cos 2t$.)

For $t < \pi$: $y = 0$. For $t \geq \pi$: $y = (1 - \cos 2t)/4$. The system is at rest until the force switches on at $t = \pi$, then oscillates.

## Comparison with the First Shifting Theorem

First shifting: multiply $f(t)$ by $e^{at}$ $\leftrightarrow$ shift $F(s)$ to $F(s-a)$ (frequency shift).
Second shifting: delay $f(t)$ by $a$ (multiply by $u(t-a)$) $\leftrightarrow$ multiply $F(s)$ by $e^{-as}$ (exponential factor in $s$).

These are dual operations: the first is translation in the frequency domain, the second is translation in the time domain.
