# Three Cases Based on the Difference of Indicial Roots

Let $r_1 \geq r_2$ be the indicial roots. The form of the second linearly independent solution of a Frobenius equation depends critically on $r_1 - r_2$.

## Case 1: $r_1 - r_2$ Is Not an Integer

Both $r_1$ and $r_2$ give valid Frobenius series:

$$y_1 = x^{r_1}\sum_{n=0}^\infty a_n x^n, \qquad y_2 = x^{r_2}\sum_{n=0}^\infty b_n x^n \quad (a_0 = b_0 = 1).$$

The two solutions are linearly independent (they behave like $x^{r_1}$ and $x^{r_2}$ near 0, and these are not proportional when $r_1 \neq r_2$ and $r_1 - r_2$ is not an integer). The general solution is $y = c_1 y_1 + c_2 y_2$.

**Example.** Euler equation $x^2 y'' + \frac{1}{2}xy' - \frac{1}{8}y = 0$: indicial equation $r^2 - \frac{1}{2}r - \frac{1}{8} = 0$, roots $r = \frac{1/2 \pm \sqrt{1/4 + 1/2}}{2}$... Let's use $r(r-1) + \frac{1}{2}r - \frac{1}{8} = 0$, i.e., $r^2 - \frac{1}{2}r - \frac{1}{8} = 0$, roots $r = (1/2 \pm \sqrt{1/4 + 1/2})/2 = (1/2 \pm \sqrt{3/4})/2$. These are irrational numbers with non-integer difference, giving two Frobenius series solutions.

## Case 2: $r_1 - r_2 = N > 0$ (Positive Integer)

The first solution $y_1 = x^{r_1}\sum a_n x^n$ is always valid. The second solution has the form

$$y_2 = c y_1 \ln x + x^{r_2}\sum_{n=0}^\infty b_n x^n, \qquad b_0 = 1,$$

where $c$ may be zero or nonzero. If $c = 0$, the second solution is a pure Frobenius series at $r_2$; if $c \neq 0$, the logarithm is essential and cannot be eliminated.

Whether $c = 0$ or not depends on the specific equation. The constant $c$ is determined by the recurrence at $r_2$: when $n = N$, the recurrence gives $0 \cdot a_N =$ (something), and if that "something" is nonzero, then the recurrence is inconsistent unless a $\ln$ term is introduced.

## Case 3: $r_1 = r_2$ (Repeated Root)

The first solution is $y_1 = x^r \sum a_n x^n$. The second solution always involves a logarithm:

$$y_2 = y_1(x)\ln x + x^r\sum_{n=1}^\infty b_n x^n.$$

Substituting this into the ODE and matching coefficients determines the $b_n$ from the $a_n$.

## Deriving the Second Solution: Method of Differentiation

A systematic way to find $y_2$ in Cases 2 and 3 is via differentiation with respect to $r$. For Case 3 ($r_1 = r_2 = r$): define $y(x, r) = x^r\sum a_n(r)x^n$ where $a_n(r)$ satisfies the recurrence for general $r$. Then $L[y(x,r)] = F(r)a_0 x^r$. Differentiating with respect to $r$:

$$L\!\left[\frac{\partial y}{\partial r}\right] = F'(r)a_0 x^r + F(r)a_0 x^r \ln x.$$

At $r = r_1$, $F(r_1) = 0$ and $F'(r_1) = 0$ (double root), so $L[\partial y/\partial r] = 0$: the function $\partial y/\partial r|_{r=r_1}$ is a solution. Computing $\partial y/\partial r = y\ln x + x^r\sum (da_n/dr)x^n$ confirms the logarithmic form.

## Physical Significance

The logarithmic second solution is not merely a mathematical artifact; it appears in physically meaningful contexts. For Bessel's equation of integer order, the second solution $Y_n(x)$ (the Neumann function) has a logarithmic singularity at $x = 0$, representing waves with a physical source at the origin. For Legendre's equation, the second solution $Q_n(\cos\theta)$ is singular at the poles of a sphere ($\theta = 0, \pi$), excluded from solutions that are finite everywhere on the sphere.
