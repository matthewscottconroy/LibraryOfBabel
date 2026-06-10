# 10.1.2 Co-Packaged Optics

## The Problem with Pluggable Transceivers

For two decades, optical transceivers were physically separate from the switch or router chips they served: pluggable modules (SFP, QSFP, OSFP) inserted into cages on the PCB, connected to the ASIC by copper traces of 5–30 cm. This worked when data rates were 1–10 Gbps, but at 100–400 Gbps, the copper traces from pluggable transceiver to ASIC cause too much loss, require too much equalization power, and limit the achievable I/O bandwidth.

The response is **co-packaged optics (CPO)**: integrating the optical transceivers directly onto the package with the switching or compute chip, reducing the copper trace length from tens of centimeters to millimeters. This eliminates the PCB trace loss and dramatically reduces the energy and footprint of the electrical-to-optical interface.

## Architecture

A CPO system places silicon photonic transceiver chiplets directly on the same substrate as the main chip (switch ASIC, GPU, or AI accelerator). Several configurations:

**2.5D CPO**: Silicon photonic chips are placed side-by-side with the compute chip on an organic or silicon interposer. Electrical connections between them are through the interposer at pitches of ~50–100 μm.

**3D CPO**: Photonic chiplets are stacked directly on top of the compute chip using micro-bump or direct bonding. Extremely short electrical connections (< 10 μm), but requires thermally compatible processes (the compute chip generates significant heat that the photonic chiplet must tolerate).

**Monolithic integration**: Compute logic and photonic components on the same silicon die. The most elegant solution but the hardest: current CMOS processes are optimized for transistors and incompatible with photonic waveguides, especially the deep UV etch steps and the dopant profiles needed for both electronics and optics.

## State of the Art

**Broadcom Bailly (2023)**: First commercial CPO switch, integrating 51.2 Tbps switching capacity with co-packaged silicon photonic transceivers. 8 × 400G OSFP pluggable equivalent in a flat panel CPO package.

**Intel Optical Disaggregated Architecture (ODA)**: Intel's CPO research uses heterogeneous integration of III-V lasers bonded to SOI waveguides (Section 4.4.3), with Ge-on-Si photodetectors and silicon EO modulators, all co-packaged with an FPGA.

**TSMC SoIC**: TSMC's System on Integrated Chips technology allows stacking of different dies (including photonic dies) with direct copper bonding at 10-μm bump pitch. This enables 3D CPO with very low electrical interface energy.

## Energy Budget for CPO

For a CPO silicon photonic transceiver:

| Component | Energy (fJ/bit) |
|-----------|----------------|
| Modulator (Si ring) | 5–50 |
| Modulator driver | 50–200 |
| Laser (external + coupling) | 100–500 |
| Photodetector + TIA | 20–100 |
| CDR/SerDes | 200–500 |
| **Total** | **~500–1500 fJ/bit** |

This compares to ~5000–20,000 fJ/bit for a 100G pluggable transceiver connected via 20-cm PCB trace. The factor of 10–40× improvement comes primarily from eliminating the SerDes equalization needed for long copper traces, and from the reduced laser power needed when the laser is close-coupled to the modulator.

The target for next-generation CPO is ~100 fJ/bit total, which requires ~20-fJ/bit modulators (ring modulators at their energy optimum) and ~30-fJ/bit TIAs. Both are within reach with silicon photonic process improvements.

---

## References

[1] Stojanovic, V., Ram, R.J., Popovic, M., Lin, S., Moazeni, S., Wade, M., ... & Hoefler, G. (2018). "Monolithic silicon-photonic platforms in state-of-the-art CMOS SOI processes." *Optics Express*, 26(10), 13106–13121. [Electronic-photonic co-integration in standard CMOS; MIT Lincoln Labs / GlobalFoundries platform.]

[2] Sun, C., Wade, M.T., Lee, Y., Orcutt, J.S., Alloatti, L., Georgas, M.S., ... & Ram, R.J. (2015). "Single-chip microprocessor that communicates directly using light." *Nature*, 528(7583), 534–538. [The first demonstration of a microprocessor with integrated optical I/O.]
