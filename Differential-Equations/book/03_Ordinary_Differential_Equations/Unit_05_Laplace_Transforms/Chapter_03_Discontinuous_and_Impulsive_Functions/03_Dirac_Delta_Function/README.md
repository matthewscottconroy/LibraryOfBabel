# The Dirac Delta Function

The **Dirac delta** $\delta(t-a)$ (for $a \geq 0$) is not a function in the classical sense but a **distribution** (generalized function) that captures the notion of an instantaneous unit impulse at time $a$. It is defined by its action on test functions: $\int_{-\infty}^\infty \delta(t-a)\phi(t)\,dt = \phi(a)$ for any continuous $\phi$.

## Motivation: The Limit of a Pulse

For $\epsilon > 0$, define the pulse $\delta_\epsilon(t-a) = 1/(2\epsilon)$ for $|t-a| < \epsilon$ and $0$ otherwise. This has unit area (total impulse = 1) and concentrates near $t = a$ as $\epsilon \to 0$. In the limit, $\delta_\epsilon \to \delta(t-a)$ in the distributional sense.

## Laplace Transform

$$\mathcal{L}\{\delta(t-a)\} = \int_0^\infty e^{-st}\delta(t-a)\,dt = e^{-as}, \qquad a \geq 0.$$

For $a = 0$: $\mathcal{L}\{\delta(t)\} = 1$.

## Application to Impulsively Forced Systems

Consider $y'' + \omega_0^2 y = \delta(t)$, $y(0) = 0$, $y'(0) = 0$ (undamped oscillator hit by a unit impulse at $t = 0$).

$(s^2 + \omega_0^2)Y = 1$, so $Y = 1/(s^2 + \omega_0^2)$. Inverting: $y(t) = \sin(\omega_0 t)/\omega_0$.

This is the **impulse response** (or Green's function for zero initial conditions): the system's response to a unit impulse. It starts at rest, receives the impulse, and then oscillates freely. The impulse imparts a velocity of $1/m$ (for mass $m = 1$) at $t = 0^+$: indeed $y(0^+) = 0$ and $y'(0^+) = 1 =$ impulse/mass.

## The Property $\delta(t-a) = u'(t-a)$

In the distributional sense, the derivative of the Heaviside step function is the Dirac delta: $u'(t-a) = \delta(t-a)$. This is consistent with $\mathcal{L}\{u'(t-a)\} = s\cdot e^{-as}/s = e^{-as} = \mathcal{L}\{\delta(t-a)\}$.

## Sifting Property and Convolution

The sifting property $\int_{-\infty}^\infty \delta(t-a)f(t)\,dt = f(a)$ shows that convolving with $\delta$ gives the identity: $(f * \delta(\cdot - a))(t) = f(t-a)$. The delta function is the identity element for convolution, just as 0 is the identity for addition and 1 for multiplication.

This is the mathematical expression of the physical idea that the impulse response (the response to $\delta$) determines the response to any forcing via convolution (the convolution theorem of the next chapter).
