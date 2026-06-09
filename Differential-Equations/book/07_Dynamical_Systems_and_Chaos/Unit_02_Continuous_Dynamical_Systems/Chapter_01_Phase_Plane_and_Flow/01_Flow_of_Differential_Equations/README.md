# Flow of Differential Equations

The existence and uniqueness theorem for ODEs guarantees that, given a smooth vector field $F$ and an initial condition $x_0$, there is a unique solution for short times. But the flow perspective goes further: it regards the entire family of solutions—parametrized by initial conditions—as a single geometric object, a one-parameter group of diffeomorphisms of the phase space. This reframing, from individual solutions to the flow as a whole, is the conceptual foundation of geometric dynamical systems.

## Definition of the Flow

Let $F: U \subset \mathbb{R}^n \to \mathbb{R}^n$ be a $C^k$ vector field ($k \geq 1$) on an open set $U$. The autonomous ODE

$$\dot{x} = F(x), \quad x(0) = x_0$$

has, by the Picard-Lindelöf theorem, a unique $C^k$ solution $t \mapsto \phi(t, x_0)$ on a maximal interval $I(x_0) = (-T_-(x_0), T_+(x_0)) \subset \mathbb{R}$.

**Definition.** The **flow** of $F$ is the map

$$\phi: \mathcal{D} \to U, \quad \mathcal{D} = \{(t, x_0) : x_0 \in U, t \in I(x_0)\},$$

where $\phi(t, x_0)$ is the unique solution with initial condition $x_0$ evaluated at time $t$. For fixed $t$, the map $\phi_t: \{x_0 : t \in I(x_0)\} \to U$ is the **time-$t$ map** of the flow.

The domain $\mathcal{D}$ is an open subset of $\mathbb{R} \times U$ (by the theorem on continuous dependence on parameters).

## Group Properties of the Flow

The flow satisfies a one-parameter group structure reflecting the autonomous (time-translation invariant) nature of the ODE:

1. **Identity:** $\phi_0(x_0) = x_0$ for all $x_0 \in U$.
2. **Group law:** $\phi_t(\phi_s(x_0)) = \phi_{t+s}(x_0)$ whenever both sides are defined.
3. **Inverse:** $\phi_{-t}$ is the inverse of $\phi_t$.

**Proof of the group law.** Fix $x_0$ and $s$. The curve $t \mapsto \phi_{t+s}(x_0)$ satisfies the ODE (since translating $t$ gives $\frac{d}{dt}\phi_{t+s}(x_0) = F(\phi_{t+s}(x_0))$) with initial condition $\phi_s(x_0)$ at $t = 0$. But the curve $t \mapsto \phi_t(\phi_s(x_0))$ also satisfies the same ODE with the same initial condition. By uniqueness, they coincide. $\square$

The group law means that computing the state at time $t + s$ in one step or in two steps (first evolving to time $s$, then continuing for time $t$) gives the same result. This is the mathematical expression of determinism and reversibility.

## Smoothness of the Flow

**Theorem.** If $F$ is $C^k$, then the flow $\phi: \mathcal{D} \to U$ is $C^k$ jointly in $(t, x_0)$.

The proof of smoothness in $x_0$ is the heart of the result. Differentiating the ODE with respect to the initial condition $x_0$ (formally), one obtains the **variational equation**:

$$\frac{d}{dt} D_{x_0}\phi_t(x_0) = DF(\phi_t(x_0)) \cdot D_{x_0}\phi_t(x_0), \quad D_{x_0}\phi_0(x_0) = I,$$

where $D_{x_0}\phi_t(x_0)$ is the $n \times n$ Jacobian matrix of $\phi_t$ with respect to $x_0$. This is a linear ODE (with time-dependent coefficients $DF(\phi_t(x_0))$) for the matrix $Y(t) = D_{x_0}\phi_t(x_0)$, the so-called **fundamental matrix solution**.

