# General, Particular, and Singular Solutions

Finding "a solution" to a differential equation is rarely the complete goal. An $n$-th order ODE typically has infinitely many solutions, parametrized by $n$ arbitrary constants. The theory provides names and a precise framework for distinguishing among these solutions and understanding how they relate to one another.

## The General Solution

The **general solution** of an $n$-th order ODE is a family of functions

$$y = \phi(x, C_1, C_2, \ldots, C_n),$$

depending on $n$ arbitrary constants $C_1, \ldots, C_n$, with the property that every solution obtainable from the equation by imposing $n$ initial or boundary conditions is represented by this family for some choice of the constants. The general solution is not itself a single function; it is a parametric family.

For a linear equation, the general solution has a clean algebraic structure. The general solution of the homogeneous equation $y'' + p(x)y' + q(x)y = 0$ is $y = C_1 y_1 + C_2 y_2$, where $y_1$ and $y_2$ are any two linearly independent solutions. The general solution of the nonhomogeneous equation $y'' + p(x)y' + q(x)y = g(x)$ is $y = C_1 y_1 + C_2 y_2 + y_p$, where $y_p$ is any one particular solution of the nonhomogeneous equation. This additive structure is a direct consequence of linearity and superposition.

For nonlinear equations, the general solution may be harder to identify, and one cannot always be certain that a given family contains every solution. Nonlinear equations may have solutions that do not arise from any choice of the arbitrary constants in the general solution.

## Particular Solutions

A **particular solution** is a single function obtained from the general solution by assigning specific values to the arbitrary constants. The assignment is typically made to satisfy imposed conditions: initial conditions specify the value of $y$ and its derivatives at a single point, while boundary conditions specify values at two or more distinct points.

**Example.** The general solution of $y'' + y = 0$ is $y = C_1\cos x + C_2 \sin x$. Imposing the initial conditions $y(0) = 1$ and $y'(0) = 0$ gives $C_1 = 1$ and $C_2 = 0$, so the particular solution is $y = \cos x$. Imposing instead $y(0) = 0$ and $y(\pi/2) = 1$ (boundary conditions) gives $C_1 = 0$ and $C_2 = 1$, yielding $y = \sin x$.

The word "particular" in this context should not be confused with "particular solution" as used for nonhomogeneous equations, where it refers to any one function satisfying the full nonhomogeneous equation (before adding the complementary solution of the homogeneous equation). The two uses are related but distinct.

## Singular Solutions

A **singular solution** is a solution that cannot be obtained from the general solution by any choice of the constants $C_1, \ldots, C_n$. Such solutions are therefore not members of the general family and arise exclusively in the nonlinear case.

The classical example is the equation

$$y' = \frac{3}{2}y^{1/3}.$$

Separating variables: $y^{-1/3}\,dy = \frac{3}{2}\,dx$, so $\frac{3}{2}y^{2/3} = \frac{3}{2}x + C$, giving $y^{2/3} = x + C$, or

$$y = (x + C)^{3/2}.$$

This is the general solution (with the understanding that $x > -C$ for real values). However, the function $y = 0$ also satisfies the equation: $y' = 0$ and $\frac{3}{2}(0)^{1/3} = 0$. This zero solution cannot be obtained from $(x+C)^{3/2}$ for any real $C$ (since $(x+C)^{3/2}$ is identically zero only if we extend to complex values in a degenerate way). The function $y = 0$ is therefore a singular solution.

A more elaborate example comes from Clairaut's equation:

$$y = xy' + f(y'),$$

where $f$ is a given smooth function. Differentiating with respect to $x$:

$$y' = y' + xy'' + f'(y')y'',$$

which simplifies to $y''[x + f'(y')] = 0$. Either $y'' = 0$, giving $y' = C$ (a constant), and then substitution yields the general solution $y = Cx + f(C)$; or $x + f'(y') = 0$, which is a parametric equation for a curve called the **envelope** of the family of lines $y = Cx + f(C)$. This envelope is the singular solution.

Geometrically, singular solutions are **envelopes** of the family of curves constituting the general solution. Each member of the general family is tangent to the envelope at exactly one point, and the envelope is itself a solution curve.

## The Envelope Condition

To find singular solutions systematically, one uses the **c-discriminant method**: write the general solution as $F(x, y, C) = 0$ (an implicit family), and eliminate $C$ between

$$F(x, y, C) = 0 \qquad \text{and} \qquad \frac{\partial F}{\partial C}(x, y, C) = 0.$$

The result is an equation in $x$ and $y$ alone, which may represent the envelope (singular solution) or may represent a locus with other geometric significance (such as a cusp locus or node locus). Each candidate must be verified by substitution into the original ODE.

**Example.** For the equation $(y')^2 + y = 1$, the general solution (after solving) turns out to be $y = 1 - \frac{1}{4}(x - C)^2$. This is a family of downward-opening parabolas. The envelope is found by differentiating $F(x, y, C) = y - 1 + \frac{1}{4}(x-C)^2 = 0$ with respect to $C$: $-\frac{1}{2}(x - C) = 0$, giving $C = x$. Substituting back: $y = 1$. Indeed, $y = 1$ satisfies the ODE: $(y')^2 + y = 0 + 1 = 1$. The constant function $y = 1$ is the singular solution.

## Why the Distinction Matters

Understanding the difference between general, particular, and singular solutions is important for several reasons. In modeling, one typically seeks the particular solution satisfying physically meaningful conditions, and knowing the general solution provides the framework within which initial or boundary conditions determine the unique answer. Singular solutions, while exotic mathematically, do arise in applications: the envelope of a family of straight-line trajectories, for instance, can represent the wavefront in geometric optics.

Moreover, the existence of singular solutions is a warning sign about the equation's behavior near certain curves. Near a singular solution, the uniqueness theorem may fail, meaning that from a point on the singular solution curve, more than one solution curve may pass. Recognizing this possibility prevents one from making unfounded uniqueness claims.
