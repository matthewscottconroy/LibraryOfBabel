# Chapter 2: Multistep Methods

One-step Runge-Kutta methods use only $y_n$ (and intermediate evaluations of $f$) to compute $y_{n+1}$. They discard all previously computed information after each step. Multistep methods exploit the historical record: they use the values $y_n, y_{n-1}, \ldots, y_{n-k+1}$ (and possibly $f$-values at these points) to compute $y_{n+1}$ with high accuracy using fewer function evaluations per step than a comparable Runge-Kutta method.

This efficiency advantage is real and significant. A four-step Adams-Bashforth method achieves fourth-order accuracy with a single new function evaluation per step (after the startup phase), compared to four evaluations per step for RK4. The trade-off is that multistep methods require special startup procedures (because the first steps have no history), changing the step size is more complicated, and the theoretical analysis is more involved.

The Adams family — Adams-Bashforth (explicit) and Adams-Moulton (implicit) — are the primary multistep methods for non-stiff problems. The Backward Differentiation Formula (BDF) family is designed specifically for stiff problems, sacrificing some accuracy in exchange for excellent stability properties. Predictor-corrector schemes pair an explicit predictor with an implicit corrector, achieving the stability benefits of implicit methods without the full cost of solving nonlinear systems at each step.

This chapter develops each family in turn, culminating in the theoretical understanding of what makes multistep methods consistent, zero-stable, and convergent — the Dahlquist equivalence theorem, one of the fundamental results in numerical analysis.
