# Probability Density Functions

A continuous random variable $X$ is described by a probability density function $f(x)\geq 0$ with $\int_{-\infty}^{\infty}f(x)\,dx = 1$, and the probability that $X$ falls in an interval $[a,b]$ is $\int_a^b f(x)\,dx$. When two random variables $X$ and $Y$ are simultaneously observed, they are described by a **joint probability density function** $f(x,y)\geq 0$ with $\iint_{\mathbb{R}^2}f(x,y)\,dx\,dy = 1$, and the probability that $(X,Y)$ falls in a region $D$ is $\iint_D f(x,y)\,dA$. This is precisely a double integral, and the rich computational machinery of this unit applies directly.

## Joint Density Functions

A function $f:\mathbb{R}^2\to[0,\infty)$ is a **joint probability density function** (joint pdf) of the pair $(X,Y)$ if:

1. $f(x,y)\geq 0$ for all $(x,y)$.
2. $\iint_{\mathbb{R}^2}f(x,y)\,dx\,dy = 1$.

The probability that $(X,Y)\in D$ for any region $D$ is:

$$P((X,Y)\in D) = \iint_D f(x,y)\,dA.$$

## Marginal Densities

The **marginal density** of $X$ (ignoring the value of $Y$) is obtained by integrating out $Y$:

$$f_X(x) = \int_{-\infty}^{\infty}f(x,y)\,dy.$$

Similarly, $f_Y(y) = \int_{-\infty}^{\infty}f(x,y)\,dx$.

The random variables $X$ and $Y$ are **independent** if and only if $f(x,y) = f_X(x)f_Y(y)$ for all $(x,y)$.

## Expected Values

The **expected value** (mean) of $X$ under the joint density is:

$$E[X] = \iint_{\mathbb{R}^2}x\,f(x,y)\,dx\,dy.$$

The expected value of any function $g(X,Y)$ is:

$$E[g(X,Y)] = \iint_{\mathbb{R}^2}g(x,y)f(x,y)\,dx\,dy.$$

The **variance** of $X$ is $\text{Var}(X) = E[(X-E[X])^2] = E[X^2]-(E[X])^2$.

## Worked Example 1: Uniform Distribution on a Triangle

$(X,Y)$ is uniformly distributed on the triangle with vertices $(0,0)$, $(1,0)$, $(0,1)$: $f(x,y) = 2$ on the triangle, $0$ outside. (The area of the triangle is $1/2$, so $f = 1/(1/2) = 2$ to normalize.)

$P(X+Y\leq 1/2)$: the region is $\{x\geq 0, y\geq 0, x+y\leq 1/2\}$, a triangle with area $(1/2)^2/2 = 1/8$.

$P(X+Y\leq 1/2) = \iint_{\{x+y\leq 1/2,x\geq 0,y\geq 0\}}2\,dA = 2\cdot\frac{1}{8} = \frac{1}{4}$.

$E[X] = \int_0^1\int_0^{1-x}2x\,dy\,dx = 2\int_0^1 x(1-x)\,dx = 2\left[\frac{x^2}{2}-\frac{x^3}{3}\right]_0^1 = 2\cdot\frac{1}{6} = \frac{1}{3}$.

By symmetry, $E[Y] = 1/3$ too.

## Worked Example 2: Gaussian Joint Distribution

The **bivariate normal distribution** with $X, Y$ independent, $X\sim N(0,1)$, $Y\sim N(0,1)$:

$$f(x,y) = \frac{1}{2\pi}e^{-(x^2+y^2)/2}.$$

Check normalization: $\iint_{\mathbb{R}^2}\frac{1}{2\pi}e^{-(x^2+y^2)/2}\,dA = \frac{1}{2\pi}\cdot I^2$ where $I = \int_{-\infty}^{\infty}e^{-x^2/2}\,dx = \sqrt{2\pi}$ (from the Gaussian integral). So $\frac{1}{2\pi}\cdot 2\pi = 1$. Normalized.

$P(X^2+Y^2\leq r^2)$ (probability inside a circle of radius $r$):

$\int_0^{2\pi}\int_0^r\frac{1}{2\pi}e^{-s^2/2}\cdot s\,ds\,d\theta = \int_0^r se^{-s^2/2}\,ds = \left[-e^{-s^2/2}\right]_0^r = 1-e^{-r^2/2}$.

This is the CDF of the **Rayleigh distribution**.

## Worked Example 3: Conditional Probability

Given the joint density $f(x,y) = 6x^2y$ for $0\leq x,y\leq 1$ and $0$ elsewhere.

Check: $\int_0^1\int_0^1 6x^2y\,dy\,dx = 6\int_0^1 x^2\,dx\cdot\int_0^1 y\,dy = 6\cdot\frac{1}{3}\cdot\frac{1}{2} = 1$. Normalized.

$f_X(x) = \int_0^1 6x^2y\,dy = 6x^2\cdot\frac{1}{2} = 3x^2$.

$f_Y(y) = \int_0^1 6x^2y\,dx = 6y\cdot\frac{1}{3} = 2y$.

Since $f(x,y) = 3x^2\cdot 2y = f_X(x)f_Y(y)$, $X$ and $Y$ are **independent**.

## The Cumulative Distribution Function

The CDF of $(X,Y)$ is $F(a,b) = P(X\leq a, Y\leq b) = \int_{-\infty}^a\int_{-\infty}^b f(x,y)\,dy\,dx$. Recovering the pdf from the CDF: $f(x,y) = \frac{\partial^2 F}{\partial x\partial y}$. This is an instance of Clairaut's theorem: the order of differentiation (with respect to $x$ and $y$) does not matter.

## Connection to Heat Equation and Fundamental Solutions

The Gaussian density $f(x,y,t) = \frac{1}{4\pi kt}e^{-(x^2+y^2)/(4kt)}$ (for $t > 0$) is simultaneously a probability density in $(x,y)$ and the **fundamental solution** of the two-dimensional heat equation $u_t = k\Delta u$. The heat equation models diffusion: $u(x,y,t)$ is the temperature at position $(x,y)$ and time $t$ given an initial point source of heat. The probability interpretation: at time $t$, the position of a diffusing particle (Brownian motion) has density $f(\cdot,\cdot,t)$. This connection between probability and partial differential equations runs deep throughout the subject, and the multiple integration tools of this chapter are essential for both.

## Multivariate Distributions

A joint density in $n$ variables $f(x_1,\ldots,x_n)$ satisfies $\int\cdots\int f\,dx_1\cdots dx_n = 1$, and probabilities and expectations are computed as $n$-fold iterated integrals. The **multivariate normal distribution** with mean $\boldsymbol{\mu}$ and covariance matrix $\Sigma$ has density proportional to $e^{-(\mathbf{x}-\boldsymbol{\mu})^T\Sigma^{-1}(\mathbf{x}-\boldsymbol{\mu})/2}$. Normalizing this density requires computing the $n$-dimensional Gaussian integral $\int_{\mathbb{R}^n}e^{-\mathbf{x}^TA\mathbf{x}}\,d^n\mathbf{x} = \pi^{n/2}/\sqrt{\det A}$ for positive definite $A$, a beautiful result connecting multivariable integration with linear algebra and spectral theory.
