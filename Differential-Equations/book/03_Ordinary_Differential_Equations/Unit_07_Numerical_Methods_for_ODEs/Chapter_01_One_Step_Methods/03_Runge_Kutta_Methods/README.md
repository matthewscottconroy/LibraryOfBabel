# Runge-Kutta Methods

The Runge-Kutta (RK) methods are a family of one-step numerical methods for ordinary differential equations that achieve high accuracy by using multiple evaluations of $f$ within each time step. They are the workhorses of scientific computing: the classical fourth-order Runge-Kutta method (RK4) is arguably the most widely used ODE solver in the world, combining high accuracy with robustness and simplicity of implementation. Understanding RK methods — their structure, their order conditions, and the rationale behind RK4 — provides the foundation for the entire field of one-step ODE integration.

## General Explicit Runge-Kutta Structure

An explicit $s$-stage Runge-Kutta method computes $s$ intermediate slope estimates $k_1, k_2, \ldots, k_s$ and combines them to advance the solution:

$$k_i = f\!\left(t_n + c_i h,\; y_n + h\sum_{j=1}^{i-1}a_{ij}k_j\right), \qquad i = 1, 2, \ldots, s,$$

$$y_{n+1} = y_n + h\sum_{i=1}^s b_i k_i.$$

The parameters are: the **nodes** $c_i$ (fractional positions within $[t_n, t_{n+1}]$), the **Runge-Kutta matrix** $A = (a_{ij})$ (lower triangular for explicit methods), and the **weights** $b_i$. These are organized in a **Butcher tableau**:

$$\begin{array}{c|c}
\mathbf{c} & A \\
\hline
 & \mathbf{b}^T
\end{array}$$

The method is explicit because each $k_i$ depends only on previously computed $k_j$ ($j < i$): the Runge-Kutta matrix $A$ is strictly lower triangular.

## Order Conditions

The order of an RK method is determined by matching the Taylor expansion of $y(t_{n+1})$ with the expansion of the RK increment $h\sum b_i k_i$. The Taylor expansion of $y$ involves iterated derivatives of $f$, which are indexed by **rooted trees** (a combinatorial structure). The conditions for a method to have order $p$ are a system of polynomial equations in the $b_i$, $c_i$, $a_{ij}$ — one equation for each rooted tree of order up to $p$.

For order $p = 1$: one condition, $\sum b_i = 1$.

For order $p = 2$: two conditions (including $\sum b_i c_i = 1/2$).

For order $p = 3$: four conditions.

For order $p = 4$: eight conditions.

The maximum achievable order for an explicit $s$-stage method is known as the **Runge-Kutta order barrier**: for $s \leq 4$ stages, order $p = s$ is achievable; for $s = 5$ stages, maximum order is $4$ (not $5$); the gap between stages and order grows further for higher stage numbers. This is why RK4 (four stages, order four) is considered optimal in the classical one-step setting: adding a fifth stage does not gain an extra order, and other methods (multistep, embedded) become more efficient.

## The Classical RK4 Method

The classical fourth-order Runge-Kutta method is defined by the Butcher tableau:

$$\begin{array}{c|cccc}
0 & & & & \\
1/2 & 1/2 & & & \\
1/2 & 0 & 1/2 & & \\
1 & 0 & 0 & 1 & \\
\hline
 & 1/6 & 1/3 & 1/3 & 1/6
\end{array}$$

Explicitly:

$$k_1 = f(t_n, y_n),$$
$$k_2 = f(t_n + h/2,\; y_n + (h/2)k_1),$$
$$k_3 = f(t_n + h/2,\; y_n + (h/2)k_2),$$
$$k_4 = f(t_n + h,\; y_n + hk_3),$$
$$y_{n+1} = y_n + \frac{h}{6}(k_1 + 2k_2 + 2k_3 + k_4).$$

The slope estimate is a weighted average of four slopes: $k_1$ at the left endpoint, $k_2$ at the midpoint using the Euler predictor, $k_3$ again at the midpoint using $k_2$ as a corrected slope, and $k_4$ at the right endpoint using $k_3$. The weights $1/6, 2/6, 2/6, 1/6$ are those of Simpson's rule applied to the four evaluation points.

