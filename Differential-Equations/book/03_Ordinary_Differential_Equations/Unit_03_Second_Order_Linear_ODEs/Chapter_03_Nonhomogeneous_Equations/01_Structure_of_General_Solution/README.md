# Structure of the General Solution

For the nonhomogeneous equation $L[y] = g(x)$, where $L[y] = a(x)y'' + b(x)y' + c(x)y$, the general solution decomposes as

$$y = y_h + y_p,$$

where $y_h$ is the general solution of the homogeneous equation $L[y] = 0$ and $y_p$ is any particular solution of $L[y] = g$. This decomposition is the fundamental structural result for nonhomogeneous linear equations.

## Proof of the Decomposition

**Theorem.** Suppose $y_p$ is a particular solution of $L[y] = g$ and $y_h = c_1 y_1 + c_2 y_2$ is the general homogeneous solution. Then $y = y_h + y_p$ is a solution of $L[y] = g$, and every solution of $L[y] = g$ has this form.

**Proof.** First, $y = y_h + y_p$ is a solution: $L[y_h + y_p] = L[y_h] + L[y_p] = 0 + g = g$.

Second, let $\tilde{y}$ be any solution of $L[y] = g$. Then $L[\tilde{y} - y_p] = L[\tilde{y}] - L[y_p] = g - g = 0$, so $\tilde{y} - y_p$ is a solution of the homogeneous equation. By the structure of the homogeneous solution space, $\tilde{y} - y_p = c_1 y_1 + c_2 y_2$ for some constants. Therefore $\tilde{y} = y_h + y_p$ for this choice of $c_1, c_2$.

## Transient and Steady State

In physical applications, the homogeneous solution is the **transient** and the particular solution is the **steady state** or **forced response**.

For a stable system (characteristic roots with negative real parts), the homogeneous solution $y_h$ decays to zero as $t \to \infty$. The particular solution $y_p$ persists and represents the long-term behavior forced by the input $g(t)$. No matter what the initial conditions are (which affect only $c_1, c_2$ in $y_h$), the steady state is the same: $y_p$.

For an undamped system (purely imaginary roots), the transient does not decay; instead, it combines with the particular solution to give quasiperiodic behavior (when the frequencies are different) or resonance (when they match).

## Choosing the Particular Solution

Any one particular solution serves. In practice, one chooses the simplest available. For constant-coefficient equations with special forcing, undetermined coefficients gives a $y_p$ that is algebraically clean. For general forcing, variation of parameters gives a definite-integral formula for $y_p$.

Note that different choices of $y_p$ differ by a solution of the homogeneous equation: if $y_{p1}$ and $y_{p2}$ are both particular solutions, then $y_{p1} - y_{p2}$ satisfies $L[y_{p1} - y_{p2}] = g - g = 0$. So $y_{p1} = y_{p2} + c_1 y_1 + c_2 y_2$. This means the particular solution is unique only up to addition of homogeneous solutions; the general solution $y = y_h + y_p$ is the same regardless of which $y_p$ one uses.

## Initial Value Problems

Given the general solution $y = c_1 y_1 + c_2 y_2 + y_p$, initial conditions $y(x_0) = y_0$ and $y'(x_0) = y_1$ determine $c_1$ and $c_2$ uniquely (since the system has nonzero Wronskian at $x_0$):

$$c_1 y_1(x_0) + c_2 y_2(x_0) = y_0 - y_p(x_0),$$
$$c_1 y_1'(x_0) + c_2 y_2'(x_0) = y_1 - y_p'(x_0).$$

The unique solution exists on the entire interval of continuity of the coefficients and $g$.
