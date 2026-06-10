# Chapter 25: Four-Vectors and Relativistic Dynamics

---

## Chapter Introduction

In Newtonian mechanics, the basic dynamical quantities — position, velocity, momentum, force, energy — are three-dimensional vectors living in space, with time playing a separate role as a parameter. In special relativity, space and time are unified. The natural objects of dynamics are **four-vectors**: vectors in Minkowski spacetime that transform covariantly under Lorentz transformations.

The conceptual payoff of four-vectors is immense. Einstein's most famous equation, $E = mc^2$, is not an isolated result — it is a component of a single four-vector equation, the conservation of 4-momentum. The unification of energy and momentum into a single object is not just convenient; it is the reason why nuclear reactions release energy, why particle collisions can create new particles, and why the "mass-energy equivalence" is a statement about spacetime geometry.

This chapter builds the four-vector formalism from scratch, defines the key four-vectors (4-velocity, 4-momentum, 4-force), derives $E = mc^2$, and applies the machinery to collision problems and decay kinematics. By the end, you will have the tools to analyze any relativistic dynamics problem — and you will have seen how the structure of spacetime constrains what is physically possible.

---

## Lorentz Scalars, Vectors, and Tensors

A **Lorentz scalar** is a quantity that is the same in all inertial frames. Examples: the rest mass $m$, the spacetime interval $ds^2$, the inner product $A_\mu B^\mu$ of any two 4-vectors.

A **4-vector** (or Lorentz vector) is a set of four quantities $(A^0, A^1, A^2, A^3)$ that transform under Lorentz transformations exactly like the coordinates $(ct, x, y, z)$:
$$A'^\mu = \Lambda^\mu_{\ \nu}A^\nu$$

The **invariant inner product** of two 4-vectors:
$$A\cdot B \equiv A_\mu B^\mu = \eta_{\mu\nu}A^\mu B^\nu = -A^0B^0 + A^1B^1 + A^2B^2 + A^3B^3$$

This is a Lorentz scalar — the same in all frames.

The **norm** of a 4-vector: $A^2 = A\cdot A = \eta_{\mu\nu}A^\mu A^\nu$. It can be negative (timelike), zero (null), or positive (spacelike).

---

## The 4-Velocity

For a massive particle with worldline $x^\mu(\tau)$ (parametrized by proper time $\tau$), the **4-velocity** is:
$$u^\mu = \frac{dx^\mu}{d\tau}$$

Since $d\tau = dt/\gamma$:
$$u^\mu = \gamma\frac{dx^\mu}{dt} = \gamma\left(c, \frac{dx}{dt}, \frac{dy}{dt}, \frac{dz}{dt}\right) = \gamma(c, \mathbf{v})$$

The norm of the 4-velocity is always:
$$u^\mu u_\mu = \eta_{\mu\nu}u^\mu u^\nu = \gamma^2(-c^2 + v^2) = \gamma^2 c^2(v^2/c^2 - 1) = -c^2$$

The 4-velocity is always timelike with norm $-c^2$. This is the relativistic generalization of "speed": all massive particles move through spacetime at the same "spacetime speed" $c$.

For a photon, there is no proper time, so the 4-velocity is not defined. Instead, one uses the **null tangent vector** (4-momentum direction) $k^\mu$ with $k^\mu k_\mu = 0$.

---

## The 4-Momentum

The **4-momentum** (or energy-momentum 4-vector) of a particle with rest mass $m$:
$$p^\mu = mu^\mu = m\gamma(c, \mathbf{v}) = \left(\frac{E}{c}, \mathbf{p}\right)$$

where:
$$E = m\gamma c^2 = \frac{mc^2}{\sqrt{1-v^2/c^2}}, \quad \mathbf{p} = m\gamma\mathbf{v} = \frac{m\mathbf{v}}{\sqrt{1-v^2/c^2}}$$

The norm of 4-momentum:
$$p^\mu p_\mu = -\frac{E^2}{c^2} + |\mathbf{p}|^2 = -(mc)^2$$

This gives the fundamental **energy-momentum relation**:
$$\boxed{E^2 = (pc)^2 + (mc^2)^2}$$

This is the relativistic dispersion relation. It is the cornerstone of relativistic dynamics.

