# Taylor's Theorem and Remainder

A polynomial is the simplest kind of function — it requires only addition and multiplication to evaluate. Taylor's theorem says that any sufficiently smooth function is locally indistinguishable from a polynomial, with a precise error bound. This result is the quantitative form of the intuition that "smooth functions are locally polynomial," and it underpins the method of series solutions to ODEs, numerical methods for differential equations, and asymptotic analysis.

## Taylor Polynomials

The $n$-th **Taylor polynomial** of $f$ at $a$ is
$$T_n(x) = \sum_{k=0}^n \frac{f^{(k)}(a)}{k!}(x-a)^k = f(a) + f'(a)(x-a) + \frac{f''(a)}{2!}(x-a)^2 + \cdots + \frac{f^{(n)}(a)}{n!}(x-a)^n.$$

This is the unique polynomial of degree at most $n$ that matches $f$ and its first $n$ derivatives at $a$: $T_n^{(k)}(a) = f^{(k)}(a)$ for $0 \leq k \leq n$.

The **remainder** (or **error**) is $R_n(x) = f(x) - T_n(x)$.

## Taylor's Theorem with Lagrange Remainder

**Theorem (Taylor's Theorem).** If $f$ is $n+1$ times differentiable on an interval $I$ containing $a$, then for each $x \in I$, there exists $c$ strictly between $a$ and $x$ such that
$$R_n(x) = \frac{f^{(n+1)}(c)}{(n+1)!}(x-a)^{n+1}.$$

*Proof.* Define $g(t) = f(x) - T_n(x; t)$, where $T_n(x;t)$ is the Taylor polynomial of $f$ at $t$ evaluated at $x$. Both $g(a) = R_n(x)$ and one relates $g(a)$ to $g(x) = 0$ (since $T_n(x;x) = f(x)$). Applying Rolle's theorem or the MVT $n+1$ times (using the auxiliary function $h(t) = g(t) - \left(\frac{x-t}{x-a}\right)^{n+1} g(a)$, which satisfies $h(a) = 0 = h(x)$) produces the result. $\square$

The Lagrange remainder $\frac{f^{(n+1)}(c)}{(n+1)!}(x-a)^{n+1}$ has the same form as the next term in the series, with the exact value of the derivative replaced by its value at some intermediate point $c$.

## Remainder Estimates

If $|f^{(n+1)}(t)| \leq M$ for all $t$ between $a$ and $x$, then
$$|R_n(x)| \leq \frac{M}{(n+1)!}|x-a|^{n+1}.$$

This is the key estimate. It says:
1. The error decreases like $(x-a)^{n+1}$ as $x \to a$ — the polynomial approximation becomes more accurate near the center.
2. The error decreases like $1/(n+1)!$ as $n$ increases — more terms give more accuracy.
3. The error is controlled by the maximum of the $(n+1)$-st derivative.

**Example.** Approximate $\cos(0.1)$ using a degree-4 Taylor polynomial at $a = 0$.

$T_4(x) = 1 - x^2/2 + x^4/24$. The remainder: $f^{(5)}(t) = -\sin t$ (the fifth derivative of cosine), bounded by $|f^{(5)}(t)| \leq 1$. So $|R_4(0.1)| \leq \frac{(0.1)^5}{5!} = \frac{10^{-5}}{120} \approx 8.3 \times 10^{-8}$.

$T_4(0.1) = 1 - 0.005 + 0.000000417 \approx 0.995004$.

The true value: $\cos(0.1) \approx 0.9950042$. Error: about $4 \times 10^{-7}$, well within the bound.

## Standard Taylor Series

At $a = 0$ (Maclaurin series):

$$e^x = \sum_{k=0}^\infty \frac{x^k}{k!}, \quad \sin x = \sum_{k=0}^\infty \frac{(-1)^k x^{2k+1}}{(2k+1)!}, \quad \cos x = \sum_{k=0}^\infty \frac{(-1)^k x^{2k}}{(2k)!}.$$

$$\frac{1}{1-x} = \sum_{k=0}^\infty x^k \text{ (}|x|<1\text{)}, \quad \ln(1+x) = \sum_{k=1}^\infty \frac{(-1)^{k+1}x^k}{k} \text{ (}|x|\leq 1\text{, }x\neq -1\text{)}.$$

These are derived by computing derivatives at $0$ and showing that the remainder $R_n(x) \to 0$.

## Taylor's Theorem and Power Series

If $f$ has a power series expansion $f(x) = \sum c_k(x-a)^k$ converging on $(a-R, a+R)$, then $c_k = f^{(k)}(a)/k!$ — the coefficients must be the Taylor coefficients. Conversely, if all derivatives of $f$ exist at $a$ and the remainder $R_n(x) \to 0$ as $n \to \infty$ for $x$ near $a$, then $f(x) = \sum \frac{f^{(k)}(a)}{k!}(x-a)^k$ is its Taylor series.

Warning: not every smooth function equals its Taylor series. The classic example is $f(x) = e^{-1/x^2}$ (set $f(0) = 0$), which is $C^\infty$, has $f^{(n)}(0) = 0$ for all $n$, so its Taylor series at $0$ is identically $0$ — not equal to $f(x)$ for $x \neq 0$.

## Application to ODEs: Method of Series Solutions

For the ODE $y'' + P(x)y' + Q(x)y = 0$ near an ordinary point $a$, assume $y = \sum_{k=0}^\infty c_k (x-a)^k$. Substituting into the ODE and requiring the coefficient of each $(x-a)^k$ to vanish gives a recurrence relation for the $c_k$. Taylor's theorem guarantees that if the series converges (radius of convergence determined by the coefficients), the result is a genuine solution.

## Application to Numerical Methods

The Euler method $y_{n+1} = y_n + hf(t_n, y_n)$ is the first-order Taylor approximation to $y(t_n + h)$:
$$y(t_n + h) = y(t_n) + hy'(t_n) + \frac{h^2}{2}y''(c) = y_n + hf(t_n, y_n) + O(h^2).$$

The $O(h^2)$ term is the local truncation error, and it follows directly from Taylor's theorem with $n = 1$. The Runge-Kutta methods are built to cancel higher-order Taylor terms, achieving $O(h^4)$ or better local error.

## Common Pitfalls

**Confusing the Taylor polynomial with the Taylor series.** $T_n$ is a polynomial — a finite sum. The Taylor series is the limit $n \to \infty$, which may or may not converge.

**Forgetting that $c$ depends on $x$.** The intermediate point $c$ in the Lagrange remainder is not a fixed point but changes with $x$. One cannot evaluate $f^{(n+1)}$ at $c$ explicitly; one bounds it.

**Assuming a smooth function equals its Taylor series.** Convergence of the Taylor series to $f$ requires that $R_n(x) \to 0$, which is additional information beyond existence of all derivatives.
