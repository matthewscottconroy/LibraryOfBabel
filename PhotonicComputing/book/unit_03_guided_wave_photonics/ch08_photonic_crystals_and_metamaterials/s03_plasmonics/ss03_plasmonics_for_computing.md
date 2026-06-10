# 8.3.3 Plasmonics for Photonic Computing: Honest Assessment

## The Promise and the Physics

Around 2005–2015, there was substantial enthusiasm in the plasmonic computing community. The argument was compelling: plasmons confine light below the diffraction limit, potentially enabling optical interconnects at the scale of electronic transistors (10–100 nm). If the speed of light can carry information at the scale of silicon transistors, the fundamental advantages of optics (bandwidth, parallelism, lack of capacitive loading) could be brought to the chip scale.

This argument collides with one hard physical fact: ohmic loss. In the previous subsections, we derived that gold SPPs propagate ~50 μm at 1550 nm, silver ~300 μm. For a chip with millimeter-scale interconnect distances, this means the signal would be attenuated by $e^{-10^{-3}/50\times10^{-6}} \approx e^{-20}$, reducing it to ~$2 \times 10^{-9}$ of its original intensity — complete signal loss.

This is not an engineering problem waiting for a better material; it is a reflection of the Kramers-Kronig relations applied to metals. Any material with a strong negative real dielectric function at optical frequencies (necessary for tight confinement) also has substantial imaginary dielectric function (absorption). Reducing loss requires reducing confinement, and reducing confinement eliminates the plasmonic advantage.

The most loss-optimized plasmonic waveguide reported to date is the **long-range SPP (LRSPP)**: a thin metal film (10–20 nm) in a symmetric dielectric environment. The mode extends micrometers into the dielectric, losing most of the confinement advantage, but achieves $L_{\text{SPP}} \approx$ 1–3 mm at 1550 nm [1]. This is useful for fiber-to-chip coupling and on-chip routing at millimeter scales, but not for nanoscale interconnects.

## Where Plasmonics Genuinely Contributes

Despite the loss limitation, plasmonics has genuine and important roles in photonic systems, including photonic computing:

### 1. Electro-Optic Modulators with Nanoscale Active Regions

The most successful application of plasmonics to optical communications is not as a waveguide but as a **modulator active region**. The concept: use a plasmonic slot waveguide (two metal rails with a narrow gap) as the electrode structure of an electro-optic modulator. The gap can be 100–200 nm wide, concentrating both the optical field and the electric field in the same nanoscale volume.

The Leuthold group at ETH Zürich has pioneered this approach using organic electro-optic (OEO) materials with large $r_{33}$ (~200 pm/V) deposited in the metallic slot:

- Electrode gap: 100 nm
- Modulator length: 10–20 μm (much shorter than Si MZI!)
- $V_\pi L$: ~0.005 V·mm (200× better than silicon!)
- Bandwidth: >170 GHz
- Insertion loss: 6–12 dB (from plasmonic absorption)

The tradeoff: 10 dB insertion loss per modulator is acceptable if the device provides 200 GHz bandwidth in a 10-μm footprint. For ultra-compact, ultra-high-speed modulation — such as in a short-reach optical interconnect from a GPU memory bank — this could be valuable [2].

### 2. Sub-Wavelength Photodetectors

Plasmonic nanoantennas integrated with semiconductor absorbers can enhance photodetector responsivity while reducing the active area. This is directly relevant to photonic computing: smaller photodetectors have lower capacitance, enabling higher bandwidth with the same TIA noise:

$$f_{\text{RC}} = \frac{1}{2\pi R_F C_j} \propto \frac{1}{C_j} \propto \frac{1}{A_{\text{det}}}$$

For a plasmonic-enhanced Ge photodetector with antenna cross-section $\sigma = 0.5$ μm² (10× the geometric area for a 200 nm × 250 nm detector):
- Physical area: 0.05 μm²
- Effective area: ~0.5 μm² (with antenna enhancement)
- Capacitance: ~1 fF (vs. 50 fF for a conventional 5 μm × 5 μm detector)
- RC bandwidth with 50 Ω: 3.2 THz (vs. 64 GHz for conventional)

This 50× bandwidth enhancement through detector miniaturization (with plasmonic area enhancement to maintain responsivity) is a genuine engineering advantage [3].

### 3. Near-Field Coupling for Chip-to-Chip Interconnects

In systems where two chips must exchange optical signals across a gap of 10–50 nm (e.g., in 3D chip stacking), plasmonics can bridge the sub-wavelength gap more efficiently than diffractive or evanescent coupling of dielectric modes. Gap plasmons couple efficiently to dielectric modes via mode-conversion structures [4].

### 4. Field-Enhanced Nonlinear Optics for Optical Logic

