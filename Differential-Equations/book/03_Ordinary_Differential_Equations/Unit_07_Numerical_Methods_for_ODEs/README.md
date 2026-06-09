# Unit 7: Numerical Methods for ODEs

Analytical solution methods occupy the preceding units of this course, but their applicability is fundamentally limited. Most ordinary differential equations arising in science and engineering cannot be solved in closed form, and even when exact solutions exist, they may involve special functions, integrals, or infinite series that are themselves computed numerically. Numerical methods transform the problem of solving a differential equation into the problem of applying a systematic computational algorithm that produces accurate approximate solutions at a discrete set of points.

Numerical ODE solvers are among the most widely used tools in scientific computing. Every major computation involving continuous dynamical systems — simulations of orbital mechanics, fluid dynamics, chemical kinetics, circuit behavior, biomechanical motion, climate models — rests on numerical ODE methods. Understanding these methods at the theoretical level, not merely as black boxes to be applied, is essential for using them correctly, interpreting their output, and recognizing when they fail.

## The Setting

The fundamental problem is: given the initial value problem $y' = f(t,y)$, $y(t_0) = y_0$, compute approximations $y_n \approx y(t_n)$ at a sequence of points $t_0 < t_1 < t_2 < \cdots$, where typically $t_n = t_0 + nh$ for a fixed step size $h > 0$. The function $f$ may be nonlinear, the solution may have stiff behavior (rapidly decaying components that impose stringent step-size requirements), and the computation may need to cover long time intervals.

## One-Step Methods

One-step methods compute $y_{n+1}$ from $y_n$ alone (plus evaluations of $f$ at intermediate points between $t_n$ and $t_{n+1}$). The Euler method is the simplest: $y_{n+1} = y_n + hf(t_n, y_n)$, equivalent to following the tangent line for one step. It is first-order accurate: the global error after $N = T/h$ steps is $O(h)$. Higher-order accuracy comes from Runge-Kutta methods, which use multiple evaluations of $f$ within each step to match higher terms of the Taylor expansion. The classical fourth-order Runge-Kutta method (RK4) uses four function evaluations per step and achieves $O(h^4)$ global error, making it accurate enough for most non-stiff problems at moderate step sizes.

## Multistep Methods

Multistep methods compute $y_{n+1}$ from several past values $y_n, y_{n-1}, \ldots, y_{n-k+1}$. By using more history, they achieve high accuracy with fewer function evaluations per step. The Adams-Bashforth and Adams-Moulton families are the primary explicit and implicit multistep methods; predictor-corrector schemes combine them. The BDF (Backward Differentiation Formula) methods are implicit multistep methods specifically designed for stiff equations.

## Error and Stability

Two intertwined concepts govern the reliability of numerical methods: accuracy and stability. Accuracy concerns how well the method approximates the true solution in the absence of round-off — quantified by the order of the method (the power of $h$ in the global error). Stability concerns whether errors (from round-off, initial data perturbations, or local truncation errors) remain bounded as the computation proceeds — even if they are initially small.

Stability is not guaranteed by accuracy: a highly accurate method can be catastrophically unstable if applied with too large a step size or to a stiff equation. The region of absolute stability of a method determines which step sizes are safe for which equations. Stiff equations require methods with large stability regions — typically implicit methods — even at the cost of solving a nonlinear system at each step.

The fundamental consistency, stability, and convergence theorem (Dahlquist's equivalence theorem for multistep methods) provides the theoretical foundation: a consistent method is convergent if and only if it is zero-stable. This theorem mirrors the principle from numerical analysis that correctness in the limit (consistency) combined with robustness to perturbations (stability) implies correctness overall (convergence).

## Coverage of This Unit

Chapter 1 develops one-step methods from first principles: the Euler method with full error analysis, the Taylor method as a conceptual bridge, and the Runge-Kutta family culminating in RK4. Chapter 2 treats multistep methods: Adams-Bashforth and Adams-Moulton families, predictor-corrector pairs, and the BDF methods for stiff problems. Chapter 3 develops the theory of error and stability: local and global truncation error, order conditions, absolute stability and stability regions, stiffness and A-stability, and adaptive step-size control. Together these three chapters provide both the practical toolkit for solving ODEs numerically and the theoretical understanding necessary to apply it correctly.
