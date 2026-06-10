# Chapter 18: Electrostatics

---

## Chapter Introduction

Electrostatics is the study of electric fields produced by static charges — charges that are fixed in space or moving slowly enough that magnetic effects are negligible. It is the foundation on which all classical electromagnetism is built, and its mathematics — Laplace's equation, Green's functions, multipole expansions — recurs throughout physics, from electrodynamics to quantum mechanics to general relativity.

The central result of electrostatics is **Coulomb's law**: the force between two point charges $q_1$ and $q_2$ separated by distance $r$ is $F = q_1 q_2/(4\pi\varepsilon_0 r^2)$. This is the electric analogue of Newton's gravitational force law. Like gravity, it is an inverse-square law — a reflection of the three-dimensionality of space and the masslessness of the photon.

But the real power of electrostatics is not Coulomb's law — it is the field concept and the **superposition principle**. Electric fields from multiple charges add linearly. This means the field of a complex charge distribution can always be computed as a sum (integral) over contributions from infinitesimal charge elements. The field concept — a physical quantity at every point in space, independent of any test charge — was Maxwell's and Faraday's great contribution: it moved physics away from "action at a distance" to local interactions mediated by fields.

---

## Coulomb's Law and the Electric Field

**Coulomb's law**: The force on a test charge $q_0$ at position $\mathbf{r}$ due to a source charge $q$ at $\mathbf{r}'$:
$$\mathbf{F} = \frac{q q_0}{4\pi\varepsilon_0}\frac{\mathbf{r}-\mathbf{r}'}{|\mathbf{r}-\mathbf{r}'|^3}$$

where $\varepsilon_0 = 8.854\times 10^{-12}$ F/m is the permittivity of free space.

The **electric field** $\mathbf{E}$ at $\mathbf{r}$ due to a source $q$ at $\mathbf{r}'$:
$$\mathbf{E}(\mathbf{r}) = \frac{q}{4\pi\varepsilon_0}\frac{\mathbf{r}-\mathbf{r}'}{|\mathbf{r}-\mathbf{r}'|^3}$$

