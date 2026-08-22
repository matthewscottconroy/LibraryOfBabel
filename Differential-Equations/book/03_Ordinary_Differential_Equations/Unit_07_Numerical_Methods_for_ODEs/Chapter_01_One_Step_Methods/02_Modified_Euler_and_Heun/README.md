# Modified Euler and Heun's Method

The first-order accuracy of Euler's method is often insufficient for practical computations: halving the step size only halves the error, and for moderate accuracy requirements one may need thousands of steps. Second-order methods, which achieve $O(h^2)$ global error, reduce this burden substantially — halving the step size reduces the error by a factor of four. The modified Euler method and Heun's method are the two most natural second-order one-step methods, both using two evaluations of $f$ per step to achieve a local truncation error of $O(h^3)$.

## The Key Idea: Improved Slope Estimation

Euler's method uses the slope $f(t_n, y_n)$ at the left endpoint of each interval $[t_n, t_{n+1}]$. The error comes from the fact that the solution curves away from the tangent line. A natural improvement is to use a better estimate of the average slope over the interval.

By the mean value theorem for integrals:

$$y(t_{n+1}) - y(t_n) = \int_{t_n}^{t_{n+1}} f(t, y(t))\,dt \approx h \cdot \overline{f},$$

where $\overline{f}$ is some average value of $f$ over the interval. The question is how to approximate $\overline{f}$ using only the available information (the value $y_n$ and evaluations of $f$).

The **trapezoidal rule** approximation of the integral uses the values at both endpoints:

$$\int_{t_n}^{t_{n+1}} f(t,y(t))\,dt \approx \frac{h}{2}[f(t_n, y(t_n)) + f(t_{n+1}, y(t_{n+1}))].$$

But $y(t_{n+1})$ is unknown. Heun's method replaces it with the Euler predictor:

$$\tilde{y}_{n+1} = y_n + hf(t_n, y_n) \qquad \text{(predict)},$$
$$y_{n+1} = y_n + \frac{h}{2}[f(t_n, y_n) + f(t_{n+1}, \tilde{y}_{n+1})] \qquad \text{(correct)}.$$

This **predictor-corrector** pair is called **Heun's method** (or the explicit trapezoidal method).

## Modified Euler: The Midpoint Method

An alternative strategy is to estimate the slope at the midpoint $t_n + h/2$. The midpoint rule for numerical integration gives:

$$\int_{t_n}^{t_{n+1}} f(t,y(t))\,dt \approx h \cdot f(t_n + h/2, y(t_n + h/2)).$$

The value $y(t_n + h/2)$ is approximated by a half-step of Euler:

$$k_1 = f(t_n, y_n), \qquad k_2 = f\!\left(t_n + \frac{h}{2}, y_n + \frac{h}{2}k_1\right),$$
$$y_{n+1} = y_n + hk_2.$$

This is the **modified Euler method** (also called the midpoint method or explicit midpoint rule). It uses one evaluation at the left endpoint ($k_1$) to predict the solution at the midpoint, then evaluates $f$ at the midpoint ($k_2$) and uses that slope for the full step.

## Order Analysis

Both methods are second order: the local truncation error is $O(h^3)$ and the global error is $O(h^2)$.

**Verification for Heun's method.** Expanding $f(t_{n+1}, \tilde{y}_{n+1})$ about $(t_n, y_n)$:

$$f(t_n + h, y_n + hf_n) = f_n + h(f_t + f\cdot f_y) + O(h^2),$$

where $f_n = f(t_n, y_n)$ and subscripts denote partial derivatives. The average slope is:

$$\frac{f_n + f(t_{n+1}, \tilde{y}_{n+1})}{2} = f_n + \frac{h}{2}(f_t + f\cdot f_y) + O(h^2).$$

The Heun increment is $hf_n + \frac{h^2}{2}(f_t + f\cdot f_y) + O(h^3)$.

