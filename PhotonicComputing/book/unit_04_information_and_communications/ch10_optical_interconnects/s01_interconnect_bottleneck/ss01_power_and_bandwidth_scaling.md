# 10.1.1 Power and Bandwidth Scaling in Interconnects

## The RC Energy of Electrical Interconnects

Transmitting a bit of information through a copper wire of length $L$ and capacitance per unit length $C_L$ (F/m) requires charging the wire to voltage $V$. The energy:

$$E_{\text{bit}} = C_L L V^2$$

For a repeaterless RC-limited copper wire:
- $C_L \approx 0.2$ pF/mm (typical CMOS metal wire)
- $V \approx 1$ V (logic voltage swing)
- $L = 10$ mm (chip-to-chip on a board)

$$E_{\text{bit}} = 0.2 \times 10^{-12} \text{ F/mm} \times 10 \text{ mm} \times 1^2 \text{ V}^2 = 2 \text{ pJ}$$

This is the minimum energy — it ignores driver and receiver overhead, which typically multiplies the total by 5–10×. For a realistic chip-to-chip electrical link: ~10–50 pJ/bit.

For an optical link over the same 10 mm distance, the energy is dominated by:
- Laser driver: ~50–200 fJ/bit
- Modulator: ~10–50 fJ/bit  
- Photodetector + TIA: ~10–100 fJ/bit
- Total: ~100–500 fJ/bit = 0.1–0.5 pJ/bit

**Optical links are 10–100× more energy-efficient for chip-to-chip distances** than electrical links at comparable data rates.

For board-to-board distances (~1 m) and rack-to-rack distances (~100 m), the electrical interconnect energy scales linearly with length while optical energy (dominated by fixed transmitter/receiver overhead) is approximately independent of distance for short links. The crossover point where optics becomes more energy-efficient is approximately 1–10 mm for CMOS-compatible optical interconnects [1].

## Bandwidth Density Limitations

Beyond energy, electrical signaling faces bandwidth density limits:

**Skin effect**: At high frequencies, current flows only in a thin surface layer of the conductor with depth $\delta_s = \sqrt{2\rho/(\omega\mu_0)} \propto 1/\sqrt{f}$. For copper at 50 GHz: $\delta_s \approx 300$ nm. This increases resistance at high frequencies, causing signal attenuation $\propto \sqrt{f}$ per unit length.

**Dielectric loss**: The printed circuit board dielectric (FR4 or similar) absorbs high-frequency signals, with loss coefficient $\alpha \propto f$. For a 200-mm PCB trace at 50 GHz: ~15 dB loss — requiring repeaters or equalization.

**Crosstalk**: Adjacent electrical traces couple capacitively and inductively, inducing noise proportional to $dI/dt$ (inductive) and $dV/dt$ (capacitive). At 100+ Gbps, this limits the density of traces.

The practical limit for PCB electrical signaling (without exotic materials) is ~50–100 Gbps per lane with equalization, and ~200 Gbps with advanced DSP. Packing more than ~100 lanes on a standard package edge (pitch ~0.5 mm) limits the total I/O bandwidth to ~10–20 Tbps per chip.

For comparison, a single-mode fiber at 1550 nm with 96-channel WDM at 400 Gbps/channel carries **38 Tbps** — in a 125-μm-diameter glass strand. The bandwidth density advantage of optics is ~100× over PCB electrical at these scales.

## Miller's Energy Argument

David Miller at Stanford articulated the energy argument for optical interconnects in a series of papers. His key result [1]: the minimum energy per bit for an optical interconnect is:

$$E_{\text{min,optical}} \approx \frac{N_{\text{photons}} \hbar\omega}{\eta_{\text{quantum}}} + E_{\text{overhead}}$$

where $N_{\text{photons}}$ is the minimum photons needed for reliable detection (from Chapter 5: ~100 photons at 1 GHz, or 0.02 aJ per photon at 1550 nm), and $E_{\text{overhead}}$ is the overhead of the driving electronics. For state-of-art optical transceivers:

- Modulator energy: 10–100 fJ/bit
- Photodetector + TIA: 10–100 fJ/bit  
- Total: 50–500 fJ/bit

Miller's argument: there is no fundamental limit preventing optical links from reaching ~1 fJ/bit at short distances — comparable to the energy of a single electronic gate transition. This is the target for "attojoule optoelectronics." Current demonstrations approach 100 fJ/bit; the path to 1 fJ/bit requires sub-fF modulator capacitances (achieved by plasmonic or PCM devices in Section 7.3) and avalanche or resonant-cavity detectors.

---

## References

[1] Miller, D.A.B. (2017). "Attojoule optoelectronics for low-energy information processing and communications." *Journal of Lightwave Technology*, 35(3), 346–396. [The comprehensive treatment of energy limits for optical interconnects; derives the attojoule target.]

[2] Chen, X., Mineo, C., & Abidi, A.A. (2021). "Bandwidth and power scaling of high-speed electrical and optical interconnects." *Proceedings of IEEE CICC*. [Comparison of electrical vs. optical energy per bit across distances and data rates.]
