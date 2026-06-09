# Classification of Equilibria

The behavior of a linear planar system $\mathbf{x}' = A\mathbf{x}$ near the origin is determined entirely by the eigenvalues of $A$. Since the linearization theorem guarantees that the nonlinear system near a hyperbolic equilibrium is locally equivalent to its linearization, the eigenvalue classification applies to nonlinear systems as well. The classification of equilibria in two dimensions is complete and elegant, organized by the trace and determinant of the Jacobian.

## The Trace-Determinant Diagram

For a $2\times 2$ matrix $A$ with characteristic polynomial $\lambda^2 - (\text{tr}\,A)\lambda + \det A = 0$, the eigenvalues are:

$$\lambda_{1,2} = \frac{\text{tr}\,A \pm \sqrt{(\text{tr}\,A)^2 - 4\det A}}{2}.$$

Let $\tau = \text{tr}\,A$ and $\Delta = \det A$. The nature of the eigenvalues depends on $\tau$ and $\Delta$:

If $\Delta < 0$: the discriminant $\tau^2 - 4\Delta > 0$, the eigenvalues are real with opposite signs. The equilibrium is a **saddle point**.

If $\Delta > 0$ and $\tau^2 - 4\Delta > 0$: both eigenvalues are real with the same sign, determined by $\text{sgn}(\tau)$. If $\tau < 0$: stable node. If $\tau > 0$: unstable node.

If $\Delta > 0$ and $\tau^2 - 4\Delta < 0$: the eigenvalues are complex conjugates $\alpha \pm i\beta$ with $\alpha = \tau/2 \neq 0$. If $\tau < 0$: stable spiral. If $\tau > 0$: unstable spiral.

If $\Delta > 0$ and $\tau = 0$: purely imaginary eigenvalues. Linearization gives a **center**; the nonlinear behavior requires further analysis.

If $\tau^2 - 4\Delta = 0$ (the parabola $\tau^2 = 4\Delta$ in the $(\tau,\Delta)$-plane): repeated real eigenvalue $\lambda = \tau/2$. This gives a **star node** (if $A$ is a scalar multiple of the identity) or a **degenerate node** (if not).

If $\Delta = 0$: at least one eigenvalue is zero. The equilibrium is non-isolated (there is a line of equilibria or other degenerate structure). This is a non-hyperbolic case.

The trace-determinant diagram ($\Delta$ vs. $\tau$) organizes all cases visually: the parabola $\Delta = \tau^2/4$ separates nodes from spirals (above: spirals; below: nodes, for $\Delta > 0$); the $\tau$-axis separates stability ($\tau < 0$) from instability ($\tau > 0$); and $\Delta < 0$ gives saddles.

## Nodes

A **node** has two real eigenvalues of the same sign. All trajectories approach (stable node) or recede from (unstable node) the equilibrium without oscillating.

For a **stable node** ($\lambda_2 < \lambda_1 < 0$): all trajectories approach the origin as $t \to +\infty$. For most initial conditions, the approach is tangent to the eigenvector direction corresponding to the eigenvalue with smaller absolute value ($\lambda_1$, the slower eigenvalue). A single pair of trajectories approach along the direction of $\lambda_2$ (the faster eigenvalue). The stable node resembles a funnel with the broad end open.

For an **unstable node** ($0 < \lambda_1 < \lambda_2$): the time-reversal of the stable node. Trajectories recede from the origin, mostly tangent to the slow eigenvector direction.

A **star node** (equal eigenvalues $\lambda_1 = \lambda_2 = \lambda$ with $A = \lambda I$): all trajectories are straight lines radiating from the equilibrium. A **degenerate node** (repeated eigenvalue but $A \neq \lambda I$, so there is only one eigenvector direction): most trajectories approach along the unique eigenvector, with the others curving to approach tangentially.

## Spirals

A **spiral** has complex conjugate eigenvalues $\alpha \pm i\beta$ with $\alpha \neq 0$ and $\beta \neq 0$. Solutions have the form $e^{\alpha t}(c_1\cos\beta t + c_2\sin\beta t)$, which oscillates with growing ($\alpha > 0$) or decaying ($\alpha < 0$) amplitude.

