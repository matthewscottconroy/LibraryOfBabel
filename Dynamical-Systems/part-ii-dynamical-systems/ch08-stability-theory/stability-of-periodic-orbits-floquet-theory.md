# 8.6 Stability of Periodic Orbits — Floquet Theory

Equilibria are the simplest invariant sets. The next simplest are periodic orbits — closed curves that the trajectory traverses in finite time, over and over again. The stability of a periodic orbit is not the same question as the stability of a fixed point, because the linearized dynamics around a periodic orbit are time-varying. Floquet theory handles this case by using the periodicity of the orbit to reduce the time-varying problem to a fixed matrix problem.

**Definition 8.6.1.** For a periodic orbit $\gamma$ of period $T$ (so $\Phi_T(p) = p$ for $p \in \gamma$), the *Floquet multipliers* are the eigenvalues of the linearized return map $D\Phi_T(p): T_pM \to T_pM$.

One eigenvalue is always $1$ (in the direction of the flow). The others determine the transverse stability.

The trivial eigenvalue 1 in the direction of the flow is a fundamental geometric fact: if you perturb an initial condition slightly along the periodic orbit, the perturbed orbit is just the same orbit with a different phase. So the perturbation neither grows nor shrinks — it stays at the same distance, giving eigenvalue 1.

The interesting eigenvalues are the *transverse* ones: they measure what happens to perturbations perpendicular to the orbit. If all transverse Floquet multipliers satisfy $|\mu_i| < 1$, nearby trajectories spiral toward the periodic orbit — it's asymptotically stable (a *limit cycle* in the plane). If any $|\mu_i| > 1$, the orbit is unstable.

**Theorem 8.6.2 (Floquet).** The variational equation $\dot{J} = A(t)J$ along a $T$-periodic orbit is equivalent (by a periodic change of variables) to a constant-coefficient linear ODE. The monodromy matrix $M = J(T)$ has eigenvalues = Floquet multipliers.

- If all Floquet multipliers satisfy $|\mu_i| < 1$ (except the trivial $1$): the periodic orbit is asymptotically stable.
- If any $|\mu_i| > 1$: unstable.
- If all $|\mu_i| = 1$ (except trivial): marginally stable (requires nonlinear analysis).

The Floquet theorem is the precise analogue of the linearization theorem for equilibria: you can always reduce the study of stability near a periodic orbit to a matrix problem. The matrix is the monodromy matrix $M = J(T)$, which maps a perturbation at time 0 to its value after one full period. The eigenvalues of $M$ — the Floquet multipliers — are the analogues of eigenvalues at an equilibrium.

The marginal case $|\mu_i| = 1$ (excluding the trivial direction) is the analogue of a center: the linear analysis is inconclusive, and you need the nonlinear terms to determine stability. This is where bifurcation theory (Chapter 10) enters.

For maps (Section 8.7), the situation is simpler: a fixed point of the map plays the role of the periodic orbit, and the Jacobian $Df(x^*)$ plays the role of the monodromy matrix. The Floquet multipliers become eigenvalues of $Df(x^*)$ directly.
