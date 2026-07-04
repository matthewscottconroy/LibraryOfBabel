# Chapter 25: Electronic-Photonic Co-Design and Benchmarking

> *"When you can measure what you are speaking about, and express it in numbers, you know something about it; but when you cannot measure it, when you cannot express it in numbers, your knowledge is of a meagre and unsatisfactory kind."*
>
> — William Thomson (Lord Kelvin), "Electrical Units of Measurement" (1883)

---

## No Photonic Computer Is Purely Photonic

Every photonic computing system described in this book — the MZI mesh of Chapter 12, the wavelength-multiplexed neural networks of Chapter 13, the phase-change synapses of Chapter 16 — is in reality an *electronic* system with a photonic core. Data arrives as digital bits held in electronic memory. It must be converted to analog voltages (DAC), imprinted onto light (modulator), transformed by the photonic circuit, converted back to photocurrent (detector), amplified (TIA), digitized (ADC), and returned to electronic memory. The laser that powers the whole transaction runs off an electrical supply at 10–25% wall-plug efficiency, and thousands of thermal tuners, bias controllers, and calibration loops keep the analog fabric on its operating point.

This has a consequence that cannot be stated too bluntly: **in nearly every photonic accelerator built to date, the majority of the energy, latency, area, and cost is in the electronics.** The photonic core can perform a multiply-accumulate for femtojoules or less; the conversions that feed it cost picojoules. A photonic matrix multiplication finishes in tens of picoseconds; the DAC/ADC round trip takes nanoseconds. Whether a photonic accelerator beats a GPU is therefore decided not by the elegance of its optics but by how well its designers amortize the electronic overhead — how many cheap optical operations they extract per expensive conversion.

This chapter treats that fact not as an embarrassment but as the design problem. The discipline that addresses it is **electronic-photonic co-design**: choosing the algorithm, the partition between domains, the circuit interfaces, and the devices *together*, so that each domain does only what it is physically best at. Photonics: linear transformations, fan-out, and transport at enormous bandwidth. Electronics: nonlinearity, memory, precision, and control.

The chapter's second concern is **benchmarking** — the measurement discipline that keeps co-design honest. The photonic computing literature contains efficiency claims spanning five orders of magnitude for nominally similar systems, mostly because different authors draw different boundaries around what they count. We develop the metrics (energy per MAC, TOPS/W, latency, throughput, effective bits), the reference points (GPUs, TPUs, analog-electronic accelerators), and the methodology (iso-accuracy, wall-plug boundaries, peak versus delivered) that make comparisons meaningful. And we practice auditing published claims, because the ability to dissect a headline number into its assumptions is the most transferable skill this chapter offers.

---

## The Central Tension

Photonic computing's value proposition rests on a scaling asymmetry: an $N \times N$ optical matrix multiplier performs $N^2$ multiply-accumulates per clock while requiring only $O(N)$ conversions at its edges. As $N$ grows, the conversion tax per MAC falls as $1/N$, and the physics of the optical core — sub-femtojoule, speed-of-light, interference-based arithmetic — begins to show through.

But three forces push back:

1. **Loss and noise grow with $N$.** Optical loss in dB grows linearly with mesh depth, so required laser power grows exponentially; analog noise and component error accumulate, eroding the effective bit precision (Section 25.2.2).
2. **Static power grows with $N^2$.** Thermo-optic phase shifters holding $N(N-1)/2$ weights consume tens of watts for $N = 64$ — often more than the computation itself (Section 25.2.1).
3. **Area grows with $N^2$ at photonic, not electronic, density.** A photonic weight occupies thousands of square micrometers; a digital weight occupies well under one. Reticle limits cap single-die meshes near $N \sim 100$–300.

The engineering question of this chapter is whether there exists an operating regime — a value of $N$, a precision, a workload, an integration technology — in which the $1/N$ amortization wins before the three penalties take over. The answer, developed quantitatively, is: yes, but the regime is narrower than the headlines suggest, and reaching it requires co-design of everything from the training algorithm (noise-aware, low-precision) to the laser (high wall-plug efficiency) to the package (3D-integrated conversion circuits).

---

## Chapter Structure

**Section 25.1 — The Electronic-Photonic Interface**: The unavoidable electronics: DACs, ADCs, TIAs, and clocking, with their energy and precision limits (25.1.1); and how the two domains are physically joined and jointly designed — 2.5D, 3D, and monolithic integration, and the co-design methodology that partitions a workload between light and charge (25.1.2).

**Section 25.2 — Metrics and Benchmarking**: Throughput and energy efficiency metrics, with a complete worked energy budget for a 64×64 photonic accelerator (25.2.1); precision, noise, and the effective-number-of-bits ceiling of analog optical computing (25.2.2); and fair comparison against GPUs, TPUs, and analog-electronic accelerators, including the standard benchmarking pitfalls and an auditing checklist for published claims (25.2.3).

**Section 25.3 — Where Photonics Wins, and Where It Doesn't**: The workload regimes where the physics genuinely favors light — fixed-weight low-latency inference (25.3.1) and bandwidth-limited, WDM-parallel, optics-native tasks (25.3.2) — and the sober complement: precision, nonlinearity, memory, density, and generality, where electronics remains dominant (25.3.3).

The chapter is the hinge of the book: everything before it supplies the components; everything after it (Unit X) surveys the companies and research frontiers that will live or die by the accounting done here.
