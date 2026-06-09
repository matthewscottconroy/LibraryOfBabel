# 4.5 The Center Manifold Theorem

When an equilibrium is *not* hyperbolic — when some eigenvalues land exactly on the imaginary axis — the linearization is inconclusive. The system is on the boundary between stability and instability, and the nonlinear terms are decisive. The center manifold theorem is the tool for analyzing these critical cases.

**Definition 4.5.1.** For $A = Df(p)$ with eigenvalues of zero, negative, and positive real part, decompose $\mathbb{R}^n = E^c \oplus E^s \oplus E^u$ (center, stable, unstable subspaces). The center subspace $E^c$ is spanned by generalized eigenvectors corresponding to eigenvalues with zero real part.

The stable and unstable subspaces behave as before — initial conditions in $E^s$ converge to the equilibrium, those in $E^u$ diverge. But initial conditions in $E^c$ stay near the equilibrium at linear order, and their fate is determined by the nonlinear terms.

**Theorem 4.5.2 (Center Manifold Theorem).** There exists a $C^k$ *center manifold* $W^c_{\text{loc}}(p)$ tangent to $E^c$ at $p$, invariant under the flow. The long-time dynamics near $p$ is governed by the restriction of $f$ to $W^c$.

The center manifold theorem gives a dramatic dimension reduction. If the center subspace is 1-dimensional (a single pair of purely imaginary eigenvalues), the dynamics near the equilibrium is governed by a 1-dimensional ODE on $W^c$ — regardless of the total dimension of the system.

Here's a concrete example to see how this works:

**Example (Supercritical Hopf Bifurcation).** Consider a 2D system where $A = Df(0)$ has purely imaginary eigenvalues $\pm i\omega$. The center subspace is all of $\mathbb{R}^2$, and the center manifold is a disk around the origin. The dynamics on this disk is an ODE on $\mathbb{R}^2$, and by converting to polar coordinates $(\rho, \theta)$ it can be analyzed using the Poincaré-Bendixson theorem.

**Application.** Bifurcation analysis (Chapter 10) takes place on the center manifold. When parameters vary, eigenvalues move through the imaginary axis. At the bifurcation value, there are center modes. The center manifold theorem reduces the analysis to a low-dimensional system — often 1D or 2D — where explicit computation is feasible.

The center manifold is not unique — there can be multiple center manifolds (they differ off $E^c$), but they all have the same Taylor expansion at $p$. In practice, you compute the center manifold as a Taylor series: assume $W^c = \{(x, h(x)) : x \in E^c\}$ for a smooth function $h$, substitute into the invariance equation $\dot{h}(x) = f_u(x, h(x))$ (where $f_u$ is the $E^u$ component), and solve order by order.

This is one of those theorems where the statement is more useful than the proof for day-to-day applications. Knowing that the center manifold exists and has a Taylor series means you can compute it; knowing the dynamics on the center manifold determines the full picture near the equilibrium.
