# Linearity of the Laplace Transform

The Laplace transform is a linear operator: for any constants $\alpha, \beta$ and functions $f, g$ whose transforms exist,

$$\mathcal{L}\{\alpha f(t) + \beta g(t)\} = \alpha\,\mathcal{L}\{f(t)\} + \beta\,\mathcal{L}\{g(t)\}.$$

This follows immediately from the linearity of the integral: $\int_0^\infty e^{-st}(\alpha f + \beta g)\,dt = \alpha\int_0^\infty e^{-st}f\,dt + \beta\int_0^\infty e^{-st}g\,dt$.

## Consequences for Problem Solving

Linearity means that one can transform each term of an ODE separately, use the basic table for individual functions, and combine the results. It is also the basis for the superposition principle in the Laplace domain: if $Y_1(s)$ corresponds to forcing $G_1(s)$ and $Y_2(s)$ to $G_2(s)$, then $\alpha Y_1 + \beta Y_2$ corresponds to $\alpha G_1 + \beta G_2$.

**Example.** $\mathcal{L}\{3e^{2t} - 5\sin(3t)\} = 3\mathcal{L}\{e^{2t}\} - 5\mathcal{L}\{\sin 3t\} = \frac{3}{s-2} - \frac{15}{s^2+9}$.

## Linearity and the Inverse Transform

The inverse Laplace transform $\mathcal{L}^{-1}$ is also linear: $\mathcal{L}^{-1}\{\alpha F + \beta G\} = \alpha\mathcal{L}^{-1}\{F\} + \beta\mathcal{L}^{-1}\{G\}$. This allows partial fractions decomposition of $Y(s)$ into a sum of terms, each recognizable in the table, and then term-by-term inversion.

Linearity is the essential reason the Laplace transform is a useful tool: it transforms the linear structure of the differential equation (which is what makes the problem tractable) into the linear structure of algebraic equations.
