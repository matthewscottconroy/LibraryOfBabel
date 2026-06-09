# Transform of Integrals

Integration in the $t$-domain corresponds to division by $s$ in the $s$-domain:

$$\mathcal{L}\!\left\{\int_0^t f(\tau)\,d\tau\right\} = \frac{F(s)}{s}.$$

**Proof.** Let $g(t) = \int_0^t f(\tau)\,d\tau$. Then $g' = f$ and $g(0) = 0$. Applying the derivative formula: $\mathcal{L}\{g'\} = s\mathcal{L}\{g\} - g(0) = sG(s)$. But $\mathcal{L}\{g'\} = F(s)$, so $G(s) = F(s)/s$.

## Applications

**Example 1.** $\mathcal{L}\!\left\{\int_0^t \sin\tau\,d\tau\right\} = \mathcal{L}\{1 - \cos t\} = \frac{1}{s} - \frac{s}{s^2+1} = \frac{1}{s(s^2+1)} = \frac{\mathcal{L}\{\sin t\}}{s}$. Verified.

**Example 2.** $\mathcal{L}\!\left\{\int_0^t e^{-\tau}\,d\tau\right\} = \frac{1/(s+1)}{s} = \frac{1}{s(s+1)}$. Alternatively: $\int_0^t e^{-\tau}\,d\tau = 1 - e^{-t}$, and $\mathcal{L}\{1 - e^{-t}\} = 1/s - 1/(s+1) = 1/(s(s+1))$. Consistent.

## Repeated Integration

The formula iterates: $\mathcal{L}\left\{\int_0^t\int_0^{t_1}f(\tau)\,d\tau\,dt_1\right\} = F(s)/s^2$.

## Connection to Integral Equations

The integral formula converts Volterra integral equations of the first kind $\int_0^t f(\tau)\,d\tau = g(t)$ into algebraic equations $F(s)/s = G(s)$, giving $F(s) = sG(s)$ and $f = \mathcal{L}^{-1}\{sG(s)\} = g'(t) + g(0)\delta(t)$. More generally, Volterra equations of the second kind $y(t) + \int_0^t k(t-\tau)y(\tau)\,d\tau = f(t)$ become $(1 + K(s))Y(s) = F(s)$ via the convolution theorem (Chapter 4).
