# Inverse Trigonometric and Hyperbolic Functions

The inverse trigonometric functions — $\arcsin z$, $\arccos z$, $\arctan z$, and their hyperbolic counterparts — extend to the complex plane via the complex logarithm. Because the trigonometric functions are not injective on $\mathbb{C}$ (they are periodic), their inverses are necessarily multivalued, with branch points and branch cuts that can be traced back to those of the logarithm and the square root. This section derives closed-form expressions for each inverse function, identifies branch cuts and domains of analyticity, and computes derivatives.

## Inverse Sine

**Derivation.** We seek $w$ such that $\sin w = z$. Using $\sin w = (e^{iw} - e^{-iw})/(2i) = z$:
$$e^{iw} - e^{-iw} = 2iz.$$
Let $\zeta = e^{iw}$: $\zeta - 1/\zeta = 2iz$, so $\zeta^2 - 2iz\zeta - 1 = 0$. Quadratic formula:
$$\zeta = iz + \sqrt{1 - z^2} \quad \text{(both signs)}.$$
Then $iw = \log\zeta$, so:
$$w = \arcsin z = -i\log\bigl(iz + \sqrt{1 - z^2}\bigr).$$

The two signs in $\sqrt{1-z^2}$ and the multivaluedness of the logarithm together produce the full set of values of $\arcsin z$ (which differ by $2\pi k$ or are related by the reflection $w \mapsto \pi - w$).

**Principal branch.** Using the principal square root and principal logarithm:
$$\arcsin z = -i\,\mathrm{Log}\!\left(iz + (1-z^2)^{1/2}\right).$$
This is analytic on $\mathbb{C} \setminus \{(-\infty, -1] \cup [1, \infty)\}$ — the complex plane with two branch cuts along the portions of the real axis where $|x| \geq 1$.

**Branch points.** The square root $\sqrt{1-z^2}$ has branch points where $1 - z^2 = 0$, i.e., at $z = \pm 1$. These are the branch points of $\arcsin z$.

**Derivative.** Differentiating $\sin(\arcsin z) = z$ implicitly:
$$\cos(\arcsin z) \cdot \frac{d}{dz}\arcsin z = 1, \qquad \frac{d}{dz}\arcsin z = \frac{1}{\cos(\arcsin z)} = \frac{1}{\sqrt{1-z^2}}.$$
This is the familiar formula from real calculus, now valid on the branch cut domain.

## Inverse Cosine

By a similar derivation, $\cos w = z$ gives $e^{iw} = z \pm \sqrt{z^2 - 1}$, so:
$$\arccos z = -i\,\mathrm{Log}\!\left(z + i\sqrt{1 - z^2}\right).$$
Analyticity domain: $\mathbb{C} \setminus \{(-\infty, -1] \cup [1, \infty)\}$. Derivative: $-1/\sqrt{1-z^2}$.

Note the identity $\arcsin z + \arccos z = \pi/2$, which holds for the principal branches.

## Inverse Tangent

**Derivation.** Solve $\tan w = z$: $(e^{iw} - e^{-iw})/(i(e^{iw} + e^{-iw})) = z$. Let $\zeta = e^{2iw}$:
$$\frac{\zeta - 1}{i(\zeta + 1)} = z \implies \zeta - 1 = iz\zeta + iz \implies \zeta(1 - iz) = 1 + iz \implies \zeta = \frac{1 + iz}{1 - iz}.$$
Then $2iw = \log\zeta$, so:
$$\arctan z = \frac{1}{2i}\log\frac{1 + iz}{1 - iz} = \frac{i}{2}\log\frac{1 - iz}{1 + iz}.$$

**Principal branch.** Use $\mathrm{Log}$ in place of $\log$. The singularities occur where $1 \pm iz = 0$, i.e., $z = \pm i$. These are the branch points. The branch cuts run along $\{iy : |y| \geq 1\}$ — the portions of the imaginary axis with $|y| \geq 1$.

**Derivative.** $\dfrac{d}{dz}\arctan z = \dfrac{1}{1 + z^2}$.

**Worked example.** Compute $\arctan(2i)$.

$\dfrac{1 + i(2i)}{1 - i(2i)} = \dfrac{1 - 2}{1 + 2} = \dfrac{-1}{3}$.

$\arctan(2i) = \dfrac{i}{2}\mathrm{Log}\!\left(\dfrac{-1}{3}\right) = \dfrac{i}{2}(\ln(1/3) + i\pi) = \dfrac{i}{2}(-\ln 3 + i\pi) = \dfrac{-\pi}{2} - \dfrac{i\ln 3}{2}.$ $\square$

## Inverse Hyperbolic Functions

The inverse hyperbolic functions are related to their trigonometric counterparts via the substitution $z \to iz$:

$$\mathrm{arcsinh}\, z = -i\arcsin(iz) = \log\bigl(z + \sqrt{z^2 + 1}\bigr).$$
$$\mathrm{arccosh}\, z = -i\arccos(z) = \log\bigl(z + \sqrt{z^2 - 1}\bigr).$$
$$\mathrm{arctanh}\, z = -i\arctan(iz) = \frac{1}{2}\log\frac{1 + z}{1 - z}.$$

**Derivatives:**
$$\frac{d}{dz}\mathrm{arcsinh}\, z = \frac{1}{\sqrt{z^2+1}}, \qquad \frac{d}{dz}\mathrm{arccosh}\, z = \frac{1}{\sqrt{z^2-1}}, \qquad \frac{d}{dz}\mathrm{arctanh}\, z = \frac{1}{1-z^2}.$$

**Branch points and cuts for $\mathrm{arctanh}$:** The singularities occur at $z = \pm 1$, with branch cuts typically placed on $(-\infty, -1]$ and $[1, \infty)$.

## Summary Table

| Function | Formula | Branch points | Derivative |
|---|---|---|---|
| $\arcsin z$ | $-i\mathrm{Log}(iz + \sqrt{1-z^2})$ | $\pm 1$ | $1/\sqrt{1-z^2}$ |
| $\arccos z$ | $-i\mathrm{Log}(z + i\sqrt{1-z^2})$ | $\pm 1$ | $-1/\sqrt{1-z^2}$ |
| $\arctan z$ | $\frac{i}{2}\mathrm{Log}\frac{1-iz}{1+iz}$ | $\pm i$ | $1/(1+z^2)$ |
| $\mathrm{arcsinh}\, z$ | $\mathrm{Log}(z + \sqrt{z^2+1})$ | $\pm i$ | $1/\sqrt{z^2+1}$ |
| $\mathrm{arccosh}\, z$ | $\mathrm{Log}(z + \sqrt{z^2-1})$ | $\pm 1$ | $1/\sqrt{z^2-1}$ |
| $\mathrm{arctanh}\, z$ | $\frac{1}{2}\mathrm{Log}\frac{1+z}{1-z}$ | $\pm 1$ | $1/(1-z^2)$ |

## Connection to Real Calculus

On the real axis, the complex inverse functions agree with the standard real ones (on appropriate intervals). The derivation via the logarithm is far more economical than the ad hoc real-variable approach: a single formula, $w = \arcsin z = -i\log(iz + \sqrt{1-z^2})$, encodes all branches and all their properties. The branch cut structure, which is purely a complex-variable phenomenon, also explains why the real functions $\arcsin$ and $\arccos$ have natural domains $[-1, 1]$: the branch cuts in the complex plane land precisely on $(-\infty, -1] \cup [1, \infty)$, and the real axis outside $[-1, 1]$ lies on or near these cuts.
