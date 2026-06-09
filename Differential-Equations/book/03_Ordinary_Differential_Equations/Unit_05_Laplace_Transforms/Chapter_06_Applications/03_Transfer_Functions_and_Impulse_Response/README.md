# Transfer Functions and Impulse Response

For a linear time-invariant system described by $L[y] = g(t)$ with zero initial conditions, the Laplace transform gives $H(s)Y(s) = G(s)$, defining the **transfer function** $H(s) = 1/L(s)$ where $L(s) = as^2 + bs + c$ is the characteristic polynomial of $L$. The transfer function encodes everything about the system's response: $Y(s) = H(s)G(s)$.

## The Impulse Response

The **impulse response** $h(t) = \mathcal{L}^{-1}\{H(s)\}$ is the system's output when the input is $g(t) = \delta(t)$ (a unit impulse). Since $\mathcal{L}\{\delta\} = 1$, the output transform is $Y = H(s)\cdot 1 = H(s)$, so $y = h(t)$.

By the convolution theorem, the response to any input $g(t)$ is

$$y(t) = (h * g)(t) = \int_0^t h(t-\tau)g(\tau)\,d\tau.$$

The impulse response completely characterizes the system: once $h(t)$ is known, the response to any causal input can be computed by convolution.

## Poles and System Behavior

The poles of $H(s)$ (zeros of $L(s)$, i.e., the characteristic roots) determine the natural behavior of the system:
- Poles in the left half-plane ($\text{Re}(s) < 0$): stable system, impulse response decays.
- Poles on the imaginary axis: marginally stable, impulse response oscillates without decay.
- Poles in the right half-plane: unstable, impulse response grows.

The **frequency response** is $H(i\omega)$ (the transfer function evaluated on the imaginary axis). Its magnitude $|H(i\omega)|$ is the gain at frequency $\omega$ and its argument $\angle H(i\omega)$ is the phase shift. For the spring-mass system $H(s) = 1/(ms^2 + \gamma s + k)$:

$$|H(i\omega)| = \frac{1}{\sqrt{(k-m\omega^2)^2 + \gamma^2\omega^2}},$$

the steady-state amplitude ratio derived in Chapter 4 of Unit 3 via undetermined coefficients. The Laplace/transfer function perspective unifies all of this in a single formula.

## Control Theory Connection

In control theory, the closed-loop transfer function of a feedback system with plant $P(s)$, controller $C(s)$, and unity feedback is $T(s) = C(s)P(s)/(1 + C(s)P(s))$. Stability is determined by the poles of $T(s)$, which are the zeros of $1 + C(s)P(s) = 0$ — the characteristic equation of the closed-loop system. The entire framework of classical control (Bode plots, Nyquist stability criterion, root locus) is built on the transfer function concept introduced here.
