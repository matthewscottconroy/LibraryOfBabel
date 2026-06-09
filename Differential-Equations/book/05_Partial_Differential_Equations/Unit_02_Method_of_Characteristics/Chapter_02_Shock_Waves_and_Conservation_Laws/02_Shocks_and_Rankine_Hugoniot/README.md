# Shocks and the Rankine-Hugoniot Condition

A shock wave is a propagating discontinuity in the solution of a conservation law. Physically, it arises when wave characteristics converge — the wave "breaks" in the same sense that a water wave curls and crashes when its crest, moving faster than the trough, overtakes it. Mathematically, the classical (smooth) solution ceases to exist at a finite time, and the theory must be extended to accommodate discontinuous solutions. The Rankine-Hugoniot condition is the fundamental relation that governs the speed of a shock and the relationship between the states on either side of it.

## Breakdown of Classical Solutions

For the conservation law $u_t + f(u)_x = 0$ with initial data $u(x,0) = \phi(x)$, the characteristics are straight lines $x = x_0 + f'(\phi(x_0))t$. The classical solution is $u(x,t) = \phi(x_0)$ where $x_0$ is the unique solution of $x = x_0 + f'(\phi(x_0))t$.

This construction fails when two characteristics collide, i.e., when the map $x_0 \mapsto x_0 + f'(\phi(x_0))t$ is not injective. Two characteristics from $x_1 < x_2$ meet when $x_1 + f'(\phi(x_1))t = x_2 + f'(\phi(x_2))t$, i.e., when

$$(x_2 - x_1) = t(f'(\phi(x_1)) - f'(\phi(x_2))).$$

This happens at finite $t$ if $f'(\phi(x_1)) > f'(\phi(x_2))$, i.e., if the characteristic speed is larger on the left than on the right. The **shock formation time** for a smooth initial datum with $f''(\phi)\phi' < 0$ somewhere is

$$t^* = \frac{-1}{\min_x [f''(\phi(x))\phi'(x)]} > 0.$$

After $t^*$, the classical solution no longer exists as a single-valued function.

## Weak Solutions

To extend the theory past $t^*$, we introduce **weak solutions**. Multiply the conservation law by a smooth test function $\varphi \in C_c^\infty(\mathbb{R}\times[0,\infty))$ and integrate by parts:

$$\int_0^\infty\!\int_{-\infty}^\infty \left(u\,\varphi_t + f(u)\,\varphi_x\right)\,dx\,dt + \int_{-\infty}^\infty u(x,0)\varphi(x,0)\,dx = 0. \tag{2}$$

A locally integrable function $u$ satisfying (2) for all test functions $\varphi$ is a **weak solution** of $u_t + f(u)_x = 0$ with initial data $u(x,0) = \phi(x)$. If $u$ is smooth, integration by parts recovers the classical equation. But (2) makes sense even if $u$ has jump discontinuities.

## Derivation of the Rankine-Hugoniot Condition

Suppose $u$ is smooth on either side of a smooth curve $\Sigma = \{x = s(t)\}$ and has a jump discontinuity across $\Sigma$. Let $u^+(x,t) = \lim_{\epsilon\to 0^+} u(s(t)+\epsilon, t)$ and $u^- = \lim_{\epsilon\to 0^+}u(s(t)-\epsilon, t)$ be the one-sided limits.

Apply the weak formulation (2) to a test function $\varphi$ supported in a narrow strip around $\Sigma$. Integration by parts in the two smooth regions $\{x < s(t)\}$ and $\{x > s(t)\}$ each give zero (since $u$ satisfies the PDE classically there). The boundary terms from $\Sigma$ yield:

$$\int_\Sigma \left([u]\dot{s} - [f(u)]\right)\varphi\,d\sigma = 0,$$

where $[\cdot] = (\cdot)^+ - (\cdot)^-$ denotes the jump (right minus left) and $d\sigma$ is arc length on $\Sigma$. Since $\varphi$ is arbitrary, the integrand must vanish:

$$[u]\dot{s} = [f(u)],$$

or equivalently,

$$\dot{s} = \frac{f(u^+) - f(u^-)}{u^+ - u^-}. \tag{3}$$

This is the **Rankine-Hugoniot (RH) condition**. It was derived independently by William Rankine (1870) and Pierre-Henri Hugoniot (1887) in the context of gas dynamics, long before the theory of weak solutions was formalized.

## Physical Interpretation

The RH condition (3) is simply conservation of flux across the shock. The shock moves at speed $\dot{s}$ because that is the speed required to balance the mass (or whatever is conserved) entering from one side against that leaving from the other. In a frame moving with the shock:

- Mass entering from the left per unit time: $(u^- - 0)\cdot(\dot{s} - f'(u^-)/\dot{s})\cdot\ldots$

A cleaner version: in the shock frame (moving at speed $\dot{s}$), the "velocities" of the left and right states are $f'(u^-) - \dot{s}$ and $f'(u^+) - \dot{s}$, and the RH condition is that the flux is conserved: the amount flowing in from the left equals the amount flowing out to the right.

## Example: Burgers' Equation

For $f(u) = u^2/2$, the RH condition gives shock speed

$$\dot{s} = \frac{(u^+)^2/2 - (u^-)^2/2}{u^+ - u^-} = \frac{u^+ + u^-}{2},$$

the arithmetic mean of the two states. This is exact and explicit.

**Worked example.** Initial data:

$$u(x,0) = \begin{cases} 1 & x < 0 \\ 0 & x > 0 \end{cases}.$$

Characteristics from $x < 0$ have speed $u=1$ (slope $1$ in the $xt$-plane); from $x > 0$ have speed $u=0$. They collide at $x=0$ for all $t > 0$. The shock speed is $(1+0)/2 = 1/2$. The solution is:

$$u(x,t) = \begin{cases} 1 & x < t/2 \\ 0 & x > t/2 \end{cases}.$$

Check: $[u] = -1$, $[f] = -1/2$, $\dot{s} = 1/2 = (-1/2)/(-1)$. RH satisfied.

## Non-Uniqueness and the Need for Entropy

The RH condition is necessary but not sufficient to identify the physical solution. For a Riemann problem with $u_L < u_R$, both a rarefaction wave (smooth) and a shock (discontinuous, with speed $(u_L+u_R)/2$) formally satisfy the RH condition. But the shock solution is physically spurious: it would require entropy to decrease, violating the second law of thermodynamics. This motivates the entropy conditions of the next section.

## The RH Condition for Systems

For a system $\mathbf{u}_t + \mathbf{f}(\mathbf{u})_x = 0$, the vector form of the RH condition is

$$[\mathbf{f}(\mathbf{u})] = \dot{s}[\mathbf{u}].$$

This is a system of $n$ equations (one for each component) relating the jump $[\mathbf{u}]$, the jump $[\mathbf{f}]$, and the scalar shock speed $\dot{s}$. For an $n\times n$ system, the Hugoniot locus (the set of right states that can be connected to a given left state by a shock of some speed) is a curve of dimension at most $n-1$ in $\mathbb{R}^n$, and selecting the physically admissible shock requires the generalization of the entropy condition to systems — a substantially more involved theory.
