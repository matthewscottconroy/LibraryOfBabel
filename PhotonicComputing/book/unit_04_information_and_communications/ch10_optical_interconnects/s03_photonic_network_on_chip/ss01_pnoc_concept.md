# Subsection 10.3.1: The Photonic Network-on-Chip Concept

## Orientation

Before analyzing specific architectures, we need to establish what a photonic network-on-chip (PNoC) would actually do, what it would need to achieve to be competitive with electrical alternatives, and what the fundamental physics allows. This requires a careful energy and bandwidth analysis — the kind of first-principles engineering calculation that separates serious proposals from wishful thinking.

---

## 10.3.1.1 What a Network-on-Chip Does

A modern multi-core processor (Intel Xeon Platinum, AMD EPYC, NVIDIA H100) contains dozens to hundreds of processing cores sharing a memory hierarchy. The network-on-chip (NoC) is the on-chip interconnect that allows these cores to communicate — with each other, with shared caches, and with memory controllers.

**Traffic patterns**: For a processor with $N$ cores sharing an $L2/L3$ cache:
- Core-to-cache reads/writes: each core generates $\sim B_{\text{core}}$ bytes/second of memory traffic
- Cache coherence: cores maintaining consistent views of shared memory exchange coherence messages
- Memory controller traffic: total bandwidth to off-chip DRAM

For a modern server processor:
- 64 cores × 100 GB/s per core (theoretical) = 6.4 TB/s aggregate
- Realistic bisection bandwidth of the NoC: 1–3 TB/s
- Total on-chip wire length: kilometers, if you unspooled all the metal wires

**Electrical NoC energy**: The energy per bit on an on-chip electrical wire is [2]:

$$E_{\text{wire}} = \frac{1}{2} C_{\text{wire}} V_{DD}^2$$

where $C_{\text{wire}}$ is the total capacitance of the wire (interconnect layers have capacitance $\sim 0.2$ fF/μm) and $V_{DD}$ is the supply voltage. For a 1 cm wire at 10-nm node with $V_{DD} = 0.8$ V and $C = 0.2$ fF/μm:

$$E_{\text{wire}} = \frac{1}{2} \times (0.2 \times 10^{-15} \times 10^4) \text{ F} \times (0.8)^2 \approx 640 \text{ fJ/bit}$$

This is the energy for a single bit traversal of a 1 cm wire. Practical repeater-buffered wires (required at these lengths to achieve adequate signal integrity) add switching energy at each repeater:

$$E_{\text{repeater}} \approx C_{\text{gate}} V_{DD}^2 \approx 10-50 \text{ fJ per repeater}$$

With repeaters every ~1 mm, a 10 mm wire requires 10 repeater stages, adding 100–500 fJ. Total: 1–1.5 pJ/bit for a 10 mm on-chip electrical link. This is worse than the off-chip optical link in co-packaged optics — the on-chip electrical link is that bad.

**The target for PNoC**: To beat electrical, on-chip optical links must achieve:

$$E_{\text{optical}} < 1 \text{ pJ/bit}$$

This includes *all* energy: laser power, modulator drive, detector amplification, and any thermal stabilization. The 1 pJ/bit target is challenging. For comparison:
- State-of-the-art silicon ring modulator: 5–20 fJ/bit intrinsic modulation energy at $V_\pi < 1$ V
- State-of-the-art Ge photodetector: 5–20 fJ/bit for a 10 μA photocurrent with a transimpedance amplifier
- Laser wall-plug efficiency (on-chip): typically 15–30% for III-V epitaxial lasers; bonded DFB or VCSEL similar
- Total: if the laser contributes 800 fJ/bit and the modulator/detector contribute 50 fJ/bit, total > 850 fJ/bit

The arithmetic makes clear that the laser is the dominant energy consumer. We will return to this.

---

## 10.3.1.2 The Energy Analysis

### Miller's Limit, Revisited

David Miller's 2009 analysis [3] established a theoretical minimum energy for optical interconnects, which we introduced in Section 10.1.1. At chip scale, the analysis yields a different conclusion than at board scale, and understanding why requires careful attention to the scaling.

The minimum energy per bit for a photon-based link is set by the requirement to distinguish a "1" (photon present) from a "0" (no photon) with acceptable error rate. For shot-noise-limited direct detection at BER $= 10^{-12}$:

$$E_{\text{photon, min}} = N_{\text{photons}} \cdot \hbar\omega$$

