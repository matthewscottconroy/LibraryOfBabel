# Variation of Parameters for Second-Order Equations

Variation of parameters is the general method for finding particular solutions of nonhomogeneous linear equations. Unlike undetermined coefficients, it works for any continuous forcing function $g(x)$, requiring only that the homogeneous solutions $y_1$ and $y_2$ be known.

## Setup

Consider $y'' + p(x)y' + q(x)y = g(x)$ with homogeneous solutions $y_1$ and $y_2$ forming a fundamental set (so $W(y_1, y_2) \neq 0$). Seek

$$y_p = v_1(x)y_1(x) + v_2(x)y_2(x),$$

where $v_1$ and $v_2$ are unknown functions to be determined. Two conditions are needed to determine two unknowns, so we impose one structural condition in addition to requiring that $y_p$ satisfy the ODE.

**Structural condition.** Impose $v_1'y_1 + v_2'y_2 = 0$. This eliminates the $v_1'$ and $v_2'$ terms from $y_p'$, giving $y_p' = v_1 y_1' + v_2 y_2'$ (a clean formula).

**Substitution.** Differentiating $y_p' = v_1 y_1' + v_2 y_2'$:

$$y_p'' = v_1' y_1' + v_1 y_1'' + v_2' y_2' + v_2 y_2''.$$

Substituting into the ODE:

$$v_1'y_1' + v_1(y_1'' + py_1' + qy_1) + v_2'y_2' + v_2(y_2'' + py_2' + qy_2) = g.$$

Since $y_1$ and $y_2$ are homogeneous solutions, the parenthetical expressions vanish:

$$v_1'y_1' + v_2'y_2' = g.$$

## The System and Its Solution

The two conditions form the linear system for $v_1'$ and $v_2'$:

$$\begin{pmatrix}y_1 & y_2 \\ y_1' & y_2'\end{pmatrix}\begin{pmatrix}v_1'\\v_2'\end{pmatrix} = \begin{pmatrix}0\\g\end{pmatrix}.$$

The coefficient matrix has determinant $W(y_1, y_2) \neq 0$. By Cramer's rule:

$$v_1' = \frac{\begin{vmatrix}0 & y_2\\g & y_2'\end{vmatrix}}{W} = \frac{-y_2 g}{W}, \qquad v_2' = \frac{\begin{vmatrix}y_1 & 0\\y_1' & g\end{vmatrix}}{W} = \frac{y_1 g}{W}.$$

Integrating:

$$v_1 = -\int \frac{y_2(x)g(x)}{W(x)}\,dx, \qquad v_2 = \int \frac{y_1(x)g(x)}{W(x)}\,dx.$$

The particular solution is

$$y_p = -y_1\int \frac{y_2 g}{W}\,dx + y_2\int \frac{y_1 g}{W}\,dx.$$

## Worked Example

Find a particular solution of $y'' + y = \sec x = 1/\cos x$.

Homogeneous solutions: $y_1 = \cos x$, $y_2 = \sin x$. Wronskian: $W = \cos^2 x + \sin^2 x = 1$.

$$v_1' = \frac{-\sin x \cdot \sec x}{1} = -\tan x, \qquad v_2' = \frac{\cos x \cdot \sec x}{1} = 1.$$

$$v_1 = \ln|\cos x|, \qquad v_2 = x.$$

$$y_p = \cos x \cdot \ln|\cos x| + \sin x \cdot x = x\sin x + \cos x\ln|\cos x|.$$

This $y_p$ involves $\ln|\cos x|$, which undetermined coefficients could never produce. The method of variation of parameters handles this effortlessly.

## Integral Form and the Green's Function

The particular solution can be written as a single integral:

$$y_p(x) = \int_{x_0}^x G(x, t)g(t)\,dt, \quad \text{where} \quad G(x, t) = \frac{y_1(t)y_2(x) - y_1(x)y_2(t)}{W(t)}.$$

The function $G(x, t)$ is the **Green's function** for the operator $L$ with zero initial conditions at $x_0$. It represents the response at $x$ to a unit impulse at $t$. The integral formula expresses the principle of superposition for continuous forcing: the solution is the "sum" of responses to infinitesimal impulses $g(t)\,dt$ at each point $t$.

This connection between variation of parameters and Green's functions is the gateway to the more general theory developed in Unit 8.
