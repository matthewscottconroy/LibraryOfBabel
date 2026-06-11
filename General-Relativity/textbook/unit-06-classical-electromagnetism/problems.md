# Unit VI Problems: Classical Electromagnetism

*Maxwell's equations in covariant form, the 4-potential, electromagnetic waves, and the transition to special relativity.*

**Difficulty:** ★ Introductory, ★★ Intermediate, ★★★ Advanced

---

## Part 1: Maxwell's Equations — Standard and Covariant Forms

**Problem 1.1** ★
Write Maxwell's equations in SI units (three-vector form):

$$\nabla\cdot\mathbf{E} = \frac{\rho}{\varepsilon_0}, \quad \nabla\cdot\mathbf{B} = 0, \quad \nabla\times\mathbf{E} = -\frac{\partial\mathbf{B}}{\partial t}, \quad \nabla\times\mathbf{B} = \mu_0\mathbf{J} + \mu_0\varepsilon_0\frac{\partial\mathbf{E}}{\partial t}$$

(a) Derive the continuity equation $\partial\rho/\partial t + \nabla\cdot\mathbf{J} = 0$ from the above.
(b) Show that the wave equation $\Box^2\mathbf{E} = 0$ (in vacuum with no sources) follows from Maxwell's equations.
(c) Identify the wave speed and show $c = 1/\sqrt{\mu_0\varepsilon_0}$.

**Problem 1.2** ★★
The electromagnetic field tensor: define the 4-potential $A^\mu = (\phi/c, \mathbf{A})$ and the field tensor $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$.

(a) With $\mu_0 = 1 = \varepsilon_0$ (Gaussian units or natural units), write out all components of $F_{\mu\nu}$ in terms of $\mathbf{E}$ and $\mathbf{B}$. Use the convention $x^\mu = (ct, x, y, z)$ and $\eta_{\mu\nu} = \text{diag}(-1,+1,+1,+1)$.

