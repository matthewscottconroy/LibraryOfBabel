# Predictor-Corrector Methods

Predictor-corrector methods combine an explicit multistep method (the predictor) with an implicit multistep method (the corrector) to achieve the stability benefits of implicit methods with a reduced computational cost. The key insight is that the implicit equation need not be solved exactly — a single iteration of fixed-point iteration using the explicit predictor as the initial guess is typically sufficient. This avoids the full Newton solve of a general implicit method while capturing most of the stability advantage.

## The PECE Strategy

The standard predictor-corrector cycle is PECE (Predict, Evaluate, Correct, Evaluate):

**P** (Predict): Use the Adams-Bashforth $k$-step formula to compute an initial approximation $y_{n+1}^P$:

$$y_{n+1}^P = y_n + h\sum_{j=0}^{k-1}\beta_j^* f_{n-j}.$$

**E** (Evaluate): Compute $f_{n+1}^P = f(t_{n+1}, y_{n+1}^P)$.

**C** (Correct): Use the Adams-Moulton formula with $f_{n+1}^P$ in place of $f(t_{n+1}, y_{n+1})$:

$$y_{n+1}^C = y_n + h\left(\beta_{-1}f_{n+1}^P + \sum_{j=0}^{k-1}\beta_j f_{n-j}\right).$$

**E** (Evaluate): Compute $f_{n+1} = f(t_{n+1}, y_{n+1}^C)$ for use in the next step.

The result is two function evaluations per step (one in P, one in the final E — the corrector uses the predictor's evaluation), compared to one for pure Adams-Bashforth (but with better stability) and potentially many for implicit Newton solves (but with higher computational cost). In practice the corrector evaluation in C uses the predictor's $f$-value ($f_{n+1}^P$), not a new one, so PECE is sometimes just called PE(CE) to emphasize that C uses the already-computed $f$-value.

## The AB4-AM4 Pair

The standard fourth-order predictor-corrector pair uses the 4-step Adams-Bashforth method as the predictor and the 4-step Adams-Moulton (3 previous values plus the predicted future value) as the corrector. Both achieve fourth-order accuracy; the corrector has a much smaller error constant.

The error constant (the coefficient in the LTE) for AB4 is $251/720 \approx 0.349$, and for AM4 it is $-19/720 \approx -0.026$ — the corrector's error constant is about 13 times smaller. This means the corrected value $y_{n+1}^C$ is significantly more accurate than the predicted $y_{n+1}^P$, even though both are fourth-order.

The predictor-corrector error can be estimated from the difference between the predicted and corrected values, since both have the same order:

$$y_{n+1}^C - y_{n+1}^P \approx \frac{19}{270}[y_{n+1}^C - y_{n+1}^P] + \text{higher order}.$$

More precisely, the local truncation error of the corrected value can be estimated as approximately $\frac{C_C}{C_P - C_C}(y_{n+1}^C - y_{n+1}^P)$, where $C_P$ and $C_C$ are the error constants of the predictor and corrector. This provides a cheap local error estimate for adaptive step-size control without the overhead of embedded Runge-Kutta methods.

## Stability of Predictor-Corrector Methods

The stability of the PECE scheme is intermediate between the pure predictor (AB4) and the pure corrector (AM4). For the test equation $y' = \lambda y$:

$$y_{n+1}^P = \sum_{j}a_j y_{n-j} + h\lambda\sum_j b_j^P y_{n-j}, \qquad y_{n+1} = \sum_j a_j y_{n-j} + h\lambda(b^C_{-1}y_{n+1}^P + \sum_j b_j^C y_{n-j}).$$

Substituting, the PECE method is equivalent to a (generally more stable) explicit linear multistep method with modified coefficients. The region of absolute stability for AB4-AM4 PECE is larger than for AB4 alone, though still smaller than for the implicit AM4.

Applying the corrector multiple times (PECECE, PECECECE, ...) increases stability at the cost of additional function evaluations, but eventually converges to the fully implicit corrector as the number of iterations tends to infinity. For most non-stiff problems, one corrector application is sufficient.

## Variable-Order, Variable-Step Adams Methods

Modern production ODE codes (such as MATLAB's `ode113`, which implements a variant of the Adams PECE scheme) use variable-order, variable-step Adams methods. The order of the method is selected adaptively based on the smoothness of the solution: high order (up to 12) where the solution is smooth, lower order where it changes rapidly or where step-size changes are needed. The step size is controlled to maintain the local error below a user-specified tolerance.

The variable-order capability is a significant advantage over Runge-Kutta methods: for smooth problems, high-order Adams methods achieve machine precision with very few function evaluations, while RK methods are limited to order 4–8 in practice. For non-stiff problems with smooth solutions over long time intervals, the Adams PECE scheme with variable order and step size is often the most efficient available method.

## Contrasting Approaches: RK vs. Adams PECE

The Runge-Kutta and Adams approaches represent different design philosophies. RK methods are self-starting, easy to change step size, have large stability regions (for explicit methods), but require multiple function evaluations per step. Adams methods are efficient in $f$-evaluations but require startup, are harder to change step size (all past values and step sizes must be consistent), and have smaller stability regions for explicit methods.

For short integrations of non-stiff problems starting from a single initial condition with a fixed or slowly varying step size, RK4 is usually the practical choice due to its simplicity. For long integrations of smooth non-stiff problems, or where $f$ is very expensive, an Adams PECE method with variable order and step size is typically more efficient. For stiff problems, BDF methods (next chapter) dominate, and neither RK nor Adams explicit/PECE methods are appropriate.
