# Section 36.1: Matter in Curved Spacetime

---

## The Stress-Energy Tensor: Geometry as Source

The stress-energy tensor $T^{\mu\nu}$ is defined by its physical content:

$$T^{\mu\nu} = \text{flux of $\mu$-component of 4-momentum across a surface of constant $x^\nu$}$$

More precisely: $T^{\mu\nu}$ is a symmetric rank-2 tensor with components:
- $T^{00}$: energy density (energy per unit volume)
- $T^{0i} = T^{i0}$: energy flux / momentum density (energy flowing in the $i$-direction per unit area per unit time, OR equivalently $c$ times the momentum density)
- $T^{ij}$: momentum flux = stress (rate at which $i$-momentum crosses a surface of constant $x^j$)

In a static fluid at rest, $T^{ij} = p\delta^{ij}$ (pressure is isotropic stress). For a viscous fluid, $T^{ij}$ includes shear stresses.

The covariant divergence-free condition $\nabla_\mu T^{\mu\nu} = 0$ is local energy-momentum conservation.

---

## Dust

**Dust** is pressureless matter: particles moving with 4-velocity $u^\mu$ that have no thermal motion relative to each other. The stress-energy tensor is:
$$T^{\mu\nu}_{\rm dust} = \rho_0 u^\mu u^\nu$$
where $\rho_0$ is the proper rest-mass density (density in the rest frame of the fluid).

Components in the rest frame ($u^\mu = (c, 0, 0, 0)$):
- $T^{00} = \rho_0 c^2$: energy density = rest mass energy density
- $T^{0i} = 0$: no momentum in the rest frame
- $T^{ij} = 0$: no pressure or stress

In a general frame with velocity $\mathbf{v}$:
- $T^{00} = \gamma^2\rho_0 c^2$: the $\gamma^2$ comes from one factor of $\gamma$ for time dilation (density increases) and one for energy ($E = \gamma mc^2$)
- $T^{0i} = \gamma^2\rho_0 c v^i$: momentum density
- $T^{ij} = \gamma^2\rho_0 v^i v^j$: momentum flux (ram pressure)

The conservation equation $\nabla_\mu T^{\mu\nu}_{\rm dust} = 0$ plus $u_\nu u^\nu = -c^2$ gives:
1. **Continuity equation:** $\nabla_\mu(\rho_0 u^\mu) = 0$ — conservation of rest mass
2. **Geodesic equation:** $u^\mu\nabla_\mu u^\nu = 0$ — each dust particle follows a geodesic

This is the key result: in the absence of non-gravitational forces, the conservation of the stress-energy tensor implies that particles follow geodesics. The geodesic equation is not an independent postulate — it follows from $\nabla_\mu T^{\mu\nu} = 0$ for pressureless matter.

---

## Perfect Fluid

A **perfect fluid** has isotropic pressure but no viscosity or heat conduction. It is characterized by its proper energy density $\varepsilon$ (energy per unit rest volume, including rest mass energy), pressure $p$, and 4-velocity $u^\mu$:
$$T^{\mu\nu}_{\rm perfect} = \frac{\varepsilon + p}{c^2}u^\mu u^\nu + p g^{\mu\nu}$$

This can be understood as follows: in the rest frame, the energy density is $\varepsilon$ and the pressure is $p$ isotropically. The form of $T^{\mu\nu}$ is the unique symmetric rank-2 tensor that equals $\text{diag}(\varepsilon, p, p, p)$ in the rest frame.

**Equations of state** for common cases:

| Matter type | Equation of state | Physical system |
|---|---|---|
| Dust | $p = 0$ | Cold dark matter, pressureless gas |
| Non-relativistic gas | $p = \frac{2}{3}\frac{\text{KE}}{V} \ll \varepsilon$ | Ordinary baryonic matter |
| Radiation | $p = \varepsilon/3$ | Photon gas, early universe |
| Stiff fluid | $p = \varepsilon$ | Maximum sound speed $c_s = c$ |
| Cosmological constant | $p = -\varepsilon$ | Vacuum energy, dark energy |
| De Sitter | $p = -\varepsilon$ | Inflation |
| Neutron star | $p(\varepsilon)$ complex | Nuclear equation of state |

The conservation equation $\nabla_\mu T^{\mu\nu}_{\rm perfect} = 0$ splits into two equations:
1. **Energy conservation:** $u_\nu\nabla_\mu T^{\mu\nu} = 0$ gives $u^\mu\nabla_\mu\varepsilon + (\varepsilon + p)\nabla_\mu u^\mu = 0$
2. **Relativistic Euler equation:** $h^{\nu}_{\ \alpha}\nabla_\mu T^{\mu\alpha} = 0$ (projecting perpendicular to $u^\mu$) gives $(\varepsilon+p)u^\mu\nabla_\mu u^\nu + (g^{\mu\nu} + u^\mu u^\nu/c^2)\nabla_\mu p = 0$

where $h^{\mu\nu} = g^{\mu\nu} + u^\mu u^\nu/c^2$ is the projection operator perpendicular to $u^\mu$.

The relativistic Euler equation reduces to the Newtonian Euler equation $\rho(\partial_t\mathbf{v} + \mathbf{v}\cdot\nabla\mathbf{v}) = -\nabla p$ in the limit $v \ll c$, $p \ll \rho c^2$.

---

## Electromagnetic Stress-Energy

The electromagnetic field contributes to the stress-energy tensor through the Faraday tensor:
$$T^{\mu\nu}_{\rm EM} = \frac{1}{\mu_0}\left(F^{\mu\alpha}F^\nu_{\ \alpha} - \frac{1}{4}g^{\mu\nu}F_{\alpha\beta}F^{\alpha\beta}\right)$$

