# Chapter 3: Error and Stability

Applying a numerical method to an ODE produces a sequence of numbers $y_0, y_1, y_2, \ldots$. Two fundamental questions arise: how close are these numbers to the true solution values $y(t_0), y(t_1), \ldots$, and under what conditions can we trust the method to give meaningful results? These questions are addressed by the theory of error and stability.

**Error** concerns the gap between the numerical approximation and the exact solution. The local truncation error measures how well the method approximates the ODE at each individual step; the global error measures the accumulated discrepancy over the entire integration. Order of convergence quantifies how the global error shrinks as the step size $h \to 0$.

**Stability** concerns the response of the numerical method to perturbations — round-off errors, errors in initial data, errors accumulated in previous steps. A stable method does not amplify small errors catastrophically; an unstable method may produce wildly inaccurate results even for small $h$.

The fundamental theorem of numerical ODEs is that **consistency + zero-stability = convergence**. A consistent method (one whose local approximation converges to the ODE as $h \to 0$) is convergent if and only if it is zero-stable (bounded response to perturbations as $h \to 0$). This is Dahlquist's equivalence theorem for linear multistep methods, and a similar result holds for Runge-Kutta methods.

Beyond zero-stability lies the more practical concept of **absolute stability**, which concerns how the method behaves for fixed $h$ on specific problems (particularly stiff ones). The region of absolute stability determines which step sizes can be used safely, and **stiff equations** — those with widely separated time scales — demand methods with large stability regions. The chapter concludes with **adaptive step-size control**, the practical mechanism by which modern ODE codes automatically adjust $h$ to maintain desired accuracy efficiently.
