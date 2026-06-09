# Chapter 2: Operational Properties

The power of the Laplace transform lies not just in its basic table but in a set of operational properties that allow complex transforms to be computed from simpler ones. These properties, the shifting theorems, the differentiation of transforms, the transform of derivatives, and the transform of integrals, convert the Laplace method into a flexible algebraic calculus.

## The Transform of Derivatives

The most important property for ODE applications: if $f$ is piecewise smooth and of exponential order, then

$$\mathcal{L}\{f'(t)\} = sF(s) - f(0), \qquad \mathcal{L}\{f''(t)\} = s^2F(s) - sf(0) - f'(0).$$

In general, $\mathcal{L}\{f^{(n)}\} = s^n F(s) - s^{n-1}f(0) - \cdots - f^{(n-1)}(0)$. This is the key formula: differentiation in $t$ becomes multiplication by $s$ (plus initial value corrections). Applying this to $y'' + py' + qy = g$:

$$(s^2 + ps + q)Y(s) - (s + p)y(0) - y'(0) = G(s),$$

which gives $Y(s)$ by algebra.

## The First Shifting Theorem

$\mathcal{L}\{e^{at}f(t)\} = F(s-a)$: multiplication by $e^{at}$ in the $t$-domain corresponds to shifting $s \to s - a$ in the $s$-domain.

## Differentiation of the Transform

$\mathcal{L}\{t^n f(t)\} = (-1)^n F^{(n)}(s)$: multiplication by $t^n$ in the $t$-domain corresponds to $n$-fold differentiation in the $s$-domain.

## Transform of Integrals

$\mathcal{L}\left\{\int_0^t f(\tau)\,d\tau\right\} = F(s)/s$: integration in the $t$-domain corresponds to division by $s$.

These four properties, combined with the basic table, cover the vast majority of transforms needed in practice.
