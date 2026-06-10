# Chapter 8: Important Concepts

---

## 1. The Photonic Bandgap

A photonic crystal — a periodic arrangement of dielectric materials — has a photonic band structure analogous to the electronic band structure of a semiconductor. When the periodic structure has sufficient dielectric contrast, a **photonic bandgap** opens: a range of frequencies for which no propagating modes exist in any direction. Light at these frequencies is reflected (in a 3D crystal) or cannot propagate in-plane (in a 2D slab). The bandgap width scales as $\Delta\omega/\omega \approx (4/\pi)\Delta n/n_{\text{avg}}$ for small index contrast. For Si/air with $n_{\text{Si}} = 3.478$, the bandgap can exceed 100% of the center frequency.

---

## 2. Bloch's Theorem and Photonic Bands

In a periodic dielectric $\varepsilon(\mathbf{r}) = \varepsilon(\mathbf{r}+\mathbf{R})$, the master equation $\hat{\Theta}\mathbf{H} = (\omega/c)^2\mathbf{H}$ has eigenmodes $\mathbf{H}_{n\mathbf{k}}(\mathbf{r}) = e^{i\mathbf{k}\cdot\mathbf{r}}\mathbf{u}_{n\mathbf{k}}(\mathbf{r})$ where $\mathbf{u}_{n\mathbf{k}}$ is periodic with the lattice. The operator $\hat{\Theta}$ is Hermitian (for lossless dielectrics), guaranteeing real eigenvalues and orthogonal modes. The photonic band structure $\omega_n(\mathbf{k})$ is fully analogous to the electronic band structure $E_n(\mathbf{k})$. The scale invariance of Maxwell's equations means the band structure in dimensionless units $\omega a/c$ vs. $ka/2\pi$ scales to any wavelength by rescaling the lattice constant $a$.

---

## 3. Photonic Crystal Cavities and the Purcell Factor

A defect in a 2D photonic crystal slab (missing holes, or a line of missing holes) creates a localized cavity mode within the bandgap. The combination of ultra-high Q factor ($Q > 10^6$ for heterostructure cavities) and ultra-small mode volume ($V \sim (\lambda/n)^3$) gives a Purcell factor $F_P = (3/4\pi^2)(\lambda/n)^3(Q/V) \sim 10^4$–$10^5$ — the highest achievable in solid-state photonics. This enables: ultra-low threshold nanolasers, fast single-photon emitters, and nonlinear optical effects at single-photon level.

---

## 4. Slow Light and the $S^2$ Enhancement

Near the edge of a photonic crystal waveguide band, the group velocity $v_g = d\omega/dk \to 0$. The slow-down factor $S = c/v_g$ enhances both the effective interaction length ($S$×) and the field intensity ($S$×), giving a total nonlinear phase shift enhancement of $S^2$. However, disorder-induced backscattering loss also scales as $S^2$, largely canceling the enhancement in disorder-limited waveguides. Practical slow-light devices operate at $S \approx 10$–30, achieving modest enhancement over useful bandwidths. The key application is compact electro-optic modulators with $V_\pi L < 1$ V·mm using PN junctions in slow-light PCW.

---

## 5. The Generalized Snell's Law and Metasurfaces

A surface that imparts a spatially varying phase gradient $d\phi/dx$ redirects transmitted light according to the generalized Snell's law: $n_2\sin\theta_t - n_1\sin\theta_i = (\lambda/2\pi)(d\phi/dx)$. A **metasurface** implements an arbitrary phase function $\phi(x,y)$ by varying the geometry of sub-wavelength meta-atoms across the surface. Phase control mechanisms include: resonant phase (0 to $\pi$ per resonance), propagation phase (pillar height/width), and geometric (Pancharatnam-Berry) phase. The best dielectric metasurfaces achieve 80–95% transmission efficiency.

---

## 6. The Pancharatnam-Berry Phase

Rotating an anisotropic half-wave plate element by angle $\alpha$ imparts a phase of $\pm 2\alpha$ to the two circular polarization components of incident light, independent of wavelength. This **geometric phase** depends only on the topology of the polarization path on the Poincaré sphere (solid angle subtended), not on any material resonance. PB phase metasurfaces use identical meta-atoms at varying orientations, providing: full $2\pi$ phase coverage with single-etch fabrication, broadband operation (hundreds of nm), and high uniformity. The limitation: requires circular polarization input and converts polarization state.

---

## 7. D²NN: Diffractive Deep Neural Networks

A diffractive neural network (D²NN) implements optical computation as light passes through multiple layers of diffractive metasurfaces separated by free-space propagation. Each layer applies a local phase modulation; diffraction between layers provides the non-local mixing equivalent to fully connected neural network layers. Training by backpropagation through a differentiable diffraction model (angular spectrum method). Demonstrated accuracy: 91–93% on MNIST digit classification. Critical limitation: **passive diffraction is linear** — a stack of linear operations equals one linear operation. Without optical nonlinearity, D²NNs cannot match the expressive power of deep nonlinear neural networks.

---

## 8. The SPP Dispersion Relation

Surface plasmon polaritons (SPPs) exist at metal-dielectric interfaces where $\varepsilon_m < -\varepsilon_d$. The SPP wavevector $k_{\text{SPP}} = (\omega/c)\sqrt{\varepsilon_m\varepsilon_d/(\varepsilon_m+\varepsilon_d)}$ exceeds the free-space wavevector, making the SPP a surface-confined mode. At 1550 nm: gold SPPs propagate ~50 μm, silver ~300 μm — far too short for chip-scale interconnects. Tighter confinement reduces propagation length further (the fundamental confinement-loss tradeoff).

---

## 9. The Confinement-Loss Tradeoff in Plasmonics

All plasmonic waveguides face a fundamental tradeoff: tighter confinement (smaller $A_{\text{eff}}$) requires larger $|k_{\text{SPP}}|/k_0$, which requires larger $|\varepsilon_m|$, which from Kramers-Kronig relations implies larger $\varepsilon_m''$ (absorption). The figure of merit $\text{FOM} = L_{\text{SPP}}/\lambda \approx |\varepsilon_m'|^2/\varepsilon_m''$ cannot be improved by material choice within the class of metals. This makes long-distance plasmonic interconnects physically nonviable.

---

## 10. Genuine Roles for Plasmonics in Computing

Despite the loss problem, plasmonics has genuine applications at the boundary between nanoscale electronics and photonics:
- **OEO slot modulators**: 10–20 μm long, $V_\pi L < 0.01$ V·mm, >200 GHz bandwidth (ETH Zürich group).
- **Sub-wavelength photodetectors**: Active area < 0.1 μm², capacitance < 1 fF, bandwidth > 1 THz via plasmonic antenna enhancement.
- **Near-field chip coupling**: Gap plasmons for sub-100-nm chip-to-chip optical coupling in 3D stacked photonics.
These are niche but genuine advantages where the confinement benefit outweighs the loss cost because the device is short.