The variational equation is the key to sensitivity analysis: it tells us how a small perturbation $\delta x_0$ in the initial condition grows (or shrinks) over time. At time $t$, the perturbation has evolved to $D_{x_0}\phi_t(x_0) \cdot \delta x_0$.

## Liouville's Formula and Area Preservation

**Theorem (Liouville).** For any region $D_0 \subset U$,

$$\text{Vol}(\phi_t(D_0)) = \text{Vol}(D_0) \cdot \int_{D_0} e^{\int_0^t \text{div}\, F(\phi_s(x_0)) \, ds} \, d^nx_0.$$

More precisely, $\det D_{x_0}\phi_t(x_0) = \exp\left(\int_0^t \text{tr}\, DF(\phi_s(x_0)) \, ds\right) = \exp\left(\int_0^t \text{div}\, F(\phi_s(x_0)) \, ds\right)$.

**Proof sketch.** Let $W(t) = \det D_{x_0}\phi_t(x_0)$. By Jacobi's formula for derivatives of determinants, $\dot{W} = W \cdot \text{tr}(DF(\phi_t(x_0)))$, from which the formula follows by integrating. $\square$

**Corollary.** If $\text{div}\, F = 0$ everywhere (a divergence-free or incompressible vector field), then $\phi_t$ is volume-preserving: $\text{Vol}(\phi_t(D_0)) = \text{Vol}(D_0)$ for all $t$. In particular, Hamiltonian systems preserve volume (Liouville's theorem in mechanics).

For dissipative systems (like the Lorenz system), $\text{div}\, F < 0$ throughout the phase space, so volumes shrink exponentially. This is the reason dissipative systems can have strange attractors with zero volume: all initial conditions in a large region converge to the attractor, which has zero volume but nonzero fractal dimension.

## Example: The Pendulum

Consider the nonlinear pendulum $\ddot{\theta} = -\sin\theta$, written as a system:

$$\dot{\theta} = \omega, \quad \dot{\omega} = -\sin\theta.$$

The vector field is $F(\theta, \omega) = (\omega, -\sin\theta)$, and $\text{div}\, F = 0$: the flow is area-preserving (this is a Hamiltonian system). The phase portrait consists of:

- Two equilibria: $(\theta, \omega) = (0, 0)$ (stable center) and $(\pi, 0)$ (unstable saddle, modulo $2\pi$).
- Closed curves surrounding $(0,0)$: oscillatory pendulum motion.
- The **separatrix**: the homoclinic orbit connecting $(\pi, 0)$ to itself, corresponding to the pendulum starting at the top with just enough energy to remain there asymptotically.
- Unbounded curves above and below the separatrix: rotation of the pendulum.

The variational equation at the equilibrium $(0,0)$ is $\dot{Y} = \begin{pmatrix} 0 & 1 \\ -1 & 0 \end{pmatrix} Y$, giving oscillatory solutions with pure imaginary eigenvalues $\pm i$. In the nonlinear system, these closed orbits persist (as a consequence of the Hamiltonian structure), and the center is not just linearly but nonlinearly stable.

## Dependence on Parameters

For a parameterized family $\dot{x} = F(x, \mu)$, the flow $\phi_t(x_0, \mu)$ is $C^k$ in all variables $(t, x_0, \mu)$ jointly when $F$ is $C^k$. This is the foundation for bifurcation theory: qualitative changes in the flow occur only at parameter values where hyperbolicity fails.

The sensitivity of the flow to initial conditions is controlled by the fundamental matrix $Y(t) = D_{x_0}\phi_t(x_0)$. When the eigenvalues of $DF(x^*)$ all have negative real parts, $Y(t)$ decays exponentially; this is stability. When some eigenvalue has positive real part, $Y(t)$ grows in that direction, reflecting the exponential divergence of nearby orbits that is the hallmark of unstable equilibria and, in bounded systems, of chaos.