For a **stable spiral** ($\alpha < 0$): trajectories spiral inward toward the equilibrium, oscillating with decreasing amplitude. The rotation direction is determined by the sign of the off-diagonal entries of $A$ (or equivalently, by the sign of $\beta$). A stable spiral is also called an attracting focus.

For an **unstable spiral** ($\alpha > 0$): trajectories spiral outward. An unstable spiral is a repelling focus.

The spiral versus node distinction is the difference between oscillatory and monotone approach. Spirals arise naturally in damped oscillations, RLC circuits with moderate damping, and many population models.

## Centers

A **center** has purely imaginary eigenvalues $\pm i\beta$ with $\beta \neq 0$. The linearized system has closed elliptical orbits surrounding the equilibrium. As discussed in the linearization section, the nonlinear system may exhibit true center behavior (if a conserved quantity exists) or may have a spiral at the equilibrium. For linear systems, a center is a center; for nonlinear systems, the classification is incomplete without additional analysis.

Centers arise in Hamiltonian systems (e.g., the undamped harmonic oscillator, the linearization of the pendulum at the rest position, the Lotka-Volterra interior equilibrium). Their stability is neutral: nearby trajectories neither approach nor recede, but orbit indefinitely.

## Saddles

A **saddle** has eigenvalues of opposite sign: $\lambda_1 < 0 < \lambda_2$. The equilibrium is unstable: almost all trajectories eventually recede from it. The stable manifold (along the $\lambda_1$-eigenvector direction, or its nonlinear analogue) consists of trajectories approaching the saddle as $t \to +\infty$; the unstable manifold (along $\lambda_2$-eigenvector) consists of trajectories receding as $t \to -\infty$.

The four quadrants defined by the stable and unstable manifolds are called the four sectors of the saddle. Trajectories in adjacent sectors behave differently (approaching along different directions), and the saddle's manifolds separate qualitatively distinct long-term behaviors. For this reason, saddles play an organizing role in phase portraits: they are the "connective tissue" between different regions.

## Worked Example: All Cases from One Family

The system $x' = y$, $y' = -\Delta x - \tau y$ has Jacobian at the origin $A = \begin{pmatrix}0&1\\-\Delta&-\tau\end{pmatrix}$ with trace $-\tau$ and determinant $\Delta$. By varying $\tau$ and $\Delta$, all cases of the trace-determinant diagram are realized:

If $\Delta = 2$ and $\tau = 3$: eigenvalues with negative real part ($\tau > 0$ means $\text{tr}\,A = -\tau < 0$): both real (discriminant $= \tau^2 - 4\Delta = 9 - 8 = 1 > 0$), both negative. Stable node.

If $\Delta = 2$ and $\tau = 1$: discriminant $= 1 - 8 < 0$. Stable spiral (since $\text{tr}\,A = -1 < 0$).

If $\Delta = 2$ and $\tau = 0$: center (pure imaginary eigenvalues $\pm i\sqrt{2}$).

If $\Delta = 2$ and $\tau = -1$: unstable spiral.

If $\Delta = -1$ and any $\tau$: saddle.

This family is the general second-order linear ODE $y'' + \tau y' + \Delta y = 0$ written as a system — the same classification applies to the characteristic roots of the scalar ODE. Over- and underdamped oscillators, resonant cases, and unstable systems all appear here, showing how the abstract classification scheme connects directly to physically meaningful behavior.

## Summary of the Classification

Equilibrium type, eigenvalue conditions, and qualitative behavior in summary: saddle points have $\Delta < 0$ and are always unstable; stable nodes have $\Delta > 0$, $\tau < 0$, and $\tau^2 \geq 4\Delta$; unstable nodes have $\Delta > 0$, $\tau > 0$, and $\tau^2 \geq 4\Delta$; stable spirals have $\Delta > 0$, $\tau < 0$, and $\tau^2 < 4\Delta$; unstable spirals have $\Delta > 0$, $\tau > 0$, and $\tau^2 < 4\Delta$; centers have $\Delta > 0$ and $\tau = 0$. This classification is complete for hyperbolic equilibria ($\Delta \neq 0$ and $\tau \neq 0$ in the appropriate cases), and the trace-determinant diagram gives an immediate visual determination of the type from the matrix entries.