(b) Show that $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$ automatically implies $\partial_{[\mu}F_{\nu\rho]} = 0$ (the Bianchi identity, equivalent to $\nabla\cdot\mathbf{B}=0$ and Faraday's law).

(c) Maxwell's equations with sources: $\partial_\nu F^{\mu\nu} = \mu_0 J^\mu$ where $J^\mu = (c\rho, \mathbf{J})$. Expand these four equations and identify which standard Maxwell equations they reproduce.

**Problem 1.3** ★★
Gauge invariance: under $A_\mu\to A_\mu + \partial_\mu\Lambda$ (for any scalar function $\Lambda$):

(a) Show that $F_{\mu\nu}$ is gauge-invariant.
(b) The Lorenz gauge condition: $\partial_\mu A^\mu = 0$. Show that this is consistent (i.e., if $A$ doesn't satisfy it, find a $\Lambda$ such that $A'_\mu = A_\mu + \partial_\mu\Lambda$ does).
(c) In the Lorenz gauge, show that Maxwell's equations reduce to $\Box A^\mu = \mu_0 J^\mu$ where $\Box = \partial_\mu\partial^\mu = -\frac{1}{c^2}\partial_t^2 + \nabla^2$.

---

## Part 2: Lorentz Force and Electromagnetic Energy

**Problem 2.1** ★★
The Lorentz force on a particle with charge $q$:

$$f^\mu = q F^\mu_{\ \nu} u^\nu$$

where $u^\nu = dx^\nu/d\tau$ is the 4-velocity.

(a) Write out all four components of $f^\mu$ and identify the spatial components as the familiar 3-force $\mathbf{f} = q(\mathbf{E} + \mathbf{v}\times\mathbf{B})$.

(b) Show that $f^\mu u_\mu = 0$ (the 4-force is orthogonal to the 4-velocity). What does this mean physically? (Hint: it means the magnetic force does no work.)

(c) The relativistic equation of motion: $m\,du^\mu/d\tau = f^\mu$. Show that for a particle moving at velocity $v$ in a uniform magnetic field $B$: the particle moves in a circle with radius $r = \gamma mv/(qB)$ (cyclotron radius).

**Problem 2.2** ★★
The electromagnetic stress-energy tensor:

$$T^{\mu\nu}_\text{EM} = \frac{1}{\mu_0}\left(F^{\mu\lambda}F^\nu_{\ \lambda} - \frac{1}{4}\eta^{\mu\nu}F_{\lambda\rho}F^{\lambda\rho}\right)$$

(a) Show $T^{00}_\text{EM} = \frac{1}{2}(\varepsilon_0 E^2 + B^2/\mu_0)$ — the electromagnetic energy density.

(b) Show $T^{0i}_\text{EM} = S^i/c$ where $\mathbf{S} = \mathbf{E}\times\mathbf{B}/\mu_0$ is the Poynting vector (energy flux).

(c) Verify $\partial_\nu T^{\mu\nu}_\text{EM} = -F^{\mu\nu}J_\nu$ (energy-momentum transferred to charged matter).

**Problem 2.3** ★★★
Electromagnetic invariants: the two quadratic invariants of $F_{\mu\nu}$:

$$\mathcal{F} = F_{\mu\nu}F^{\mu\nu} = 2(B^2 - E^2/c^2), \qquad \mathcal{G} = F_{\mu\nu}\tilde{F}^{\mu\nu} = -4\mathbf{E}\cdot\mathbf{B}/c$$

where $\tilde{F}^{\mu\nu} = \frac{1}{2}\varepsilon^{\mu\nu\rho\sigma}F_{\rho\sigma}$ is the dual.

(a) Verify these expressions for a plane wave $\mathbf{E} = E_0\hat{x}\cos(kz-\omega t)$, $\mathbf{B} = (E_0/c)\hat{y}\cos(kz-\omega t)$.

(b) A configuration has $\mathcal{F} > 0$. Show that in any reference frame, $|\mathbf{B}| > |\mathbf{E}|/c$ — you cannot boost to make $\mathbf{B} = 0$.

(c) The condition $\mathcal{F} = 0$ and $\mathcal{G} = 0$ characterizes null electromagnetic fields (plane waves). Show that the field of a point charge in uniform motion satisfies $\mathcal{G} = 0$ but not $\mathcal{F} = 0$.

---

## Part 3: Radiation and Retarded Potentials

**Problem 3.1** ★★
Retarded potentials: the solution to $\Box A^\mu = \mu_0 J^\mu$ with retarded boundary conditions:

$$A^\mu(\mathbf{x},t) = \frac{\mu_0}{4\pi}\int\frac{J^\mu(\mathbf{x}',t_\text{ret})}{|\mathbf{x}-\mathbf{x}'|}d^3x'$$

where $t_\text{ret} = t - |\mathbf{x}-\mathbf{x}'|/c$.

(a) For a point charge $q$ at position $\mathbf{r}(t)$, the Liénard-Wiechert potentials are:
$$\phi = \frac{q}{4\pi\varepsilon_0}\frac{1}{R - \mathbf{R}\cdot\boldsymbol{\beta}}, \qquad \mathbf{A} = \frac{\phi\boldsymbol{\beta}}{c}$$
where $\mathbf{R} = \mathbf{x} - \mathbf{r}(t_\text{ret})$ and $\boldsymbol{\beta} = \mathbf{v}/c$. For a charge at rest ($\boldsymbol{\beta} = 0$): show this reduces to the Coulomb potential.

(b) A charge undergoing acceleration $\dot{\mathbf{v}}\neq 0$ radiates. The Larmor formula: power radiated $P = q^2|\dot{\mathbf{v}}|^2/(6\pi\varepsilon_0 c^3)$. For a relativistic particle: write the manifestly covariant form $P = -q^2/(6\pi\varepsilon_0 m^2c^3)(du_\mu/d\tau)(du^\mu/d\tau)$.

(c) In GR, a freely falling charge does not radiate (equivalence principle). Does this contradict the Larmor formula? Reconcile the apparent contradiction.

**Problem 3.2** ★★★
The electromagnetic analogy for gravitational waves: linearized GR has the same mathematical structure as electromagnetism with the substitutions:

| Electromagnetism | Linearized GR |
|---|---|
| $A_\mu$ | $\bar{h}_{\mu\nu} = h_{\mu\nu} - \frac{1}{2}\eta_{\mu\nu}h$ |
| $J^\mu = (c\rho, \mathbf{J})$ | $T^{\mu\nu}$ |
| $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$ | $R_{\mu\nu\rho\sigma}$ (linearized) |
| Lorenz gauge $\partial_\mu A^\mu = 0$ | Lorenz gauge $\partial_\nu\bar{h}^{\mu\nu} = 0$ |
| $\Box A^\mu = \mu_0 J^\mu$ | $\Box\bar{h}^{\mu\nu} = -16\pi G/c^4 T^{\mu\nu}$ |

(a) Write the gravitational wave equation explicitly. Identify the coupling constant $16\pi G/c^4$ and discuss why it is so small (weak gravitational radiation).

(b) For the "gravitomagnetic" analogy: the gravitoelectric field $\mathbf{g}$ (ordinary gravity) and gravitomagnetic field $\mathbf{B}_g$ satisfy "gravitational Maxwell equations." Write these equations and compare to the electromagnetic case.

(c) The quadrupole formula for gravitational wave power: $P_\text{GW} = G/(5c^5)\langle\dddot{Q}_{ij}\dddot{Q}^{ij}\rangle$ where $Q_{ij}$ is the reduced quadrupole moment. Why does gravity radiate at quadrupole order while EM radiates at dipole order? (Hint: conservation of mass $\Leftrightarrow$ conservation of charge, but mass dipole conservation is a different statement.)
