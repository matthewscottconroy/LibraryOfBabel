# Unit IX: Benchmarking and the Computing Landscape

> *"For a successful technology, reality must take precedence over public relations, for nature cannot be fooled."*
>
> — Richard Feynman, Appendix F to the Report of the Presidential Commission on the Space Shuttle Challenger Accident (1986)

---

## What This Unit Is About

Eight units of this book have been spent building things: waves, lasers, waveguides, modulators, matrix multipliers, photonic neurons, quantum interferometers, fabrication flows. This unit builds nothing. Its job is to weigh what the previous units built — honestly, quantitatively, and against the strongest possible competition.

The need for such a unit is peculiar to this field. Photonic computing is an area where the gap between a device-level number and a system-level number can span four orders of magnitude, and where that gap is routinely elided in publications, press releases, and funding pitches. A photonic multiply-accumulate operation can, in a carefully defined sense, cost less than a femtojoule. A photonic multiply-accumulate operation, delivered as part of a complete system that accepts digital inputs and returns digital outputs, can simultaneously cost more than a picojoule — because the digital-to-analog converters, analog-to-digital converters, transimpedance amplifiers, laser wall-plug inefficiency, thermal tuning, and control electronics that surround the photonic core consume almost all of the energy. Both numbers are true. Only one of them determines whether anyone should buy the chip.

This unit is therefore the book's audit function. It develops three disciplines:

1. **System accounting.** Drawing the energy and latency boundary around the *whole* computation — from digital input to digital output — and itemizing everything inside it: lasers (divided by their wall-plug efficiency), modulators, detectors, DACs, ADCs, TIAs, clock distribution, calibration, and the static power that holds analog weights in place.

2. **Fair comparison.** Placing photonic accelerators on the same axes as the machines they must displace: GPUs, TPUs, and analog-electronic in-memory accelerators. This requires normalizing for numeric precision, distinguishing peak from delivered throughput, holding task accuracy constant, and resisting the arithmetic sleights-of-hand (a MAC counted as two operations here, one operation there) that inflate headline numbers.

3. **Co-design.** Accepting that the electronic-photonic interface is not an implementation detail but the central design problem, and jointly optimizing algorithm, architecture, circuit, and device so that the expensive conversions are amortized over as much cheap optical computation as possible.

---

## One Chapter, One Discipline

Unit IX contains a single chapter, but it is the chapter that determines whether the rest of the book describes a future computing industry or an elegant laboratory curiosity.

**Chapter 25: Electronic-Photonic Co-Design and Benchmarking** proceeds in three sections. Section 25.1 examines the electronic-photonic interface — the DACs, ADCs, and amplifiers that dominate real power budgets, and the 2.5D, 3D, and monolithic integration strategies that determine how tightly electronics and photonics can be coupled. Section 25.2 builds the metrology: energy per MAC, TOPS/W, latency, throughput, effective bit precision, and the benchmarking methodology (with its many failure modes) needed to compare a photonic accelerator fairly against an NVIDIA H100 or a Google TPU. Section 25.3 draws the map that results: the workload regimes where photonics wins on physics — fixed-weight low-latency inference, bandwidth-limited and WDM-parallel tasks, computing on signals that are already optical — and the larger territory where electronics wins today and will keep winning: precision, nonlinearity, memory, density, and general-purpose control.

---

## How to Read the Numbers in This Unit

Three warnings apply throughout.

First, the numbers are dated. GPU specifications, ADC survey envelopes, and photonic device records all move; the A100 and H100 figures quoted here are anchors from the early 2020s, not eternal constants. The *methodology* — what to count, what to normalize, what to distrust — is the durable content.

Second, ranges are honest and point values are rhetorical. Where this unit quotes "1–10 pJ per conversion" rather than a single figure, it is because the honest answer depends on speed, precision, and process node. Be suspicious of any comparison, including ours, that collapses a distribution into its most convenient tail.

Third, every efficiency claim has a boundary, and the boundary is where the bodies are buried. The single most useful habit this unit can teach is to ask, of any TOPS/W figure: *what, exactly, is inside the denominator?*

---

## References for the Unit Introduction

[1] Feynman, R.P. (1986). "Personal observations on the reliability of the Shuttle." Appendix F to the *Report of the Presidential Commission on the Space Shuttle Challenger Accident*. [Source of the epigraph; the canonical statement of engineering honesty.]

[2] Reuther, A., Michaleas, P., Jones, M., Gadepally, V., Samsi, S., & Kepner, J. (2019; updated annually through 2022). "Survey and benchmarking of machine learning accelerators." *IEEE High Performance Extreme Computing Conference (HPEC)*. [The standard peak-performance vs. power scatter plots on which every accelerator — electronic or photonic — can be located.]

[3] Reddi, V.J., et al. (2020). "MLPerf inference benchmark." *Proceedings of the 47th Annual International Symposium on Computer Architecture (ISCA)*, 446–459. [The industry-standard methodology for accuracy-constrained, system-level inference benchmarking.]
