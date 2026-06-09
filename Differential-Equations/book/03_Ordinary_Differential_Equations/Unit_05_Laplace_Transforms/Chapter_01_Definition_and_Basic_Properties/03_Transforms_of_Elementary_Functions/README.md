# Transforms of Elementary Functions

The basic Laplace transform table contains the transforms of the functions most commonly encountered in ODE applications. These are derived directly from the definition and serve as the building blocks for all subsequent calculations.

## Derivations

**Constant: $f(t) = 1$.** $\mathcal{L}\{1\} = \int_0^\infty e^{-st}\,dt = [-e^{-st}/s]_0^\infty = 1/s$ for $s > 0$.

**Power: $f(t) = t^n$, $n \geq 0$ integer.** Integrating by parts $n$ times (or using the gamma function $\Gamma(n+1) = n!$): $\mathcal{L}\{t^n\} = n!/s^{n+1}$ for $s > 0$. More generally, $\mathcal{L}\{t^\alpha\} = \Gamma(\alpha+1)/s^{\alpha+1}$ for $\alpha > -1$.

**Exponential: $f(t) = e^{at}$.** $\mathcal{L}\{e^{at}\} = \int_0^\infty e^{-(s-a)t}\,dt = 1/(s-a)$ for $s > a$.

**Sine and cosine.** Using $e^{ibt} = \cos bt + i\sin bt$ and $\mathcal{L}\{e^{ibt}\} = 1/(s - ib) = (s + ib)/(s^2 + b^2)$:
$$\mathcal{L}\{\cos bt\} = \frac{s}{s^2 + b^2}, \qquad \mathcal{L}\{\sin bt\} = \frac{b}{s^2 + b^2}, \qquad s > 0.$$

**Hyperbolic functions.** $\mathcal{L}\{\cosh bt\} = s/(s^2 - b^2)$ and $\mathcal{L}\{\sinh bt\} = b/(s^2 - b^2)$ for $s > |b|$.

## Using the Table with Linearity

$$\mathcal{L}\{4t^3 - 2e^{3t} + 5\cos 2t\} = \frac{4\cdot 6}{s^4} - \frac{2}{s-3} + \frac{5s}{s^2+4} = \frac{24}{s^4} - \frac{2}{s-3} + \frac{5s}{s^2+4}.$$

## Completing the Table via Operational Properties

The table is extended enormously by the operational properties of Chapter 2. The first shifting theorem gives $\mathcal{L}\{e^{at}f(t)\} = F(s-a)$, so $\mathcal{L}\{e^{at}\sin bt\} = b/((s-a)^2 + b^2)$. The transform of $t^n f(t)$ is $(-1)^n F^{(n)}(s)$. These two operations together generate the transforms of exponential-polynomial-trigonometric functions, the class that undetermined coefficients handles on the ODE side.
