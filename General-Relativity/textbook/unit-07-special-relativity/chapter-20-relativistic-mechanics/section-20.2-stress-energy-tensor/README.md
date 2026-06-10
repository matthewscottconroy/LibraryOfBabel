# Section 20.2: The Stress-Energy Tensor

---

## Section Introduction

The stress-energy tensor $T^{\mu\nu}$ is the source of gravity in general relativity. It encodes, in a single object, the density and flux of energy and momentum: the 16 components (10 independent, since $T^{\mu\nu} = T^{\nu\mu}$) describe energy density, energy flux, momentum density, momentum flux (pressure and shear stress). In special relativity, its conservation $\partial_\mu T^{\mu\nu} = 0$ encodes the local conservation of energy and momentum. In GR, this becomes $\nabla_\mu T^{\mu\nu} = 0$ — a consequence of the contracted Bianchi identity and the Einstein equations.

Understanding $T^{\mu\nu}$ is not optional for GR. Every gravitational calculation involves the stress-energy content of matter. The Friedmann equations of cosmology use the stress-energy of the cosmic fluid. The Tolman-Oppenheimer-Volkoff equation of stellar structure uses $T^{\mu\nu}$ for a perfect fluid. The gravitational wave formula involves the time-varying quadrupole of $T^{\mu\nu}$. The Einstein equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}$ are precisely the statement that spacetime curvature equals the stress-energy content.

---

## 20.2.1 Definition and Physical Meaning

The stress-energy tensor $T^{\mu\nu}$ is defined by:

$$T^{\mu\nu} = \text{(flux of }\mu\text{-momentum in the }\nu\text{-direction)}$$

More precisely: $T^{\mu\nu}dx_\nu$ is the flux of 4-momentum $p^\mu$ through a surface element perpendicular to the $\nu$-direction.

**Components in a given frame** (coordinates $(x^0, x^1, x^2, x^3) = (ct, x, y, z)$):

- $T^{00}$: flux of energy in the $t$-direction = **energy density** $\rho_{\rm energy} = \epsilon$ (J/m³ or kg·m⁻¹·s⁻²)
- $T^{0i} = T^{i0}$: flux of energy in $i$-direction (= energy current density = $c^2 \times$ momentum density)
- $T^{ij}$: flux of $i$-momentum in $j$-direction = **stress tensor** (Pa)
  - Diagonal $T^{ii}$: pressure in the $i$-direction
  - Off-diagonal $T^{ij}$ ($i\neq j$): shear stress

**Symmetry**: $T^{\mu\nu} = T^{\nu\mu}$. The equality $T^{0i} = T^{i0}$ (energy flux = momentum density $\times c^2$) is a deep result: it follows from angular momentum conservation and is not obvious from Newtonian physics.

---

## 20.2.2 Conservation Laws

In flat spacetime, conservation of energy and momentum:

$$\partial_\mu T^{\mu\nu} = 0$$

This is a single covariant equation encoding:
- $\nu = 0$: $\partial_t T^{00} + \partial_i T^{i0} = 0$ — energy continuity equation
- $\nu = j$: $\partial_t T^{0j} + \partial_i T^{ij} = 0$ — momentum continuity equation

In integral form (using the 4D divergence theorem):

$$\frac{d}{dt}P^\nu = \frac{d}{dt}\int T^{0\nu}d^3x = -\oint T^{i\nu}dA_i$$

The rate of change of total 4-momentum equals the flux through the boundary.

**In curved spacetime**: The conservation law becomes $\nabla_\mu T^{\mu\nu} = 0$ — with the covariant derivative replacing the partial derivative. This is not an additional assumption; it follows from the Einstein equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}$ and the contracted Bianchi identity $\nabla_\mu G^{\mu\nu} = 0$:

$$0 = \nabla_\mu G^{\mu\nu} = 8\pi G\,\nabla_\mu T^{\mu\nu}$$

The covariant conservation $\nabla_\mu T^{\mu\nu} = 0$ includes the exchange of energy between matter and the gravitational field — matter can gain or lose energy to gravity (e.g., a particle gaining kinetic energy by falling into a gravitational potential well).

