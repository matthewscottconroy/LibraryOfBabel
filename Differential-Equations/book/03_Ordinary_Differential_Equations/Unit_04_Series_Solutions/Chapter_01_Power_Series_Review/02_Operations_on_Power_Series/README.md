# Operations on Power Series

Within the radius of convergence, power series behave like "infinite polynomials": they can be added, multiplied, shifted, differentiated, and integrated term by term, with each operation producing a valid power series representation of the resulting function. These operations are the technical tools of the series method for ODEs.

## Addition and Scalar Multiplication

$$\sum_{n=0}^\infty a_n x^n + \sum_{n=0}^\infty b_n x^n = \sum_{n=0}^\infty (a_n + b_n)x^n, \qquad |x| < \min(R_a, R_b).$$

## Multiplication (Cauchy Product)

$$\left(\sum_{n=0}^\infty a_n x^n\right)\!\left(\sum_{n=0}^\infty b_n x^n\right) = \sum_{n=0}^\infty c_n x^n, \quad c_n = \sum_{k=0}^n a_k b_{n-k}, \qquad |x| < \min(R_a, R_b).$$

## Index Shifting

A crucial operation in the series method is shifting the summation index to combine series with different starting powers of $x$ under a single sum. Given $\sum_{n=0}^\infty a_n x^n$, let $m = n - k$ (so $n = m + k$):

$$\sum_{n=0}^\infty a_n x^n = \sum_{m=-k}^\infty a_{m+k} x^{m+k} = x^k \sum_{m=0}^\infty a_{m+k}x^m + \sum_{m=-k}^{-1}a_{m+k}x^{m+k}.$$

In ODE work, one typically shifts to combine $\sum a_n x^n$, $\sum b_n x^{n-1}$, and $\sum c_n x^{n-2}$ (from $y$, $y'$, $y''$) under a single sum in $x^n$.

**Example.** Write $y'' + y = 0$ as a series equation. With $y = \sum_{n=0}^\infty a_n x^n$:

$$y'' = \sum_{n=2}^\infty n(n-1)a_n x^{n-2} = \sum_{n=0}^\infty (n+2)(n+1)a_{n+2}x^n$$

(shifting $n \to n+2$ in the sum, starting index becomes 0). The equation $y'' + y = 0$ becomes

$$\sum_{n=0}^\infty [(n+2)(n+1)a_{n+2} + a_n]x^n = 0.$$

## Differentiation and Integration

$$\frac{d}{dx}\sum_{n=0}^\infty a_n x^n = \sum_{n=1}^\infty na_n x^{n-1} = \sum_{n=0}^\infty (n+1)a_{n+1}x^n.$$

$$\int_0^x \sum_{n=0}^\infty a_n t^n\,dt = \sum_{n=0}^\infty \frac{a_n}{n+1}x^{n+1}.$$

Both operations preserve the radius of convergence.

## Equating Coefficients

If $\sum_{n=0}^\infty c_n x^n = 0$ for all $x$ in some interval, then $c_n = 0$ for all $n$. This identity theorem for power series (a consequence of continuity and differentiability) is the key step in the series method: after substituting $y = \sum a_n x^n$ into the ODE and combining into a single power series, setting each coefficient equal to zero gives the recurrence relation for the $a_n$.

## Shifting the Center

For series centered at $x_0 \neq 0$, replace $x$ by $x - x_0$ everywhere. All operations work the same way with $x - x_0$ in place of $x$. The ODE method is applied at $x_0$ by substituting $y = \sum a_n(x - x_0)^n$, which requires expressing the coefficient functions $p(x)$ and $q(x)$ as series in $(x - x_0)$ as well.
