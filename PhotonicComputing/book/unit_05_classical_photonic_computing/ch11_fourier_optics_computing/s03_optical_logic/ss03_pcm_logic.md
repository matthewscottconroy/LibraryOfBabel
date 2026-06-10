# Subsection 11.3.3: Phase-Change Material Logic

## Orientation

Phase-change materials (GST, GSST, and related alloys) were introduced in Section 7.4.3 as non-volatile optical switches for analog memory. Here we examine their application to optical logic — a genuinely different use case that avoids some of the fundamental limitations of Kerr and SOA logic, while introducing new constraints.

The key property that makes PCM potentially interesting for logic: PCM is *non-volatile*. The logical state (amorphous = high transmission vs. crystalline = low transmission) is maintained without power. This is different from every other optical logic approach, where maintaining the logical state requires continuous power. PCM "computes by materializing" — the computation is encoded in the physical state of the material.

---

## 11.3.3.1 PCM as a Bistable Optical Element

### Two-State Operation

As established in Section 7.4.3, GSST at 1550 nm has:
- Amorphous state: $n_a = 3.35$, $k_a \approx 0$ (transparent)
- Crystalline state: $n_c = 5.1$, $k_c = 0.3$ (absorbing)

A waveguide-coupled PCM cell (GSST patch on a silicon waveguide, $\sim 1$ μm thick, $\sim 5$ μm long) switches between:
- Amorphous: 85–95% transmission (the "1" state)
- Crystalline: 5–15% transmission (the "0" state)

Switching operations:
- **Amorphization** (RESET): $\sim 30$ ns pulse, $> 500$ mW peak power → melts and quenches the GST above the melting point ($T_m \approx 600°C$ for GSST) → amorphous
- **Crystallization** (SET): $\sim 1$ μs pulse, $\sim 10$ mW → heats to crystallization temperature ($T_x \approx 250°C$ for GSST) without melting → crystalline

Write energy per operation:
$$E_{\text{write}} = P_{\text{pulse}} \times \tau_{\text{pulse}} \approx 500 \text{ mW} \times 30 \text{ ns} = 15 \text{ nJ}$$

This is enormous compared to CMOS (10–100 aJ) or even SOA logic (10–100 fJ). PCM switching is **5–8 orders of magnitude more energy-intensive than a transistor flip**.

### Why PCM Logic Requires Different Justification

For PCM logic to be competitive, it cannot compete on a per-operation energy basis. The justification must be different:

1. **Non-volatility**: The state persists for years without power. This eliminates the leakage energy cost of SRAM (which requires continuous power to maintain state) for applications where data is stored more often than it is computed.

2. **In-memory computing**: PCM elements can store and compute simultaneously, avoiding the von Neumann bottleneck of data transfer between separate memory and processor.

3. **Optical parallelism**: Many PCM cells can be switched simultaneously with spatially-patterned optical pulses, enabling massively parallel memory write operations.

---

## 11.3.3.2 PCM-Based Optical Logic Gates

### Demonstrated Implementations

**Feldmann et al. (2019)** demonstrated a PCM-based all-optical logic gate [1]:
- Architecture: Two input waveguides coupled to a PCM cell via evanescent coupling; a probe laser reads out the PCM state after writing
- Logic function: The PCM state after writing depends on the combination of input optical pulses. Two pulses applied together (AND condition) have sufficient combined energy to crystallize the PCM; a single pulse does not (insufficient energy). This implements optical AND.

**Accumulated switching**: Since PCM transitions require cumulative energy input, multiple below-threshold pulses can accumulate and eventually trigger a state change. This "integrate-and-fire" behavior is reminiscent of biological neuron models (spiking neural networks, Chapter 15) and is exploited in the Feldmann 2019 synaptic device.

### PCM Optical NAND Gate

A NAND gate (the universal logic gate) requires:
- Output = 1 unless both inputs are 1

PCM NAND implementation:
1. Initialize PCM to amorphous (output transmission = 1)
2. Input A modulates one waveguide, Input B modulates the other
3. If both A and B are 1: combined power crystallizes PCM → low transmission (output 0)
4. Otherwise: PCM remains amorphous → high transmission (output 1)

Demonstrated contrast ratio: 8–12 dB between states. Switching energy: 10–50 nJ per operation.

---

## 11.3.3.3 The Honest Assessment

PCM optical logic is a genuine demonstration of principle, but the comparison to CMOS is unfavorable on virtually every metric:

| Metric | CMOS NAND (5nm) | SOA-MZI gate | PCM optical gate |
|--------|----------------|--------------|------------------|
| Energy per operation | 20–50 aJ | 10–100 fJ | 10–50 nJ |
| Speed | 10–30 ps | 1–10 ps | 10–1000 ns |
| Non-volatile | No (SRAM) | No | **Yes** |
| Read power | 0 | 0 | ~1 μW (probe) |
| Device density | $10^{10}$/cm² | $10^3$/cm² | $10^6$/cm² |
| Cascadability | Excellent | Limited | Poor (no optical gain) |

The PCM gate uses 5–9 orders of magnitude more energy per operation than CMOS. Even if we give PCM credit for non-volatility (eliminating standby power), the write energy comparison still strongly favors CMOS-based NVRAM (Flash, PCM-DRAM hybrids like Intel Optane) for any computation-heavy workload.

**Where PCM logic makes genuine sense**: Not for general-purpose Boolean logic, but for:
1. **Analog memory with in-situ computation** (the Feldmann 2021 tensor core of Section 7.4.3): The PCM element stores a weight value (as an intermediate conductance state, not just binary) and simultaneously participates in MAC computation via the optical transmission. The "write" energy is paid for storage, not for computation — a different accounting.
2. **One-time programmable optical elements**: Optical masks, routing switches, or beam-forming networks that are set once and read many times. The non-volatility and low read power make PCM attractive when read operations are overwhelmingly more frequent than write operations.

The lesson from PCM logic mirrors the lesson from SOA logic: optical nonlinear elements cannot compete with CMOS transistors for Boolean computation. The path to useful optical computing lies not in replacing transistors with photons, but in exploiting the specific physical capabilities of light — high bandwidth, parallelism, and linear superposition — that electronics lacks.

---

## References

[1] Feldmann, J., et al. (2019). "Calculating with light using a chip-scale all-optical abacus." *Nature Communications*, 10, 1256. [Demonstrates PCM-based optical logic gates on chip; the primary reference for PCM optical computing.]

[2] Ríos, C., et al. (2015). "Integrated all-photonic non-volatile multi-level memory." *Nature Photonics*, 9, 725–732. [First demonstration of multi-level optical memory using PCM; the device physics basis for PCM logic.]

[3] Wuttig, M., Bhaskaran, H., & Taubner, T. (2017). "Phase-change materials for non-volatile photonic applications." *Nature Photonics*, 11, 465–476. [Review of PCM materials for photonic memory and computing; covers GSST, GST, and emerging alternatives.]
