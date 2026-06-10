# 4.4.3 Heterogeneously Integrated III-V-on-Silicon Lasers

## The On-Chip Laser Problem

Silicon photonics offers the promise of integrating complete photonic computing circuits on a single CMOS chip: waveguides, modulators, ring resonators, MZI networks, and photodetectors, all fabricated in the same foundry run. Everything except the laser.

Silicon cannot lase. Its indirect bandgap means that electrons and holes recombine predominantly by emitting phonons, not photons. The radiative efficiency of bulk silicon is approximately $10^{-6}$, making it essentially useless as a gain medium. Germanium can be strained and doped to achieve near-direct-gap behavior at 1550 nm, and Ge-on-Si lasers have been demonstrated, but with threshold current densities orders of magnitude higher than III-V lasers and without room-temperature cw operation that approaches practical utility [1].

This leaves a fundamental gap: silicon photonic computing requires laser sources, but silicon cannot produce them natively. The solution, which has been the subject of intense research since ~2005, is to bring III-V gain material (InP, GaAs, AlGaAs) to the silicon platform, either by:

1. **Hybrid integration**: Coupling a separately packaged III-V laser chip to the silicon photonic chip via fiber or lens
2. **Flip-chip bonding**: Bonding a pre-fabricated III-V laser die onto the silicon chip with sub-micron alignment
3. **Heterogeneous integration (direct wafer bonding)**: Bonding an unpatterned or pre-patterned III-V wafer directly onto the silicon photonic wafer and then patterning laser structures using standard lithography

The heterogeneous integration approach — primarily developed at UCSB by Bowers et al. from 2006 onward — is the current state of the art for foundry-compatible on-chip lasers.

## Heterogeneous Integration: Physics and Process

In the UCSB/Intel approach, InP-based quantum well layers are wafer-bonded to a silicon-on-insulator (SOI) wafer using oxygen plasma-activated hydrophilic bonding [2]. The bonding occurs at the interface between the III-V material and the SiO₂ surface of the SOI wafer, with no adhesive. After bonding, the InP substrate is removed by selective wet etching, leaving a thin InP membrane with quantum well layers on top of the SOI waveguide layer.

Laser structures (DFB gratings, ridge waveguides, contacts) are then patterned using e-beam or deep-UV lithography on the III-V material. Light is generated in the III-V gain region and evanescently coupled into the underlying silicon waveguide through the thin SiO₂ bonding layer.

**Key physics — evanescent coupling**: The optical mode of the hybrid III-V/Si waveguide has its electric field maximum in the III-V region (for gain) but also extends into the silicon layer. The confinement factor $\Gamma_{III-V}$ determines what fraction of the mode interacts with the gain medium. For a typical hybrid laser: $\Gamma_{III-V} \approx 0.1$–0.3, while $\Gamma_{Si} \approx 0.4$–0.6.

## Performance and Integration Density

Heterogeneously integrated III-V/Si lasers achieve:

| Parameter | State of art (2023) | Requirement for photonic computing |
|---|---|---|
| Threshold current | 5–30 mA | < 10 mA preferred |
| Output power into Si waveguide | 1–10 mW | > 1 mW |
| Linewidth | 1–5 MHz | < 10 MHz (for coherent computing) |
| Operating temperature | 0–70°C | 0–50°C (uncooled operation targeted) |
| Fabrication | 200 mm wafer-scale | CMOS-compatible foundry process |

Intel Photonics has used heterogeneous integration in products since ~2016 (silicon photonic transceivers for 100G and 400G Ethernet). Foundries including IMEC (Belgium) and CEA-LETI (France) offer heterogeneous integration processes via MPW services.

## Remaining Challenges

1. **Thermal management**: III-V gain material on SiO₂ (low thermal conductivity, ~1 W/m·K) has poor heat dissipation. High injection currents increase junction temperature, shifting wavelength, reducing efficiency, and accelerating degradation. Thermal vias and buried heat spreaders partially mitigate this.

2. **Reliability**: III-V semiconductors on silicon have higher defect densities than native-substrate devices, due to lattice mismatch at the bonded interface. Long-term reliability (lifetime > $10^5$ hours required for telecom qualification) is still being established for wafer-bonded devices.

3. **Yield and integration density**: Bonding and lithography yield across full 200 mm wafers is below that of all-silicon processes. Integration of many (> 10) lasers on a single chip with high yield is still challenging.

4. **On-chip laser coherence**: Most on-chip lasers are Fabry-Pérot or simple DFB; high-performance narrow-linewidth lasers (needed for coherent photonic matrix processors) require complex cavity designs (spiral resonators, feedback tuning) that are still early in integration maturity.

## Alternative Approaches

**III-V epitaxial growth on silicon**: Direct epitaxial growth of III-V on Si (e.g., InAs quantum dots on Si) avoids the wafer bonding step. InAs/Si quantum dot lasers, pioneered by groups at UCSB, UCL, and Huawei, have achieved room-temperature cw operation with threshold currents competitive with bonded devices [3]. Threading dislocation densities have been reduced below $10^6$ cm$^{-2}$ — acceptable for some applications though still much higher than on native InP substrates.

**Erbium-doped waveguide amplifiers and lasers**: Si₃N₄ waveguides doped with erbium (Er³⁺) can be pumped optically at 980 nm to provide gain at 1550 nm. Er:waveguide amplifiers (EDWA) have been demonstrated in Si₃N₄ with gain > 10 dB [4]. While not yet competitive with III-V lasers in output power, they offer CMOS-compatible fabrication with no III-V wafer bonding — a potentially simpler integration path for low-power applications.

## References

[1] Liu, J., Sun, X., Camacho-Aguilera, R., Kimerling, L.C., & Michel, J. (2010). "Ge-on-Si laser operating at room temperature." *Optics Letters*, 35(5), 679–681.

[2] Fang, A.W., Park, H., Cohen, O., Jones, R., Paniccia, M.J., & Bowers, J.E. (2006). "Electrically pumped hybrid AlGaInAs-silicon evanescent laser." *Optics Express*, 14(20), 9203–9210.

[3] Liu, A.Y., et al. (2014). "High performance continuous wave 1.3 μm quantum dot lasers on silicon." *Applied Physics Letters*, 104(4), 041104.

[4] Belt, M., & Blumenthal, D.J. (2014). "Erbium-doped waveguide DBR and DFB laser arrays integrated within an ultra-low-loss Si₃N₄ platform." *Optics Express*, 22(9), 10655–10660.
