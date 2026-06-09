# Chapter 1: One-Step Methods

A one-step method for the initial value problem $y' = f(t,y)$, $y(t_0) = y_0$ computes each new approximate value $y_{n+1}$ from the single previous value $y_n$ (and possibly several evaluations of $f$ at points between $t_n$ and $t_{n+1}$), without reference to earlier values $y_{n-1}, y_{n-2}, \ldots$. This self-contained structure makes one-step methods easy to start (no special initialization required), easy to change step size (each step is independent of the others), and amenable to theoretical analysis.

The Euler method is the conceptual foundation: it advances the solution by following the tangent line for one step. Its simplicity makes the structure of all higher-order one-step methods transparent. The modified Euler and Heun methods are second-order improvements that use two function evaluations per step. The Runge-Kutta family, culminating in the classical fourth-order method (RK4), systematically achieves higher accuracy by using additional intermediate evaluations.

## Error Analysis Framework

For any one-step method of the form $y_{n+1} = y_n + h\Phi(t_n, y_n; h)$, the **local truncation error** (LTE) at step $n$ is the amount by which the exact solution fails to satisfy the method's formula:

$$\tau_{n+1} = \frac{y(t_{n+1}) - y(t_n)}{h} - \Phi(t_n, y(t_n); h).$$

A method is **consistent** (or of order $p$) if $\tau_{n+1} = O(h^p)$ as $h \to 0$, uniformly in $n$. The **global error** $e_n = y(t_n) - y_n$ is not the same as the LTE: it accumulates over $N = (T - t_0)/h$ steps. For a Lipschitz continuous $f$ and a consistent method of order $p$, the global error satisfies $|e_n| = O(h^p)$: the global error has the same order as the local truncation error.

## Three Methods in Context

Euler's method represents first-order one-step integration, providing the conceptual entry point and establishing the error analysis framework. The modified Euler and Heun methods show how two function evaluations per step can double the order, introducing the idea of using intermediate slopes. The Runge-Kutta family generalizes this by showing how $s$ function evaluations per step can achieve order $p \leq s$ (with equality up to $p = 4$), optimizing the accuracy-to-work ratio. Together these three sections develop one-step methods from first principles to the classical workhorse of scientific computing.
