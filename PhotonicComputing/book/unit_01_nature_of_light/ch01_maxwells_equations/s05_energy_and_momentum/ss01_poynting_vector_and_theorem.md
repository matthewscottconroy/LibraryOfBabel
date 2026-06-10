# 1.5.1 The Poynting Vector and Energy Conservation

## The Energy Density of the Electromagnetic Field

The electromagnetic field stores energy in both its electric and magnetic components. For a linear dielectric, the energy density (energy per unit volume, J/m³) is:

$$u = \frac{1}{2}\varepsilon_0 E^2 + \frac{1}{2\mu_0}B^2 = \frac{\varepsilon_0}{2}E^2 + \frac{1}{2\mu_0}B^2$$

For a plane wave in vacuum, with $B = E/c$ and $\varepsilon_0 c^2 = 1/\mu_0$:

$$u = \frac{\varepsilon_0}{2}E^2 + \frac{E^2}{2\mu_0 c^2} = \varepsilon_0 E^2$$

The electric and magnetic energy densities are equal in a plane wave — a beautiful symmetry.

## Deriving the Poynting Theorem

We want to derive an energy conservation equation — a statement that the rate of change of electromagnetic energy in a volume equals the power put in by sources minus the power flowing out.

Start with the Ampère-Maxwell equation:
$$\mathbf{J} = \frac{1}{\mu_0}(\nabla \times \mathbf{B}) - \varepsilon_0\frac{\partial\mathbf{E}}{\partial t}$$

Take the dot product with $\mathbf{E}$:
$$\mathbf{E} \cdot \mathbf{J} = \frac{\mathbf{E}\cdot(\nabla\times\mathbf{B})}{\mu_0} - \varepsilon_0\mathbf{E}\cdot\frac{\partial\mathbf{E}}{\partial t}$$

The term $\mathbf{E}\cdot\partial\mathbf{E}/\partial t = \frac{1}{2}\partial E^2/\partial t$.

Use the vector identity $\nabla\cdot(\mathbf{E}\times\mathbf{B}) = \mathbf{B}\cdot(\nabla\times\mathbf{E}) - \mathbf{E}\cdot(\nabla\times\mathbf{B})$. Then:

$$\mathbf{E}\cdot(\nabla\times\mathbf{B}) = \mathbf{B}\cdot(\nabla\times\mathbf{E}) - \nabla\cdot(\mathbf{E}\times\mathbf{B})$$

Substituting Faraday's law $\nabla\times\mathbf{E} = -\partial\mathbf{B}/\partial t$:

$$\mathbf{E}\cdot(\nabla\times\mathbf{B}) = -\mathbf{B}\cdot\frac{\partial\mathbf{B}}{\partial t} - \nabla\cdot(\mathbf{E}\times\mathbf{B}) = -\frac{1}{2}\frac{\partial B^2}{\partial t} - \nabla\cdot(\mathbf{E}\times\mathbf{B})$$

Collecting all terms:

$$\mathbf{E}\cdot\mathbf{J} = -\frac{\varepsilon_0}{2}\frac{\partial E^2}{\partial t} - \frac{1}{2\mu_0}\frac{\partial B^2}{\partial t} - \frac{1}{\mu_0}\nabla\cdot(\mathbf{E}\times\mathbf{B})$$

Rearranging:

$$\boxed{\frac{\partial u}{\partial t} + \nabla\cdot\mathbf{S} = -\mathbf{E}\cdot\mathbf{J}}$$

where $u = \frac{\varepsilon_0}{2}E^2 + \frac{1}{2\mu_0}B^2$ is the electromagnetic energy density and

$$\mathbf{S} = \frac{1}{\mu_0}(\mathbf{E}\times\mathbf{B}) = \mathbf{E}\times\mathbf{H}$$

is the **Poynting vector** [1].

## Physical Interpretation of the Poynting Theorem

The equation $\partial u/\partial t + \nabla\cdot\mathbf{S} = -\mathbf{E}\cdot\mathbf{J}$ is an energy conservation equation:

- $\partial u/\partial t$: rate of change of electromagnetic energy per unit volume
- $\nabla\cdot\mathbf{S}$: rate of energy flowing *out* of a unit volume (divergence of energy flux)
- $-\mathbf{E}\cdot\mathbf{J}$: rate at which the field does work on charges (power delivered to matter per unit volume)

In integral form (using the divergence theorem over volume $V$ bounded by surface $S$):

$$\frac{d}{dt}\int_V u\,dV = -\oint_S \mathbf{S}\cdot d\mathbf{A} - \int_V \mathbf{E}\cdot\mathbf{J}\,dV$$

The left side is the rate of increase of electromagnetic energy inside $V$. The first term on the right is the net energy inflow through the surface. The second term is the power delivered to currents (lost to Ohmic heating in resistors, or gained by sources like generators).

The Poynting vector $\mathbf{S}$ is the **energy flux density** of the electromagnetic field — the power per unit area flowing in the direction of $\mathbf{S}$, with units W/m².

## The Poynting Vector of a Plane Wave

For a plane wave $\mathbf{E} = E_0\cos(\mathbf{k}\cdot\mathbf{r}-\omega t)\hat{\mathbf{x}}$ in vacuum with $\mathbf{B} = (E_0/c)\cos(\mathbf{k}\cdot\mathbf{r}-\omega t)\hat{\mathbf{y}}$ (taking $\mathbf{k}$ along $\hat{\mathbf{z}}$):

$$\mathbf{S} = \frac{1}{\mu_0}\mathbf{E}\times\mathbf{B} = \frac{E_0^2}{\mu_0 c}\cos^2(\mathbf{k}\cdot\mathbf{r}-\omega t)\hat{\mathbf{z}}$$

Time-averaging: $\langle\cos^2\rangle = 1/2$, so:

$$\langle\mathbf{S}\rangle = \frac{E_0^2}{2\mu_0 c}\hat{\mathbf{z}} = \frac{\varepsilon_0 c}{2}E_0^2\hat{\mathbf{z}}$$

This is the **intensity** (irradiance) of the wave: $I = \langle|\mathbf{S}|\rangle = \frac{1}{2}\varepsilon_0 c E_0^2$ [W/m²].

**Worked example**: A 1 mW laser beam with $\lambda = 1550$ nm is focused to a spot of radius $w = 5\ \mu\text{m}$ (Gaussian waist). The intensity at the center:

$$I = \frac{P}{\pi w^2} = \frac{10^{-3}}{\pi \times (5\times10^{-6})^2} \approx 1.27 \times 10^7\ \text{W/m}^2$$

The electric field amplitude:
$$E_0 = \sqrt{\frac{2I}{\varepsilon_0 c}} = \sqrt{\frac{2\times1.27\times10^7}{8.854\times10^{-12}\times3\times10^8}} \approx 3.1 \times 10^6\ \text{V/m}$$

This is a substantial field — comparable to the breakdown field of air at ~3 MV/m. Pulsed lasers focused to small spots can easily reach breakdown.

---

## References

[1] Poynting, J.H. (1884). "On the transfer of energy in the electromagnetic field." *Philosophical Transactions of the Royal Society of London*, 175, 343–361. [The original paper introducing the energy flux vector.]