---

## 20.2.3 Dust

**Dust** = a collection of pressureless particles (no interactions, no random thermal velocities relative to the flow). Each particle has rest mass $m$ and 4-velocity $u^\mu$.

The stress-energy tensor of dust with rest-mass density $\rho_0$ (measured in the rest frame) and 4-velocity $u^\mu$:

$$T^{\mu\nu}_{\rm dust} = \rho_0 u^\mu u^\nu$$

**Components** (in a frame where the dust moves at velocity $\mathbf{v} = 0$, i.e., the rest frame):
- $T^{00} = \rho_0 c^2$ (energy density = rest mass energy density)
- $T^{0i} = T^{i0} = 0$ (no momentum flux)
- $T^{ij} = 0$ (no pressure, no stress)

**In a moving frame** (dust moving at velocity $\mathbf{v}$):
- $T^{00} = \rho_0\gamma^2 c^2$ (energy density includes kinetic energy and length contraction)
- $T^{0i} = \rho_0\gamma^2 v^i c$ (momentum density)
- $T^{ij} = \rho_0\gamma^2 v^i v^j$ (momentum flux)

The conservation equation $\partial_\mu T^{\mu\nu}_{\rm dust} = 0$ gives:
- Continuity: $\partial_\mu(\rho_0 u^\mu) = 0$ (conservation of particle number)
- Geodesic equation: $u^\mu\partial_\mu u^\nu = 0$ (each dust particle follows a straight line)

In GR with a curved metric, dust particles follow geodesics ($u^\mu\nabla_\mu u^\nu = 0$), and $\nabla_\mu T^{\mu\nu} = 0$ encodes both geodesic motion and particle number conservation.

---

## 20.2.4 The Perfect Fluid

A **perfect fluid** has isotropic pressure $p$ (no viscosity, no heat conduction). In the fluid's rest frame, $T^{\mu\nu}$ is diagonal:

$$T^{\mu\nu}_{\rm rest\ frame} = \text{diag}(\epsilon, p, p, p)$$

where $\epsilon$ is the energy density (not just rest mass — includes thermal energy) and $p$ is the pressure.

In a general frame with 4-velocity $u^\mu$:

$$T^{\mu\nu}_{\rm fluid} = (\epsilon + p)u^\mu u^\nu + p\eta^{\mu\nu}$$

*Verification*: In the rest frame $u^\mu = (c, 0, 0, 0)$: $T^{00} = (\epsilon + p)c^2 \cdot c^{-2} \cdot c^{-2} \cdot c^{-2}$... let me redo with conventions. With $u^\mu = (c, 0, 0, 0)$ and $\eta^{\mu\nu} = \text{diag}(-1, +1, +1, +1)$:

$T^{00} = (\epsilon + p) u^0 u^0 / c^2 + p\eta^{00} \cdot c^{-2}$...

Actually more carefully, with natural units $c = 1$ and $u^\mu = (1, 0, 0, 0)$ in the rest frame:
- $T^{00} = (\epsilon + p)(1)(1) + p(-1) = \epsilon$ ✓
- $T^{11} = (\epsilon + p)(0)(0) + p(+1) = p$ ✓
- $T^{0i} = (\epsilon + p)(1)(0) + p(0) = 0$ ✓

**Equation of state**: Relates $p$ and $\epsilon$. Examples:
- **Non-relativistic matter**: $p = 0$ (dust); $p \ll \epsilon/c^2$
- **Radiation** (photons): $p = \epsilon/3$ — from tracelessness $T^\mu_{\ \mu} = 0$ for massless particles
- **Cosmological constant**: $p = -\epsilon$ (negative pressure) — drives accelerated expansion
- **Stiff matter**: $p = \epsilon$ (maximum pressure consistent with causality $v_{\rm sound} \leq c$)

**In GR**: The Friedmann equations for cosmological expansion use:

$$\dot{\rho} + 3\frac{\dot{a}}{a}(\rho + p/c^2) = 0$$

