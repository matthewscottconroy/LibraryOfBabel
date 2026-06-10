# 7.4.1 Thermo-Optic Switching

## The Thermo-Optic Effect in Silicon

The thermo-optic effect is the simplest physical mechanism for controlling the phase of light in a waveguide. Temperature changes the refractive index; refractive index changes shift the phase of light propagating through the waveguide. In silicon, the thermo-optic coefficient is:

$$\frac{dn}{dT} = 1.87 \times 10^{-4} \text{ K}^{-1} \quad (1550 \text{ nm})$$

This value arises from two contributions: the thermal expansion of the silicon lattice (which changes the lattice constant and thus the bandgap), and the Kramers-Kronig-coupled change in the real part of the index that follows from the shift in the absorption edge. For silicon at room temperature, the dominant contribution is the electronic (band structure) term, not the thermal expansion term.

For a waveguide of length $L$ heated by $\Delta T$ degrees above ambient:

$$\Delta\phi = \frac{2\pi}{\lambda} \frac{dn}{dT} \Delta T \cdot L$$

For a $\pi$ phase shift at 1550 nm over $L = 100$ μm:

$$\Delta T_\pi = \frac{\lambda}{2 \frac{dn}{dT} L} = \frac{1550 \times 10^{-9}}{2 \times 1.87 \times 10^{-4} \times 100 \times 10^{-6}} \approx 41 \text{ K}$$

Over $L = 500$ μm: $\Delta T_\pi \approx 8.3$ K. Over $L = 1$ mm: $\Delta T_\pi \approx 4.1$ K.

A temperature change of 4–41 K above ambient is readily achieved with a local microheater consuming 5–40 mW. This makes thermo-optic phase shifting technically straightforward — but the power consumption is the key limitation for large-scale photonic computing.

## Microheater Geometries

The microheater is a resistive element placed in close proximity to the optical waveguide. The goal is to maximize the temperature rise in the waveguide per unit of electrical power. Several geometries are in use:

### Titanium Nitride (TiN) Strip Heater

The most common geometry uses a strip of TiN (sheet resistance ~30–80 Ω/□) deposited directly above the waveguide in the inter-metal dielectric layer. TiN is preferred over aluminum (too reflective, absorbs at 1550 nm if close to mode) and tungsten (too resistive for long contacts). Typical dimensions: 2 μm wide × 100 nm thick, placed 500 nm above the waveguide top surface.

The thermal resistance between heater and waveguide determines the $\Delta T$ per unit power. For a heater directly above a buried waveguide in SiO₂ cladding:

$$R_{th} = \frac{d_{ox}}{k_{\text{SiO}_2} A_{\text{eff}}} \approx \frac{0.5 \times 10^{-6}}{1.4 \times 1 \times 10^{-6}} \approx 0.36 \text{ K/μW·μm}$$

where $d_{ox} \approx 500$ nm is the oxide thickness between heater and waveguide, $k_{\text{SiO}_2} = 1.4$ W/(m·K) is the oxide thermal conductivity, and $A_{\text{eff}}$ is the effective area per unit length. For a 100-μm-long heater with $P = 10$ mW:

$$\Delta T \approx 10 \times 10^{-3} \text{ W} \times 0.36 \times 10^6 \text{ K/W} \times 10^{-6} \text{ m} / \text{(length correction)} \approx 20\text{–}40 \text{ K}$$

The exact value depends on 3D heat flow modeling; typical measured efficiencies are 1–4 K/mW for strip heaters above a suspended or cladded waveguide [1].

### Doped Silicon Heater

An alternative to metal heaters is to dope the silicon itself: ion-implanted silicon with $N_D \approx 10^{18}$ cm⁻³ has sheet resistance ~10–100 Ω/□ and can serve as a resistive heater. The advantage is that the heat is generated inside the silicon layer, immediately adjacent to the waveguide. The disadvantage is that free carriers in the doped silicon also absorb the waveguide mode (from the Soref-Bennett equations: $\Delta\alpha = 8.5 \times 10^{-18} \times 10^{18} \approx 0.9$ cm⁻¹ at 1550 nm — significant).

To avoid the absorption, the doped heater must be placed laterally adjacent to the waveguide, not directly below or above it. Typical offset: 1–3 μm from the waveguide edge. This reduces thermal efficiency but avoids absorption loss.

### Suspended Waveguide Heater

The most power-efficient thermo-optic phase shifters thermally isolate the waveguide from the substrate by suspending it in air. The silicon waveguide is released by undercutting the BOX oxide with HF vapor or wet etching, creating a freestanding silicon wire. With the substrate thermal path removed, the heat generated in the waveguide itself (via doped heater or TiN above) is confined to the thin waveguide, achieving much higher temperature rise per watt.

Suspended Si thermo-optic phase shifters have demonstrated $P_\pi \approx 0.5$–2 mW — 10–20× improvement over substrate-attached waveguides — at the cost of mechanical fragility and process complexity [2].

## Thermal Dynamics and Bandwidth

The thermal response time of a thermo-optic switch is set by the heat capacity and thermal resistance of the heated region. For a silicon waveguide of dimensions 450 × 220 nm and length 100 μm:

$$\tau_{th} = R_{th} C_{th}$$

where $C_{th} = \rho c_p V = 2330 \times 700 \times (450 \times 220 \times 100 \times 10^{-21}) \approx 7.3 \times 10^{-12}$ J/K is the heat capacity, and $R_{th}$ is the thermal resistance to the substrate.

For a typical silicon-on-insulator waveguide with 2 μm BOX layer:

