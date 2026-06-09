# Potential Functions

Once you know that a vector field $\mathbf{F}$ is conservative, the next task is to find its potential function $f$ — the scalar field satisfying $\mathbf{F} = \nabla f$. This is not merely a formal exercise. The potential function encodes the physics of conservative systems (potential energy in mechanics, electrostatic potential in electrodynamics) and makes the computation of line integrals trivial: the work done by $\mathbf{F}$ moving from point $A$ to point $B$ is simply $f(B) - f(A)$, regardless of path.

This section develops the systematic method for recovering $f$ from its gradient components, works through examples in two and three dimensions, discusses the non-uniqueness of potential functions, and explains the connection to antidifferentiation.

## The Problem

Given $\mathbf{F} = P\,\mathbf{i} + Q\,\mathbf{j}$ on a simply connected domain, and given that $\mathbf{F}$ is conservative, find $f$ such that

$$\frac{\partial f}{\partial x} = P(x, y), \qquad \frac{\partial f}{\partial y} = Q(x, y).$$

This is a system of two first-order partial differential equations for the single unknown $f$. The consistency condition $\partial P/\partial y = \partial Q/\partial x$ (Clairaut's theorem applied to $f$) ensures that a solution exists.

## The Integration Method in Two Dimensions

The standard method proceeds by integrating one equation partially, introducing an undetermined function of the remaining variable, and then using the second equation to determine that function.

**Step 1.** Integrate $\partial f/\partial x = P$ with respect to $x$, treating $y$ as a constant:

$$f(x, y) = \int P(x, y)\,dx + g(y),$$

where $g(y)$ is an arbitrary function of $y$ (the "constant of integration" in this partial setting).

**Step 2.** Differentiate this expression for $f$ with respect to $y$:

$$\frac{\partial f}{\partial y} = \frac{\partial}{\partial y}\int P(x, y)\,dx + g'(y).$$

**Step 3.** Set this equal to $Q$ and solve for $g'(y)$:

$$g'(y) = Q(x, y) - \frac{\partial}{\partial y}\int P(x, y)\,dx.$$

The right-hand side must depend only on $y$ (not on $x$) — a fact guaranteed by the consistency condition. Integrate to find $g(y)$, and hence $f$.

**Worked Example.** Let $\mathbf{F}(x,y) = (2xy + 1)\,\mathbf{i} + (x^2 - 3y^2)\,\mathbf{j}$.

First, verify conservativity: $\partial P/\partial y = 2x$ and $\partial Q/\partial x = 2x$. The condition holds, so $\mathbf{F}$ is conservative on $\mathbb{R}^2$.

**Step 1.** $f(x,y) = \int (2xy + 1)\,dx = x^2 y + x + g(y)$.

**Step 2.** $\partial f/\partial y = x^2 + g'(y)$.

**Step 3.** Set equal to $Q = x^2 - 3y^2$: $g'(y) = -3y^2$, so $g(y) = -y^3 + C$.

Therefore $f(x,y) = x^2 y + x - y^3 + C$.

**Verification.** $\nabla f = (2xy + 1)\,\mathbf{i} + (x^2 - 3y^2)\,\mathbf{j} = \mathbf{F}$. Correct.

## Non-Uniqueness of the Potential

Potential functions are not unique: if $f$ is a potential for $\mathbf{F}$, then so is $f + C$ for any constant $C$, since $\nabla C = \mathbf{0}$. Conversely, if $f_1$ and $f_2$ are both potentials for $\mathbf{F}$ on a connected domain, then $\nabla(f_1 - f_2) = \mathbf{0}$, so $f_1 - f_2$ is constant. Potential functions are therefore unique up to an additive constant — exactly analogous to the situation with antiderivatives in one-variable calculus, which are unique up to a constant of integration.

This non-uniqueness is harmless for computing work integrals: $f(B) - f(A)$ is the same for $f$ and $f + C$.

## The Method in Three Dimensions

In three dimensions, $\mathbf{F} = P\,\mathbf{i} + Q\,\mathbf{j} + R\,\mathbf{k}$ and we need $f$ satisfying

$$\frac{\partial f}{\partial x} = P, \quad \frac{\partial f}{\partial y} = Q, \quad \frac{\partial f}{\partial z} = R.$$

**Step 1.** Integrate $\partial f/\partial x = P$ with respect to $x$:

$$f(x, y, z) = \int P\,dx + g(y, z).$$

**Step 2.** Differentiate with respect to $y$ and set equal to $Q$:

$$\frac{\partial}{\partial y}\int P\,dx + \frac{\partial g}{\partial y} = Q \implies \frac{\partial g}{\partial y} = Q - \frac{\partial}{\partial y}\int P\,dx.$$

The right side depends only on $y$ and $z$. Integrate with respect to $y$:

$$g(y,z) = \int \left(Q - \frac{\partial}{\partial y}\int P\,dx\right)dy + h(z).$$

**Step 3.** Differentiate $f$ with respect to $z$, set equal to $R$, and solve for $h'(z)$. Integrate to find $h(z)$.

**Worked Example.** Let $\mathbf{F} = (yz)\,\mathbf{i} + (xz + 2y)\,\mathbf{j} + (xy - 3z^2)\,\mathbf{k}$.

Check: $\partial P/\partial y = z = \partial Q/\partial x$; $\partial P/\partial z = y = \partial R/\partial x$; $\partial Q/\partial z = x = \partial R/\partial y$. Conservative.

**Step 1.** $f = \int yz\,dx = xyz + g(y,z)$.

**Step 2.** $\partial f/\partial y = xz + \partial g/\partial y = xz + 2y$, so $\partial g/\partial y = 2y$, giving $g = y^2 + h(z)$.

**Step 3.** $\partial f/\partial z = xy + h'(z) = xy - 3z^2$, so $h'(z) = -3z^2$, giving $h = -z^3 + C$.

Result: $f(x,y,z) = xyz + y^2 - z^3 + C$.

## The Line Integral Payoff

Having found $f$, any line integral of $\mathbf{F}$ along a smooth curve $C$ from $A$ to $B$ is computed instantly:

$$\int_C \mathbf{F} \cdot d\mathbf{r} = f(B) - f(A).$$

**Example.** Using $\mathbf{F} = (2xy+1)\,\mathbf{i} + (x^2-3y^2)\,\mathbf{j}$ from above, compute the work done moving from $(0,0)$ to $(1,2)$ along the parabola $y = 2x^2$.

Without the potential function, this requires a parametric integration. With the potential $f = x^2 y + x - y^3$:

$$W = f(1, 2) - f(0, 0) = (1 \cdot 2 + 1 - 8) - 0 = -5.$$

## Connection to Exact Differential Equations

The potential-function problem is closely related to **exact differential equations** from the study of ODEs. A first-order equation written in the form

$$P(x, y)\,dx + Q(x, y)\,dy = 0$$

is called **exact** if $\partial P/\partial y = \partial Q/\partial x$, precisely the condition that $P\,\mathbf{i} + Q\,\mathbf{j}$ is conservative. Solving an exact ODE is identical to finding a potential function: seek $f$ with $\partial f/\partial x = P$ and $\partial f/\partial y = Q$, then the general solution is $f(x,y) = C$. This is one of the first and most important intersections between the study of vector fields and the study of differential equations.

## Summary

Finding a potential function for a conservative vector field is a structured multi-step integration. The process integrates one component at a time, introduces a function of the remaining variables, and uses the remaining component equations to pin that function down. The result is unique up to an additive constant. Once the potential is known, line integrals reduce to endpoint evaluations, and the connection to exact differential equations provides a bridge between vector calculus and the broader theory of ODEs.
