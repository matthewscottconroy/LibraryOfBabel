# Stiff Equations

Stiffness is one of the most important and practically consequential phenomena in numerical ODEs. A stiff equation is one where explicit numerical methods require a step size far smaller than accuracy demands, just to maintain numerical stability. The small step size is not dictated by the need to resolve the solution — the solution may be slowly varying and well-approximated by large steps — but by the presence of rapidly decaying components that force explicit methods to take tiny steps even when those components have become negligible. Stiff equations arise ubiquitously: in chemical kinetics, circuit analysis, control systems, and any problem involving processes on widely separated time scales.

## Definition and Characterization

A precise definition of stiffness is elusive, but the most useful characterization for linear systems is as follows. Consider the linear system $\mathbf{y}' = A\mathbf{y}$ with eigenvalues $\lambda_1, \ldots, \lambda_n$ all having negative real part (a stable system). The **stiffness ratio** is:

$$S = \frac{\max_i|\text{Re}(\lambda_i)|}{\min_i|\text{Re}(\lambda_i)|}.$$

A large stiffness ratio indicates widely separated time scales. The fastest decaying mode (largest $|\text{Re}(\lambda_i)|$) imposes a step-size requirement of $h \lesssim C/|\lambda_{\max}|$ for explicit methods, while the slowest mode (smallest $|\text{Re}(\lambda_i)|$) determines the time scale over which the solution evolves. If $S \gg 1$, one must integrate over a time interval much longer than $1/|\lambda_{\max}|$ using steps much smaller than $1/|\lambda_{\max}|$ — a computationally wasteful requirement.

**Example.** The system $y_1' = -1000y_1 + y_2$, $y_2' = -y_2$ has eigenvalues $-1000$ and $-1$. The stiffness ratio is $1000$. The general solution is $y_1 = c_1 e^{-1000t} + c_2 te^{-t}$ (approximately), $y_2 = c_2 e^{-t}$. For $t > 0.01$, the fast component $e^{-1000t}$ is negligible, and the solution is well-approximated by $y_1 \approx 0$, $y_2 = c_2 e^{-t}$ — a slowly varying function. Yet explicit Euler requires $h < 2/1000 = 0.002$ to remain stable, even when the solution is barely changing.

## Absolute Stability and the Stability Region

The analysis of stiffness centers on **absolute stability**. Consider the scalar test equation $y' = \lambda y$, $y(0) = 1$, with $\text{Re}(\lambda) < 0$. The exact solution $e^{\lambda t} \to 0$. A numerical method is **absolutely stable** for the parameter $z = h\lambda$ if the numerical solution also decays to zero (or remains bounded).

For Euler's method: $y_{n+1} = (1 + z)^n y_0$. The method is stable iff $|1 + z| < 1$, i.e., $z$ lies inside the disk of radius 1 centered at $-1$. For $\lambda = -1000$ (real, negative), stability requires $|1 - 1000h| < 1$, i.e., $h < 0.002$. This is the stiffness constraint.

The **region of absolute stability** $\mathcal{S}$ of a method is the set of $z = h\lambda \in \mathbb{C}$ for which the method is stable. Explicit methods have bounded stability regions; implicit methods can have unbounded stability regions.

For RK4: the stability region is the set $\{z : |R(z)| < 1\}$ where $R(z) = 1 + z + z^2/2 + z^3/6 + z^4/24$ (the degree-4 Taylor polynomial of $e^z$). Along the negative real axis, the boundary is at $z \approx -2.785$, so $h < 2.785/|\lambda|$. Better than Euler but still bounded: $h < 0.00279$ for $\lambda = -1000$.

## A-Stability, L-Stability, and Stiff Accuracy

A method is **A-stable** if its stability region contains the entire left half-plane $\{z : \text{Re}(z) < 0\}$. An A-stable method can take any step size for any stable ODE without stability issues — only accuracy limits the step size.

By Dahlquist's first barrier, no explicit linear multistep method is A-stable. No explicit Runge-Kutta method is A-stable. A-stability requires implicit methods.

The backward Euler method $y_{n+1} = y_n + hf(t_{n+1}, y_{n+1})$ applied to the test equation gives $y_{n+1} = y_n/(1-z)$, stable for all $z$ with $\text{Re}(z) < 0$: A-stable.

The trapezoidal (Crank-Nicolson) method gives $y_{n+1} = y_n(1+z/2)/(1-z/2)$, also A-stable, with $|y_{n+1}/y_n| = |(2+z)/(2-z)|$ — the approximation to $e^z$ from the $(1,1)$ Padé approximant, exact on the imaginary axis.

**L-stability** (a stronger condition) requires additionally that $|R(z)| \to 0$ as $z \to \infty$ in the left half-plane: the method strongly damps all fast components. Backward Euler is L-stable (since $1/(1-z) \to 0$ as $\text{Re}(z) \to -\infty$). The trapezoidal rule is not L-stable ($(1+z/2)/(1-z/2) \to -1$ as $z \to -\infty$). L-stability is important for problems where fast transients need to be damped quickly; without it, the method may produce spurious oscillations when large steps are taken across fast transients.

## BDF Methods

The **Backward Differentiation Formula (BDF)** methods are implicit linear multistep methods designed specifically for stiff problems. The $k$-step BDF method is:

$$\sum_{j=0}^k \alpha_j y_{n+1-j} = h\beta f_{n+1},$$

where the coefficients are chosen so that the left side approximates $y'(t_{n+1})$ to order $k$. The right side uses only $f_{n+1}$ (the current step's function value, evaluated at $t_{n+1}$), so the method is implicit.

BDF1 is backward Euler. BDF2 is:

$$y_{n+1} - \frac{4}{3}y_n + \frac{1}{3}y_{n-1} = \frac{2h}{3}f_{n+1}.$$

BDF2 is second-order and A-stable.

BDF3 through BDF6 are of orders 3 through 6 and are A($\alpha$)-stable (stable in a wedge-shaped region of the left half-plane, not the full half-plane). BDF methods of order $k \geq 7$ are not zero-stable and are not used.

BDF methods are the standard for stiff ODEs. MATLAB's `ode15s`, Python's `scipy.integrate.solve_ivp` with method `BDF`, and Fortran codes LSODE/VODE all use BDF-based algorithms with variable order (1 through 5) and variable step size.

## Recognizing Stiffness

A problem is practically stiff if: explicit methods require step sizes much smaller than the solution's time scale; the Jacobian $\partial f/\partial \mathbf{y}$ has eigenvalues with very different magnitudes; or the problem involves chemical reactions with very fast and slow species, electrical circuits with components on different RC time scales, or control systems with stiff feedback loops.

If a non-stiff solver (like RK4 or MATLAB's `ode45`) runs very slowly (taking many steps with small $h$), stiffness is likely the cause. Switching to a stiff solver (BDF or Radau) typically results in orders-of-magnitude speedup on stiff problems.
