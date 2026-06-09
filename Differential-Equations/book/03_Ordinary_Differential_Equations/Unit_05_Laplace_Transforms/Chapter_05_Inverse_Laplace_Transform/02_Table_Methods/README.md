# Table Methods for Inverse Laplace Transforms

Beyond partial fractions, the operational properties, combined with a comprehensive table, allow many transforms to be inverted by pattern recognition and algebraic manipulation.

## Completing the Square

When the denominator is a quadratic $s^2 + bs + c$ with no real roots (discriminant $< 0$), complete the square: $s^2 + bs + c = (s + b/2)^2 + (c - b^2/4)$. Write $\alpha = -b/2$ and $\beta = \sqrt{c - b^2/4}$. Then $\mathcal{L}^{-1}\{1/((s-\alpha)^2+\beta^2)\} = e^{\alpha t}\sin(\beta t)/\beta$.

**Example.** $\mathcal{L}^{-1}\{(2s+3)/(s^2+6s+13)\}$: complete the square: $s^2+6s+13 = (s+3)^2+4$. Write $2s+3 = 2(s+3)-3$. So $\frac{2s+3}{(s+3)^2+4} = 2\frac{s+3}{(s+3)^2+4} - \frac{3}{2}\frac{2}{(s+3)^2+4}$. Invert: $2e^{-3t}\cos 2t - \frac{3}{2}e^{-3t}\sin 2t$.

## Using the Differentiation Property

If $F(s)$ is a known transform and one needs $\mathcal{L}^{-1}\{F'(s)\}$: since $F'(s) = \mathcal{L}\{-tf(t)\}$, we have $\mathcal{L}^{-1}\{F'(s)\} = -tf(t)$.

**Example.** $\mathcal{L}^{-1}\!\left\{\frac{2s}{(s^2+4)^2}\right\}$: recognize $(s^2+4)^{-2}$ as $(-1/2)$ times the derivative of $(s^2+4)^{-1}$ with respect to $s$. Since $\mathcal{L}^{-1}\{1/(s^2+4)\} = \sin(2t)/2$, we get $\mathcal{L}^{-1}\{-d/ds(1/(s^2+4))\} = t\sin(2t)/2$, so $\mathcal{L}^{-1}\{2s/(s^2+4)^2\} = t\sin(2t)/2$.

## The Integration Property for Inversion

If $F(s)/s$ is the transform, then the inverse is $\int_0^t f(\tau)\,d\tau$. Conversely, if $\mathcal{L}^{-1}\{F(s)\} = f(t)$, then $\mathcal{L}^{-1}\{F(s)/s\} = \int_0^t f(\tau)\,d\tau$.

This is used when the transform to be inverted has an extra factor of $1/s$ compared to a known entry.

## A Systematic Approach

When facing $Y(s)$:
1. Factor the denominator. Are all roots known?
2. If rational and proper: partial fractions.
3. If quadratic denominator with complex roots: complete the square, use first shifting theorem.
4. If there is a factor $e^{-as}$: second shifting theorem.
5. If $Y(s) = F(s)G(s)$: convolution theorem.
6. If $Y(s)$ is a derivative or integral of a known transform: differentiation/integration property.

Working through this hierarchy identifies the appropriate method for any transform arising in ODE applications.
