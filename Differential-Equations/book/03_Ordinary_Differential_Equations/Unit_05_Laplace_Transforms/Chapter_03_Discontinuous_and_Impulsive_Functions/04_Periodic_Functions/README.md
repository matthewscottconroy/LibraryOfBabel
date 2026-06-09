# Periodic Functions and Their Transforms

A function $f(t)$ is **periodic with period $T > 0$** if $f(t+T) = f(t)$ for all $t \geq 0$. Periodic forcing arises in mechanical and electrical systems driven by sinusoidal or non-sinusoidal repeating forces.

## The Transform Formula

**Theorem.** If $f$ is piecewise continuous, periodic with period $T$, and of exponential order, then

$$\mathcal{L}\{f(t)\} = \frac{1}{1-e^{-Ts}}\int_0^T e^{-st}f(t)\,dt, \qquad s > 0.$$

**Proof.** Split the integral over each period:

$$\int_0^\infty e^{-st}f(t)\,dt = \sum_{n=0}^\infty\int_{nT}^{(n+1)T}e^{-st}f(t)\,dt.$$

In each integral, substitute $t = \tau + nT$ and use $f(\tau + nT) = f(\tau)$:

$$= \sum_{n=0}^\infty e^{-nsT}\int_0^T e^{-s\tau}f(\tau)\,d\tau = \frac{1}{1-e^{-sT}}\int_0^T e^{-st}f(t)\,dt,$$

using the geometric series $\sum_{n=0}^\infty e^{-nsT} = 1/(1 - e^{-sT})$ for $s > 0$.

## Example: Square Wave

The square wave $f(t) = \begin{cases}1 & 0 \leq t < T/2,\\-1 & T/2 \leq t < T\end{cases}$ (period $T$) has transform:

$$\int_0^T e^{-st}f(t)\,dt = \int_0^{T/2}e^{-st}\,dt - \int_{T/2}^T e^{-st}\,dt = \frac{1}{s}(1 - e^{-sT/2}) - \frac{1}{s}(e^{-sT/2} - e^{-sT}) = \frac{1}{s}(1 - e^{-sT/2})^2.$$

So $\mathcal{L}\{f\} = \frac{(1-e^{-sT/2})^2}{s(1-e^{-sT})} = \frac{(1-e^{-sT/2})^2}{s(1-e^{-sT/2})(1+e^{-sT/2})} = \frac{1-e^{-sT/2}}{s(1+e^{-sT/2})} = \frac{1}{s}\tanh(sT/4)$.

## Connection to Fourier Series

A periodic function has a Fourier series $f(t) = \sum_{n=-\infty}^\infty c_n e^{2\pi i n t/T}$. The Laplace transform of $e^{2\pi i n t/T}$ (restricted to $t \geq 0$) is $1/(s - 2\pi i n/T)$. Summing: $\mathcal{L}\{f\}$ is formally the sum $\sum c_n/(s - 2\pi i n/T)$, which converges for $\text{Re}(s) > 0$ and gives a meromorphic function in $s$ with poles at $s = 2\pi i n/T$ on the imaginary axis. The periodic formula $\mathcal{L}\{f\} = (\int_0^T e^{-st}f\,dt)/(1-e^{-sT})$ is a closed form for this sum.