The Taylor expansion of the exact solution: $y(t_{n+1}) = y_n + hf_n + \frac{h^2}{2}y''(t_n) + O(h^3)$, with $y'' = f_t + f_y \cdot y' = f_t + f\cdot f_y$. So the Heun increment matches the Taylor expansion through order $h^2$, giving LTE $= O(h^3)$ and global error $O(h^2)$.

An identical analysis applies to the modified Euler method. Both are second-order Runge-Kutta methods.

## Worked Example: Comparing Methods

Solve $y' = t - y$, $y(0) = 1$ at $t = 1$ using $h = 0.5$.

Exact solution: $y = t - 1 + 2e^{-t}$, so $y(1) = 0 + 2e^{-1} \approx 0.7358$.

**Euler:** $y_1 = 1 + 0.5(0 - 1) = 0.5$. $y_2 = 0.5 + 0.5(0.5 - 0.5) = 0.5$. Error: $|0.7358 - 0.5| = 0.2358$.

**Heun's method:** 
Step 1: $k_1 = f(0, 1) = 0 - 1 = -1$. $\tilde{y}_1 = 1 + 0.5(-1) = 0.5$. $k_2 = f(0.5, 0.5) = 0.5 - 0.5 = 0$. $y_1 = 1 + 0.5\cdot(-1+0)/2 = 1 - 0.25 = 0.75$.

Step 2: $k_1 = f(0.5, 0.75) = 0.5 - 0.75 = -0.25$. $\tilde{y}_2 = 0.75 + 0.5(-0.25) = 0.625$. $k_2 = f(1, 0.625) = 1 - 0.625 = 0.375$. $y_2 = 0.75 + 0.5(-0.25 + 0.375)/2 = 0.75 + 0.03125 = 0.78125$.

Error: $|0.7358 - 0.78125| \approx 0.0455$.

Euler error ($0.2358$) versus Heun error ($0.0455$): ratio $\approx 5.2$. With $h = 0.5$, one expects the ratio of global errors to be about $(h_E/h_H)^{\text{order}} = 1^1 / 1^2 =$ — since $h$ is the same, the factor comes from the order difference. More precisely, with $h$ fixed, higher-order methods produce smaller errors; the actual comparison requires varying $h$.

## The General Framework: Runge-Kutta Stage Structure

Both methods fit into the general **explicit two-stage Runge-Kutta** framework:

$$k_1 = f(t_n, y_n),$$
$$k_2 = f(t_n + c_2 h, y_n + a_{21}hk_1),$$
$$y_{n+1} = y_n + h(b_1 k_1 + b_2 k_2).$$

The parameters $(c_2, a_{21}, b_1, b_2)$ determine the specific method. For Heun's method: $c_2 = 1$, $a_{21} = 1$, $b_1 = b_2 = 1/2$. For the modified Euler (midpoint method): $c_2 = 1/2$, $a_{21} = 1/2$, $b_1 = 0$, $b_2 = 1$.

Second-order accuracy requires three conditions: $b_1 + b_2 = 1$ (first-order condition), $b_2 c_2 = 1/2$ (second-order condition for $t$ derivative), $b_2 a_{21} = 1/2$ (second-order condition for $y$ derivative). These three equations in four unknowns admit a one-parameter family of solutions, of which Heun's method and the midpoint method are two special cases. This flexibility in choosing Runge-Kutta coefficients is the starting point for the theory of the Runge-Kutta order conditions, developed systematically in the next section.

## Comparison and Practical Guidance

Both modified Euler and Heun are second-order, two-stage methods requiring the same work per step. Their error constants differ: for smooth problems, Heun's method generally has a slightly smaller error than the midpoint method, but the difference is problem-dependent. For most purposes, either is an acceptable second-order method.

Their primary role in practice is pedagogical and as predictor steps in predictor-corrector schemes. For serious computation, fourth-order Runge-Kutta or higher is preferred, as it achieves much better accuracy (16 times better than second-order when the step size is halved) for only twice the work per step. The second-order methods remain important for understanding the structure of higher-order methods and for applications where computational resources are severely limited.
