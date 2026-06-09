# Chapter 2: Solutions Near Ordinary Points

When $x_0$ is an ordinary point of the ODE $y'' + p(x)y' + q(x)y = 0$ (meaning $p$ and $q$ are analytic at $x_0$), the equation has two linearly independent power series solutions converging in a neighborhood of $x_0$. The method is direct: substitute $y = \sum a_n(x-x_0)^n$, differentiate term by term, substitute into the ODE, and match powers of $(x-x_0)$ to derive a recurrence relation for the coefficients $a_n$.

## The Method and Its Guarantee

The existence theorem for series solutions near ordinary points guarantees that the series solutions converge for $|x - x_0| < R$, where $R$ is at least the distance to the nearest singular point of the equation (in the complex plane). The two free constants $a_0$ and $a_1$ (corresponding to initial conditions $y(x_0) = a_0$ and $y'(x_0) = a_1$) parametrize the full two-dimensional solution space.

## Recurrence Relations

The recurrence relation expresses $a_{n+2}$ (or $a_{n+k}$ for a $k$-th order equation) in terms of earlier coefficients. Solving the recurrence gives the complete power series solution. For many equations, the recurrence splits into two independent sequences (one driven by $a_0$, one by $a_1$), each giving one of the two fundamental solutions.

## Radius of Convergence of Solutions

The third section examines how the radius of convergence of the solution is determined by the nearest singularity of the coefficients. This is a structural result connecting the complex analysis of the coefficient functions to the analytic behavior of the solutions, and it explains why solutions of equations like Legendre's equation (singular at $\pm 1$) have power series convergent only for $|x| < 1$.

## Connection to the Frobenius Method

When $x_0$ is not ordinary (i.e., it is a singular point), the power series method breaks down or gives only one solution. The Frobenius method of Chapter 3 generalizes the approach to regular singular points, where the singularity is mild enough (pole of order at most 1 in $p$, pole of order at most 2 in $q$) to allow a modified series method.
