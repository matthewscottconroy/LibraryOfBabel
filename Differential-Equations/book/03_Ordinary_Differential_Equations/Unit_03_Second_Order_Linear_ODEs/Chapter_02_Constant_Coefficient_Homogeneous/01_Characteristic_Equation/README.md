# The Characteristic Equation

For the constant-coefficient homogeneous ODE $ay'' + by' + cy = 0$ (with $a \neq 0$), the solution method begins by substituting the trial solution $y = e^{rx}$ and reading off the algebraic condition on $r$. The resulting polynomial equation in $r$ is the **characteristic equation** (or auxiliary equation), and its roots determine the general solution completely.

## Derivation

Substitute $y = e^{rx}$, $y' = re^{rx}$, $y'' = r^2 e^{rx}$ into $ay'' + by' + cy = 0$:

$$ar^2 e^{rx} + bre^{rx} + ce^{rx} = (ar^2 + br + c)e^{rx} = 0.$$

Since $e^{rx} > 0$ for all $x$, the equation holds if and only if

$$ar^2 + br + c = 0.$$

This is the **characteristic equation**. Its discriminant is $\Delta = b^2 - 4ac$, and by the quadratic formula:

$$r = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}.$$

The three cases are:
- $\Delta > 0$: two distinct real roots $r_1, r_2$.
- $\Delta = 0$: one repeated real root $r = -b/(2a)$.
- $\Delta < 0$: two complex conjugate roots $r = \alpha \pm \beta i$ where $\alpha = -b/(2a)$ and $\beta = \sqrt{4ac - b^2}/(2a)$.

## Why This Works: The Operator Perspective

Define the linear differential operator $L = aD^2 + bD + c$ where $D = d/dx$. Then $L[e^{rx}] = (ar^2 + br + c)e^{rx} = p(r)e^{rx}$ where $p(r) = ar^2 + br + c$ is the characteristic polynomial. If $r$ is a root of $p$, then $L[e^{rx}] = 0$, confirming that $e^{rx}$ is a solution.

This factorization $L[e^{rx}] = p(r)e^{rx}$ is the fundamental reason the characteristic equation approach works. The differentiation operator $D$ acts on exponentials as multiplication by the exponent; the polynomial $p(D)$ acts on exponentials as multiplication by $p(r)$. The equation $p(r) = 0$ is the eigenvalue equation for the operator $L$ with eigenvector $e^{rx}$.

## Example

For the equation $2y'' - 5y' + 2y = 0$, the characteristic equation is $2r^2 - 5r + 2 = 0$. Using the quadratic formula: $r = (5 \pm \sqrt{25 - 16})/4 = (5 \pm 3)/4$. So $r_1 = 2$ and $r_2 = 1/2$. The general solution is $y = c_1 e^{2x} + c_2 e^{x/2}$.

For $y'' + 4y = 0$: characteristic equation $r^2 + 4 = 0$, roots $r = \pm 2i$. General solution $y = c_1\cos(2x) + c_2\sin(2x)$ (after converting complex exponentials to real form).

## The Algebra-Analysis Correspondence

The three cases of the discriminant correspond to three qualitatively different physical behaviors. This correspondence between an algebraic condition ($\Delta$ positive, zero, or negative) and qualitative analytic behavior (pure exponential, polynomial-exponential, oscillatory) is one of the most elegant and useful results in elementary mathematics. It is the reason why the classification of roots of the characteristic polynomial is so emphasized in the study of mechanical and electrical systems.
