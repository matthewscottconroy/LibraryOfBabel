# Subsection 14.2.3: Metasurface D2NN

## Orientation

A metasurface pushes the diffractive pixel below the wavelength. Where a printed plate spends a millimetre of plastic per $2\pi$ and an SLM spends an $8\ \mu$m pixel, a metasurface imparts a full phase range with a single sub-wavelength dielectric post, collapsing each modulating layer to a flat, CMOS-compatible film about a wavelength thick. This subsection covers how meta-atoms encode phase, the enormous space-bandwidth product that follows, and the on-chip route that carries diffractive computing into an integrated waveguide.

---

## 14.2.3.1 Meta-Atoms and Sub-Wavelength Phase Control

A metasurface is a dense array of engineered scatterers — dielectric pillars, nanofins, or holes ("meta-atoms") — on a sub-wavelength grid (Yu & Capasso 2014). Each meta-atom imparts a chosen phase between $0$ and $2\pi$ through its geometry, by either of two mechanisms. In the **truncated-waveguide (resonance) phase**, a high-index pillar behaves as a short, weakly-coupled waveguide whose fundamental-mode effective index depends on its cross-section, so the transmitted phase $\phi \approx \frac{2\pi}{\lambda}\,n_\text{eff}(w)\,t$ is tuned by the pillar width $w$ at fixed height $t$. In the **geometric (Pancharatnam–Berry) phase**, an anisotropic nanofin rotated by angle $\vartheta$ imparts $\phi = 2\vartheta$ on the converted circular polarization, so phase is set purely by orientation. Either way the trainable $\phi^l(x,y)$ of Section 14.1 becomes a map of meta-atom geometry, and the layer is a lithographically patterned film rather than a bulk relief.

## 14.2.3.2 Worked Example: Space-Bandwidth at 1550 nm

Take the telecom wavelength $\lambda = 1550$ nm and a meta-atom pitch of $\lambda/2 = 775$ nm — fine enough that the array supports no unwanted propagating diffraction orders, giving smooth, alias-free wavefront control. A modest 1 mm aperture then holds

$$N = \frac{1\ \text{mm}}{775\ \text{nm}} \approx 1290$$

meta-atoms per side, or $N^2 \approx 1.6\times10^6$ trainable neurons in a single 1 mm² layer. Contrast the terahertz plate of Section 14.2.1: a 1 mm aperture there, at a 0.4 mm pitch, holds barely 2–3 neurons per side. The metasurface packs on the order of $10^5$–$10^6$ times more neurons into the same footprint — its space-bandwidth product, $(\text{aperture}/\text{pitch})^2$, is larger by exactly that ratio.

Sub-wavelength pitch buys angular reach as well as neuron count. A $\lambda/2$ pixel diffracts into essentially the full forward hemisphere (there is no diffraction null within $90^\circ$), so each meta-atom couples to a wide fan of downstream neurons — the dense, wide-angle connectivity the SLM sacrificed with its $15\lambda$ pixel. In étendue terms ($A\,\Omega$, area times solid angle), the metasurface maximizes the solid-angle factor $\Omega$ at every point, recovering the richly connected regime of the terahertz demonstrations but at a million-fold higher neuron density. The price is paid in fabrication and reconfigurability, addressed below.

## 14.2.3.3 On-Chip Integrated Diffractive Optics

The metasurfaces above still operate in free space, with air gaps between layers. A complementary route folds the whole architecture onto a photonic chip. Fu et al. (2023) confined light to a planar slab waveguide and etched the phase pattern directly into it, so that light propagates in two dimensions within the slab while etched sub-wavelength features act as the diffractive pixels; cascading such etched regions builds a multilayer diffractive network monolithically on-chip. This brings diffractive computing into the integrated photonic platform of Chapter 12 — lithographically defined, alignment-free, and compatible with on-chip sources and detectors — trading the free-space network's effortless massive parallelism for the stability and manufacturability of a waveguide circuit.

## 14.2.3.4 Trade-offs: Tolerance, Bandwidth, Fixity

Sub-wavelength control is unforgiving. A meta-atom's phase depends on nanometre-scale critical dimensions, so fabrication tolerance — electron-beam or deep-UV lithography, etch fidelity, sidewall angle — directly limits the achievable phase accuracy across a million-element layer, the analog of the quantization and fabrication errors of Section 14.4. Resonant meta-atoms are also inherently dispersive: a geometry that gives the right phase at one wavelength drifts at another, making purely resonant metasurfaces narrowband, whereas truncated-waveguide and geometric-phase designs are comparatively broadband but still bounded (multi-wavelength and dispersion-engineered metasurfaces are an active response to this limit). And like the printed plate, a metasurface is fixed at fabrication: it is ultracompact and passive, but not reconfigurable — the SLM's one advantage it cannot match.

---

## References

[1] Yu, N., & Capasso, F. (2014). "Flat optics with designer metasurfaces." *Nature Materials*, 13, 139–150. [The foundational account of sub-wavelength phase control by meta-atoms — resonance and geometric phase — underlying every metasurface D2NN.]

[2] Fu, T., Zang, Y., Huang, Y., Du, Z., Huang, H., Hu, C., Chen, M., Yang, S., & Chen, H. (2023). "Photonic machine learning with on-chip diffractive optics." *Nature Communications*, 14, 70. [The on-chip integrated diffractive network in a slab waveguide, bridging free-space D2NNs to the integrated platforms of Chapter 12.]

[3] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [The diffractive framework and terahertz baseline whose space-bandwidth product the metasurface multiplies by six orders of magnitude.]
