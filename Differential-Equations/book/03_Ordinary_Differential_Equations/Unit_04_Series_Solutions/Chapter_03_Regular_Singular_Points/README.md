# Chapter 3: Regular Singular Points and the Frobenius Method

When $x_0$ is a **singular point** of the equation $y'' + p(x)y' + q(x)y = 0$ (meaning $p$ or $q$ is not analytic at $x_0$), the ordinary power series method generally fails. However, if the singularity is of a specific "mild" type, called **regular**, a generalized power series of the form $y = (x-x_0)^r\sum_{n=0}^\infty a_n(x-x_0)^n$ (a **Frobenius series**) can still be found. The exponent $r$ is determined by the **indicial equation**, a quadratic whose roots control both the form of the solutions and the difficulty of finding them.

## Classification of Singular Points

The chapter begins by defining and classifying singular points. A point $x_0$ is a **regular singular point** if $(x-x_0)p(x)$ and $(x-x_0)^2 q(x)$ are both analytic at $x_0$. Intuitively, $p$ has at most a simple pole and $q$ has at most a double pole at $x_0$. All other singular points are **irregular**, and the Frobenius method does not apply to them.

## The Frobenius Method

The method substitutes $y = (x-x_0)^r \sum_{n=0}^\infty a_n(x-x_0)^n = \sum_{n=0}^\infty a_n(x-x_0)^{n+r}$ into the ODE and derives the **indicial equation** for $r$ by requiring the coefficient of the lowest power $(x-x_0)^r$ to vanish. The two roots $r_1$ and $r_2$ (with $r_1 \geq r_2$ by convention) give two possible Frobenius series.

## Three Cases

The form of the second solution depends on $r_1 - r_2$:
- If $r_1 - r_2$ is not an integer, both roots give independent Frobenius series solutions.
- If $r_1 - r_2$ is a positive integer, the second solution may involve a logarithm.
- If $r_1 = r_2$ (repeated root), the second solution always involves a logarithm.

## Historical Significance

The Frobenius method was developed by Georg Frobenius in 1873 and is one of the central results of nineteenth-century mathematical analysis. It gives access to the solutions of all the classical equations of mathematical physics, which arise at regular singular points (Bessel's equation at $x = 0$, Legendre's at $x = \pm 1$, the hypergeometric equation at $x = 0, 1, \infty$). Without it, these equations and their special function solutions would be inaccessible to series methods.
