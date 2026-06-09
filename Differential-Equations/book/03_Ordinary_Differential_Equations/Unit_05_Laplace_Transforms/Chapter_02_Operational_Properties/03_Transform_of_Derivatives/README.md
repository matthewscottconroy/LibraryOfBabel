# Transform of Derivatives

The transform of $f'(t)$ is the single most important operational property for applying the Laplace method to ODEs:

$$\mathcal{L}\{f'(t)\} = sF(s) - f(0).$$

**Proof.** Integration by parts:

$$\int_0^\infty e^{-st}f'(t)\,dt = \left[e^{-st}f(t)\right]_0^\infty + s\int_0^\infty e^{-st}f(t)\,dt = 0 - f(0) + sF(s).$$

(The boundary term at $\infty$ vanishes because $f$ is of exponential order and $\text{Re}(s) > c$.)

## Higher Derivatives

Applying the formula repeatedly:

$$\mathcal{L}\{f''(t)\} = s\mathcal{L}\{f'(t)\} - f'(0) = s(sF(s) - f(0)) - f'(0) = s^2F(s) - sf(0) - f'(0).$$

In general:

$$\mathcal{L}\{f^{(n)}(t)\} = s^n F(s) - s^{n-1}f(0) - s^{n-2}f'(0) - \cdots - f^{(n-1)}(0).$$

## Applying to an IVP

For the IVP $y'' - 3y' + 2y = e^{4t}$, $y(0) = 1$, $y'(0) = 0$:

$$\mathcal{L}\{y''\} - 3\mathcal{L}\{y'\} + 2\mathcal{L}\{y\} = \mathcal{L}\{e^{4t}\}.$$

$$(s^2Y - s\cdot 1 - 0) - 3(sY - 1) + 2Y = \frac{1}{s-4}.$$

$$(s^2 - 3s + 2)Y - s + 3 = \frac{1}{s-4}.$$

$$Y = \frac{s - 3}{s^2 - 3s + 2} + \frac{1}{(s-4)(s^2 - 3s + 2)} = \frac{s - 3}{(s-1)(s-2)} + \frac{1}{(s-4)(s-1)(s-2)}.$$

Partial fractions decompose this into terms of the form $A/(s-r)$, each inverted by $\mathcal{L}^{-1}\{1/(s-r)\} = e^{rt}$.

## Why Initial Conditions Appear Automatically

The formula $\mathcal{L}\{f'\} = sF - f(0)$ automatically incorporates the initial condition $f(0)$ into the transform. This is the key advantage of the Laplace method over undetermined coefficients: there is no need to first find the general solution and then apply initial conditions as a separate step. The initial conditions are built into the algebra from the start.
