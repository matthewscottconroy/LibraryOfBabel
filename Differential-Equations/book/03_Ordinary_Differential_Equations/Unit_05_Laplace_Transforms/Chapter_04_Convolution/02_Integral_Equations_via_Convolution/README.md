# Integral Equations via Convolution

The Laplace transform and convolution theorem provide a powerful method for solving Volterra integral equations of the convolution type:

$$y(t) + \int_0^t k(t-\tau)y(\tau)\,d\tau = f(t) \qquad \text{(Volterra equation of the second kind),}$$

$$\int_0^t k(t-\tau)y(\tau)\,d\tau = f(t) \qquad \text{(Volterra equation of the first kind).}$$

## Solution Method

For the second kind: write $y + k*y = f$. Taking the Laplace transform: $Y + K \cdot Y = F$, so $Y(1 + K) = F$ and $Y = F/(1+K)$. Inverting: $y = \mathcal{L}^{-1}\{F/(1+K)\}$.

For the first kind: $k*y = f$, so $KY = F$, giving $Y = F/K$ and $y = \mathcal{L}^{-1}\{F/K\}$.

## Worked Example: Volterra Equation of the Second Kind

Solve $y(t) + \int_0^t e^{-(t-\tau)}y(\tau)\,d\tau = t$.

Here $k(t) = e^{-t}$ and $f(t) = t$. $K(s) = 1/(s+1)$ and $F(s) = 1/s^2$. Then

$$Y = \frac{1/s^2}{1 + 1/(s+1)} = \frac{1/s^2}{(s+2)/(s+1)} = \frac{s+1}{s^2(s+2)}.$$

Partial fractions: $\frac{s+1}{s^2(s+2)} = \frac{A}{s} + \frac{B}{s^2} + \frac{C}{s+2}$. Clearing denominators: $s+1 = As(s+2) + B(s+2) + Cs^2$. Setting $s = 0$: $1 = 2B$, $B = 1/2$. Setting $s = -2$: $-1 = 4C$, $C = -1/4$. Matching $s^2$: $0 = A + C$, $A = 1/4$.

$$y(t) = \frac{1}{4} + \frac{t}{2} - \frac{e^{-2t}}{4}.$$

## Abel's Integral Equation

The classic Abel integral equation $\int_0^t \frac{y(\tau)}{\sqrt{t-\tau}}\,d\tau = f(t)$ is a Volterra equation of the first kind with $k(t) = t^{-1/2}$. Using $\mathcal{L}\{t^{-1/2}\} = \sqrt{\pi/s}$: $\sqrt{\pi/s}\cdot Y = F$, so $Y = F\sqrt{s/\pi}$ and $y = \mathcal{L}^{-1}\{F(s)\sqrt{s/\pi}\} = \frac{1}{\pi}\frac{d}{dt}\int_0^t \frac{f(\tau)}{\sqrt{t-\tau}}\,d\tau$.

Abel's equation arises in the tautochrone problem (finding the curve for which the period of oscillation is independent of amplitude), which Niels Abel solved in 1823 using this integral equation.
