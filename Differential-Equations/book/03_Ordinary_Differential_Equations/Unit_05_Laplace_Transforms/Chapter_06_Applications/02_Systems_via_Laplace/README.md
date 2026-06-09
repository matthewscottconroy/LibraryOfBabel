# Systems of ODEs via Laplace Transform

For a system of $n$ linear constant-coefficient ODEs, applying the Laplace transform to each equation converts the system into $n$ algebraic equations in the $n$ unknown transforms $X_1(s), \ldots, X_n(s)$. Solving this algebraic system and inverting gives the solution.

## The Method for Two Equations

For the system $x' = ax + by + f_1(t)$, $y' = cx + dy + f_2(t)$ with initial conditions $x(0) = x_0$, $y(0) = y_0$:

Transform: $(s-a)X - bY = F_1 + x_0$, $-cX + (s-d)Y = F_2 + y_0$.

This is a $2 \times 2$ linear system for $X$ and $Y$:

$$\begin{pmatrix}s-a & -b\\-c & s-d\end{pmatrix}\begin{pmatrix}X\\Y\end{pmatrix} = \begin{pmatrix}F_1+x_0\\F_2+y_0\end{pmatrix}.$$

The determinant of the coefficient matrix is $(s-a)(s-d) - bc = s^2 - (a+d)s + (ad-bc)$, which is the characteristic polynomial of the matrix $A = \begin{pmatrix}a&b\\c&d\end{pmatrix}$. Solving by Cramer's rule gives $X$ and $Y$ as rational functions of $s$, then partial-fraction inversion gives $x(t)$ and $y(t)$.

## Worked Example

Solve $x' = x + y$, $y' = -x + y$, $x(0) = 1$, $y(0) = 0$.

Transform: $(s-1)X - Y = 1$ and $X + (s-1)Y = 0$ ... wait: $X + (s-1)Y = 0$ should be $X - (s-1)Y = 0$? Let me redo: transform $y' = -x + y$: $sY - 0 = -X + Y$, so $X + (s-1)Y = 0$... actually $(s-1)Y = -X$... No: $sY - y(0) = -X + Y$, so $(s-1)Y = -X$, giving $X + (s-1)Y = 0$? That gives $X = -(s-1)Y$. And $(s-1)X - Y = 1$: $-(s-1)^2 Y - Y = 1$, so $-[(s-1)^2+1]Y = 1$, $Y = -1/((s-1)^2+1)$. Then $X = (s-1)/((s-1)^2+1)$.

By the first shifting theorem with $a = 1$: $x(t) = e^t\cos t$ and $y(t) = -e^t\sin t$.

Verification: $x' = e^t\cos t - e^t\sin t = x + y$. $y' = -e^t\sin t - e^t\cos t = -x + y$. Both correct.

## Comparison with Eigenvalue Methods

The characteristic polynomial of the coefficient matrix appears as the denominator of both $X$ and $Y$, the same polynomial whose roots (the eigenvalues) drive the exponential behavior. The Laplace method and the eigenvalue method of Unit 6 give the same result; the Laplace method automatically incorporates initial conditions and is particularly useful for nonhomogeneous systems.