so $\mathbf{F} = q_0\mathbf{E}$. By superposition, the field of a continuous charge distribution $\rho(\mathbf{r}')$:
$$\mathbf{E}(\mathbf{r}) = \frac{1}{4\pi\varepsilon_0}\int\frac{(\mathbf{r}-\mathbf{r}')\rho(\mathbf{r}')}{|\mathbf{r}-\mathbf{r}'|^3}d^3r'$$

---

## Gauss's Law

**Gauss's law** (integral form): The electric flux through any closed surface $S$ equals the enclosed charge divided by $\varepsilon_0$:
$$\oint_S\mathbf{E}\cdot d\mathbf{A} = \frac{Q_{\rm enc}}{\varepsilon_0}$$

**Gauss's law** (differential form): Using the divergence theorem:
$$\nabla\cdot\mathbf{E} = \frac{\rho}{\varepsilon_0}$$

This is the first of Maxwell's four equations. It is equivalent to Coulomb's law for static charges.

**Applications of Gauss's law** (by symmetry):
- Spherical symmetry ($\rho = \rho(r)$): $E(r) = Q(r)/(4\pi\varepsilon_0 r^2)$ where $Q(r)$ is the charge enclosed within radius $r$
- Infinite line charge ($\lambda$ C/m): $E = \lambda/(2\pi\varepsilon_0 r)$ (radially outward)
- Infinite plane ($\sigma$ C/m$^2$): $E = \sigma/(2\varepsilon_0)$ (normal to plane)

---

## The Scalar Potential

Since $\nabla\times\mathbf{E} = 0$ for electrostatics (no time-varying magnetic field), by the converse of the curl theorem, $\mathbf{E}$ is conservative:
$$\mathbf{E} = -\nabla\phi$$

where $\phi$ is the **electrostatic potential** (in volts). The potential energy of charge $q_0$ in the field is $U = q_0\phi$.

For a point charge: $\phi(r) = q/(4\pi\varepsilon_0 r)$.

By superposition: $\phi(\mathbf{r}) = \frac{1}{4\pi\varepsilon_0}\int\frac{\rho(\mathbf{r}')}{|\mathbf{r}-\mathbf{r}'|}d^3r'$.

Substituting $\mathbf{E} = -\nabla\phi$ into Gauss's law:
$$\nabla\cdot\mathbf{E} = -\nabla^2\phi = \frac{\rho}{\varepsilon_0}$$

$$\boxed{\nabla^2\phi = -\frac{\rho}{\varepsilon_0}}$$

This is **Poisson's equation** — the fundamental equation of electrostatics. In vacuum ($\rho = 0$): $\nabla^2\phi = 0$ — **Laplace's equation**.

---

## Boundary Conditions and Uniqueness

**Boundary conditions** at an interface between two media:
- $E_n^{(2)} - E_n^{(1)} = \sigma/\varepsilon_0$ (normal component jumps by surface charge $\sigma$)
- $E_t^{(2)} - E_t^{(1)} = 0$ (tangential component continuous)

**Uniqueness theorem**: Poisson's equation $\nabla^2\phi = -\rho/\varepsilon_0$ with boundary conditions (either $\phi$ or $\partial_n\phi$ specified on the boundary) has a unique solution. Two solutions satisfying the same BCs must be equal.

**Conductors**: Inside a perfect conductor, $\mathbf{E} = 0$ (charges rearrange to cancel any interior field). The conductor surface is an equipotential. Any excess charge resides on the surface.

---

## Multipole Expansion

For a charge distribution localized near the origin, far from the distribution ($r\gg r'$):
$$\phi(\mathbf{r}) = \frac{1}{4\pi\varepsilon_0}\left[\frac{Q}{r} + \frac{\mathbf{p}\cdot\hat{r}}{r^2} + \frac{1}{2}\frac{Q_{ij}\hat{r}^i\hat{r}^j}{r^3} + \cdots\right]$$

where:
- $Q = \int\rho\,d^3r$ (total charge — monopole)
- $\mathbf{p} = \int\mathbf{r}'\rho\,d^3r'$ (electric dipole moment)
- $Q_{ij} = \int(3r'_ir'_j - r'^2\delta_{ij})\rho\,d^3r'$ (quadrupole tensor)

**Physical significance**: The monopole term falls as $r^{-1}$, the dipole as $r^{-2}$, the quadrupole as $r^{-3}$. Higher multipoles are progressively less important at large distances. For a neutral object ($Q = 0$), the leading term is dipole; for a neutral, non-polar object, it is quadrupole.

**Gravity analogy**: The multipole expansion for the gravitational potential $\Phi = -GM/r + \cdots$ is identical in structure. The quadrupole moment of a mass distribution determines the leading tidal effects and the gravitational wave radiation (Chapter 44). The gravitational wave strain $h \propto \ddot{Q}_{ij}$ — the second time derivative of the quadrupole moment.

---

## Energy of the Electric Field

The energy stored in an electrostatic configuration:
$$U = \frac{\varepsilon_0}{2}\int|\mathbf{E}|^2\,d^3r = \frac{1}{2}\int\rho\phi\,d^3r$$

The **electrostatic energy density**: $u = \frac{\varepsilon_0}{2}E^2$.

For two point charges: $U = q_1q_2/(4\pi\varepsilon_0 r_{12})$ (the interaction energy). The self-energy of a point charge diverges — this is the classical precursor of the ultraviolet divergence in QED, resolved by renormalization.

---

## Important Concepts

- **Coulomb's law**: $\mathbf{F} = q_1q_2(\mathbf{r}-\mathbf{r}')/(4\pi\varepsilon_0|\mathbf{r}-\mathbf{r}'|^3)$
- **Superposition principle**: Electric fields add linearly
- **Gauss's law**: $\nabla\cdot\mathbf{E} = \rho/\varepsilon_0$; flux through closed surface = enclosed charge/$\varepsilon_0$
- **Scalar potential**: $\mathbf{E} = -\nabla\phi$; exists because $\nabla\times\mathbf{E} = 0$ in statics
- **Poisson's equation**: $\nabla^2\phi = -\rho/\varepsilon_0$; Laplace's equation in vacuum
- **Uniqueness theorem**: Boundary value problem for $\phi$ has a unique solution
- **Multipole expansion**: $\phi \sim Q/r + p/r^2 + Q_{ij}/r^3 + \cdots$; leading terms dominate far away
- **Electric field energy density**: $u = \varepsilon_0 E^2/2$

---

## Further Reading

- Griffiths, D.J. (2017). *Introduction to Electrodynamics* (4th ed.). Cambridge. — Chapters 2–3.
- Jackson, J.D. (1999). *Classical Electrodynamics* (3rd ed.). Wiley. — Chapters 1–2.
- Purcell, E.M. & Morin, D.J. (2013). *Electricity and Magnetism* (3rd ed.). Cambridge. — Physical emphasis.

---

## Exercises

**18.1.** *Gauss's law applications.*

(a) A sphere of radius $R$ has uniform charge density $\rho$. Find $\mathbf{E}$ everywhere (inside and outside) using Gauss's law. Express outside field in terms of total charge $Q$.

(b) An infinite cylindrical shell of radius $R$ carries surface charge $\sigma$. Find $\mathbf{E}$ everywhere.

(c) A thin spherical shell of radius $R$ carries total charge $Q$. What is the field inside? Outside?

---

**18.2.** *Poisson and Laplace.*

(a) Verify by direct differentiation that $\phi = q/(4\pi\varepsilon_0 r)$ satisfies $\nabla^2\phi = 0$ for $r > 0$.

(b) Show that $\nabla^2(1/r) = -4\pi\delta^3(\mathbf{r})$ (use Gauss's theorem on a sphere around the origin).

(c) A grounded spherical conductor of radius $R$ is placed in a uniform external field $\mathbf{E}_0 = E_0\hat{z}$. Use the uniqueness theorem to argue that the solution outside the sphere is $\phi = -E_0r\cos\theta + E_0R^3\cos\theta/r^2$. Verify it satisfies $\nabla^2\phi = 0$ and the boundary conditions.

---

**18.3.** *Multipole expansion.*

(a) A pair of charges $+q$ at $(0,0,d/2)$ and $-q$ at $(0,0,-d/2)$. Compute the monopole, dipole, and quadrupole contributions to $\phi$ far away. Which terms survive?

(b) The exact potential on the $z$-axis is $\phi(z) = q/(4\pi\varepsilon_0)\cdot[1/|z-d/2| - 1/|z+d/2|]$. Expand for $z\gg d$ and verify your multipole result.

(c) The gravitational quadrupole radiation power (Chapter 44): $P = G\dddot{Q}_{ij}\dddot{Q}^{ij}/(5c^5)$. What is the EM analogue (Larmor formula for quadrupole radiation)? By what power of $c$ does EM quadrupole radiation differ from gravitational?

---

**Thought Experiment T18.1.** *Fields vs. action at a distance.*

Before Faraday and Maxwell, Coulomb's law was understood as "action at a distance": charge $q_1$ exerts a force directly on charge $q_2$, with no intermediary. The field concept changes this: $q_1$ creates a field at every point in space; $q_2$ then responds to the field locally at its own location.

The field carries energy ($u = \varepsilon_0 E^2/2$). If charge $q_1$ is moved, the field change propagates outward at speed $c$ — information does not travel instantaneously.

Is the field "real"? A pragmatist might say: both descriptions make the same predictions for static situations. But for dynamic situations (radiation), the field is essential — it carries energy and momentum away from accelerating charges. Can you think of an experiment that distinguishes the field picture from action-at-a-distance? Is there a classical test, or must you go to quantum theory (photons)?
