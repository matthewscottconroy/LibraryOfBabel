# Riccati Equations

The **Riccati equation** has the form

$$y' = p(x) + q(x)\,y + r(x)\,y^2,$$

a first-order ODE that is quadratic in $y$. Named after Jacopo Riccati, who studied particular cases in the early eighteenth century, the Riccati equation occupies a special place in ODE theory: it is the simplest nonlinear ODE that cannot be solved in closed form by a universal algorithm. However, if one particular solution is known by any means (observation, symmetry, or guessing), the equation can be reduced to a first-order linear equation, giving the complete general solution.

## Why Riccati Is Difficult in General

The Riccati equation is a Bernoulli equation only when $p = 0$ (then it is $y' = q(x)y + r(x)y^2$, which is Bernoulli with $n = 2$). In the general case with all three terms present, no substitution of the form $w = y^\alpha$ linearizes it. The equation arises in many important contexts: optimal control theory (where it is called the matrix Riccati equation), the theory of continued fractions, the Schwarzian derivative in complex analysis, and as a central object in the Ermakov-Pinney system.

## Reduction When One Solution Is Known

**Theorem.** If $y_1(x)$ is a particular solution of $y' = p + qy + ry^2$, then the substitution $y = y_1 + 1/v$ reduces the equation to the linear first-order ODE

$$v' + (q + 2ry_1)v = -r.$$

**Proof.** Compute $y' = y_1' + (-1/v^2)v' = y_1' - v'/v^2$. Substitute:

$$y_1' - \frac{v'}{v^2} = p + q\!\left(y_1 + \frac{1}{v}\right) + r\!\left(y_1 + \frac{1}{v}\right)^2.$$

Expanding the right side:

$$p + qy_1 + \frac{q}{v} + ry_1^2 + \frac{2ry_1}{v} + \frac{r}{v^2}.$$

Since $y_1$ is a solution: $y_1' = p + qy_1 + ry_1^2$. Canceling these terms from both sides:

$$-\frac{v'}{v^2} = \frac{q}{v} + \frac{2ry_1}{v} + \frac{r}{v^2}.$$

Multiplying through by $-v^2$:

$$v' = -qv - 2ry_1 v - r \implies v' + (q + 2ry_1)v = -r.$$

This is linear in $v$ with integrating factor $\mu = e^{\int(q + 2ry_1)\,dx}$.

## Worked Example

Solve $y' = 1 + x^2 - 2xy + y^2$, given that $y_1 = x$ is a particular solution.

Verify: $y_1' = 1$ and $1 + x^2 - 2x\cdot x + x^2 = 1 + x^2 - 2x^2 + x^2 = 1$. Confirmed.

Substitute $y = x + 1/v$. With $p = 1 + x^2$, $q = -2x$, $r = 1$, $y_1 = x$:

$$v' + (-2x + 2\cdot 1\cdot x)v = -1 \implies v' + 0\cdot v = -1 \implies v' = -1.$$

So $v = -x + C$. Therefore $y = x + 1/(C - x)$.

**Verification.** $y' = 1 + 1/(C-x)^2$. And $1 + x^2 - 2xy + y^2 = 1 + x^2 - 2x(x + 1/(C-x)) + (x + 1/(C-x))^2$. Expanding: $= 1 + x^2 - 2x^2 - 2x/(C-x) + x^2 + 2x/(C-x) + 1/(C-x)^2 = 1 + 1/(C-x)^2$. Correct.

## General Solution Structure

Once one particular solution $y_1$ is found, the general solution is $y = y_1 + 1/v$ where $v$ satisfies a linear equation. The general solution of the linear equation has the form $v = v_h + v_p$, giving

$$y = y_1 + \frac{1}{v_h + v_p}.$$

If a second particular solution $y_2$ is also known, the cross-ratio of any four solutions is constant (a classical result): the Riccati equation has a very rigid symmetry group (the Mobius transformations of the solution space).

## The Matrix Riccati Equation

In control theory, the state equation $\dot{x} = Ax + Bu$ and performance criterion lead to the **matrix Riccati equation**

$$\dot{P} + PA + A^T P - PBR^{-1}B^T P + Q = 0,$$

where $P(t)$ is a symmetric matrix-valued function. The scalar Riccati equation is the $1 \times 1$ case. The matrix version has no general closed-form solution, but steady-state solutions (the **algebraic Riccati equation**, obtained by setting $\dot{P} = 0$) can be found by solving an eigenvalue problem. This is one of the most important equations in optimal control and signal processing.
