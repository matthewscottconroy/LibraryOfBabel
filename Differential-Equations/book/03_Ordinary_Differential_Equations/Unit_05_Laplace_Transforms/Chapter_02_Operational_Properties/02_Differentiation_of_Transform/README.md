# Differentiation of the Transform

If $F(s) = \mathcal{L}\{f(t)\}$ exists for $s > c$, then $F$ is differentiable and

$$F'(s) = \mathcal{L}\{-tf(t)\}, \qquad F^{(n)}(s) = \mathcal{L}\{(-t)^n f(t)\} = (-1)^n\mathcal{L}\{t^n f(t)\}.$$

**Proof.** Differentiate under the integral sign (justified by uniform convergence for $s > c$):

$$\frac{d}{ds}F(s) = \frac{d}{ds}\int_0^\infty e^{-st}f(t)\,dt = \int_0^\infty \frac{\partial}{\partial s}e^{-st}f(t)\,dt = \int_0^\infty (-t)e^{-st}f(t)\,dt = \mathcal{L}\{-tf(t)\}.$$

## Applications

**Example 1.** $\mathcal{L}\{t\sin bt\}$: differentiate $\mathcal{L}\{\sin bt\} = b/(s^2 + b^2)$ with respect to $s$:

$$\mathcal{L}\{-t\sin bt\} = \frac{d}{ds}\frac{b}{s^2+b^2} = \frac{-2bs}{(s^2+b^2)^2}, \quad \text{so} \quad \mathcal{L}\{t\sin bt\} = \frac{2bs}{(s^2+b^2)^2}.$$

**Example 2.** $\mathcal{L}\{t^2 e^{3t}\}$: differentiate $\mathcal{L}\{e^{3t}\} = 1/(s-3)$ twice:

$$\mathcal{L}\{te^{3t}\} = \frac{1}{(s-3)^2}, \quad \mathcal{L}\{t^2 e^{3t}\} = \frac{2}{(s-3)^3}.$$

This extends: $\mathcal{L}\{t^n e^{at}\} = n!/(s-a)^{n+1}$.

## Inverse Transform Use

$\mathcal{L}^{-1}\{F'(s)\} = -tf(t)$ where $f = \mathcal{L}^{-1}\{F\}$. This is used when the transform to be inverted is recognizable as the derivative of a known transform.

## Relation to Moments

The formula $F^{(n)}(s)|_{s=0} = (-1)^n\int_0^\infty t^n f(t)\,dt = (-1)^n\langle t^n \rangle$ gives the moments of $f$ (as a probability distribution if $f \geq 0$ and $\int_0^\infty f\,dt = 1$). This connects the Laplace transform to moment-generating functions in probability theory.