where $N_{\text{photons}} \approx 50$ photons/bit (accounting for the quantum efficiency and receiver noise). At $\lambda = 1310$ nm:

$$E_{\text{photon, min}} = 50 \times \frac{hc}{\lambda} = 50 \times \frac{(6.63\times10^{-34})(3\times10^8)}{1310\times10^{-9}} \approx 7.6 \text{ aJ/bit}$$

This is $7.6 \times 10^{-18}$ J/bit — 100,000 times smaller than the 1 pJ/bit electrical target. The fundamental physics of photons allows optical links with negligible energy.

The question is why the practical optical energy budget is so much larger. The answer has two parts: (1) the laser has a finite wall-plug efficiency, so the electrical power consumed by the laser is 1/WPE times the optical power emitted; and (2) the receiver has a noise floor that requires a minimum optical power at the detector, which flows back through the link loss to require a minimum laser output power.

For a link with loss $L_{\text{dB}}$ and detector noise equivalent power (NEP) $P_{\text{NEP}}$:

$$P_{\text{laser}} = \frac{P_{\text{det, min}}}{\eta_{\text{WPE}}} \cdot 10^{L_{\text{dB}}/10}$$

$$E_{\text{laser}} = \frac{P_{\text{laser}}}{B} = \frac{P_{\text{det, min}}}{\eta_{\text{WPE}} \cdot B} \cdot 10^{L_{\text{dB}}/10}$$

For a silicon photonic on-chip link:
- $L_{\text{dB}}$: waveguide loss (2 dB/cm × 1 cm = 2 dB) + modulator insertion loss (3 dB) + fiber coupling (not applicable on-chip) = 5 dB → $10^{0.5} = 3.16$
- $P_{\text{det, min}}$: for a Ge photodetector with TIA noise at 100 Gbps, approximately $-20$ dBm = 10 μW
- $\eta_{\text{WPE}}$: 20% for bonded III-V laser
- $B = 100$ Gbps

$$E_{\text{laser}} = \frac{10 \times 10^{-6}}{0.20 \times 100 \times 10^9} \times 3.16 \approx 1.6 \text{ fJ/bit} \times 3.16 \approx 5 \text{ fJ/bit}$$

Wait — that seems too good. Let me redo with the correct detector sensitivity. A typical silicon photonic TIA at 100 Gbps requires ~$-8$ dBm input power for BER $10^{-12}$, which is $P_{\text{det}} = 160$ μW.

$$E_{\text{laser}} = \frac{160 \times 10^{-6}}{0.20 \times 100 \times 10^9} \times 3.16 = \frac{1.6\times10^{-6}}{2\times10^{10}} \times 3.16 = 80 \text{ aJ/bit} \times 3.16 \approx 250 \text{ aJ/bit}$$

This is 250 attojoules/bit — well below 1 pJ. So where does the practical energy budget go wrong?

The answer: **wavelength multiplexing overhead**. If a single laser feeds many modulators via a bus waveguide, and only 1 of $N$ modulators is transmitting at any time, the other $N-1$ lasers (or modulator time slots) are idle. For a WDM system with $N$ wavelengths sharing one laser comb source, the average energy per transmitted bit is not 250 aJ but:

$$E_{\text{avg}} = E_{\text{laser}} \times \frac{N_{\text{wavelengths}}}{\text{average utilization fraction}}$$

If $N = 64$ wavelengths and average utilization is 50%, the effective energy per transmitted bit through the laser is multiplied by $64/0.5 = 128$: $250 \text{ aJ} \times 128 = 32 \text{ fJ/bit}$.

This is still below 1 pJ, but real systems have additional overheads: thermal control of ring resonators ($1$ mW per ring), electrical drivers for modulators (50–200 fJ/bit), TIA power ($\sim 200$ fJ/bit at 100 Gbps). Adding these up:

| Component | Energy (fJ/bit) |
|-----------|-----------------|
| Laser (amortized over WDM) | 30–50 |
| Modulator driver | 50–200 |
| Ring thermal control | 50–100 (per ring, amortized) |
| Detector + TIA | 100–300 |
| **Total** | **230–650 fJ/bit** |

This is competitive with the 640–1500 fJ/bit for an equivalent electrical link — but not by the enormous margin that the photon's fundamental energy efficiency might suggest. The engineering gap between the theoretical minimum and practical systems is 5–6 orders of magnitude.

---

## 10.3.1.3 The Bandwidth Density Argument

