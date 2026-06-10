# 1.3.3 Maxwell's Equations in Matter

## Why Matter Changes Things

So far we have written Maxwell's equations for free space — vacuum — with only "free" charges and currents as sources. But in photonic computing, light propagates inside silicon waveguides, optical fibers, lithium niobate modulators, and semiconductor lasers. All of these are materials, and materials respond to electromagnetic fields in ways that modify how the fields behave.

The key physical processes are:
1. **Electric polarization**: Atoms in a dielectric distort under an applied electric field — the electron cloud shifts relative to the nucleus, creating small electric dipoles. These dipoles contribute to the effective field.
2. **Magnetization**: Atoms with magnetic moments align under an applied magnetic field. (For most optical materials, magnetization effects are negligible at optical frequencies.)
3. **Free carrier effects**: Conductors and semiconductors have free electrons that respond to electric fields by flowing, creating currents.

## Macroscopic Maxwell's Equations

Rather than tracking every atomic dipole, we describe the response of matter through macroscopic averages:

- The **electric polarization** $\mathbf{P}$ (dipole moment per unit volume, C/m²)
- The **magnetization** $\mathbf{M}$ (magnetic moment per unit volume, A/m)
- The **free charge density** $\rho_f$ and **free current density** $\mathbf{J}_f$

We define two auxiliary fields:
$$\mathbf{D} = \varepsilon_0 \mathbf{E} + \mathbf{P}$$
$$\mathbf{H} = \frac{\mathbf{B}}{\mu_0} - \mathbf{M}$$

**$\mathbf{D}$** (electric displacement, C/m²) and **$\mathbf{H}$** (magnetic field intensity, A/m) absorb the material response into their definitions.

Maxwell's equations in matter then take the form:

$$\nabla \cdot \mathbf{D} = \rho_f$$

$$\nabla \cdot \mathbf{B} = 0$$

$$\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}$$

$$\nabla \times \mathbf{H} = \mathbf{J}_f + \frac{\partial \mathbf{D}}{\partial t}$$

These equations are exact — no approximations have been made — but they are not complete until we specify the **constitutive relations** connecting $\mathbf{D}$ to $\mathbf{E}$ and $\mathbf{B}$ to $\mathbf{H}$ for a given material.

## Linear Isotropic Dielectrics

For most optical materials of interest (glass, silicon, lithium niobate at moderate field strengths), the polarization is proportional to and in the same direction as the applied electric field:

$$\mathbf{P} = \varepsilon_0 \chi_e \mathbf{E}$$

where $\chi_e$ is the **electric susceptibility** (dimensionless). Then:

$$\mathbf{D} = \varepsilon_0(1 + \chi_e)\mathbf{E} = \varepsilon_0 \varepsilon_r \mathbf{E} = \varepsilon \mathbf{E}$$

where $\varepsilon_r = 1 + \chi_e$ is the **relative permittivity** (dielectric constant) and $\varepsilon = \varepsilon_0 \varepsilon_r$.

Similarly, for non-magnetic materials (all common optical materials at optical frequencies):
$$\mathbf{B} = \mu_0 \mathbf{H}$$

so $\mu_r = 1$.

The wave speed in such a material is:
$$v = \frac{1}{\sqrt{\varepsilon\mu_0}} = \frac{c}{\sqrt{\varepsilon_r}} = \frac{c}{n}$$

where $n = \sqrt{\varepsilon_r}$ is the **refractive index**. This is the fundamental relationship between the refractive index and the material's electric polarizability.

**Why does silicon have such a high refractive index?** Silicon has $\varepsilon_r \approx 12$ at 1550 nm (below the bandgap energy), giving $n \approx 3.47$. The high $\varepsilon_r$ arises because the silicon lattice has strongly polarizable covalent bonds — the shared electron density responds strongly to an applied electric field. This high refractive index is precisely what makes silicon so useful for photonic waveguides: it provides a large index contrast with the surrounding SiO₂ cladding ($n \approx 1.44$), enabling tight optical confinement in small waveguides [1].

**Dispersion.** In general, $\varepsilon_r$ (and therefore $n$) depends on frequency: $n = n(\omega)$. This is called dispersion, and it arises from the resonant response of the atomic oscillators in the material (Chapter 3). Dispersion is responsible for the fact that different wavelengths travel at different speeds in a medium — which is what causes pulse broadening in fiber-optic communications and must be carefully managed in photonic computing systems.

## Nonlinear Dielectrics

For sufficiently intense fields, the linear relationship $\mathbf{P} = \varepsilon_0 \chi^{(1)} \mathbf{E}$ breaks down. The polarization must be expanded as:

$$\mathbf{P} = \varepsilon_0 \left[\chi^{(1)}\mathbf{E} + \chi^{(2)}\mathbf{E}^2 + \chi^{(3)}\mathbf{E}^3 + \cdots\right]$$

The higher-order terms ($\chi^{(2)}$, $\chi^{(3)}$, ...) describe nonlinear optical effects: second-harmonic generation, the optical Kerr effect, four-wave mixing, self-phase modulation. These effects are the basis of all-optical switching, optical frequency combs, and many quantum photonic processes. They are treated in detail in Chapter 3.

---

## References

[1] Soref, R.A., & Bennett, B.R. (1987). "Electrooptical effects in silicon." *IEEE Journal of Quantum Electronics*, 23(1), 123–129. [The foundational paper establishing the optical properties of silicon for photonics.]

[2] Griffiths, D.J. (2017). *Introduction to Electrodynamics*, 4th ed. Cambridge University Press. Ch. 4. [Clear treatment of macroscopic Maxwell equations and constitutive relations.]
