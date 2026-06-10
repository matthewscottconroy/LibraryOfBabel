# Chapter 20: Relativistic Mechanics and the Stress-Energy Tensor

---

## Chapter Introduction

Special relativity does not merely modify kinematics — it transforms all of mechanics. Momentum, energy, and force must be redefined so that the laws of mechanics are valid in all inertial frames (Lorentz-covariant). The result is a framework built on 4-vectors: the 4-momentum $p^\mu = (E/c, \mathbf{p})$, the 4-velocity $u^\mu = \gamma(c, \mathbf{v})$, the 4-force $f^\mu$. The energy-momentum relation $E^2 = p^2c^2 + m^2c^4$ unifies energy and momentum as components of a single 4-vector.

The most important object in this chapter is the **stress-energy tensor** $T^{\mu\nu}$: a symmetric rank-2 tensor that encodes the density and flux of energy and momentum. This is the object that appears on the right-hand side of the Einstein equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}$ — the source of all gravitational fields. Understanding $T^{\mu\nu}$ is essential for GR.

---

## Chapter Contents

- **Section 20.1**: The 4-velocity, 4-momentum, and 4-force; the energy-momentum relation; mass-energy equivalence; relativistic dynamics; Compton scattering and threshold phenomena

- **Section 20.2**: The stress-energy tensor $T^{\mu\nu}$; physical interpretation; conservation law $\partial_\mu T^{\mu\nu} = 0$; dust, perfect fluid, electromagnetic field; the stress-energy tensor in curved spacetime ($\nabla_\mu T^{\mu\nu} = 0$)

---

## $E = mc^2$

The famous formula appears in a September 1905 paper by Einstein: "Does the inertia of a body depend upon its energy content?" (*Annalen der Physik*, 18, 639–641). The argument: if a body emits radiation of energy $L$, its mass decreases by $L/c^2$. The full formula $E = \gamma mc^2$ (rest plus kinetic energy) follows from the 4-momentum formalism.

The significance goes far deeper than atomic bombs: all mass is energy. The mass of a proton is $938$ MeV/$c^2$, but the rest mass of its constituent quarks contributes only $\sim 11$ MeV. The remaining $\sim 927$ MeV comes from the kinetic energy of the quarks and the energy of the gluon field — the strong interaction binding energy. Binding energy has gravitational mass. The sun loses $4.3 \times 10^9$ kg/s to radiation — this is the $\Delta m = \Delta E/c^2$ from nuclear fusion.

---

## The Stress-Energy Tensor: Preview

The stress-energy tensor $T^{\mu\nu}$ has a $4\times 4$ array of components, symmetric ($T^{\mu\nu} = T^{\nu\mu}$):

$$T^{\mu\nu} = \begin{pmatrix} T^{00} & T^{01} & T^{02} & T^{03} \\ T^{10} & T^{11} & T^{12} & T^{13} \\ T^{20} & T^{21} & T^{22} & T^{23} \\ T^{30} & T^{31} & T^{32} & T^{33} \end{pmatrix}$$

Physical meaning:
- $T^{00}$ = energy density
- $T^{0i} = T^{i0}$ = energy flux in $i$-direction = momentum density $\times c^2$
- $T^{ij}$ = flux of $i$-momentum in $j$-direction = stress tensor

Conservation law: $\partial_\mu T^{\mu\nu} = 0$ (for $\nu = 0$: energy conservation; for $\nu = i$: momentum conservation). In curved spacetime: $\nabla_\mu T^{\mu\nu} = 0$.

This is the right-hand side of the Einstein equations. The entire causal structure of GR — black holes, gravitational waves, cosmological expansion — is driven by the stress-energy content of matter.
