# 8.3.2 Sub-Wavelength Confinement and Field Enhancement

## Gap Plasmons

The flat-interface SPP described in Section 8.3.1 achieves modest confinement (micrometer-scale in the dielectric). Much stronger confinement is possible in **gap plasmons**: SPP modes in a thin dielectric gap between two metal surfaces.

Consider two parallel metal surfaces separated by a gap of thickness $g$ with dielectric filling (index $n_d$). Each metal surface supports an SPP; when the gap is narrow enough ($g \ll \lambda$), the two SPPs couple and form symmetric and antisymmetric superpositions.

The symmetric gap plasmon (both surface charges oscillating in phase) is the **gap SPP**: its electric field has a strong component normal to the metal surfaces ($E_z$), concentrated in the gap. For $g \ll \lambda$, the gap SPP dispersion can be solved analytically [1]:

$$\tanh\left(\frac{\kappa_d g}{2}\right) = -\frac{\varepsilon_d \kappa_m}{\varepsilon_m \kappa_d}$$

The effective mode index of the gap SPP is much larger than $n_d$, meaning the field is confined to much smaller than a wavelength. For $g = 10$ nm, gold walls, $n_d = 1$:

$$n_{\text{eff}}^{\text{gap}} \approx \sqrt{\varepsilon_d}\left(1 + \frac{2|\varepsilon_d|}{\varepsilon_m'' g \kappa_0}\right) \sim 30\text{–}100$$

Confinement of $\lambda/n_{\text{eff}} \approx 1550/50 \approx 31$ nm — smaller than the gap itself. This is the remarkable feature of gap plasmons: the mode can be smaller than the gap because the field is enhanced within it.

## Field Enhancement in Metallic Nanogaps

The electric field intensity in a metallic nanogap can be dramatically enhanced above the incident field. For a gap of width $g$ between two metallic surfaces with $|\varepsilon_m| \gg \varepsilon_d$, the boundary conditions require continuity of $D_n = \varepsilon E_n$:

$$\varepsilon_m E_m^n = \varepsilon_d E_d^n$$

$$E_d^n = \frac{\varepsilon_m}{\varepsilon_d} E_m^n \approx \frac{|\varepsilon_m|}{\varepsilon_d} E_m^n$$

For gold at 1550 nm: $|\varepsilon_m|/\varepsilon_d \approx 114$ — the normal electric field in the gap is 114× larger than in the metal. For light incident on a gold nanogap structure, the enhancement of the field intensity in the gap can reach $10^3$–$10^6$ (field enhancement of 30–1000×) depending on the gap geometry and resonance conditions.

This extreme field enhancement has direct applications:

### Surface-Enhanced Raman Spectroscopy (SERS)

The Raman scattering cross-section scales as $|E|^4$ (both excitation and emission fields are enhanced). For gap enhancement $|E|/|E_0| = 30$, the Raman signal is enhanced by $30^4 = 810{,}000$ — enabling single-molecule detection. SERS is perhaps the most mature application of plasmonics, with commercial sensors available [2].

### Nonlinear Optics at Extremely Low Power

The second-order nonlinear susceptibility produces SHG with efficiency $\eta \propto |E|^4 d_{\text{eff}}^2 L^2$ (for field enhancement $|E/E_0| = F$ in length $L$). Gap plasmons with $F = 30$ and $L = 100$ nm provide an effective interaction length enhancement of $F^4 \times L = 810{,}000 \times 100$ nm = 0.081 m·equivalent, competitive with macroscopic crystals. Demonstrations of SHG at single-photon level in plasmonic nanogaps have been reported [3].

### Sub-Diffraction Photodetection

A photodetector integrated with a metallic nanogap or nanoantenna can absorb photons in an active region smaller than $\lambda^2/4n^2$. For a 30-nm-gap germanium photodetector:
- Active area: $(30 \text{ nm})^2 = 900$ nm² — 10,000× smaller than a diffraction-limited Si detector
- Field enhancement: ~30–100× in the gap
- Enhanced responsivity from: (1) geometric cross-section enhancement (antenna effect), and (2) field enhancement within the active material