**Special cases**:
- At rest ($p = 0$): $E = mc^2$ — rest energy
- Massless particles ($m = 0$): $E = pc$ — photons and (to good approximation) ultrarelativistic particles
- Non-relativistic ($v\ll c$): $E \approx mc^2 + \frac{1}{2}mv^2$ — rest energy plus kinetic energy

---

## Mass-Energy Equivalence: $E = mc^2$

The equation $E = mc^2$ is one of the most famous in science — and often the most misunderstood.

**What it says**: A particle at rest has energy $E_0 = mc^2$. This is not the kinetic energy — it is the energy *stored in the mass itself*. Mass is a form of energy.

**What it implies**: 
- In a nuclear reaction, if the rest masses of the products are less than those of the reactants, the difference $\Delta m$ appears as kinetic energy: $\Delta E = \Delta mc^2$.
- In annihilation ($e^+ + e^- \to 2\gamma$): two electrons' rest mass ($2\times 0.511$ MeV/c$^2$) is entirely converted to photon energy.
- A compressed spring has more mass than a relaxed one (by $\Delta m = U_{\rm spring}/c^2$, utterly unmeasurable).

**Origin**: From the derivation, $E = mc^2$ follows from:
1. The requirement that momentum be conserved in all frames
2. The form $\mathbf{p} = m\gamma\mathbf{v}$
3. Expanding for $v\ll c$: the conserved quantity reduces to $mc^2 + \frac{1}{2}mv^2 + \cdots$

Einstein derived this in a companion 1905 paper: "Does the Inertia of a Body Depend Upon Its Energy Content?"

---

## 4-Momentum Conservation

In any isolated system, the **total 4-momentum is conserved**:
$$\sum_i p^\mu_i = \text{const}$$

This unifies conservation of energy and conservation of 3-momentum into a single 4-vector law.

**Elastic collision**: Particles scatter but masses don't change. $E$ and $|\mathbf{p}|$ are conserved.

**Inelastic collision**: Total 4-momentum conserved; individual masses may change. A particle can be created from kinetic energy.

**Particle creation threshold**: To create a particle of mass $M$ from two colliding particles, the total invariant mass of the initial state must be $\geq M + m_1 + m_2$. The **invariant mass** $s = -(p_1+p_2)^2/c^2$ (in units where $c=1$: $s = (p_1+p_2)^2$) is a Lorentz scalar — and is the same in *any* reference frame, including the center-of-mass frame. In the center-of-mass frame, $\sqrt{s}$ equals the total energy available for new particle creation.

**Threshold calculation**: If two protons ($m_p = 938$ MeV/$c^2$) collide, and one is at rest, to create a pion ($m_\pi = 135$ MeV/$c^2$) via $p + p \to p + p + \pi^0$, the minimum kinetic energy of the beam proton is found by setting $\sqrt{s} = 2m_p + m_\pi$ in the lab frame.

---

## The 4-Force

