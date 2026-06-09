# The Heaviside Step Function

The **Heaviside unit step function** (or **unit step function**) is

$$u(t-a) = \begin{cases}0 & t < a,\\ 1 & t \geq a,\end{cases}$$

where $a \geq 0$. It models a switch that turns on at time $t = a$: the quantity it multiplies is zero before $a$ and active (at its full value) after $a$.

## Laplace Transform

$$\mathcal{L}\{u(t-a)\} = \int_0^\infty e^{-st}u(t-a)\,dt = \int_a^\infty e^{-st}\,dt = \frac{e^{-as}}{s}, \qquad s > 0, \; a \geq 0.$$

For $a = 0$: $\mathcal{L}\{u(t)\} = 1/s = \mathcal{L}\{1\}$, consistent with $u(t) = 1$ for $t \geq 0$.

## Expressing Piecewise Functions

The Heaviside function allows piecewise functions to be written as single expressions:

$$f(t) = \begin{cases}g_1(t) & 0 \leq t < a,\\ g_2(t) & t \geq a,\end{cases} = g_1(t) + [g_2(t) - g_1(t)]u(t-a).$$

More generally, a function with $n$ pieces can be expressed using $n-1$ Heaviside functions:

$$f(t) = g_0(t) + \sum_{k=1}^n [g_k(t) - g_{k-1}(t)]u(t - a_k).$$

**Example.** $f(t) = \begin{cases}2 & 0 \leq t < 3,\\ t & t \geq 3\end{cases} = 2 + (t - 2)u(t-3)$.

$\mathcal{L}\{f\} = \frac{2}{s} + \mathcal{L}\{(t-3+1)u(t-3)\} = \frac{2}{s} + \mathcal{L}\{(t-3)u(t-3)\} + \mathcal{L}\{u(t-3)\}$.

By the second shifting theorem: $\mathcal{L}\{(t-3)u(t-3)\} = e^{-3s}\mathcal{L}\{t\} = e^{-3s}/s^2$. So $\mathcal{L}\{f\} = 2/s + e^{-3s}/s^2 + e^{-3s}/s$.

## Derivative in the Distribution Sense

The derivative of $u(t-a)$ in the distributional sense is $\delta(t-a)$ (the Dirac delta at $a$): this is the idealization of an instantaneous switch. Physically, the Heaviside function models a force switching on, and the Dirac delta models a sudden impulse (the derivative of the step).
