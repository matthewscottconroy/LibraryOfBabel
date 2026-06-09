# Partial Fractions for Inverse Laplace Transforms

When $Y(s) = P(s)/Q(s)$ is a proper rational function (degree of numerator less than degree of denominator), the method of partial fractions decomposes $Y$ into simpler fractions, each directly invertible from the Laplace table.

## Decomposition Rules

**Simple real root** $r$: contribute $\frac{A}{s-r}$, where $A = (s-r)Y(s)|_{s=r}$.

**Real root of multiplicity $m$**: contribute $\frac{A_1}{s-r} + \frac{A_2}{(s-r)^2} + \cdots + \frac{A_m}{(s-r)^m}$. Coefficients found by multiplying by $(s-r)^m$, differentiating, and evaluating at $s = r$.

**Pair of complex conjugate roots $\alpha \pm \beta i$**: contribute $\frac{As + B}{(s-\alpha)^2 + \beta^2}$. Rewrite as $A\frac{s-\alpha}{(s-\alpha)^2+\beta^2} + \frac{A\alpha+B}{\beta}\cdot\frac{\beta}{(s-\alpha)^2+\beta^2}$. Invert using $\mathcal{L}^{-1}\!\left\{\frac{s-\alpha}{(s-\alpha)^2+\beta^2}\right\} = e^{\alpha t}\cos\beta t$ and $\mathcal{L}^{-1}\!\left\{\frac{\beta}{(s-\alpha)^2+\beta^2}\right\} = e^{\alpha t}\sin\beta t$.

## Worked Example

Find $y(t) = \mathcal{L}^{-1}\!\left\{\frac{2s+5}{(s-1)(s^2+4)}\right\}$.

Partial fractions: $\frac{2s+5}{(s-1)(s^2+4)} = \frac{A}{s-1} + \frac{Bs+C}{s^2+4}$.

$A = \frac{2(1)+5}{1^2+4} = \frac{7}{5}$.

Clearing denominators: $2s+5 = A(s^2+4) + (Bs+C)(s-1) = \frac{7}{5}(s^2+4) + (Bs+C)(s-1)$.

Expanding and matching: coefficient of $s^2$: $0 = 7/5 + B$, so $B = -7/5$. Coefficient of $s^0$: $5 = 28/5 - C$, so $C = 28/5 - 5 = 3/5$. Coefficient of $s^1$: $2 = -B - C = 7/5 - 3/5 = 4/5$... this doesn't match. Let me redo.

$2s+5 = (7/5)(s^2+4) + (Bs+C)(s-1) = 7s^2/5 + 28/5 + Bs^2 + Cs - Bs - C$.

Matching $s^2$: $0 = 7/5 + B$, $B = -7/5$. Matching $s^1$: $2 = C - B = C + 7/5$, $C = 2 - 7/5 = 3/5$. Matching $s^0$: $5 = 28/5 - C = 28/5 - 3/5 = 25/5 = 5$. Consistent.

$$Y = \frac{7/5}{s-1} + \frac{-(7/5)s + 3/5}{s^2+4} = \frac{7}{5}\cdot\frac{1}{s-1} - \frac{7}{5}\cdot\frac{s}{s^2+4} + \frac{3/5}{2}\cdot\frac{2}{s^2+4}.$$

$$y(t) = \frac{7}{5}e^t - \frac{7}{5}\cos 2t + \frac{3}{10}\sin 2t.$$

## Improper Rational Functions

If $\deg P \geq \deg Q$, perform polynomial long division first to write $Y = (\text{polynomial}) + (\text{proper fraction})$. The polynomial part inverts via $\mathcal{L}^{-1}\{s^n\} = \delta^{(n)}(t)$ (derivatives of the Dirac delta), which arise physically as impulsive sources.
