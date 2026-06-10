# Section 14.2: Conservation Laws in Newtonian Mechanics

---

## Section Introduction

Conservation laws are not separate postulates layered on top of Newton's laws — they are consequences of them, and more deeply, consequences of the symmetries of space and time. Energy is conserved because the laws of physics don't change from moment to moment. Momentum is conserved because the laws don't depend on where you are. Angular momentum is conserved because they don't depend on which direction you face.

This connection between symmetry and conservation — Noether's theorem, previewed here and proven in Section 15.2 — is one of the deepest structural features of physics. It means that finding a conservation law is equivalent to finding a symmetry; the two are the same discovery stated in different languages.

For the student who will go on to GR: in a generic curved spacetime, the metric has no symmetries (no Killing vectors), and energy is not conserved. In the Schwarzschild spacetime, there are two Killing vectors (time translation and rotation), giving two conserved quantities — which are exactly the constants of motion needed to solve for geodesics. The conservation laws of this section are the Newtonian shadows of much deeper geometric structures.

---

## 14.2.1 Conservation of Energy

**Work-energy theorem**: For a particle with $m\ddot{\mathbf{r}} = \mathbf{F}$:

$$W = \int_{\mathbf{r}_1}^{\mathbf{r}_2} \mathbf{F} \cdot d\mathbf{r} = \int_{t_1}^{t_2} \mathbf{F} \cdot \dot{\mathbf{r}}\, dt = \int_{t_1}^{t_2} m\ddot{\mathbf{r}} \cdot \dot{\mathbf{r}}\, dt = \left[\frac{1}{2}m|\dot{\mathbf{r}}|^2\right]_{t_1}^{t_2} = T_2 - T_1$$

The work done equals the change in kinetic energy $T = \frac{1}{2}m|\dot{\mathbf{r}}|^2$.