The relativistic equation of motion (Newton's second law in 4-vector form):
$$f^\mu = \frac{dp^\mu}{d\tau} = m\frac{du^\mu}{d\tau}$$

where $f^\mu$ is the **4-force** (or Minkowski force).

Since $u^\mu u_\mu = -c^2$ (constant), differentiating:
$$u_\mu\frac{du^\mu}{d\tau} = 0 \implies u_\mu f^\mu = 0$$

The 4-force is always perpendicular to the 4-velocity. The time component of $f^\mu$ is related to the power:
$$f^0 = \frac{1}{c}\frac{dE}{d\tau} = \frac{\gamma}{c}\mathbf{f}\cdot\mathbf{v}$$

where $\mathbf{f} = d\mathbf{p}/dt$ is the 3-force.

**The Lorentz 4-force**: For a particle with charge $q$ in an electromagnetic field:
$$f^\mu = qF^{\mu}_{\ \nu}u^\nu$$

where $F^{\mu\nu}$ is the electromagnetic field tensor (Chapter 26). This gives the covariant form of the Lorentz force.

---

## 4-Momentum for Photons

Photons have zero rest mass. Their 4-momentum:
$$p^\mu = \frac{\hbar\omega}{c}(1, \hat{n}) = \hbar k^\mu$$

where $\omega$ is the angular frequency, $\hat{n}$ is the propagation direction, and $k^\mu = (\omega/c, \mathbf{k})$ is the 4-wavevector.

The norm: $p^\mu p_\mu = 0$ (null) — consistent with $m = 0$.

**Photon energy-momentum**: $E = \hbar\omega = pc$. In a moving frame, frequency is Doppler-shifted — this is just Lorentz transformation of $p^\mu$.

**Compton scattering**: A photon scatters off an electron at rest. Conservation of 4-momentum:
$$p_\gamma^\mu + p_e^\mu = p_{\gamma'}^\mu + p_{e'}^\mu$$

Working in the lab frame and using the invariant: $(p_\gamma + p_e - p_{\gamma'})^2 = p_{e'}^2$, gives the **Compton formula**:
$$\lambda' - \lambda = \frac{h}{m_e c}(1-\cos\theta) = \lambda_C(1-\cos\theta)$$

where $\lambda_C = h/(m_e c) = 2.426$ pm is the Compton wavelength. This was the first relativistic quantum result confirmed experimentally (Compton, 1923).

---

## Relativistic Kinematics: Collision Examples

**Example 1: Threshold for pion creation.**

$p + p \to p + p + \pi^0$. In the lab frame (proton target at rest):
$$s = -(p_1 + p_2)^2 = m_p^2 c^2 + 2m_p E_{\rm beam}/c^2 + m_p^2 c^2$$

Wait — in natural units ($c = 1$): $s = (p_1+p_2)^2 = 2m_p^2 + 2m_p E_{\rm beam}$ (using $p_2 = (m_p, 0)$).

At threshold: $\sqrt{s} = 2m_p + m_\pi$, so $s = (2m_p + m_\pi)^2$.

$$E_{\rm beam} = \frac{(2m_p+m_\pi)^2 - 2m_p^2}{2m_p} = 2m_p + 2m_\pi + \frac{m_\pi^2}{2m_p} \approx 1.22\ \text{GeV}$$

At a collider (both protons equal and opposite): $\sqrt{s} = 2E$. Threshold: $E_{\rm threshold} = m_p + m_\pi/2 = 0.96\ \text{GeV}$. Much lower threshold — this is why colliders are powerful.

**Example 2: Decay kinematics.**

A pion at rest decays: $\pi^0 \to 2\gamma$. Conservation of 4-momentum:
$$p_\pi^\mu = p_1^\mu + p_2^\mu$$

In the pion rest frame: $\mathbf{p}_1 + \mathbf{p}_2 = 0$ (momenta equal and opposite), $E_1 + E_2 = m_\pi c^2$. By symmetry: $E_1 = E_2 = m_\pi c^2/2$. The photons go in opposite directions with energy $67.5$ MeV each.

In a frame where the pion moves at velocity $v$: apply the Lorentz transformation to find the boosted photon energies (they depend on the emission angle — Doppler effect).

---

## Important Concepts

- **4-vector**: A Lorentz-covariant object transforming as $A'^\mu = \Lambda^\mu_{\ \nu}A^\nu$
- **Lorentz scalar**: Inner product $A^\mu B_\mu = \eta_{\mu\nu}A^\mu B^\nu$; invariant under Lorentz transformations
- **4-velocity**: $u^\mu = \gamma(c, \mathbf{v})$; norm always $-c^2$; all massive particles "move at speed $c$" in spacetime
- **4-momentum**: $p^\mu = (E/c, \mathbf{p})$; dispersion relation $E^2 = (pc)^2 + (mc^2)^2$
- **Rest energy**: $E_0 = mc^2$ — mass is a form of energy
- **Invariant mass**: $s = -(p_1+p_2)^2$; equals total center-of-mass energy squared
- **4-momentum conservation**: Unifies energy and momentum conservation; single Lorentz-covariant law
- **4-force**: $f^\mu = dp^\mu/d\tau$; always perpendicular to 4-velocity ($f\cdot u = 0$)
- **Compton scattering**: Photon-electron scattering; $\lambda' - \lambda = \lambda_C(1-\cos\theta)$; direct test of SR + QM
- **Threshold energy**: Minimum energy to create particles; much lower at colliders than fixed-target

---

## Important Figures

**Albert Einstein** (1879–1955): Derived $E = mc^2$ in his 1905 companion paper; recognized the unification of mass and energy.

**Arthur Compton** (1892–1962): Measured photon-electron scattering (1923) confirming relativistic kinematics and quantization; Nobel Prize 1927.

**Paul Dirac** (1902–1984): Combined SR with quantum mechanics to produce the Dirac equation, predicting antimatter; Nobel Prize 1933.

**Enrico Fermi** (1901–1954): Applied relativistic kinematics to nuclear reactions; designed the first sustained nuclear chain reaction (1942).

---

## Further Reading

**Primary Sources**
- Einstein, A. (1905). "Ist die Trägheit eines Körpers von seinem Energieinhalt abhängig?" *Annalen der Physik*, 18, 639. [English: "Does the Inertia of a Body Depend Upon Its Energy Content?"]
- Compton, A.H. (1923). "A Quantum Theory of the Scattering of X-rays by Light Elements." *Physical Review*, 21, 483.

**Textbooks**
- Taylor, E.F. & Wheeler, J.A. (1992). *Spacetime Physics*. Freeman. — Chapters 7–8 on energy-momentum.
- Griffiths, D.J. (2008). *Introduction to Elementary Particles* (2nd ed.). Wiley. — Chapter 3 on relativistic kinematics.
- Rindler, W. (2006). *Relativity*. Oxford. — Chapters 7–9.

---

## Exercises

**25.1.** *The energy-momentum relation.*

(a) A proton has kinetic energy $K = 3m_pc^2$. Compute its total energy $E$, momentum $p$, velocity $v/c$, and Lorentz factor $\gamma$.

(b) An electron has $\gamma = 1000$. Compute $E$, $p$, and $v/c$. How does its velocity compare to $c$ (in parts per billion)?

(c) A photon has wavelength $\lambda = 500$ nm. Compute its energy in eV and its momentum in kg$\cdot$m/s. Using the de Broglie relation $p = h/\lambda$ (quantum): how does this compare to $E/c$?

---

**25.2.** *Threshold energy.*

(a) The Large Electron-Positron collider (LEP) was designed to produce $Z^0$ bosons with mass $M_Z = 91.2$ GeV/$c^2$ via $e^+ + e^- \to Z^0$. What minimum energy must each beam have?

(b) If instead you collide an electron beam on a fixed positron target, what beam energy is needed? By what factor must LEP's fixed-target equivalent energy be increased?

(c) At the LHC, proton-proton collisions at $\sqrt{s} = 14$ TeV produce Higgs bosons with $M_H = 125$ GeV/$c^2$ via $gg\to H$. In a fixed-target experiment (one proton at rest), what beam energy would be needed to reach $\sqrt{s} = 14$ TeV?

---

**25.3.** *Compton scattering.*

A photon with energy $E_0 = 0.511$ MeV (equal to $m_e c^2$) scatters off an electron at rest. It scatters through $\theta = 90°$.

(a) Find the scattered photon energy $E'$ and the electron recoil energy.

(b) Find the electron's recoil momentum direction $\phi$ (angle with original photon direction).

(c) Verify 4-momentum conservation by checking that the sum of scattered 4-momenta equals the initial total.

---

**Thought Experiment T25.1.** *Is $E = mc^2$ the right equation?*

The formula $E = mc^2$ is often stated for a particle at rest. The full formula is $E^2 = (pc)^2 + (mc^2)^2$, so $E = mc^2$ only when $p = 0$.

Some physicists write "$E = mc^2$" using "relativistic mass" $m_{\rm rel} = \gamma m$, making the formula true for all velocities. Others (the modern convention) use $m$ only for rest mass, and write $E = \gamma mc^2$.

(a) What is the conceptual problem with "relativistic mass"? (Hint: what direction does $F = m_{\rm rel}a$ fail in?)

(b) In what sense is rest mass $m$ more fundamental than $m_{\rm rel} = \gamma m$? What is a Lorentz-invariant quantity that encodes mass?

(c) Einstein himself used "relativistic mass" in some papers. Why might the modern convention (rest mass only) be better for GR? (Hint: in GR, what replaces mass as the source of gravity?)