$$\tau_{th} \approx R_{th} C_{th} \approx 10^4 \text{ K/W} \times 7.3 \times 10^{-12} \text{ J/K} \approx 73 \text{ ns}$$

Actually, the thermal time constant for a microscale heater involves both the heater volume *and* the substrate heat sink. For realistic device geometries, the thermal response time measured experimentally is typically 1–10 μs for strip heaters on standard SOI, and 10–100 μs for large-area heaters [3].

This corresponds to switching rates of 0.1–1 MHz — adequate for many reconfigurable photonic computing applications (weight update at ~100 kHz–1 MHz) but far too slow for GHz-rate optical switching.

The thermal bandwidth can be increased by using shorter heaters (reduced $C_{th}$) and closer substrate proximity (reduced $R_{th}$), but these trade off against efficiency. For suspended waveguides with reduced $R_{th}$ to substrate, the thermal time constant can be reduced to ~100 ns.

## Power-Phase Shift Relations

The relationship between heater power and phase shift depends on the heater geometry and thermal isolation. For a standard TiN strip heater on 220-nm SOI with 2 μm BOX:

$$\Delta\phi \approx K_\phi \times P$$

where $K_\phi$ is typically 50–100 mrad/mW for 100-μm interaction length, or equivalently:

$$P_\pi = \frac{\pi}{K_\phi} \approx 30\text{–}60 \text{ mW}$$

For optimized heater designs with partial thermal isolation:

$$P_\pi \approx 10\text{–}20 \text{ mW}$$

For suspended waveguides:

$$P_\pi \approx 1\text{–}5 \text{ mW}$$

State-of-art thermo-optic phase shifters achieve $P_\pi \approx 0.5$ mW in full suspended designs [2], though at the cost of considerable process complexity.

## Heating-Induced Crosstalk

In a dense photonic computing chip, neighboring waveguides are typically spaced 5–10 μm apart. When one heater is activated, the temperature rise is not perfectly localized — heat diffuses through the silicon and oxide layers, raising the temperature (and thus phase) of adjacent waveguides.

The thermal crosstalk between waveguides separated by distance $d$ in a uniform SiO₂ cladding is approximately:

$$\frac{\Delta T_{\text{cross}}}{\Delta T_{\text{self}}} \approx \frac{1}{1 + (d/d_0)^2}$$

where $d_0$ is a characteristic diffusion length that depends on the geometry and boundary conditions. For silicon photonics geometries, $d_0 \approx 10$–20 μm, so waveguides at 10-μm spacing experience ~50% crosstalk — a severe problem.

Mitigation strategies include:
1. **Thermal isolation trenches**: Etching through the BOX and into the substrate between waveguides creates air gaps that impede lateral heat flow. Demonstrated 10× crosstalk reduction [4].
2. **Differential heating**: Using adjacent pairs of heaters driven in a push-pull configuration, so that the temperature *difference* (which drives the phase) is localized, while the common-mode temperature rise cancels.
3. **Sparse placement**: Placing heaters only on widely spaced phase elements, with passive interconnects routing signals between heated sections.

For large photonic meshes ($N > 16$), thermal management is a first-order design constraint, not an afterthought.

## Thermo-Optic Tuning in MZI Meshes: System-Level View

In a programmable MZI mesh used for photonic matrix-vector multiplication (Section 7.2.4), each MZI contains one or two thermo-optic phase shifters. The complete power budget for an $N \times N$ mesh:

$$P_{\text{total}} = N^2 \times \bar{P}_\phi$$

where $\bar{P}_\phi$ is the average power per phase element for the current matrix setting. For a random unitary matrix, the expected average is approximately $P_\pi/2$.

For $N = 16$, $P_\pi = 20$ mW: $P_{\text{total}} = 256 \times 10 = 2.56$ W. This is already approaching the thermal limit for a 1–2 cm² chip.

For $N = 32$: $P_{\text{total}} = 1024 \times 10 = 10.24$ W.

This scaling is unsustainable. The thermo-optic mesh approach faces a hard wall at moderate $N$. This motivates the MEMS and PCM approaches in the following subsections, and also explains why much of the photonic computing community focuses on electro-optic (plasma dispersion or Pockels) modulators for weight programming — they can, in principle, hold state with much lower static power than thermal approaches.

---

## References

[1] Harris, N.C., Ma, Y., Mower, J., Baehr-Jones, T., Englund, D., Hochberg, M., & Galland, C. (2014). "Efficient, compact and low loss thermo-optic phase shifter in silicon." *Optics Express*, 22(9), 10487–10493. [Characterization of TiN heater geometry and efficiency on SOI; measured 36 mW for π shift, 3 μs response time.]

[2] Watts, M.R., Sun, J., DeRose, C., Trotter, D.C., Young, R.W., & Vawter, G.A. (2013). "Adiabatic thermo-optic Mach-Zehnder switch." *Optics Letters*, 38(5), 733–735. [Suspended waveguide thermo-optic switch with 1.1 mW for π shift.]

[3] Dong, P., Preble, S.F., & Lipson, M. (2007). "All-optical compact silicon comb switch." *Optics Express*, 15(15), 9600–9605. [Thermal time constants measured for various Si heater geometries.]

[4] Milanizadeh, M., Safaee Rad, S., Zanettini, M., Morichetti, F., Melloni, A. (2021). "Separating thermal crosstalk in a programmable photonic circuit." *Journal of Lightwave Technology*, 39(13), 4476–4484. [Thermal crosstalk characterization and mitigation in programmable photonic meshes.]
