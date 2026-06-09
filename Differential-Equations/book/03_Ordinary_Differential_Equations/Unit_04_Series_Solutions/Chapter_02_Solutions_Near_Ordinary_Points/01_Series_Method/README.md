# The Series Method Near Ordinary Points

At an ordinary point $x_0$, the equation $y'' + p(x)y' + q(x)y = 0$ has two linearly independent solutions, each expressible as a power series in $(x - x_0)$ with positive radius of convergence. The method for finding these series is entirely systematic.

## The Algorithm

**Step 1.** Assume $y = \sum_{n=0}^\infty a_n(x-x_0)^n$.

**Step 2.** Compute $y' = \sum_{n=1}^\infty na_n(x-x_0)^{n-1}$ and $y'' = \sum_{n=2}^\infty n(n-1)a_n(x-x_0)^{n-2}$.

**Step 3.** Substitute into $y'' + p(x)y' + q(x)y = 0$. If $p$ and $q$ are not constants, expand them in power series in $(x-x_0)$ first.

**Step 4.** Re-index all sums so that each is $\sum_{n=0}^\infty (\ldots)(x-x_0)^n$.

**Step 5.** Collect coefficients of each power $(x-x_0)^n$ and set them to zero.

**Step 6.** The resulting system of equations gives a recurrence relation. Use it to express $a_n$ for $n \geq 2$ in terms of $a_0$ and $a_1$.

**Step 7.** The general solution is $y = a_0 y_1(x) + a_1 y_2(x)$ where $y_1$ and $y_2$ are the series obtained by setting $(a_0, a_1) = (1, 0)$ and $(a_0, a_1) = (0, 1)$ respectively.

## Worked Example: Airy's Equation

The Airy equation $y'' - xy = 0$ arises in the study of diffraction and quantum tunneling. The origin $x_0 = 0$ is ordinary (the coefficients $p = 0$ and $q = -x$ are both analytic everywhere).

Substitute $y = \sum_{n=0}^\infty a_n x^n$:

$$y'' = \sum_{n=2}^\infty n(n-1)a_n x^{n-2} = \sum_{n=0}^\infty (n+2)(n+1)a_{n+2}x^n.$$

$$xy = \sum_{n=0}^\infty a_n x^{n+1} = \sum_{n=1}^\infty a_{n-1}x^n.$$

The equation $y'' - xy = 0$ becomes:

$$\sum_{n=0}^\infty (n+2)(n+1)a_{n+2}x^n - \sum_{n=1}^\infty a_{n-1}x^n = 0.$$

For $n = 0$: $2\cdot 1 \cdot a_2 = 0$, so $a_2 = 0$.

For $n \geq 1$: $(n+2)(n+1)a_{n+2} = a_{n-1}$.

Recurrence: $a_{n+2} = \frac{a_{n-1}}{(n+2)(n+1)}$ for $n \geq 1$, i.e., $a_{n+3} = \frac{a_n}{(n+3)(n+2)}$ for $n \geq 0$.

The two free parameters are $a_0$ and $a_1$.

**Solution with $a_0 = 1$, $a_1 = 0$:** The recurrence $a_{n+3} = a_n/((n+3)(n+2))$ driven by $a_0 = 1$ gives $a_3 = 1/(3\cdot 2) = 1/6$, $a_6 = 1/(6\cdot 5\cdot 3\cdot 2) = 1/180$, etc. All terms with indices not divisible by 3 come from $a_0$:

$$y_1(x) = 1 + \frac{x^3}{3!} + \frac{x^6}{6\cdot 5 \cdot 3 \cdot 2} + \cdots = 1 + \frac{x^3}{6} + \frac{x^6}{180} + \cdots$$

**Solution with $a_0 = 0$, $a_1 = 1$:** The terms with indices $\equiv 1 \pmod{3}$ come from $a_1$:

$$y_2(x) = x + \frac{x^4}{4\cdot 3} + \frac{x^7}{7\cdot 6\cdot 4\cdot 3} + \cdots = x + \frac{x^4}{12} + \frac{x^7}{504} + \cdots$$

Both series converge for all $x$ (since the equation has no finite singular points). The general solution is $y = a_0 y_1(x) + a_1 y_2(x)$.

The Airy functions $\text{Ai}(x)$ and $\text{Bi}(x)$ are specific linear combinations of $y_1$ and $y_2$ normalized by the asymptotic behavior as $x \to +\infty$.