(the fluid conservation equation $\nabla_\mu T^{\mu\nu} = 0$ in FLRW spacetime). Different equations of state give different expansion histories: matter-dominated ($a \propto t^{2/3}$), radiation-dominated ($a \propto t^{1/2}$), dark energy/cosmological constant ($a \propto e^{Ht}$, exponential expansion).

---

## 20.2.5 The Electromagnetic Stress-Energy Tensor

(See Section 17.2.4 for the derivation.)

$$T^{\mu\nu}_{\rm EM} = \frac{1}{\mu_0}\left(F^{\mu\alpha}F^\nu_{\ \alpha} - \frac{1}{4}\eta^{\mu\nu}F_{\alpha\beta}F^{\alpha\beta}\right)$$

**Properties**:
- Symmetric: $T^{\mu\nu}_{\rm EM} = T^{\nu\mu}_{\rm EM}$
- Traceless: $\eta_{\mu\nu}T^{\mu\nu}_{\rm EM} = 0$ (massless photons; radiation equation of state $p = \epsilon/3$)
- Conserved in vacuum: $\partial_\mu T^{\mu\nu}_{\rm EM} = 0$ for $J^\mu = 0$
- Components: $T^{00}_{\rm EM} = u_{\rm EM}$ (field energy density); $T^{0i}_{\rm EM} = S^i/c$ (Poynting vector); $T^{ij}_{\rm EM}$ = Maxwell stress tensor

**As gravitational source**: A region of strong electromagnetic field curves spacetime. The Reissner-Nordström black hole metric — the solution of Einstein's equations for a charged, spherically symmetric body — is sourced by the electromagnetic stress-energy of the charge's Coulomb field.

---

## 20.2.6 The Stress-Energy Tensor in Curved Spacetime

In curved spacetime, $T^{\mu\nu}$ must be defined in a way that is consistent with general covariance. The **covariant** definition uses the metric variation of the matter action:

$$T^{\mu\nu} = -\frac{2}{\sqrt{-g}}\frac{\delta S_{\rm matter}}{\delta g_{\mu\nu}}$$

This definition ensures that $T^{\mu\nu}$ is a symmetric tensor and that $\nabla_\mu T^{\mu\nu} = 0$ follows from the diffeomorphism invariance of the action.

For the perfect fluid in curved spacetime:

$$T^{\mu\nu}_{\rm fluid} = (\epsilon + p)u^\mu u^\nu + p g^{\mu\nu}$$

($\eta^{\mu\nu}$ replaced by $g^{\mu\nu}$). The conservation law $\nabla_\mu T^{\mu\nu} = 0$ gives the relativistic Euler equation (fluid dynamics in curved spacetime) and the energy equation.

**The Einstein equations**: $G_{\mu\nu} = 8\pi G T_{\mu\nu}$ (in units $c = 1$). The right-hand side is the total stress-energy tensor (summed over all matter fields: fluid, electromagnetic, scalar, etc.). The left-hand side is the Einstein tensor $G_{\mu\nu} = R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R$ — the curvature of spacetime. The equations say: **curvature = stress-energy content**. Mass-energy curves spacetime; curved spacetime directs the motion of mass-energy. This circular dependence is what makes GR nonlinear.

---

## References

- Tolman, R.C. (1934). *Relativity, Thermodynamics, and Cosmology.* Oxford University Press. [Develops the relativistic theory of thermodynamics; the Tolman-Oppenheimer-Volkoff equation; the perfect fluid stress-energy tensor.]
- Hawking, S.W. and Ellis, G.F.R. (1973). *The Large Scale Structure of Space-Time.* Cambridge University Press. [Chapter 2 on the stress-energy tensor; the energy conditions (weak, strong, dominant, null); Chapters 4–5 on the causal structure of spacetime.]
- Wald, R.M. (1984). *General Relativity.* University of Chicago Press. [Chapter 4.3 on the stress-energy tensor; the energy conditions and their physical interpretation; the role of $T_{\mu\nu}$ in the Einstein equations.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation.* W.H. Freeman. [Chapter 5 on stress-energy; the decomposition of $T^{\mu\nu}$ into energy density, momentum density, and stress; worked examples for dust, perfect fluid, and electromagnetic field.]
