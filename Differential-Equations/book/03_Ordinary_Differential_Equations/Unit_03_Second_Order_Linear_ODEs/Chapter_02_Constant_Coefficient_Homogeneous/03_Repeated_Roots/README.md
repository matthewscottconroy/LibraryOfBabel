# Repeated Roots

When the characteristic equation $ar^2 + br + c = 0$ has a repeated root $r_1 = r_2 = r = -b/(2a)$ (corresponding to $\Delta = b^2 - 4ac = 0$), only one exponential solution $y_1 = e^{rx}$ is produced directly. A second linearly independent solution must be found, and reduction of order reveals it to be $y_2 = xe^{rx}$.

## Derivation of the Second Solution

Apply reduction of order with $y_1 = e^{rx}$. Write $y_2 = v(x)e^{rx}$ and substitute into $ay'' + by' + cy = 0$, or equivalently use the formula derived in the reduction of order section with $p = b/a$:

$$v' = \frac{e^{-\int(b/a)\,dx}}{(e^{rx})^2} = \frac{e^{-(b/a)x}}{e^{2rx}} = e^{-(b/a+2r)x}.$$

Now $r = -b/(2a)$, so $b/a = -2r$, giving $b/a + 2r = -2r + 2r = 0$. Therefore $v' = e^0 = 1$, and $v = x$. The second solution is

$$y_2 = xe^{rx}.$$

## Linear Independence

The Wronskian:

$$W(e^{rx}, xe^{rx}) = e^{rx}(e^{rx} + rxe^{rx}) - re^{rx}\cdot xe^{rx} = e^{2rx} + rxe^{2rx} - rxe^{2rx} = e^{2rx} \neq 0.$$

The two solutions $e^{rx}$ and $xe^{rx}$ are linearly independent.

## General Solution

$$y = (c_1 + c_2 x)e^{rx}.$$

## Why the Extra Factor of $x$?

The repeated root case is a limit phenomenon. For the equation with distinct roots $r_1$ and $r_2$, the general solution is $c_1 e^{r_1 x} + c_2 e^{r_2 x}$. As $r_2 \to r_1 = r$, this becomes $c_1 e^{rx} + c_2 e^{rx} = (c_1 + c_2)e^{rx}$, which is only one-dimensional. The limit process loses a dimension because both solutions collapse to the same exponential. To recover the lost dimension, one "differentiates" with respect to $r$: $\frac{\partial}{\partial r}e^{rx} = xe^{rx}$. This is precisely the second solution.

More formally, when the characteristic polynomial has a double root, $p(r) = a(r - r_1)^2$ and $p'(r_1) = 0$. The function $xe^{rx}$ satisfies $L[xe^{rx}] = p(r)xe^{rx} + p'(r)e^{rx}$, which equals zero at $r = r_1$ since both $p(r_1) = 0$ and $p'(r_1) = 0$. This "differentiate with respect to the parameter" trick generalizes to higher-order equations with higher-order repeated roots.

## Worked Example

Solve $y'' - 4y' + 4y = 0$, $y(0) = 2$, $y'(0) = 1$.

Characteristic equation: $r^2 - 4r + 4 = (r-2)^2 = 0$, repeated root $r = 2$.

General solution: $y = (c_1 + c_2 x)e^{2x}$.

$y' = c_2 e^{2x} + 2(c_1 + c_2 x)e^{2x} = (2c_1 + c_2 + 2c_2 x)e^{2x}$.

Imposing $y(0) = 2$: $c_1 = 2$. Imposing $y'(0) = 1$: $2c_1 + c_2 = 1$, so $c_2 = 1 - 4 = -3$.

Solution: $y = (2 - 3x)e^{2x}$.

Note: The factor $(2 - 3x)$ equals zero at $x = 2/3$, so the solution changes sign there despite growing overall (since $e^{2x}$ dominates for large $x$).

## Critical Damping in Applications

In the spring-mass-dashpot system $my'' + \gamma y' + ky = 0$ (with $m, \gamma, k > 0$), the characteristic roots have negative real parts (by Routh's criterion). The repeated-root case occurs at $\gamma^2 = 4mk$, giving $r = -\gamma/(2m) < 0$. Solutions decay to zero as $(c_1 + c_2 t)e^{-\gamma t/(2m)}$, returning to equilibrium without oscillation. This is **critical damping**, the borderline between overdamping (two negative real roots) and underdamping (complex conjugate roots). Critically damped systems return to equilibrium fastest among all non-oscillatory responses, an important design criterion for shock absorbers, door closers, and galvanometers.
