# 1.2.1 Gauss's Law for Electric Fields

## The Equation

$$\oint_S \mathbf{E} \cdot d\mathbf{A} = \frac{Q_{\text{enc}}}{\varepsilon_0}$$

The left side is the total electric flux through a closed surface $S$. The right side is the total charge enclosed by $S$, divided by $\varepsilon_0$.

## Unpacking the Notation

The symbol $\oint_S$ denotes a surface integral over a *closed* surface — one that completely encloses a volume, like the surface of a sphere or a box. The integrand is the dot product $\mathbf{E} \cdot d\mathbf{A}$, where $d\mathbf{A}$ is an infinitesimal area element whose direction is the outward-pointing normal to the surface at that point.

The dot product picks out the component of $\mathbf{E}$ perpendicular to the surface. When $\mathbf{E}$ is pointing outward (the same direction as $d\mathbf{A}$), the contribution is positive. When it is pointing inward, the contribution is negative.

The integral sums up all these contributions over the entire surface, giving a number — the total electric flux — with units of V·m (volts times meters).

## Physical Meaning: The Counting Argument

Here is the most useful way to think about Gauss's law: imagine field lines. Every charge $q > 0$ emits field lines in all directions; every charge $q < 0$ absorbs them. The number of field lines emitted or absorbed is proportional to the magnitude of the charge.

Now draw any closed surface you like around some distribution of charges. Every field line that begins inside the surface must eventually cross the surface (going outward). Every field line that ends inside the surface must have crossed the surface (going inward) from outside. Field lines that begin and end outside the surface cross it twice: once inward, once outward, contributing zero net flux.

Therefore: the **total net flux** through the surface equals the total number of "outward" field lines minus "inward" field lines — which equals the total net charge inside, times a proportionality constant.

This is exactly what Gauss's law says. The choice of surface is irrelevant; the enclosed charge is what matters.

## Why the $\varepsilon_0$?

The factor $\varepsilon_0$ is the permittivity of free space. It appears because of the SI unit conventions: we define the coulomb (unit of charge), the volt (unit of electric potential), and the meter independently, and $\varepsilon_0$ is the proportionality constant that reconciles the units. In the Gaussian system of units, $\varepsilon_0$ does not appear explicitly, but the physics is identical.

Physically, $\varepsilon_0$ characterizes how "strongly" free space responds to electric fields. A larger $\varepsilon_0$ would mean that a given charge produces a weaker field. In a material medium, we replace $\varepsilon_0$ with $\varepsilon = \varepsilon_r \varepsilon_0$, where $\varepsilon_r$ is the relative permittivity (dielectric constant), which is always $\geq 1$ for ordinary materials. This is why electric fields are weaker inside dielectric materials than in vacuum — and why optical waveguides, made of materials with $\varepsilon_r > 1$, guide light by total internal reflection (as we will see in Chapter 6).

## Applications

**Symmetric configurations.** When the charge distribution has high symmetry (spherical, cylindrical, or planar), Gauss's law determines the field with almost no calculation.

*Example: Spherical shell of charge.* A thin spherical shell of radius $R$ carries total charge $Q$. By symmetry, $\mathbf{E}$ must be radial and its magnitude can depend only on $r$. Choose $S$ to be a concentric sphere of radius $r$.

- If $r > R$ (outside): $Q_{\text{enc}} = Q$, so $E \cdot 4\pi r^2 = Q/\varepsilon_0$, giving $E = Q/(4\pi\varepsilon_0 r^2)$. Outside, the shell looks like a point charge.
- If $r < R$ (inside): $Q_{\text{enc}} = 0$, so $E = 0$. There is no electric field inside a uniformly charged spherical shell.

This second result — $E = 0$ inside a spherical shell — is not obvious without Gauss's law. It follows immediately from it.

*Example: Infinite line charge.* A long wire with linear charge density $\lambda$ (charge per unit length). Choose $S$ to be a cylinder of radius $r$ and length $L$ coaxial with the wire. The field on the curved surface is radial with magnitude $E$; on the flat end caps, $\mathbf{E}$ is parallel to the surface so $\mathbf{E} \cdot d\mathbf{A} = 0$. Then $E \cdot 2\pi r L = \lambda L / \varepsilon_0$, giving $E = \lambda/(2\pi\varepsilon_0 r)$.

**Relevance to photonic computing.** The $1/r^2$ field of a point charge and the $1/r$ field of a line charge are the building blocks of the electrostatic fields that drive modulators and electro-optic devices. The PN junction in a silicon modulator creates a depletion region with a built-in electric field that changes the refractive index — an effect that depends on Gauss's law for its electrostatic analysis (Chapter 7).

## Important Subtlety: Gauss's Law vs. Coulomb's Law

Gauss's law is not equivalent to Coulomb's law in general. Gauss's law holds exactly in all situations, including time-varying fields. Coulomb's law describes the static field of a point charge and does not account for retardation (the finite propagation speed of changes in the field). For static situations, the two are equivalent. For time-varying situations, Gauss's law is the correct one.

This distinction matters: in photonic devices where the fields are rapidly oscillating (GHz to THz frequencies), the static approximation is inadequate, and Maxwell's full equations must be used.
