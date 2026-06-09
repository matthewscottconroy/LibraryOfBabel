# Consistency, Stability, and Convergence

The three fundamental properties of a finite difference scheme — consistency, stability, and convergence — are linked by a theorem that stands as one of the cornerstones of numerical analysis: the **Lax equivalence theorem** (Lax and Richtmyer, 1956). For linear problems, stability is the critical additional ingredient that transforms a merely consistent approximation into a provably convergent method. This section gives precise definitions, proves the Lax theorem, derives convergence rates, and discusses extensions to nonlinear problems.

## Definitions

Let $Lu = f$ be a well-posed linear initial value problem on a Banach space $V$ (typically $L^2$ or $\ell^2$ over the grid). Let $L_h U_h = f_h$ be a finite difference scheme with mesh parameter $h = (\Delta x, \Delta t)$.

**Definition (Consistency).** The scheme is **consistent** with the PDE if the local truncation error $\tau_h$ satisfies $\|\tau_h\| \to 0$ as $h \to 0$, where $\tau_h$ is obtained by substituting the exact solution $u$ into the scheme:

$$\tau_h = L_h u_h - f_h,$$

with $u_h = (u(x_j, t_n))_{j,n}$ the restriction of $u$ to the grid. The scheme is **consistent of order** $(p, q)$ if $\|\tau_h\| = O(\Delta t^p + (\Delta x)^q)$.

**Definition (Stability).** The scheme is **stable** if there exist constants $C > 0$ and $h_0 > 0$ such that for all $h$ with $|h| < h_0$:

$$\|L_h^{-1}\|_{V_h \to V_h} \leq C. \tag{stability bound}$$

Equivalently, for evolution equations $U^{n+1} = A_h U^n + b^n$: stability means $\|A_h^n\| \leq C$ uniformly in $n$ and $h$ (the solution operator is uniformly bounded).

**Definition (Convergence).** The scheme is **convergent** if the numerical solution $U_h$ satisfies $\|U_h - u_h\| \to 0$ as $h \to 0$.

## The Lax Equivalence Theorem

**Theorem (Lax-Richtmyer, 1956).** Let $Lu = f$ be a well-posed linear problem and $L_h U_h = f_h$ a consistent finite difference scheme. Then:

$$\text{stability} \iff \text{convergence.}$$

That is, the scheme is convergent if and only if it is stable (given consistency).

**Proof.**

$({\Rightarrow})$ **Stability implies convergence.** Let $e^n = U^n - u_h^n$ be the global error at time level $n$ and $\tau^n$ the local truncation error at level $n$. Write the error equation:

$$e^{n+1} = A_h e^n + \Delta t\,\tau^n,$$

since $U^{n+1} = A_h U^n + b^n$ and $u_h^{n+1} = A_h u_h^n + b^n + \Delta t\,\tau^n$ (the exact solution satisfies the scheme up to the LTE). By induction:

$$e^N = A_h^N e^0 + \Delta t\sum_{n=0}^{N-1} A_h^{N-1-n}\tau^n. \tag{error sum}$$

If the initial data is exactly represented: $e^0 = 0$. Then:

$$\|e^N\| \leq \Delta t \sum_{n=0}^{N-1}\|A_h^{N-1-n}\|\|\tau^n\| \leq \Delta t \cdot N \cdot C \cdot \max_n\|\tau^n\| = T \cdot C \cdot \max_n\|\tau^n\|.$$

Since $N\Delta t = T$ is fixed and $\max_n\|\tau^n\| \to 0$ by consistency: $\|e^N\| \to 0$. Convergence follows.

$({\Leftarrow})$ **Convergence implies stability.** (Proof by contrapositive.) Suppose the scheme is not stable. Then there exists a sequence of meshes $h_k \to 0$ and grid data $f_{h_k}$ with $\|f_{h_k}\| \to 0$ but $\|L_{h_k}^{-1}f_{h_k}\| \not\to 0$. Construct initial data $u^0 \equiv 0$ (so the exact solution is $u \equiv 0$) and replace the right-hand side by $f_{h_k}$. The numerical solution $U_{h_k}$ does not converge to the exact solution $u \equiv 0$, contradicting convergence. $\square$

**Remark.** The theorem is stated for linear schemes and linear PDEs. The linearity is used in the error equation: $e^{n+1} = A_h e^n + \Delta t\tau^n$ follows from linearity of $A_h$. For nonlinear schemes, the argument fails and convergence is harder to establish.

## Convergence Rates

Under the assumptions of the Lax theorem, the error bound (error sum) yields a quantitative rate:

**Theorem.** If the scheme is stable ($\|A_h^n\| \leq C$ uniformly) and consistent of order $(p,q)$ ($\max_n\|\tau^n\| \leq K(\Delta t^p + (\Delta x)^q)$), then:

$$\max_n\|U^n - u_h^n\| \leq CKT(\Delta t^p + (\Delta x)^q) = O(\Delta t^p + (\Delta x)^q). \tag{convergence rate}$$

The convergence order equals the consistency order (under stability).

**Examples:**
- FTCS for heat equation ($p=1$, $q=2$, $r\leq 1/2$): $\|e\| = O(\Delta t + (\Delta x)^2)$.
- Crank-Nicolson ($p=2$, $q=2$, unconditionally stable): $\|e\| = O(\Delta t^2 + (\Delta x)^2)$.
- Leapfrog for wave equation ($p=2$, $q=2$, $\lambda \leq 1$): $\|e\| = O(\Delta t^2 + (\Delta x)^2)$.

