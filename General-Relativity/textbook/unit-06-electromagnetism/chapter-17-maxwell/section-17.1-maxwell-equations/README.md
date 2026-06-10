# Section 17.1: Maxwell's Equations

---

## Section Introduction

Four equations. Four centuries of empirical discovery, compressed into four lines of mathematics. And hidden within them: light, relativity, gauge theory, and the structure of spacetime.

We develop Maxwell's equations from their empirical bases, then rewrite them in the language of differential forms to reveal their geometric content. The geometric formulation — $dF = 0$ and $d\star F = \star J$ — is not merely more compact; it reveals that the two pairs of Maxwell equations have profoundly different characters. The first pair ($dF = 0$) is an identity — it follows from the definition of $F$ as a closed form. The second pair ($d\star F = \star J$) is the dynamical content. This distinction carries directly into GR: the Bianchi identity $\nabla_{[\mu}R_{\rho\sigma]\nu\lambda} = 0$ is the GR analog of $dF = 0$.

---

## 17.1.1 The Four Maxwell Equations

**Gauss's Law for Electricity**: Electric charges are sources (and sinks) of electric field lines.

$$\nabla \cdot \mathbf{E} = \frac{\rho}{\varepsilon_0}$$

In integral form: $\oint_S \mathbf{E}\cdot d\mathbf{A} = Q_{\rm enc}/\varepsilon_0$ (flux of $\mathbf{E}$ through a closed surface equals the enclosed charge). This is Newton's inverse-square law restated as a field equation.

**Gauss's Law for Magnetism**: There are no magnetic monopoles — magnetic field lines have no sources or sinks.

$$\nabla \cdot \mathbf{B} = 0$$

In integral form: $\oint_S \mathbf{B}\cdot d\mathbf{A} = 0$ (net magnetic flux through any closed surface is zero). Field lines of $\mathbf{B}$ are always closed loops.

**Faraday's Law**: A changing magnetic field induces a circulating electric field.

$$\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}$$