Sub-diffraction plasmonic photodetectors have been demonstrated with bandwidth > 1 THz (transit time through 30-nm gap at $v_s \approx 10^5$ m/s: $\tau \approx 300$ fs, $f_T \approx 0.45/\tau \approx 1.5$ THz) [4].

## Localized Surface Plasmon Resonances (LSPRs)

Metal nanoparticles support localized surface plasmon resonances (LSPRs): collective oscillation of conduction electrons that can be driven resonantly by incoming light. Unlike propagating SPPs on flat surfaces, LSPRs are spatially confined to the nanoparticle and do not propagate.

The resonance frequency of a small spherical metal nanoparticle (Mie theory, dipole approximation for $r \ll \lambda$):

$$\omega_{\text{LSPR}} \approx \frac{\omega_p}{\sqrt{1 + 2\varepsilon_{\text{med}}}}$$

For gold in air ($\varepsilon_{\text{med}} = 1$): $\omega_{\text{LSPR}} = \omega_p/\sqrt{3} = 7.9 \times 10^{15}$ rad/s, corresponding to $\lambda_{\text{LSPR}} \approx 240$ nm — ultraviolet.

In practice, gold nanoparticles resonate in the visible (520–600 nm) due to interband transitions not captured by the Drude model. Silver nanoparticles resonate at 400–450 nm.

For anisotropic shapes (nanorods, nanostars), the LSPR red-shifts toward longer wavelengths. Gold nanorods with aspect ratio ~5 resonate near 900 nm; with aspect ratio ~10 near 1550 nm. This is the basis for gold nanorod-based optical antennas at telecom wavelengths.

## Plasmonic Nanoantennas

A plasmonic nanoantenna couples propagating light (far-field) to localized near-field energy. The analogy to radio frequency antennas is direct: an RF dipole antenna with length $\lambda/2$ resonates and concentrates the electromagnetic field at its gap; a plasmonic optical antenna with length $\lambda_{\text{eff}}/2$ (where $\lambda_{\text{eff}} < \lambda$ due to the high $n_{\text{eff}}$) does the same.

Key properties of gold dipole nanoantennas at 1550 nm:
- Physical length at resonance: ~400 nm (shorter than $\lambda/2$ in free space due to $n_{\text{eff}} > 1$)
- Near-field enhancement in gap: $|E/E_0| \approx 10$–100
- Far-field coupling efficiency ("radiation efficiency"): 10–40% (rest lost to absorption)
- Bandwidth: ~200 nm (determined by the LSPR linewidth)

Nanoantennas have been used to enhance the coupling of waveguide modes to photodetectors, reducing the detector area while maintaining high absorption efficiency — a strategy for combining plasmonic concentration with semiconductor detection.

---

## References

[1] Takahara, J., Yamagishi, S., Taki, H., Morimoto, A., & Kobayashi, T. (1997). "Guiding of a one-dimensional optical beam with nanometer diameter." *Optics Letters*, 22(7), 475–477. [Theoretical analysis of gap plasmon modes; early prediction of sub-diffraction optical guiding.]

[2] Nie, S. & Emory, S.R. (1997). "Probing single molecules and single nanoparticles by surface-enhanced Raman scattering." *Science*, 275(5303), 1102–1106. [Single-molecule SERS demonstration; establishes the $|E|^4$ field enhancement mechanism.]

[3] Bozhevolnyi, S.I. & Søndergaard, T. (2007). "General properties of slow-plasmon resonant nanostructures: nano-antennas and resonators." *Optics Express*, 15(17), 10869–10877. [Theoretical treatment of gap plasmon resonances and field enhancement for nonlinear optics.]

[4] Salamin, Y., Heni, W., Haffner, C., Fedoryshyn, Y., Hoessbacher, C., Bonjour, R., ... & Leuthold, J. (2018). "Direct conversion of free space millimeter waves to optical domain by plasmonic modulator antenna." *Nano Letters*, 18(2), 1331–1338. [Plasmonic photodetector with >1 THz bandwidth via nanoscale active area.]