The local truncation error is $O(h^5)$, giving global error $O(h^4)$. Halving the step size reduces the error by a factor of $2^4 = 16$, at the cost of doubling the number of steps (hence doubling the number of function evaluations per unit of $t$). In contrast, halving the step size for Euler only reduces the error by $2^1 = 2$. The efficiency advantage of RK4 is enormous.

## Worked Example

Solve $y' = y$, $y(0) = 1$ at $t = 1$ with RK4 using $h = 0.5$. (Exact: $y(1) = e \approx 2.71828$.)

Step 1 ($t_0 = 0$, $y_0 = 1$, $h = 0.5$):

$k_1 = f(0, 1) = 1$.
$k_2 = f(0.25, 1 + 0.25) = f(0.25, 1.25) = 1.25$.
$k_3 = f(0.25, 1 + 0.5 \cdot 0.625) = f(0.25, 1.3125) = 1.3125$.
$k_4 = f(0.5, 1 + 0.5 \cdot 1.3125) = f(0.5, 1.65625) = 1.65625$.

$y_1 = 1 + \frac{0.5}{6}(1 + 2(1.25) + 2(1.3125) + 1.65625) = 1 + \frac{0.5}{6}(7.78125) = 1 + 0.648437 = 1.648437$.

Exact: $e^{0.5} \approx 1.648721$. Error: $|1.648721 - 1.648437| \approx 2.84 \times 10^{-4}$.

Step 2 ($t_1 = 0.5$, $y_1 = 1.648437$):

By the same computation (scaling by $y_1$): $y_2 = y_1 \cdot (y_1/y_0)_{\text{from step 1}} = y_1^2/y_0 \cdot$... more directly: $y_2 = 1.648437 \times 1.648437 \approx$ ... applying RK4 to $y' = y$ gives $y_{n+1} = y_n(1 + h + h^2/2 + h^3/6 + h^4/24)$ (the degree-4 Taylor polynomial of $e^h$). For $h = 0.5$: $e^{0.5} \approx 1.6487213$, and RK4 gives $(1 + 0.5 + 0.125 + 0.02083 + 0.002604) = 1.648437$. So $y_2 = 1.648437^2/1 \cdot$... the RK4 approximation at $t=1$ is $y_2 = (1.648437)^2 \approx 2.717264$.

Error: $|e - 2.717264| \approx 1.5 \times 10^{-4}$. Compare to Euler with $h = 0.5$: error $\approx 0.277$. RK4 is about 1800 times more accurate for the same number of steps.

## Embedded Runge-Kutta and Adaptive Methods

An important extension is **embedded Runge-Kutta pairs**: two RK methods of orders $p$ and $p+1$ that share the same stage evaluations $k_1, \ldots, k_s$ but use different weight vectors $\mathbf{b}$ and $\hat{\mathbf{b}}$. The difference $y_{n+1} - \hat{y}_{n+1}$ provides an estimate of the local error, which can be used to adaptively control the step size.

The most widely used embedded pair is **Dormand-Prince (DOPRI5)**: a 7-stage method providing fourth- and fifth-order approximations with a single set of $f$-evaluations. It underlies MATLAB's `ode45` solver and many other production ODE codes. The step size is adjusted to keep the estimated local error below a user-specified tolerance, and the number of function evaluations is minimized by accepting steps as large as possible.

## Implicit Runge-Kutta Methods

Implicit RK (IRK) methods allow the Runge-Kutta matrix $A$ to be full (not lower triangular), requiring the simultaneous solution of all $k_i$ at each step. This is more expensive per step but allows very large stability regions. The **Gauss-Legendre** methods are IRK methods that achieve order $2s$ from $s$ stages — far exceeding the explicit order barrier — and have the exceptional property of being A-stable (stable for all step sizes on any problem with $\text{Re}(\lambda) \leq 0$) and even B-stable (stable for nonlinear stiff systems). **Radau** methods are a related family designed specifically for stiff problems, combining high order with excellent stability. These implicit methods, though more expensive per step, are the methods of choice for stiff ODEs.
