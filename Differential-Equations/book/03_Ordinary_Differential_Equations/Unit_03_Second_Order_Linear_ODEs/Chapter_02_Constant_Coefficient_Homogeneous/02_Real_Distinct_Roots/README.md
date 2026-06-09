# Real Distinct Roots

When the characteristic equation $ar^2 + br + c = 0$ has two distinct real roots $r_1 \neq r_2$ (corresponding to $\Delta = b^2 - 4ac > 0$), the two solutions $e^{r_1 x}$ and $e^{r_2 x}$ are linearly independent, and the general solution is their linear combination.

## The General Solution

The two functions $y_1 = e^{r_1 x}$ and $y_2 = e^{r_2 x}$ are solutions (by the characteristic equation derivation). Their Wronskian is

$$W = e^{r_1 x}(r_2 e^{r_2 x}) - r_1 e^{r_1 x}(e^{r_2 x}) = (r_2 - r_1)e^{(r_1 + r_2)x} \neq 0$$

since $r_1 \neq r_2$. They are therefore linearly independent, forming a fundamental set. The general solution is

$$y = c_1 e^{r_1 x} + c_2 e^{r_2 x}.$$

## Qualitative Behavior

The behavior of $y$ for large $|x|$ is determined by the signs of $r_1$ and $r_2$.

If both roots are negative ($r_1 < r_2 < 0$): every solution decays to zero as $x \to +\infty$. This is the **overdamped** case in mechanical/electrical applications. The dominant behavior for large $x$ is $e^{r_2 x}$ (the less negative exponent decays more slowly).

If both roots are positive ($0 < r_1 < r_2$): every nontrivial solution grows without bound as $x \to +\infty$, dominated by $e^{r_2 x}$.

If the roots have opposite signs ($r_1 < 0 < r_2$): the equilibrium $y = 0$ is unstable. Solutions with $c_2 \neq 0$ are dominated by $e^{r_2 x}$ (growing) for $x \to +\infty$, while for $x \to -\infty$ they are dominated by $e^{r_1 x}$ (growing in the reverse direction). The special solution with $c_2 = 0$ decays; all others diverge.

## Worked Example

Solve $y'' - 5y' + 6y = 0$, $y(0) = 1$, $y'(0) = 0$.

Characteristic equation: $r^2 - 5r + 6 = 0$, $(r-2)(r-3) = 0$, roots $r_1 = 2$, $r_2 = 3$.

General solution: $y = c_1 e^{2x} + c_2 e^{3x}$.

Imposing initial conditions: $y(0) = c_1 + c_2 = 1$ and $y'(0) = 2c_1 + 3c_2 = 0$. From these: $c_1 = 3$, $c_2 = -2$. Solution:

$$y = 3e^{2x} - 2e^{3x}.$$

For large $x$, the $e^{3x}$ term dominates: the solution grows rapidly despite the initial conditions. This reflects the instability of the zero equilibrium when positive roots are present.

## Negative Roots: Decay to Equilibrium

For $r'' + 5y' + 6y = 0$: roots $r = (-5 \pm 1)/2$, giving $r_1 = -2$, $r_2 = -3$. General solution $y = c_1 e^{-2x} + c_2 e^{-3x}$. All solutions decay to zero: the system is overdamped. The slower decay rate $e^{-2x}$ dominates for large $x$ (since $e^{-3x}$ decays faster and becomes negligible).

## Initial Value Problems

For the IVP $y(0) = y_0$, $y'(0) = y_1$: the constants $c_1, c_2$ satisfy the linear system

$$\begin{pmatrix} 1 & 1 \\ r_1 & r_2 \end{pmatrix}\begin{pmatrix}c_1\\c_2\end{pmatrix} = \begin{pmatrix}y_0\\y_1\end{pmatrix}.$$

The coefficient matrix has determinant $r_2 - r_1 \neq 0$ (the Wronskian at $x = 0$), so the system always has a unique solution:

$$c_1 = \frac{r_2 y_0 - y_1}{r_2 - r_1}, \qquad c_2 = \frac{y_1 - r_1 y_0}{r_2 - r_1}.$$

This explicit formula for the constants in terms of initial data is useful in applications.