**Conservative forces**: $\mathbf{F}$ is **conservative** if $\oint \mathbf{F} \cdot d\mathbf{r} = 0$ for all closed loops. Equivalently (by Stokes' theorem), if $\nabla \times \mathbf{F} = 0$. In a simply connected domain, this is equivalent to $\mathbf{F} = -\nabla V$ for some scalar function V (the potential energy).

For a conservative force: $W = -[V]_{1}^{2} = V_1 - V_2$. Combined with the work-energy theorem:

$$T_1 + V_1 = T_2 + V_2$$

The **mechanical energy** $E = T + V = \frac{1}{2}m|\dot{\mathbf{r}}|^2 + V(\mathbf{r})$ is conserved.

*Verification by direct differentiation*:

$$\frac{dE}{dt} = m\dot{\mathbf{r}} \cdot \ddot{\mathbf{r}} + \nabla V \cdot \dot{\mathbf{r}} = \dot{\mathbf{r}} \cdot (m\ddot{\mathbf{r}} + \nabla V) = \dot{\mathbf{r}} \cdot (\mathbf{F} - \mathbf{F}) = 0 \qquad \checkmark$$

**Energy as generator of time evolution**: In Hamiltonian mechanics (Section 15.3), the Hamiltonian H generates time evolution via Poisson brackets: $\dot{f} = \{f, H\}$. Energy is both a conserved quantity and the generator of the time flow. This duality is fundamental.

**Dissipative forces**: Friction ($\mathbf{F}_{\rm friction} = -b\dot{\mathbf{r}}$) is not conservative ($\nabla \times \mathbf{F} \neq 0$). The work done is path-dependent; mechanical energy decreases (but total energy including thermal energy is conserved — a deeper statement about the microstructure).

---

## 14.2.2 Conservation of Momentum

**For a single particle**: $\dot{\mathbf{p}} = \mathbf{F}$. If $\mathbf{F} = 0$, then $\mathbf{p} = m\dot{\mathbf{r}}$ is constant. This is trivial.

**For a system of N particles**: Label them $i = 1, \ldots, N$ with positions $\mathbf{r}_i$ and masses $m_i$. Let $\mathbf{F}_i^{\rm ext}$ be the external force on particle $i$, and $\mathbf{F}_{ij}$ be the internal force of particle $j$ on particle $i$. Newton's third law: $\mathbf{F}_{ij} = -\mathbf{F}_{ji}$.

Total momentum: $\mathbf{P} = \sum_i m_i \dot{\mathbf{r}}_i$.

$$\dot{\mathbf{P}} = \sum_i m_i \ddot{\mathbf{r}}_i = \sum_i \mathbf{F}_i^{\rm ext} + \sum_{i \neq j} \mathbf{F}_{ij} = \mathbf{F}^{\rm ext}$$

(Internal forces cancel in pairs by Newton's third law.) So $\dot{\mathbf{P}} = \mathbf{F}^{\rm ext}$: the total momentum changes only due to external forces. If $\mathbf{F}^{\rm ext} = 0$, the total momentum is conserved.

**Center of mass**: $\mathbf{R}_{\rm cm} = \frac{1}{M}\sum_i m_i \mathbf{r}_i$ where $M = \sum_i m_i$. Then $M\ddot{\mathbf{R}}_{\rm cm} = \mathbf{F}^{\rm ext}$: the center of mass moves as a single particle of total mass M under the total external force. This **reduction principle** is exact for Newtonian gravity.

**Connection to translation invariance**: Momentum conservation follows from the invariance of the total potential energy $V(\mathbf{r}_1, \ldots, \mathbf{r}_N)$ under overall translations $\mathbf{r}_i \to \mathbf{r}_i + \mathbf{a}$. If V depends only on relative positions $\mathbf{r}_i - \mathbf{r}_j$ (no external fields), it is translation-invariant, and total momentum is conserved. This is Noether's theorem for translations.

---

## 14.2.3 Conservation of Angular Momentum

**Torque and angular momentum**: For a particle, the angular momentum about the origin is $\mathbf{L} = \mathbf{r} \times \mathbf{p} = m\mathbf{r} \times \dot{\mathbf{r}}$. The torque is $\boldsymbol{\tau} = \mathbf{r} \times \mathbf{F}$.

$$\frac{d\mathbf{L}}{dt} = \dot{\mathbf{r}} \times m\dot{\mathbf{r}} + \mathbf{r} \times m\ddot{\mathbf{r}} = 0 + \mathbf{r} \times \mathbf{F} = \boldsymbol{\tau}$$

($\dot{\mathbf{r}} \times \dot{\mathbf{r}} = 0$ since a vector crossed with itself vanishes.) So $\dot{\mathbf{L}} = \boldsymbol{\tau}$.

**Central force**: If $\mathbf{F} = F(r)\hat{\mathbf{r}}$ (force directed radially), then $\boldsymbol{\tau} = \mathbf{r} \times F(r)\hat{\mathbf{r}} = 0$ (since $\mathbf{r} \parallel \hat{\mathbf{r}}$). The angular momentum $\mathbf{L} = m\mathbf{r} \times \dot{\mathbf{r}}$ is conserved. This forces the orbit to lie in a plane perpendicular to $\mathbf{L}$.

**For a system of particles**: Total angular momentum $\mathbf{L} = \sum_i \mathbf{r}_i \times m_i\dot{\mathbf{r}}_i$.

$$\dot{\mathbf{L}} = \sum_i \boldsymbol{\tau}_i^{\rm ext} + \sum_{i \neq j} \mathbf{r}_i \times \mathbf{F}_{ij}$$

The internal torques cancel if $\mathbf{F}_{ij}$ acts along the line connecting particles $i$ and $j$ (central forces):

$$\mathbf{r}_i \times \mathbf{F}_{ij} + \mathbf{r}_j \times \mathbf{F}_{ji} = (\mathbf{r}_i - \mathbf{r}_j) \times \mathbf{F}_{ij} = 0$$

(since $\mathbf{F}_{ij} \parallel (\mathbf{r}_i - \mathbf{r}_j)$ for central forces). So internal forces contribute no torque, and $\dot{\mathbf{L}} = \boldsymbol{\tau}^{\rm ext}$. If $\boldsymbol{\tau}^{\rm ext} = 0$, $\mathbf{L}$ is conserved.

**Kepler's second law** is a direct consequence: the areal velocity $dA/dt = \frac{1}{2}|\mathbf{r} \times \dot{\mathbf{r}}| = L/(2m)$ is constant (see Section 16.2).

---

## 14.2.4 The Virial Theorem

For a system whose trajectories remain bounded in phase space, the time-average of kinetic energy relates to the time-average of the potential:

**Virial theorem**: For a potential $V = \alpha r^n$:

$$\langle T \rangle = \frac{n}{2}\langle V \rangle$$

**Proof**: Define the virial $G = \sum_i \mathbf{r}_i \cdot \mathbf{p}_i$. Then:

$$\frac{dG}{dt} = \sum_i \dot{\mathbf{r}}_i \cdot \mathbf{p}_i + \sum_i \mathbf{r}_i \cdot \dot{\mathbf{p}}_i = 2T + \sum_i \mathbf{r}_i \cdot \mathbf{F}_i$$

For a power-law potential $V = \alpha r^n$, the force is $\mathbf{F}_i = -\nabla_i V$ and $\sum_i \mathbf{r}_i \cdot \mathbf{F}_i = -nV$ (Euler's theorem for homogeneous functions: $\sum r_i \partial f/\partial r_i = n f$ for $f$ homogeneous of degree $n$). So:

$$\frac{dG}{dt} = 2T - nV$$

For a bounded, periodic trajectory: $\langle dG/dt \rangle = 0$ (G is bounded). Thus $\langle 2T \rangle = n\langle V \rangle$, i.e., $\langle T \rangle = \frac{n}{2}\langle V \rangle$. □

**Applications**:
- **Gravity** ($V = -GMm/r$, $n = -1$): $\langle T \rangle = -\frac{1}{2}\langle V \rangle$, so $\langle E \rangle = \langle T \rangle + \langle V \rangle = \frac{1}{2}\langle V \rangle < 0$ (bound orbits have negative total energy). This is why adding energy to a satellite (firing rockets) can actually lower its orbital speed — the gain in potential energy more than compensates.
- **Harmonic oscillator** ($V = \frac{1}{2}k r^2$, $n = 2$): $\langle T \rangle = \langle V \rangle$. Average kinetic and potential energies are equal.
- **Self-gravitating gas** ($n = -1$): $\langle T \rangle = -\frac{1}{2}\langle E_{\rm grav} \rangle$. A star that loses energy by radiation ($E$ decreases) contracts and heats up ($T$ increases). Stars heat up when they cool! This is the **negative heat capacity of self-gravitating systems** — one of the most counterintuitive facts in astrophysics.

**GR connection**: The virial theorem remains valid in the post-Newtonian approximation (with relativistic corrections to $T$ and $V$). For a system of $N$ black holes, the GR virial theorem relates the average kinetic energy to the gravitational binding energy and radiation losses [Chandrasekhar and Nutku (1969)].

[Clausius, R. (1870). "On a mechanical theorem applicable to heat." *Philosophical Magazine*, 40(265), 122–127. The original statement of the virial theorem, derived from a mechanical argument about bounded systems. The name "virial" is from the Latin *vires* (forces).]

---

## 14.2.5 The Effective Potential and Reduction to 1D

For a particle in a central force $F(r)$ with conserved angular momentum $\ell = mr^2\dot\phi$, the radial motion is governed by:

$$\frac{1}{2}m\dot{r}^2 + V_{\rm eff}(r) = E$$

where the **effective potential** is:

$$V_{\rm eff}(r) = V(r) + \frac{\ell^2}{2mr^2}$$

The centrifugal term $\ell^2/(2mr^2)$ repels the particle from $r = 0$ (for $\ell \neq 0$). The interplay between the attractive potential $V(r)$ and the repulsive centrifugal barrier determines the topology of the orbit:

| $E$ vs $V_{\rm eff}^{\rm min}$ | Orbit type |
|--------------------------------|------------|
| $E = V_{\rm eff}(r_0)$ | Circular orbit at $r_0$ |
| $V_{\rm eff}^{\rm min} < E < 0$ | Bound, non-circular (ellipse for gravity) |
| $E = 0$ | Parabolic orbit (escape velocity) |
| $E > 0$ | Hyperbolic (unbound orbit) |

For the Newtonian potential $V = -GMm/r$:

$$V_{\rm eff}(r) = -\frac{GMm}{r} + \frac{\ell^2}{2mr^2}$$

Circular orbit condition: $dV_{\rm eff}/dr = 0$ gives $r_0 = \ell^2/(GMm^2)$.

**GR effective potential** (Schwarzschild, $m = 1$, $c = 1$):

$$V_{\rm eff}^{\rm GR}(r) = -\frac{GM}{r} + \frac{\ell^2}{2r^2} - \frac{GM\ell^2}{r^3}$$

The extra $-GM\ell^2/r^3$ term (the GR correction) deepens the potential well near the origin. For $r \lesssim 3r_s = 6GM$ (the innermost stable circular orbit, ISCO), there is no longer a stable circular orbit — particles plunge inward. This purely relativistic feature has no Newtonian analog.

---

## References

- Clausius, R. (1870). "On a mechanical theorem applicable to heat." *Philosophical Magazine*, 40(265), 122–127. [The virial theorem: kinetic energy of a bounded mechanical system relates to the forces (the "virial") by a time-averaging argument.]
- Noether, E. (1918). "Invariante Variationsprobleme." *Nachrichten von der Gesellschaft der Wissenschaften zu Göttingen*, 235–257. [The fundamental theorem: symmetries → conservation laws. Energy from time translation, momentum from space translation, angular momentum from rotation.]
- Chandrasekhar, S. and Nutku, Y. (1969). "The second post-Newtonian equations of hydrodynamics in general relativity." *Astrophysical Journal*, 158, 55–79. [Virial theorem with GR corrections for astrophysical systems.]
- Goldstein, H., Poole, C., and Safko, J. (2002). *Classical Mechanics*, 3rd ed. Addison-Wesley. [Chapter 3 on central force motion; Chapter 1 on conservation laws. The standard graduate reference for analytical mechanics.]
