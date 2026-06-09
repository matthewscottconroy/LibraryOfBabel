# Local and Global Truncation Error

The accuracy of a numerical ODE method is characterized by two related but distinct quantities: the **local truncation error** (LTE) and the **global error**. The local truncation error measures the discrepancy introduced at a single step, while the global error measures the total accumulated discrepancy after many steps. Understanding the relationship between these quantities — specifically, how local errors accumulate and why the global order equals the local order — is fundamental to numerical ODE theory.

## Local Truncation Error

For a one-step method $y_{n+1} = y_n + h\Phi(t_n, y_n; h)$, the **local truncation error** at step $n+1$ is defined by inserting the exact solution and measuring the residual:

$$\tau_{n+1}(h) = \frac{y(t_{n+1}) - y(t_n)}{h} - \Phi(t_n, y(t_n); h).$$

Equivalently, $\tau_{n+1}$ is the amount by which the exact solution fails to satisfy the method's formula (up to a factor of $h$). The LTE measures how well the method approximates the ODE at a single step, assuming the previous value is exact.

A method is **consistent of order $p$** if $|\tau_{n+1}(h)| = O(h^p)$ uniformly in $n$ as $h \to 0$. The LTE for Euler's method is $O(h)$ (first order); for RK4, it is $O(h^4)$ (fourth order).

**Computing the LTE:** Expand the exact solution in a Taylor series and compare with the method's formula.

For Euler's method: $y(t_{n+1}) = y(t_n) + hy'(t_n) + \frac{h^2}{2}y''(t_n) + O(h^3)$. The Euler increment is $hf(t_n, y(t_n)) = hy'(t_n)$. So $\tau_{n+1} = \frac{h}{2}y''(t_n) + O(h^2)$: LTE $= O(h)$.

For RK4: the LTE is $\frac{h^4}{120}y^{(5)}(\xi_n)$ for some $\xi_n \in (t_n, t_{n+1})$: LTE $= O(h^4)$.

## Global Error

The **global error** at step $n$ is $e_n = y(t_n) - y_n$. It is not the same as the LTE: $e_n$ accumulates from $e_0 = 0$ through $n$ steps, with each step contributing a local truncation error and amplifying previous errors via the Lipschitz constant.

**Error propagation:** For a consistent method, the global error satisfies:

$$e_{n+1} = e_n + h[\Phi(t_n, y(t_n)) - \Phi(t_n, y_n)] + h\tau_{n+1}.$$

The first difference is bounded by the Lipschitz constant of $\Phi$: $|\Phi(t, u) - \Phi(t, v)| \leq \Lambda |u - v|$ for some $\Lambda > 0$ (related to the Lipschitz constant $L$ of $f$). So:

$$|e_{n+1}| \leq (1 + h\Lambda)|e_n| + h|\tau_{n+1}|.$$

With $|e_0| = 0$ and $|\tau_{n+1}| \leq Ch^p$, this recurrence solves to:

$$|e_n| \leq \frac{Ch^p}{\Lambda}\left[(1+h\Lambda)^n - 1\right] \leq \frac{Ch^p}{\Lambda}\left[e^{\Lambda(t_n - t_0)} - 1\right].$$

For fixed $T = t_0 + Nh$ and $h \to 0$, $|e_n| \leq Ch^p \cdot \frac{e^{\Lambda T} - 1}{\Lambda} = O(h^p)$.

**Conclusion:** A consistent method of order $p$ (LTE $= O(h^{p+1})$, or equivalently $\tau = O(h^p)$) has global error $O(h^p)$. The global order equals the local order — errors do not accumulate to change the order.

This result depends on the Lipschitz condition and the boundedness of the exact solution on $[t_0, T]$. The constant in $O(h^p)$ grows exponentially with $\Lambda T$, so for long integrations or large Lipschitz constants, the global error can be large even if $h$ is small.

## Multistep Methods: Consistency and Zero-Stability

For a linear $k$-step method $\sum_{j=0}^k \alpha_j y_{n-j} = h\sum_{j=0}^k \beta_j f_{n-j}$, the analysis is more involved. The LTE is defined similarly. The method is **consistent of order $p$** if the coefficients satisfy the order conditions up to order $p$:

$$\sum_{j=0}^k \alpha_j = 0, \qquad \sum_{j=0}^k(-j\alpha_j - \beta_j) = 0, \quad \ldots$$

The global error analysis for multistep methods requires an additional concept: **zero-stability**. A multistep method is zero-stable if all roots of its first characteristic polynomial $\rho(\zeta) = \sum_j \alpha_j \zeta^{k-j}$ satisfy $|\zeta| \leq 1$ with simple roots on the unit circle. Zero-stability ensures that the extraneous solutions of the multistep recurrence (solutions corresponding to roots of $\rho$ other than $\zeta = 1$, the root giving the true solution) do not grow and contaminate the numerical solution.

**Dahlquist's equivalence theorem:** A linear multistep method is convergent (global error $\to 0$ as $h \to 0$ with $nh \to T$) if and only if it is consistent and zero-stable.

## Worked Example: Error Growth for Euler's Method

Apply Euler's method to $y' = \lambda y$, $y(0) = 1$ (exact: $y(t) = e^{\lambda t}$) with $\lambda = -10$ and $h = 0.1$.

The Euler recurrence is $y_{n+1} = y_n(1 + h\lambda) = y_n(1 - 1) = 0$. After one step, $y_1 = 0$, and all subsequent values are zero. Exact: $y(0.1) = e^{-1} \approx 0.368$. Global error: $0.368$.

With $h = 0.05$: $y_{n+1} = y_n(1 + 0.05 \cdot (-10)) = y_n \cdot 0.5$. After $n$ steps: $y_n = (0.5)^n$. At $t = 1$ ($n = 20$): $y_{20} = (0.5)^{20} \approx 9.5 \times 10^{-7}$. Exact: $e^{-10} \approx 4.5 \times 10^{-5}$. Error: $\approx 4.5 \times 10^{-5}$. This is better, but $h = 0.05$ still gives poor accuracy because $|1 + h\lambda| = 0.5 < 1$: the method is stable but inaccurate.

For accuracy, we need $h$ small enough to resolve the time scale $1/|\lambda| = 0.1$: taking $h = 0.01$ gives $|1 + h\lambda| = 0.9$, and $y_{100} = (0.9)^{100} \approx 2.66 \times 10^{-5}$. Exact $e^{-10} \approx 4.54 \times 10^{-5}$. Error $\approx 1.9 \times 10^{-5}$. Still $O(h)$ error.

## Order vs. Error Constant

Two methods of the same order can have very different error constants. For example, AM4 (Adams-Moulton 4-step corrector) and AB4 (Adams-Bashforth 4-step predictor) are both fourth-order, but the AM4 error constant ($19/720$) is about 13 times smaller than the AB4 error constant ($251/720$). For the same step size, AM4 gives much more accurate results.

Comparing methods across orders: a method of order $p+1$ can often use a larger $h$ than a method of order $p$ to achieve the same global error. Specifically, if $|e_n| \approx C_p h^p$ for an order-$p$ method and $|e_n| \approx C_{p+1} h^{p+1}$ for an order-$(p+1)$ method, the higher-order method achieves the same accuracy with step size $h^* = (C_p/C_{p+1})^{1/(p+1)} h_p^{p/(p+1)}$ — generally a much larger step, and hence fewer function evaluations.

This analysis motivates the use of higher-order methods and adaptive step-size control: the optimal strategy for a given accuracy requirement is to use the highest feasible order with the largest feasible step size, not to use low-order methods with very small $h$.