In the rest frame (for $\mathbf{B} = 0$, $\mathbf{E}$ in the $x$-direction):
$$T^{\mu\nu}_{\rm EM} = \frac{\varepsilon_0}{2}E^2\,\text{diag}(1, 1, -1, -1)$$

This tensor is:
- **Symmetric:** $T^{\mu\nu}_{\rm EM} = T^{\nu\mu}_{\rm EM}$ ✓
- **Traceless:** $T^\mu_{\ \mu,{\rm EM}} = 0$ (photons are massless; radiation has $p = \varepsilon/3$, so $\varepsilon - 3p = 0$) ✓
- **Divergence-free (in vacuum):** $\nabla_\nu T^{\mu\nu}_{\rm EM} = F^{\mu}_{\ \nu}J^\nu$ — the 4-force density exerted by the field on charges

For $J^\mu = 0$ (vacuum EM), $\nabla_\nu T^{\mu\nu}_{\rm EM} = 0$.

**The Reissner-Nordström metric** is the exact solution for a charged, non-rotating black hole of mass $M$ and charge $Q$. Its stress-energy is entirely electromagnetic:
$$ds^2 = -\left(1 - \frac{2GM}{rc^2} + \frac{GQ^2}{4\pi\varepsilon_0 r^2 c^4}\right)c^2 dt^2 + \left(\cdots\right)^{-1}dr^2 + r^2d\Omega^2$$
The electric field $E = Q/(4\pi\varepsilon_0 r^2)$ contributes to $T^{\mu\nu}_{\rm EM}$, which sources the metric even outside the horizon. Interestingly, the electromagnetic field energy *reduces* the effective gravitational mass: the effective source is $M_{\rm eff}(r) = M - Q^2/(8\pi\varepsilon_0 r c^2)$.

---

## Energy Conditions in Practice

The energy conditions constrain the physical reasonableness of the stress-energy tensor.

**Weak Energy Condition (WEC):** $T_{\mu\nu}t^\mu t^\nu \geq 0$ for all timelike $t^\mu$. For a perfect fluid: $\varepsilon \geq 0$ and $\varepsilon + p \geq 0$. Physical requirement: energy density is non-negative.

**Null Energy Condition (NEC):** $T_{\mu\nu}k^\mu k^\nu \geq 0$ for all null $k^\mu$. For a perfect fluid: $\varepsilon + p \geq 0$. Weaker than WEC.

**Strong Energy Condition (SEC):** $(T_{\mu\nu} - \frac{1}{2}g_{\mu\nu}T)t^\mu t^\nu \geq 0$. Equivalent to $R_{\mu\nu}t^\mu t^\nu \geq 0$ (by the Einstein equations). For a perfect fluid: $\varepsilon + p \geq 0$ AND $\varepsilon + 3p \geq 0$. Physical: gravity is attractive.

**Dominant Energy Condition (DEC):** WEC plus $-T^\mu_{\ \nu}t^\nu$ is causal for all timelike $t^\mu$. Physical: energy flux is causal (energy doesn't flow faster than light).

| Matter type | WEC | NEC | SEC | DEC |
|---|---|---|---|---|
| Ordinary matter ($p \geq 0$) | ✓ | ✓ | ✓ | ✓ |
| Radiation ($p = \varepsilon/3$) | ✓ | ✓ | ✓ | ✓ |
| $\Lambda > 0$ (dark energy, $p = -\varepsilon$) | ✓ | ✓ | ✗ | ✓ |
| Quantum vacuum fluctuations | ✗ | ✗ (locally) | ✗ | ✗ |

The SEC violation for $\Lambda > 0$ is physically significant: a positive cosmological constant causes *repulsive* gravity (the universe accelerates). The SEC is required for the Penrose-Hawking singularity theorems to apply — dark energy can in principle avoid singularity theorems.

---

## The Variational Definition

The most elegant definition of $T^{\mu\nu}$ in GR is through the matter action:
$$T_{\mu\nu} = -\frac{2}{\sqrt{-g}}\frac{\delta S_{\rm matter}}{\delta g^{\mu\nu}}$$

This definition is manifestly symmetric (since $g^{\mu\nu}$ is symmetric), automatically covariant, and gives the correct stress-energy for all known matter fields. It is the "canonical" definition in GR.

For example:
- **Scalar field** $\phi$ with action $S = -\frac{1}{2}\int(\nabla_\mu\phi\nabla^\mu\phi + m^2\phi^2)\sqrt{-g}\,d^4x$:
$$T_{\mu\nu} = \nabla_\mu\phi\nabla_\nu\phi - \frac{1}{2}g_{\mu\nu}(\nabla_\alpha\phi\nabla^\alpha\phi + m^2\phi^2)$$
- **Perfect fluid** $S = -\int\rho_0 c^2\sqrt{-g}\,d^4x$ (in the rest frame): gives $T^{\mu\nu} = \rho_0 c^2 u^\mu u^\nu$ (dust) ✓
- **Electromagnetic field** $S = -\frac{1}{4\mu_0}\int F_{\mu\nu}F^{\mu\nu}\sqrt{-g}\,d^4x$: gives the EM stress-energy tensor above ✓

This variational definition shows that $T_{\mu\nu}$ is the derivative of the matter action with respect to the metric — it measures how the matter action responds to changes in the geometry. Matter curves spacetime (through $G_{\mu\nu} = 8\pi G T_{\mu\nu}$) and spacetime curves matter's motion (through the geodesic equation or the relativistic Euler equation).

