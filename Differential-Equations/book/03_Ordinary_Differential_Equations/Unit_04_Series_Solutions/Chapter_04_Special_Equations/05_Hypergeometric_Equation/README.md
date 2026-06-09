# The Hypergeometric Equation

The **hypergeometric equation** (Gauss hypergeometric equation) is

$$x(1-x)y'' + [c - (a+b+1)x]y' - aby = 0,$$

where $a, b, c$ are parameters. It has regular singular points at $x = 0$, $x = 1$, and $x = \infty$, and every second-order linear ODE with exactly three regular singular points in the Riemann sphere can be transformed into this equation by a Mobius transformation. This makes it a universal model for second-order linear equations with three regular singularities.

## The Hypergeometric Function

At $x = 0$: $P_0 = c$, $Q_0 = 0$. Indicial equation: $r(r-1) + cr = r(r + c - 1) = 0$, roots $r_1 = 0$, $r_2 = 1 - c$.

For $r = 0$, the Frobenius series solution is the **hypergeometric function** (or Gauss hypergeometric function):

$$_2F_1(a, b; c; x) = 1 + \frac{ab}{c}\frac{x}{1!} + \frac{a(a+1)b(b+1)}{c(c+1)}\frac{x^2}{2!} + \cdots = \sum_{n=0}^\infty \frac{(a)_n(b)_n}{(c)_n n!}x^n,$$

where $(a)_n = a(a+1)\cdots(a+n-1)$ is the Pochhammer symbol (rising factorial). The series converges for $|x| < 1$; it terminates (giving a polynomial) when $a$ or $b$ is a non-positive integer.

## Special Cases

The hypergeometric function unifies many classical functions:

$$\ln(1+x) = x\,{_2F_1}(1,1;2;-x), \qquad \arcsin x = x\,{_2F_1}(1/2, 1/2; 3/2; x^2),$$

$$P_n(x) = {_2F_1}(-n, n+1; 1; (1-x)/2) \quad \text{(Legendre polynomials)},$$

$$(1-x)^{-a} = {_2F_1}(a, 1; 1; x) \quad \text{(binomial series)}.$$

This universality makes the hypergeometric equation a central object in the theory of special functions: studying its connection coefficients (how solutions near different singular points relate) unifies the transformation properties of all the classical special functions.

## The Riemann Scheme and Connection Formulae

Riemann's approach encodes the singularities and exponents of the hypergeometric equation in the **Riemann scheme**:

$$\left\{\begin{array}{ccc} 0 & 1 & \infty \\ 0 & 0 & a \\ 1-c & c-a-b & b \end{array}\right\}.$$

The analytic continuation of solutions from one singular region to another is expressed by the connection formulae (Kummer's 24 solutions), which give $_2F_1$ near $x = 1$ and $x = \infty$ in terms of the function near $x = 0$. These formulae include transformations such as $_{2}F_1(a,b;c;x) = (1-x)^{c-a-b}\,{_2F_1}(c-a,c-b;c;x)$.
