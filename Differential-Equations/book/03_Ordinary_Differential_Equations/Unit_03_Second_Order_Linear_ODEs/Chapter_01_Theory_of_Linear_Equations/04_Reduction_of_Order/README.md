# Reduction of Order

If one solution $y_1(x)$ of the second-order linear homogeneous equation $y'' + p(x)y' + q(x)y = 0$ is known, the **reduction of order** method finds a second, linearly independent solution $y_2$ by seeking $y_2 = v(x)y_1(x)$ and solving a first-order ODE for $v$. The name "reduction of order" reflects that the second-order problem for $y_2$ reduces to a first-order problem for $v'$.

## The Method

Write $y_2 = v(x)y_1(x)$ and substitute into the equation. Computing derivatives:

$$y_2' = v'y_1 + vy_1', \qquad y_2'' = v''y_1 + 2v'y_1' + vy_1''.$$

Substituting into $y_2'' + py_2' + qy_2 = 0$:

$$(v''y_1 + 2v'y_1' + vy_1'') + p(v'y_1 + vy_1') + q(vy_1) = 0.$$

Collecting by powers of $v$:

$$v''y_1 + v'(2y_1' + py_1) + v(y_1'' + py_1' + qy_1) = 0.$$

The last term vanishes because $y_1$ is a solution: $y_1'' + py_1' + qy_1 = 0$. Setting $w = v'$:

$$w'y_1 + w(2y_1' + py_1) = 0 \implies \frac{w'}{w} = -\frac{2y_1'}{y_1} - p.$$

Integrating: $\ln|w| = -2\ln|y_1| - \int p\,dx + C_1$, giving

$$w = v' = \frac{A}{y_1^2}\,e^{-\int p\,dx},$$

where $A$ is a constant (taken as 1 by convention). Finally, $v = \int \frac{e^{-\int p\,dx}}{y_1^2}\,dx$, and

$$y_2 = y_1\int \frac{e^{-\int p\,dx}}{y_1^2}\,dx.$$

## Why $y_2$ Is Independent of $y_1$

The Wronskian of $y_1$ and $y_2 = vy_1$ is

$$W(y_1, y_2) = y_1(v'y_1 + vy_1') - y_1'(vy_1) = y_1^2 v' = y_1^2 \cdot \frac{e^{-\int p\,dx}}{y_1^2} = e^{-\int p\,dx} \neq 0.$$

So $y_2$ is linearly independent of $y_1$ wherever $y_1 \neq 0$.

## Worked Example 1

Given that $y_1 = x$ is a solution of $x^2 y'' - xy' + y = 0$ (Euler equation), find a second solution.

Rewrite in standard form: $y'' - (1/x)y' + (1/x^2)y = 0$, so $p = -1/x$. Using the formula:

$$v' = \frac{e^{-\int(-1/x)\,dx}}{x^2} = \frac{e^{\ln x}}{x^2} = \frac{x}{x^2} = \frac{1}{x}.$$

So $v = \ln x$, and $y_2 = x\ln x$.

Verify: $y_2 = x\ln x$, $y_2' = \ln x + 1$, $y_2'' = 1/x$. Then $x^2(1/x) - x(\ln x + 1) + x\ln x = x - x\ln x - x + x\ln x = 0$. Correct.

General solution: $y = c_1 x + c_2 x\ln x$.

## Worked Example 2

Given that $y_1 = e^x$ is a solution of $y'' - 2y' + y = 0$, find a second solution.

$p = -2$. Then $v' = e^{-\int(-2)\,dx}/(e^x)^2 = e^{2x}/e^{2x} = 1$, so $v = x$ and $y_2 = xe^x$.

This recovers the standard result for the repeated-root case of constant-coefficient equations.

## Application: Reduction for Variable Coefficient Equations

For equations with variable coefficients (like Bessel's equation or Legendre's equation), one solution is often found by power series (or identified as a known special function), and the second solution is then found by reduction of order, yielding either another series or a series with a logarithm.

For example, Bessel's equation of order zero $x^2y'' + xy' + x^2y = 0$ has first solution $J_0(x) = \sum_{n=0}^\infty \frac{(-1)^n x^{2n}}{4^n(n!)^2}$. The reduction of order formula gives the second solution $Y_0(x)$, which involves $\ln x$ and is the Neumann function of order zero.

## Connection to Variation of Parameters

The reduction of order formula for the second solution is closely related to the variation of parameters formula for particular solutions. In both cases, the known solution $y_1$ serves as a "basis" that is varied (multiplied by an unknown function $v$), and the ODE for $v$ reduces to a first-order problem by the fact that $y_1$ kills the zeroth-order term. The Wronskian formula $W = e^{-\int p\,dx}$ (Abel's identity) appears naturally in the solution.
