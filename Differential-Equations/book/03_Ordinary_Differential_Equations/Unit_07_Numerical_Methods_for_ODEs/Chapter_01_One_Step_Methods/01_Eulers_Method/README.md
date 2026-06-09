# Euler's Method

Euler's method is the oldest, simplest, and most fundamental numerical method for ordinary differential equations. It transforms the initial value problem $y' = f(t,y)$, $y(t_0) = y_0$ into a discrete recurrence by replacing the derivative with a forward difference approximation. Despite being too inaccurate for most practical applications, Euler's method is the conceptual foundation upon which all higher-order methods are built: understanding its derivation, error analysis, and failure modes is prerequisite to understanding the entire field of numerical ODEs.

## Derivation

The key idea is simple: the derivative $y'(t)$ at $t_n$ is the slope of the solution curve, and the ODE gives $y'(t_n) = f(t_n, y(t_n))$. Replacing $y(t_n)$ by the approximate value $y_n$ and using the slope to advance one step of size $h$:

$$y_{n+1} = y_n + hf(t_n, y_n).$$

This is **Euler's method** (also called the forward Euler method, or explicit Euler method). Starting from $y_0 = y(t_0)$, one computes $y_1, y_2, \ldots$ at the mesh points $t_n = t_0 + nh$.

Geometrically, Euler's method follows the tangent line to the solution curve at each mesh point for one step. Since the true solution is curved (unless $f$ is constant), the tangent line diverges from the solution curve, introducing a **local error** at each step. These local errors accumulate over the course of the computation.

## Derivation from Taylor's Theorem

A more rigorous derivation uses Taylor's theorem. The exact solution satisfies:

$$y(t_{n+1}) = y(t_n) + hy'(t_n) + \frac{h^2}{2}y''(t_n) + O(h^3).$$

Since $y'(t_n) = f(t_n, y(t_n))$, and Euler's method gives $y_{n+1} = y_n + hf(t_n, y_n)$, the **local truncation error** (LTE) is (assuming $y_n = y(t_n)$ exactly):

$$\tau_{n+1} = \frac{y(t_{n+1}) - y_{n+1}}{h} = \frac{h}{2}y''(t_n) + O(h^2) = \frac{h}{2}y''(\xi_n)$$

for some $\xi_n \in (t_n, t_{n+1})$, by the mean value theorem. So the LTE is $O(h)$: Euler's method is a **first-order method**.

## Error Analysis: Global Error

The global error $e_n = y(t_n) - y_n$ accumulates over $N = (T-t_0)/h$ steps. At step $n+1$:

$$e_{n+1} = y(t_{n+1}) - y_{n+1} = [y(t_n) + hf(t_n, y(t_n)) + \tau_{n+1}h] - [y_n + hf(t_n, y_n)]$$
$$= e_n + h[f(t_n, y(t_n)) - f(t_n, y_n)] + \tau_{n+1}h.$$

By the Lipschitz condition $|f(t,u) - f(t,v)| \leq L|u-v|$:

$$|e_{n+1}| \leq (1 + hL)|e_n| + Mh^2/2,$$

where $M = \max|y''(t)|$ and $\tau_{n+1} \leq Mh/2$. With $e_0 = 0$, this recurrence gives:

$$|e_n| \leq \frac{Mh}{2L}[(1+hL)^n - 1] \leq \frac{Mh}{2L}[e^{Lnh} - 1] = \frac{Mh}{2L}[e^{L(t_n - t_0)} - 1].$$

For fixed $T = t_0 + Nh$ and $h \to 0$:

$$|e_n| \leq \frac{M(e^{L(T-t_0)} - 1)}{2L} \cdot h = O(h).$$

**Conclusion:** The global error of Euler's method is $O(h)$ — first-order convergence. Halving the step size halves the error, requiring twice as many steps and twice the computational work.

## Worked Example

Solve $y' = y$, $y(0) = 1$ on $[0, 1]$ with Euler's method using $h = 0.25$.

Exact solution: $y(t) = e^t$, so $y(1) = e \approx 2.71828$.

Euler: $y_0 = 1$. $y_1 = y_0 + 0.25 \cdot y_0 = 1.25$. $y_2 = 1.25 + 0.25(1.25) = 1.5625$. $y_3 = 1.5625 + 0.25(1.5625) = 1.953125$. $y_4 = 1.953125 + 0.25(1.953125) = 2.44140625$.

Error at $t = 1$: $|e - 2.44141| \approx 0.277$. Global error $\approx 0.277$.

With $h = 0.125$ (halved): the Euler approximation at $t=1$ is $(1.125)^8 \approx 2.5660$, error $\approx 0.152$. Ratio: $0.277/0.152 \approx 1.82 \approx 2^{0.9}$, consistent with first-order convergence (ratio should approach 2 as $h \to 0$).

## Geometric Interpretation and Failure Modes

The tangent line approximation is exact only if $y'' = 0$ (linear solutions). For curved solutions, each step introduces an error proportional to $h^2 y''(t_n)/2$ — the curvature of the solution. Over $1/h$ steps, this gives a global error of $O(h)$.

Euler's method fails badly when: the solution changes rapidly (large $|y''|$ or large $L$, requiring very small $h$ for accuracy); the problem is stiff (the Lipschitz constant $L$ is very large, making the error bound vacuous); or when applied over very long time intervals (exponential growth of the error bound).

For stiff problems — where the ODE has components decaying at very different rates — explicit Euler requires step sizes far smaller than accuracy would require, just to maintain stability. This is the fundamental limitation of explicit one-step methods and motivates implicit methods.

## The Implicit (Backward) Euler Method

The **backward Euler method** replaces the slope at $t_n$ by the slope at $t_{n+1}$:

$$y_{n+1} = y_n + hf(t_{n+1}, y_{n+1}).$$

This is an implicit equation for $y_{n+1}$ (it appears on both sides), requiring a solve at each step — typically by Newton's method. The additional work is compensated by greatly superior stability properties: backward Euler is unconditionally stable (A-stable), meaning it remains stable for all step sizes $h > 0$ regardless of the stiffness of the problem. The accuracy is still $O(h)$ (first-order), the same as forward Euler, but the stability is dramatically better.

The contrast between forward and backward Euler illustrates a fundamental trade-off in numerical ODEs: explicit methods are cheap per step but restricted in step size by stability; implicit methods are more expensive per step but can take large steps on stiff problems.

## Euler's Method as a Prototype

Despite its limitations, Euler's method embodies the essential structure of all higher-order methods. The recurrence $y_{n+1} = y_n + h\Phi(t_n, y_n; h)$ with $\Phi = f(t_n,y_n)$ for Euler generalizes to $\Phi$ being a weighted combination of function evaluations at intermediate points for Runge-Kutta methods, or a combination of previous values for multistep methods. The error analysis — local truncation error, Lipschitz-based propagation, global error bound — carries over to all methods. And the fundamental trade-off between step size, accuracy, and stability that Euler makes explicit is the organizing principle for numerical ODE theory.
