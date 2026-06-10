# Chapter 17: Maxwell's Equations

---

## Chapter Introduction

James Clerk Maxwell did not discover electricity. He did not discover magnetism. He did not even discover most of the laws that bear his name. What Maxwell did was unprecedented: he looked at four separate empirical laws (Gauss's law for electricity, Gauss's law for magnetism, Faraday's law of induction, and Ampère's circuital law), found that they were inconsistent as a set of partial differential equations, added a single new term to repair the inconsistency, and in doing so predicted the existence of electromagnetic waves traveling at the speed of light.

The term he added — the "displacement current" $\varepsilon_0 \partial\mathbf{E}/\partial t$ — had no direct experimental basis in 1865. He added it because without it, the equations did not conserve charge. This is an example of theoretical physics at its best: using mathematical consistency as a guide to physical reality. When Hertz experimentally confirmed electromagnetic waves in 1888, twenty-two years after Maxwell's death, the theory was completely vindicated.

Maxwell's equations unify electricity and magnetism into a single theory. They contain light — light is electromagnetic waves. They imply special relativity. They are a U(1) gauge theory and the template for all of modern particle physics. And when written in the language of differential forms, they prefigure GR itself.

---

## Chapter Contents

- **Section 17.1**: Maxwell's equations in integral and differential form; the Faraday 2-form; conservation of charge; the displacement current; electromagnetic duality

- **Section 17.2**: Electromagnetic waves; dispersion relation; polarization; energy (Poynting vector); momentum and radiation pressure; the electromagnetic stress-energy tensor $T^{\mu\nu}$; radiation from accelerating charges

---

## Maxwell's Equations: Preview

The four Maxwell equations in SI units, in differential form:

| Equation | Name | Meaning |
|----------|------|---------|
| $\nabla \cdot \mathbf{E} = \rho/\varepsilon_0$ | Gauss's law | Electric charges are sources of $\mathbf{E}$ |
| $\nabla \cdot \mathbf{B} = 0$ | Gauss's law (magnetic) | No magnetic monopoles |
| $\nabla \times \mathbf{E} = -\partial_t\mathbf{B}$ | Faraday's law | Changing $\mathbf{B}$ induces $\mathbf{E}$ |
| $\nabla \times \mathbf{B} = \mu_0\mathbf{J} + \mu_0\varepsilon_0\partial_t\mathbf{E}$ | Ampère-Maxwell | Currents and changing $\mathbf{E}$ induce $\mathbf{B}$ |

The four equations contain physics of extraordinary depth. From charge conservation to the existence of light to the structure of spacetime — all of it follows, once you know how to look.

The relativistic form — in which $\mathbf{E}$ and $\mathbf{B}$ are two aspects of the Faraday tensor $F_{\mu\nu}$ — reduces the four equations to two: $\partial_\nu F^{\mu\nu} = \mu_0 J^\mu$ and $\partial_{[\alpha}F_{\beta\gamma]} = 0$.

In differential form language (Section 17.1): $dF = 0$ and $d\star F = \star J$ — arguably the most compact and beautiful encoding of electrodynamics.

---

## Historical Background

**1820**: Ørsted discovers that electric current deflects a compass needle — electricity and magnetism are related. Ampère immediately extends this: parallel currents attract, anti-parallel currents repel.

**1831**: Faraday discovers electromagnetic induction — a changing magnetic field induces an EMF. Also discovers that a changing magnetic flux through a loop drives a current. The concept of "field lines" is Faraday's invention; Maxwell gave it mathematical form.

**1855–1865**: Maxwell develops the mathematical theory of the electromagnetic field, building on Faraday's physical intuition. His four papers culminate in *A Dynamical Theory of the Electromagnetic Field* (1865), containing the displacement current and the prediction of electromagnetic waves.

**1888**: Hertz generates and detects radio waves (wavelength ~66 cm) in his laboratory, confirming Maxwell's prediction. The speed of propagation matches $c = 1/\sqrt{\mu_0\varepsilon_0}$.

**1905**: Einstein, in "Zur Elektrodynamik bewegter Körper," shows that Maxwell's equations require a new kinematics — special relativity.

The story of electromagnetism is one of the great intellectual achievements of the 19th century.