**Balanced accuracy.** If $\Delta t = C(\Delta x)^{q/p}$ (the mesh refinement path that balances temporal and spatial errors), then $\|e\| = O((\Delta x)^q)$. For CN with $p = q = 2$: take $\Delta t \sim (\Delta x)$, giving $\|e\| = O((\Delta x)^2)$.

## Relation to Stability: A Worked Example

**FTCS with $r = 0.6 > 1/2$.** The amplification factor for the Nyquist mode $\theta = \pi$ is $\xi = 1 - 4r = 1 - 2.4 = -1.4$. After $N$ steps: $|\xi|^N = (1.4)^N$. The consistency order is $O(\Delta t + (\Delta x)^2)$, so $\|\tau^n\| = O(h)$ as the grid is refined. The error bound becomes:

$$\|e^N\| \leq \Delta t \cdot N \cdot (1.4)^{N-1} \cdot O(h) = T \cdot (1.4)^{N-1} \cdot O(h).$$

As $h \to 0$ with $r = 0.6$ fixed: $\Delta t = 0.6(\Delta x)^2/\kappa$ and $N = T/\Delta t = T\kappa/(0.6(\Delta x)^2) \to \infty$. The factor $(1.4)^N = (1.4)^{O(1/(\Delta x)^2)} \to \infty$ faster than $O(h)$ shrinks — the error diverges. The scheme is consistent but not convergent: stability is violated and the Lax theorem correctly predicts failure.

## Norm Equivalence and the Choice of Norm

The Lax theorem is stated in a Banach space norm $\|\cdot\|_{V_h}$. The choice of norm affects what "stability" means:

- **$\ell^2$ norm** ($\|U\|_2 = \sqrt{\Delta x\sum_j|U_j|^2}$): the natural choice for von Neumann analysis. Stability in $\ell^2$ follows from $|\xi(\theta)| \leq 1$ for all $\theta$.

- **$\ell^\infty$ norm** ($\|U\|_\infty = \max_j|U_j|$): suitable for problems with maximum principles. FTCS for the heat equation with $r \leq 1/2$ is stable in $\ell^\infty$ (discrete maximum principle); for $r > 1/2$, it is not.

- **$\ell^1$ norm** ($\|U\|_1 = \Delta x\sum_j|U_j|$): for conservation laws, $\ell^1$-stability (total variation diminishing, or TVD) ensures no spurious oscillations. Upwind is TVD for $\lambda \leq 1$; Lax-Wendroff is not TVD (oscillates near discontinuities).

The Lax theorem applies in any of these norms, provided both consistency and stability are stated in the same norm.

## Extensions to Nonlinear Problems

For nonlinear evolution equations $U^{n+1} = F_h(U^n)$, the error equation becomes:

$$e^{n+1} = F_h(U^n) - F_h(u_h^n) + \Delta t\tau^n = DF_h(u_h^n)e^n + O(\|e^n\|^2) + \Delta t\tau^n.$$

Linearization around the exact solution gives a time-varying linear scheme with Jacobian $DF_h(u_h^n)$. Stability requires $\|DF_h(u_h^n)^N\| \leq C$ uniformly — which now depends on the solution $u$ itself. Three frameworks apply:

1. **Invariant regions.** If $F_h$ maps a closed convex set $\mathcal{S}$ into itself (invariant region), then $\|U^n\|$ remains bounded for $U^0 \in \mathcal{S}$. This gives $\ell^\infty$ stability without linearization.

2. **Energy methods.** Multiply the scheme by $U^n$ and sum; derive a discrete energy identity $E^{n+1} \leq E^n + C\Delta t E^n$. Gronwall's lemma gives $E^n \leq e^{CT}E^0$, bounding the solution.

3. **TVD schemes.** For conservation laws $u_t + f(u)_x = 0$: a scheme is **total variation diminishing** if $TV(U^{n+1}) \leq TV(U^n)$ where $TV(U) = \sum_j|U_{j+1}-U_j|$. TVD schemes are $\ell^1$-stable and provably convergent (by the Helly selection theorem) to a weak solution (Harten, 1983).

## Consistency Without Convergence: A Cautionary Tale

The **Lax-Wendroff theorem** (1960) provides a converse for conservation laws: if a consistent, conservative scheme converges (in $L^1_{loc}$), then the limit is a weak solution satisfying the Rankine-Hugoniot conditions. However, convergence to a wrong weak solution is possible if the scheme lacks a numerical entropy condition.

**Example.** For Burgers' equation $u_t + (u^2/2)_x = 0$, a consistent scheme might converge to an entropy-violating (nonphysical) weak solution — one that violates the Lax entropy condition $u_L > s > u_R$ at the shock. The scheme must be designed to automatically select the entropy solution (via numerical diffusion or explicit entropy fixes).

This shows that for nonlinear conservation laws, **convergence alone is not sufficient** — the limit must also be the physically correct (entropy) solution. The complete theory requires: consistency + stability + entropy condition $\Rightarrow$ convergence to the entropy solution.

## Summary Table

| Property | Definition | Verified by |
|---|---|---|
| Consistency | LTE $\to 0$ as $h\to 0$ | Taylor expansion |
| Stability | $\|A_h^n\| \leq C$ uniformly | von Neumann analysis |
| Convergence | $\|U_h - u_h\| \to 0$ | Lax theorem + above |
| Rate | $O(\Delta t^p + (\Delta x)^q)$ | Error sum formula |

The Lax equivalence theorem completes the theory: for linear problems, **consistency + stability $\Leftrightarrow$ convergence**. The rate of convergence equals the consistency order. For nonlinear problems, additional structure (invariant regions, TVD, entropy conditions) is required to guarantee convergence to the physical solution.