In integral form (Faraday's flux rule): $\oint_C \mathbf{E}\cdot d\boldsymbol{\ell} = -\frac{d}{dt}\int_S \mathbf{B}\cdot d\mathbf{A}$ (the EMF around a loop equals the rate of change of magnetic flux through the loop).

**Ampère-Maxwell Law**: Electric currents and changing electric fields induce a circulating magnetic field.

$$\nabla \times \mathbf{B} = \mu_0\mathbf{J} + \mu_0\varepsilon_0\frac{\partial\mathbf{E}}{\partial t}$$

Maxwell added the $\mu_0\varepsilon_0\partial_t\mathbf{E}$ term (the "displacement current"). Without it, $\nabla\cdot(\nabla\times\mathbf{B}) = 0$ but $\nabla\cdot\mathbf{J} \neq 0$ in general — violating charge conservation. With it: $0 = \nabla\cdot(\mu_0\mathbf{J} + \mu_0\varepsilon_0\partial_t\mathbf{E}) = \mu_0(\nabla\cdot\mathbf{J} + \partial_t\rho/\varepsilon_0 \cdot \varepsilon_0)$ gives the continuity equation $\partial_t\rho + \nabla\cdot\mathbf{J} = 0$ (charge conservation). □

---

## 17.1.2 The Displacement Current and Charge Conservation

**Conservation of charge**: The continuity equation $\partial_t\rho + \nabla\cdot\mathbf{J} = 0$ expresses local conservation — charge doesn't teleport. It is derivable from the Maxwell equations with the displacement current included.

*Derivation*: Take the divergence of the Ampère-Maxwell law:
$$0 = \nabla\cdot(\nabla\times\mathbf{B}) = \mu_0\nabla\cdot\mathbf{J} + \mu_0\varepsilon_0\partial_t(\nabla\cdot\mathbf{E}) = \mu_0\nabla\cdot\mathbf{J} + \mu_0\varepsilon_0\partial_t(\rho/\varepsilon_0) = \mu_0(\nabla\cdot\mathbf{J} + \partial_t\rho)$$

So $\partial_t\rho + \nabla\cdot\mathbf{J} = 0$. The displacement current $\varepsilon_0\partial_t\mathbf{E}$ is precisely what makes this work. □

**Physical picture**: A capacitor being charged: current flows in the wires but not across the gap. Yet a magnetic field loops around the gap, just as it loops around the wire. The displacement current $\varepsilon_0\partial_t\mathbf{E}$ inside the gap equals the conduction current in the wires. Faraday was able to detect the magnetic field due to the displacement current in this configuration (using a coil near the capacitor gap).

---

## 17.1.3 The Faraday Tensor

In special relativity (Unit VII), $\mathbf{E}$ and $\mathbf{B}$ are not separate fields — they are components of the **Faraday tensor** (field strength tensor), an antisymmetric rank-2 covariant tensor:

$$F_{\mu\nu} = \begin{pmatrix} 0 & E_x/c & E_y/c & E_z/c \\ -E_x/c & 0 & -B_z & B_y \\ -E_y/c & B_z & 0 & -B_x \\ -E_z/c & -B_y & B_x & 0 \end{pmatrix}$$

(with metric signature $(-,+,+,+)$ and coordinates $(t, x, y, z)$).

The Faraday tensor is a **2-form**: an antisymmetric covariant tensor of rank 2. Explicitly:

$$F = \frac{1}{2}F_{\mu\nu}dx^\mu\wedge dx^\nu = E_x(dx^1/c)\wedge dx^0 + \ldots - B_z\,dx^1\wedge dx^2 + \ldots$$

or more compactly: $F = \mathbf{E}/c\cdot(dt\wedge d\mathbf{x}) - \mathbf{B}\cdot\star(dt\wedge d\mathbf{x})$ (schematically).

The 6 independent components of $F_{\mu\nu}$ encode the 3 components of $\mathbf{E}$ and 3 of $\mathbf{B}$.

---

## 17.1.4 Maxwell's Equations in the Language of Differential Forms

In the language of differential forms on Minkowski spacetime:

**First pair of Maxwell equations** ($\nabla\cdot\mathbf{B} = 0$ and $\nabla\times\mathbf{E} = -\partial_t\mathbf{B}$):

$$dF = 0$$

This is a single equation: the Faraday 2-form is **closed** ($dF = 0$, i.e., the exterior derivative of $F$ vanishes). It is equivalent to both $\nabla\cdot\mathbf{B} = 0$ and $\nabla\times\mathbf{E} + \partial_t\mathbf{B} = 0$ simultaneously.

*Why $dF = 0$ is automatic in 3+1 dimensions*: If the 4-potential $A_\mu$ exists such that $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$ (i.e., $F = dA$), then $dF = d(dA) = 0$ identically (since $d^2 = 0$ for any form). The equation $dF = 0$ is not a constraint — it is the statement that $F$ is the curvature of the U(1) connection $A$.

However, $dF = 0$ does not guarantee $F = dA$ globally (only locally, by the Poincaré lemma). The topological obstruction is the cohomology class $[F] \in H^2(M)$. The Aharonov-Bohm effect (Section 18.1) is precisely this topological subtlety.

**Second pair of Maxwell equations** ($\nabla\cdot\mathbf{E} = \rho/\varepsilon_0$ and $\nabla\times\mathbf{B} = \mu_0\mathbf{J} + \mu_0\varepsilon_0\partial_t\mathbf{E}$):

$$d\star F = \mu_0 \star J$$

where $\star$ is the Hodge star (mapping $k$-forms to $(4-k)$-forms in 4D). $\star F$ is the dual 2-form with components $(\star F)_{\mu\nu} = \frac{1}{2}\varepsilon_{\mu\nu\rho\sigma}F^{\rho\sigma}$. The current 1-form is $J = J^\mu\partial_\mu$, and $\star J$ is its Hodge dual 3-form.

Equivalently, in index notation: $\partial_\mu F^{\mu\nu} = \mu_0 J^\nu$ (where indices are raised with the Minkowski metric $\eta^{\mu\nu}$).

The **current density 4-vector** is $J^\mu = (c\rho, J^x, J^y, J^z)$, and charge conservation $\partial_t\rho + \nabla\cdot\mathbf{J} = 0$ is $\partial_\mu J^\mu = 0$ — a consequence of $d(\star J) = 0$ (which follows from $d(d\star F) = 0$ since $d^2 = 0$).

**Summary**:

| Geometric form | Component form | Physical content |
|---------------|----------------|-----------------|
| $dF = 0$ | $\partial_{[\alpha}F_{\beta\gamma]} = 0$ | No magnetic monopoles; Faraday's law |
| $d\star F = \mu_0\star J$ | $\partial_\mu F^{\mu\nu} = \mu_0 J^\nu$ | Gauss's law; Ampère-Maxwell |

---

## 17.1.5 Electromagnetic Duality

The Maxwell equations in vacuum ($J^\mu = 0$) have a remarkable symmetry: they are invariant under the **dual transformation** $F \to \star F$ (i.e., $\mathbf{E} \to c\mathbf{B}$, $c\mathbf{B} \to -\mathbf{E}$, or in 4D notation, $F_{\mu\nu} \to \tilde F_{\mu\nu} = \frac{1}{2}\varepsilon_{\mu\nu\rho\sigma}F^{\rho\sigma}$).

In vacuum: $dF = 0$ and $d\tilde F = 0$ are both satisfied by both $F$ and $\tilde F$. This electromagnetic duality interchanges electric and magnetic fields.

**Magnetic monopoles**: If magnetic monopoles existed (Dirac, 1931), the Maxwell equations would be fully symmetric: $d\tilde F = \mu_0 \star J_m$ (magnetic current). The lack of observed magnetic monopoles breaks this symmetry in nature, but the Dirac quantization condition $qg = n\hbar c/2$ (for electric charge $q$ and magnetic charge $g$) implies that if even one monopole exists, all electric charges are quantized — one possible explanation for the observed quantization of charge.

**S-duality in string theory**: Electromagnetic duality is related to the S-duality of string theory (strong-weak coupling duality), which exchanges electrically charged particles with magnetically charged ones (monopoles). This suggests a deep connection between Maxwell's classical duality and quantum gravity.

---

## 17.1.6 Maxwell's Equations in Curved Spacetime

In curved spacetime with metric $g_{\mu\nu}$, the Maxwell equations generalize by replacing partial derivatives with covariant derivatives:

$$\nabla_\mu F^{\mu\nu} = \mu_0 J^\nu, \qquad \nabla_{[\alpha}F_{\beta\gamma]} = 0$$

But since $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu = \nabla_\mu A_\nu - \nabla_\nu A_\mu$ (the Christoffel symbols cancel in the antisymmetric combination), the second equation is automatically satisfied when $F = dA$. The first becomes:

$$\frac{1}{\sqrt{-g}}\partial_\mu(\sqrt{-g}F^{\mu\nu}) = \mu_0 J^\nu$$

(using the identity $\nabla_\mu V^\mu = (1/\sqrt{-g})\partial_\mu(\sqrt{-g}V^\mu)$ for any vector $V^\mu$).

This is the equation of motion for electromagnetic fields in a gravitational background — used, for example, to compute the electromagnetic radiation from a charged particle orbiting a black hole, or the photon propagation in the early universe.

**The electromagnetic stress-energy tensor** (source of gravity):

$$T^{\mu\nu}_{\rm EM} = \frac{1}{\mu_0}\left(F^{\mu\alpha}F^\nu_{\ \alpha} - \frac{1}{4}g^{\mu\nu}F_{\alpha\beta}F^{\alpha\beta}\right)$$

This tensor is traceless ($g_{\mu\nu}T^{\mu\nu}_{\rm EM} = 0$) and satisfies $\nabla_\mu T^{\mu\nu}_{\rm EM} = -F^{\nu\mu}J_\mu$ (work done by the field on the charges). In vacuum ($J^\mu = 0$): $\nabla_\mu T^{\mu\nu}_{\rm EM} = 0$ — electromagnetic energy-momentum is locally conserved.

---

## References

- Maxwell, J.C. (1865). "A dynamical theory of the electromagnetic field." *Philosophical Transactions of the Royal Society*, 155, 459–512. [The paper containing all four Maxwell equations, including the displacement current, and the derivation that electromagnetic waves travel at speed $c$. One of the most important papers in physics.]
- Faraday, M. (1831). "On the induction of electric currents." *Philosophical Transactions of the Royal Society*, 122, 125–162. [The discovery of electromagnetic induction: the experimental basis for Faraday's law. Also introduces the concept of "lines of force" — field lines.]
- Dirac, P.A.M. (1931). "Quantised singularities in the electromagnetic field." *Proceedings of the Royal Society A*, 133, 60–72. [Proposes magnetic monopoles and derives the Dirac quantization condition $qg = n\hbar c/2$. Magnetic monopoles would explain the observed quantization of electric charge.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation.* W.H. Freeman. [§3.4: Maxwell's equations as $dF = 0$, $d\star F = 4\pi\star J$ (Gaussian units). The treatment of EM in the language of forms and curved spacetime.]
- Frankel, T. (2011). *The Geometry of Physics*, 3rd ed. Cambridge University Press. [Chapter 2: differential forms; Chapter 7: the Maxwell equations as geometric statements. The clearest treatment of the geometric formulation.]
