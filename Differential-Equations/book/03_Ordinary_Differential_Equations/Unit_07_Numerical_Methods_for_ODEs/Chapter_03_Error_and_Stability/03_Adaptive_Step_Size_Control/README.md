# Adaptive Step-Size Control

In practice, ODE solvers do not use a fixed step size. The solution may vary rapidly in some regions (requiring small $h$ for accuracy) and slowly in others (allowing large $h$ for efficiency). Adaptive step-size control automatically selects the step size at each point in the integration to maintain the local error within a user-specified tolerance, taking large steps where possible and small steps where necessary. This is what makes modern ODE solvers efficient and reliable without requiring the user to tune the step size manually.

## The Goal: Controlling Local Error

The user specifies a tolerance — either an absolute tolerance `atol` (acceptable error in absolute terms) or a relative tolerance `rtol` (acceptable error relative to the solution magnitude), or both. The solver attempts to maintain the local truncation error $|\tau_{n+1}|$ below `tol` at each step.

The local error at step $n+1$ with step size $h$ is approximately $|\tau_{n+1}| \approx C h^{p+1}|y^{(p+1)}(\xi)|$ for a method of order $p$. If an estimate $\hat{\tau}$ of the local error is available, the optimal step size $h^*$ satisfying $|C (h^*)^{p+1}| = \text{tol}$ is:

$$h^* = h\left(\frac{\text{tol}}{|\hat{\tau}|}\right)^{1/(p+1)}.$$

This is the fundamental step-size selection formula.

## Error Estimation: Embedded Runge-Kutta

For Runge-Kutta methods, local error estimation uses **embedded pairs**: two methods of orders $p$ and $p+1$ sharing the same stage evaluations. The difference between the two approximations:

$$\hat{\tau}_{n+1} = y_{n+1}^{(p+1)} - y_{n+1}^{(p)} \approx C h^{p+1} y^{(p+1)}(\xi_n)$$

provides a cheap estimate of the local error of the lower-order method. No extra function evaluations are needed beyond those already computed for the higher-order method.

The **Dormand-Prince (DOPRI5)** method is the standard embedded pair: a 6-stage method (with one extra evaluation for the next step's predictor — the FSAL "first same as last" trick, giving effectively 6 evaluations for 5 stages) providing order-4 and order-5 approximations. The step is accepted if $|\hat{\tau}_{n+1}| \leq \text{tol}$; otherwise it is rejected and retried with a smaller $h$.

## The Step-Size Selection Algorithm

A standard adaptive algorithm:

(1) **Attempt** the step with current $h$.

(2) **Estimate** the local error $\hat{\tau}$.

(3) **Compute** the optimal step size:

$$h_{\text{new}} = h \cdot S \cdot \left(\frac{\text{tol}}{|\hat{\tau}|}\right)^{1/(p+1)},$$

where $S \in (0,1)$ is a safety factor (typically $S = 0.9$) to avoid frequent step rejections.

(4) **Accept** the step if $|\hat{\tau}| \leq \text{tol}$. Use $h_{\text{new}}$ for the next step.

**Reject** the step if $|\hat{\tau}| > \text{tol}$. Retry with $h_{\text{new}}$ (which is smaller).

(5) **Limit** $h_{\text{new}}$ to avoid too-large increases (typically cap at $5h$ or $10h$) and to prevent stepping past $t_{\text{end}}$.

## Step Rejection and Efficiency

A rejected step wastes function evaluations. The safety factor $S = 0.9$ reduces the rejection rate: the actual error is slightly below tolerance, and the next step is moderately larger, not aggressively so. Step rejections are rare (typically less than 10% of steps) for smooth problems with well-chosen tolerances.

The total number of function evaluations is approximately $N_{\text{accept}} \cdot s + N_{\text{reject}} \cdot s$, where $s$ is the number of stages. For DOPRI5 with $S = 0.9$ and a smooth problem, efficiency is excellent — the number of steps taken is close to the optimal number for the requested tolerance.

## PI and PID Controllers for Step Size

The simple step-size formula above is a proportional controller: $h_{\text{new}} \propto h \cdot (\text{tol}/\hat{\tau})^{1/(p+1)}$. It can be oscillatory (alternating between slightly too large and slightly too small) on some problems. More sophisticated **PI controllers** use both the current and previous error estimates:

$$h_{\text{new}} = h \cdot \left(\frac{\text{tol}}{\hat{\tau}_n}\right)^{\alpha} \cdot \left(\frac{\hat{\tau}_{n-1}}{\hat{\tau}_n}\right)^{\beta},$$

with $\alpha = 0.7/(p+1)$ and $\beta = 0.4/(p+1)$ (Gustafsson's PI controller). The second factor damps oscillations by comparing the current error to the previous one: if the error is decreasing, step up more aggressively; if increasing, be more conservative. PI-controlled step size selection is smoother and more robust than proportional control.

## Adaptive Methods for Multistep Methods

For Adams methods, error estimation uses the difference between the predictor and corrector values, as discussed in the predictor-corrector chapter. Step-size changes for multistep methods are more expensive: changing $h$ requires reinterpolating the history values to the new mesh, or restarting the method. Modern Adams codes (MATLAB's `ode113`) use variable-order as well as variable-step control, increasing the order when the solution is smooth to improve efficiency, and decreasing it when step sizes need to change.

## Dense Output

Many applications require solution values at specific points $t^*$ (not necessarily mesh points), or the solution as a continuous function. **Dense output** (continuous extension) provides a polynomial interpolant $\tilde{y}(t)$ valid on $[t_n, t_{n+1}]$, agreeing with $y_n$ and $y_{n+1}$ and achieving accuracy comparable to the method's order throughout the interval. For DOPRI5, the dense output is a fourth-order polynomial constructed from the Runge-Kutta stage values without any additional function evaluations. Dense output is used internally by ODE event detection (finding zeros of $y$ or functions of $y$ within steps) and externally for smooth plotting.

## Practical Guidance on Tolerances

Setting `rtol` and `atol` appropriately is important for efficiency and correctness. Very tight tolerances ($10^{-10}$) require many steps and may encounter round-off error issues; very loose tolerances ($10^{-2}$) are fast but may miss important features. As a rule of thumb, `rtol = 1e-6` and `atol = 1e-8` give good results for most non-stiff problems. For stiff problems with components near zero, the absolute tolerance should be set below the expected magnitude of those components to avoid spuriously tight step-size requirements.

The interplay between tolerance, order, and step size is the practical core of adaptive ODE integration: higher-order methods achieve tight tolerances with fewer steps; adaptive step size focuses computational effort where the solution changes rapidly; and well-chosen tolerances balance accuracy against computational cost. These principles, rather than the specific algorithms, are what a practitioner needs to use modern ODE codes effectively.
