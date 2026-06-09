# The Convolution Theorem

**Theorem (Convolution).** If $F(s) = \mathcal{L}\{f(t)\}$ and $G(s) = \mathcal{L}\{g(t)\}$, then

$$\mathcal{L}\{(f*g)(t)\} = F(s)G(s), \quad \text{where} \quad (f*g)(t) = \int_0^t f(t-\tau)g(\tau)\,d\tau.$$

**Proof.** Compute $\mathcal{L}\{f*g\} = \int_0^\infty e^{-st}\int_0^t f(t-\tau)g(\tau)\,d\tau\,dt$. Change the order of integration (the region $0 \leq \tau \leq t < \infty$ becomes $0 \leq \tau < \infty$, $\tau \leq t < \infty$):

$$= \int_0^\infty g(\tau)\int_\tau^\infty e^{-st}f(t-\tau)\,dt\,d\tau.$$

In the inner integral, substitute $u = t - \tau$:

$$= \int_0^\infty g(\tau)\int_0^\infty e^{-s(\tau+u)}f(u)\,du\,d\tau = \int_0^\infty g(\tau)e^{-s\tau}\,d\tau\cdot\int_0^\infty e^{-su}f(u)\,du = G(s)F(s).$$

## Commutativity

$(f*g)(t) = (g*f)(t)$: changing variables $u = t - \tau$ in the convolution integral. Correspondingly, $F(s)G(s) = G(s)F(s)$.

## Worked Example: Finding the Inverse Transform

Find $\mathcal{L}^{-1}\!\left\{\frac{1}{s^2(s^2+\omega^2)}\right\}$.

Write $1/(s^2(s^2+\omega^2)) = (1/s^2)\cdot(1/(s^2+\omega^2))$. We have $\mathcal{L}^{-1}\{1/s^2\} = t$ and $\mathcal{L}^{-1}\{1/(s^2+\omega^2)\} = \sin(\omega t)/\omega$. By the convolution theorem:

$$\mathcal{L}^{-1}\!\left\{\frac{1}{s^2(s^2+\omega^2)}\right\} = t * \frac{\sin\omega t}{\omega} = \frac{1}{\omega}\int_0^t (t-\tau)\sin\omega\tau\,d\tau.$$

Evaluating: $= \frac{1}{\omega}\left[t\int_0^t\sin\omega\tau\,d\tau - \int_0^t\tau\sin\omega\tau\,d\tau\right] = \frac{1}{\omega}\left[t\cdot\frac{1-\cos\omega t}{\omega} - \frac{\sin\omega t - \omega t\cos\omega t}{\omega^2}\right] = \frac{\omega t - \sin\omega t}{\omega^3}$.

This could also be obtained by partial fractions: $1/(s^2(s^2+\omega^2)) = 1/(\omega^2 s^2) - 1/(\omega^2(s^2+\omega^2))$, giving $t/\omega^2 - \sin(\omega t)/\omega^3 = (\omega t - \sin\omega t)/\omega^3$. Both methods agree.

## Relation to the Green's Function

For the IVP $y'' + py' + qy = g(t)$, $y(0) = y'(0) = 0$, the Laplace transform gives $Y(s) = G(s)/(s^2+ps+q) = H(s)G(s)$, so $y(t) = h(t)*g(t) = \int_0^t h(t-\tau)g(\tau)\,d\tau$, where $h(t) = \mathcal{L}^{-1}\{1/(s^2+ps+q)\}$ is the impulse response. The function $h(t-\tau)$ is the **Green's function** of the operator, giving the response at time $t$ to a unit impulse at time $\tau$. The convolution integral accumulates these responses over all impulses delivered by $g$.