The field enhancement in plasmonic gaps ($|E/E_0|^2 \approx 10^3$–$10^6$) can enable nonlinear optical interactions at very low intensities. For a Kerr material ($n_2 > 0$) in a plasmonic gap with $|E/E_0|^2 = 10^4$, the effective nonlinear coefficient is enhanced by $10^4$. The self-phase modulation threshold drops by $10^4$, potentially enabling all-optical switching at sub-fJ energies.

Experimental demonstrations of all-optical switching in plasmonic structures have been reported at the sub-pJ energy level [5]. Whether this can be developed into a practical photonic computing element is an open question.

## The Loss Problem Cannot Be Engineered Away

To be clear about what cannot be changed: the loss in metals at optical frequencies is an intrinsic property of the electronic structure. The optical conductivity of metals near the plasma frequency is dominated by intraband (Drude) and interband transitions, both of which are dissipative.

Proposed alternatives to noble metals:
- **Alternative plasmonic materials** (graphene, TiN, ITO): These have $\varepsilon'_m < 0$ in certain frequency ranges, but their losses are comparable to or worse than gold at 1550 nm.
- **Superconductors**: Below $T_c$, the Drude scattering rate drops dramatically. Niobium at 4 K has much lower THz losses than at room temperature. But at optical frequencies ($\hbar\omega > 2\Delta_{\text{SC}}$ for optical photons), the Cooper pair binding energy is exceeded and normal-state losses return.
- **Epsilon-near-zero (ENZ) materials**: Materials with $|\varepsilon'| \ll 1$ support unusual modes, but they are not the same as SPP modes and do not provide sub-diffraction confinement.

The conclusion, which the plasmonic computing community has largely accepted since ~2015: plasmonics is most useful at the interface between nanoscale electronics and microscale photonics — in modulators with nanoscale gaps, detectors with enhanced absorption in small volumes, and near-field coupling in dense chip stacking. Long-distance plasmonic interconnects at optical frequencies are not viable.

## Where the Field Stands

By 2024, the realistic applications of plasmonics in photonic computing are:

| Application | Status | Genuine value |
|-------------|--------|---------------|
| OEO slot modulator | Demonstrated, not yet commercial | 200+ GHz in 20 μm; highest bandwidth/footprint |
| Plasmonic photodetector | Early demonstrations | Sub-fF capacitance, > 1 THz bandwidth |
| Plasmonic SERS sensor | Commercial products | Not directly computing-relevant |
| Plasmonic interconnect (long) | Not viable | Ohmic loss too high |
| Plasmonic all-optical switch | Research stage | Lowest switching energy demonstrated |
| Near-field chip coupling | Research stage | Relevant for 3D stacked photonics |

The lesson from plasmonics for photonic computing is a general one: extraordinary physical phenomena that look revolutionary on the bench often encounter practical engineering constraints that limit their system-level impact. Understanding where those constraints come from — in this case, the Kramers-Kronig relations and the fundamental connection between metallic conduction and optical absorption — is what allows honest evaluation of the technology.

---

## References

[1] Berini, P. (2009). "Long-range surface plasmon polaritons." *Advances in Optics and Photonics*, 1(3), 484–588. [Comprehensive review of LRSPPs; propagation lengths of millimeters demonstrated.]

[2] Haffner, C., Heni, W., Fedoryshyn, Y., Niegemann, J., Melikyan, A., Elder, D.L., ... & Leuthold, J. (2015). "All-plasmonic Mach–Zehnder modulator enabling optical high-speed communication at the microscale." *Nature Photonics*, 9(8), 525–528. [OEO plasmonic modulator: $V_\pi L = 0.004$ V·mm, >100 GHz bandwidth, 12 μm length.]

[3] Alavirad, M., Kovacs, A., Roy, L., & Bhattacharyya, P. (2016). "Schottky-contact plasmonic metal-semiconductor nano-antennas for high-responsivity small-area photodetectors." *Optics Express*, 24(8), 22544–22554. [Plasmonic nanoantenna-enhanced photodetector with sub-μm² active area.]

[4] Nielsen, M.P., Lafone, L., Rakovich, A., Sidiropoulos, T.P.H., Rahmani, M., Maier, S.A., & Oulton, R.F. (2016). "Adiabatic nanofocusing in hybrid gap plasmon waveguides on the silicon-on-insulator platform." *Nano Letters*, 16(2), 1410–1414. [Near-field coupling from silicon photonics to plasmonic gap modes.]

[5] Neira, A.D., Olivier, N., Nasir, M.E., Dickson, W., Wurtz, G.A., & Zayats, A.V. (2015). "Eliminating material constraints for nonlinearity with plasmonic metamaterials." *Nature Communications*, 6, 7757. [Sub-pJ all-optical switching in plasmonic structures via enhanced nonlinearity.]
