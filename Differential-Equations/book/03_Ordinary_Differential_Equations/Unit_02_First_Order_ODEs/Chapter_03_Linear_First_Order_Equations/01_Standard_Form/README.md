# Standard Form of Linear First-Order Equations

A first-order ODE is **linear** if it can be written in the form

$$y' + p(x)\,y = q(x),$$

where $p$ and $q$ are given functions of $x$ alone. This is the **standard form**. The unknown function $y$ and its derivative $y'$ appear only to the first power and are not multiplied together; the coefficients $p(x)$ and $q(x)$ are entirely free as functions of $x$.

## Identifying Standard Form

An equation may need to be rearranged before the standard form is visible. The equation $2xy' + 4y = x^3$ is not in standard form because of the factor 2 in front of $y'$. Dividing through by $2x$ (assuming $x \neq 0$) gives $y' + (2/x)y = x^2/2$, which is standard form with $p(x) = 2/x$ and $q(x) = x^2/2$. Similarly, $y' = ay + b(x)$ becomes $y' - ay = b(x)$ in standard form, with $p(x) = -a$ (a constant).

**Nonlinear look-alikes.** The equation $y' + y^2 = q(x)$ has a $y^2$ term: it is nonlinear (Bernoulli for $n=2$). The equation $y' \cdot y = q(x)$ has a product of $y'$ and $y$: also nonlinear. The equation $y' + p(x)y = q(x)y^n$ for $n \neq 0, 1$ is Bernoulli, treated in the substitution methods chapter. Only when $y$ and $y'$ appear exactly to the first power is the equation linear.

## Homogeneous and Nonhomogeneous

When $q(x) = 0$ the equation $y' + p(x)y = 0$ is **homogeneous** (not to be confused with a homogeneous equation of degree zero in $x$ and $y$). It is separable: $dy/y = -p(x)\,dx$, giving $y = Ce^{-\int p(x)\,dx}$. The set of all solutions forms a one-dimensional vector space (a line through the origin in function space), parametrized by the single constant $C$.

When $q(x) \not\equiv 0$, the equation is **nonhomogeneous**. The general solution is $y = y_h + y_p$ where $y_h = Ce^{-\int p\,dx}$ is the general solution of the homogeneous equation and $y_p$ is any one particular solution of the nonhomogeneous equation.

## The Solution Space Structure

The superposition principle follows directly from linearity. If $y_1$ and $y_2$ both satisfy $y' + p(x)y = q(x)$, then their difference $z = y_1 - y_2$ satisfies $z' + p(x)z = 0$: the homogeneous equation. Therefore $z = Ce^{-\int p\,dx}$, which means $y_1 = y_2 + Ce^{-\int p\,dx}$. Any two solutions differ by a solution of the homogeneous equation, confirming that the general solution is $y = y_h + y_p$.

This structure, general = homogeneous + particular, is the fundamental algebraic fact about linear equations and will recur in every linear setting throughout the course.

## Existence and Uniqueness for Linear First-Order Equations

For the linear equation $y' + p(x)y = q(x)$, the function $f(x, y) = q(x) - p(x)y$ has $\partial f/\partial y = -p(x)$. If $p$ is continuous on an interval $I$ containing $x_0$, then $|\partial f/\partial y| = |p(x)|$ is bounded on any compact subinterval, so the Lipschitz condition is satisfied. By Picard's theorem, the IVP has a unique solution. Moreover, since $p$ is continuous on all of $I$, the Lipschitz bound holds globally on $I$, and the solution exists on all of $I$.

**Theorem.** If $p$ and $q$ are continuous on an open interval $I$ and $x_0 \in I$, then the IVP $y' + p(x)y = q(x)$, $y(x_0) = y_0$ has a unique solution on all of $I$.

This global existence result, stronger than what Picard gives for nonlinear equations, is the key advantage of linearity.
