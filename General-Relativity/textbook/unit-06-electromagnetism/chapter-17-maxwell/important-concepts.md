# Chapter 17: Important Concepts

---

**Maxwell's Equations**
The four equations of classical electrodynamics: (1) $\nabla\cdot\mathbf{E} = \rho/\varepsilon_0$ (Gauss, sources); (2) $\nabla\cdot\mathbf{B} = 0$ (no monopoles); (3) $\nabla\times\mathbf{E} = -\partial_t\mathbf{B}$ (Faraday induction); (4) $\nabla\times\mathbf{B} = \mu_0\mathbf{J} + \mu_0\varepsilon_0\partial_t\mathbf{E}$ (Ampère-Maxwell). Four laws in differential form; four laws in integral form; two laws in covariant tensor form; one line in differential form language.

**Displacement Current**
The term $\varepsilon_0\partial_t\mathbf{E}$ added by Maxwell to Ampère's law. Without it, the equations violate charge conservation. With it, the equations become consistent and predict electromagnetic waves. The displacement current is not a physical current of charges — it is the effect of a changing electric field. Maxwell added it on theoretical grounds (mathematical consistency) before any experimental evidence — one of the greatest theoretical predictions.

**The Faraday Tensor**
$F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$: an antisymmetric rank-2 covariant tensor with 6 independent components encoding $\mathbf{E}$ (off-diagonal time-space components) and $\mathbf{B}$ (spatial components). The correct relativistic object for the electromagnetic field. Under Lorentz boosts, $\mathbf{E}$ and $\mathbf{B}$ mix — they are not separately Lorentz-invariant.

**Maxwell's Equations as Differential Forms**
$dF = 0$ (the Faraday 2-form is closed) and $d\star F = \mu_0\star J$ (the source equation). The first is automatically satisfied when $F = dA$ (since $d^2 = 0$). The second contains the dynamical content. This is the most compact encoding: 4 equations in 2 lines, with manifest geometric meaning.

**Charge Conservation**
$\partial_\mu J^\mu = \partial_t\rho + \nabla\cdot\mathbf{J} = 0$: the local conservation of charge. Follows from $\partial_\mu(\partial_\nu F^{\mu\nu}) = 0$ (antisymmetry), applied to the Maxwell equation $\partial_\nu F^{\mu\nu} = \mu_0 J^\mu$. Charge conservation is a consequence of the gauge invariance of electrodynamics — a special case of Noether's theorem.

**Electromagnetic Waves**
In vacuum, each component of $\mathbf{E}$ and $\mathbf{B}$ satisfies $\Box\psi = 0$ (d'Alembert wave equation with speed $c = 1/\sqrt{\mu_0\varepsilon_0}$). Light is electromagnetic waves. Predicted by Maxwell (1865) from the values of $\mu_0$ and $\varepsilon_0$; confirmed by Hertz (1888) with radio waves.

**Transversality of EM Waves**
In a plane wave, $\mathbf{E}$, $\mathbf{B}$, and $\hat{\mathbf{k}}$ (propagation direction) are mutually orthogonal. $|\mathbf{B}| = |\mathbf{E}|/c$. There are two independent polarizations. Gravitational waves also have two polarizations (but with spin-2 helicity $\pm 2$ vs. spin-1 helicity $\pm 1$ for photons).

**Poynting Vector**
$\mathbf{S} = \mathbf{E}\times\mathbf{B}/\mu_0$: the energy flux density (power per unit area) of the electromagnetic field. The energy density is $u = (\varepsilon_0|\mathbf{E}|^2 + |\mathbf{B}|^2/\mu_0)/2$. Poynting's theorem $\partial_t u + \nabla\cdot\mathbf{S} = -\mathbf{J}\cdot\mathbf{E}$ is the electromagnetic energy conservation equation.

**Electromagnetic Stress-Energy Tensor**
$T^{\mu\nu}_{\rm EM} = (F^{\mu\alpha}F^\nu_{\ \alpha} - \frac{1}{4}\eta^{\mu\nu}F_{\alpha\beta}F^{\alpha\beta})/\mu_0$: encodes energy density, momentum density (Poynting vector/c), and Maxwell stress. Traceless (photons are massless). Source of gravity: electromagnetic fields curve spacetime (Reissner-Nordström charged black hole). In GR: $G_{\mu\nu} = 8\pi G T_{\mu\nu}^{\rm EM}$ for the electrovacuum.

**Larmor Formula**
Power radiated by an accelerating charge: $P = q^2a^2/(6\pi\varepsilon_0 c^3)$. Relativistic generalization (Liénard): $P \propto \gamma^6(|\dot v|^2 - |v\times\dot v/c|^2)$. The gravitational wave analog: binary system radiates power $P_{\rm GW} = G\langle\dddot{Q}_{ij}\dddot{Q}^{ij}\rangle/(5c^5)$ (the quadrupole formula, leading order).

**Electromagnetic Duality**
Vacuum Maxwell equations ($J = 0$) are invariant under $F \to \star F$ ($\mathbf{E} \to c\mathbf{B}$, $c\mathbf{B} \to -\mathbf{E}$). With magnetic monopoles, the symmetry is exact with source terms. The Dirac quantization condition $qg = n\hbar c/2$ would explain charge quantization. S-duality in string theory generalizes this to a quantum symmetry of string theory.

**Maxwell's Equations in Curved Spacetime**
$(1/\sqrt{-g})\partial_\mu(\sqrt{-g}F^{\mu\nu}) = \mu_0 J^\nu$: the generally covariant form. The factor $\sqrt{-g}$ accounts for the curved volume element. The first pair ($dF = 0$) is unchanged. Electromagnetic fields serve as sources in the Einstein equations via $T_{\mu\nu}^{\rm EM}$.