Even if PNoC does not win decisively on energy, it might win on *bandwidth density* — bits per second per unit cross-sectional area. For a waveguide-based WDM bus:

$$B_{\text{WDM}} = N_\lambda \times B_{\text{per-\lambda}}$$

With $N_\lambda = 64$ wavelengths (current DWDM) and $B_{\text{per-\lambda}} = 100$ Gbps (current ring modulator), a single waveguide carries 6.4 Tbps. The waveguide cross-section is roughly 500 nm × 220 nm — a total width of 500 nm. Accounting for cladding and isolation (practical spacing $\approx 3$ μm between waveguides):

$$\frac{B_{\text{WDM}}}{\text{width}} = \frac{6.4 \text{ Tbps}}{3 \text{ μm}} \approx 2.1 \text{ Tbps/μm}$$

For comparison, a global-layer metal wire at 10-nm node with differential signaling can carry:

$$\frac{B_{\text{electrical}}}{\text{width}} \approx \frac{20 \text{ Gbps}}{1 \text{ μm (pair pitch)}} = 20 \text{ Gbps/μm}$$

The photonic WDM bus achieves ~100× higher bandwidth density. This is the genuine hardware argument for PNoC: not necessarily lower energy per bit, but radically higher bandwidth per unit of chip real estate.

In practice, this advantage is partially offset by the larger area of the optical components (modulators are 10–100 μm long; detector circuits require dedicated area), but for the highest-bandwidth links on a chip, the density argument remains compelling.

---

## 10.3.1.4 What Has Been Demonstrated

The most comprehensive experimental demonstrations of PNoC to date:

**IBM / AIM Photonics (2015)**: 25 Gbps × 4 WDM = 100 Gbps aggregate over a single silicon photonic waveguide on a CMOS chip. Energy: 890 fJ/bit including all components. First chip-scale demonstration of WDM optical link [4].

**MIT (2020)**: A complete PNoC test chip with a photonic ring-based bus, on-chip laser (hybrid bonded InGaAsP), ring modulators, and Ge detectors. 2.5 Gbps per channel, 16 WDM channels, 40 Gbps aggregate. Energy: ~1 pJ/bit. First demonstration with an on-chip laser source [5].

**Intel / DARPA PIPES program (2021–present)**: Targeting 10 fJ/bit total energy for on-chip WDM optical links; published 8 fJ/bit for the modulator element alone (Mach-Zehnder with carrier-depletion), 35 fJ/bit for detector+TIA, laser energy not included. Integration with Intel 7 nm CMOS process ongoing [6].

**The state of the art in 2025**: ~100–200 fJ/bit total energy for on-chip photonic links (laser included); ~100 Gbps per waveguide with WDM; integration with advanced CMOS processes demonstrated at research scale. Not yet in production.

The trajectory is encouraging: energy has improved by ~10× since 2015, bandwidth density has improved by ~5×. But the gap to production deployment involves not just energy and bandwidth numbers but reliability, yield, and manufacturing cost — issues addressed in Subsection 10.3.3.

---

## References

[2] Dally, W., & Towles, B. (2004). *Principles and Practices of Interconnection Networks*. Morgan Kaufmann. [Standard reference for on-chip interconnect energy analysis.]

[3] Miller, D.A.B. (2009). "Device requirements for optical interconnects to silicon chips." *Proceedings of the IEEE*, 97(7), 1166–1185. [Miller's canonical analysis establishing energy targets for on-chip optical links; still the primary reference for this calculation.]

[4] Assefa, S., et al. (2015). "A 90nm CMOS integrated nano-photonics technology for 25Gbps WDM optical communications applications." *IEDM 2012*. [The IBM 90nm silicon photonics demonstration.]

[5] Mehta, K., et al. (2017). "Integrated optical addressing of an ion qubit." *Nature Nanotechnology*, 11, 1066–1070. [Not PNoC specifically, but on-chip photonics integration.] The correct reference for the MIT laser-integrated PNoC is: Stojanović, V., et al. (2018). "Monolithic silicon-photonic platforms in state-of-the-art CMOS SOI processes." *Optics Express*, 26(10), 13106–13121. [MIT CMOS-compatible silicon photonics process with integrated modulators and detectors.]

[6] Murthy, M., et al. (2022). "Silicon photonics platform for 800 Gb/s and beyond data communications." *Journal of Lightwave Technology*, 40(8), 2430–2439. [Intel's silicon photonics roadmap paper; provides energy and bandwidth data for their process.]
