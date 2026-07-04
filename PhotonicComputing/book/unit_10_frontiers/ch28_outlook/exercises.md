# Chapter 28: Exercises

*As befits a closing outlook, these exercises favor analysis, order-of-magnitude estimation, and honest argument over closed-form derivation. Where a number depends on an assumption, state the assumption; where a claim depends on a source, weigh the source. The aim is to leave you able to evaluate photonic-computing claims that have not yet been made.*

---

## Mathematical / Estimation Exercises

**M28.1 — How Far Above Landauer?**

Take the Landauer floor as $k_B T \ln 2 \approx 3 \times 10^{-21}$ J at room temperature (Section 28.1.1).

(a) A photonic multiply-accumulate dissipates about 1 pJ end-to-end. How many orders of magnitude above the Landauer floor is it?

(b) An aggressive future MAC reaches 10 fJ. Recompute the ratio. Does closing this remaining gap look like a thermodynamics problem or an engineering problem, and why?

(c) In one paragraph, explain why quoting the Landauer limit as a design *target* for a near-term photonic accelerator is misleading.

**M28.2 — Bits Cost Photons**

Use the analog channel capacity $B = \tfrac{1}{2}\log_2(1+\mathrm{SNR})$ (Section 28.1.2) and assume shot-noise-limited detection, so the power SNR scales linearly with the detected photon number $N$.

(a) Show that increasing resolution by one bit requires the SNR — and hence $N$ — to increase by a factor of 4.

(b) By what factor does the photon (and energy) budget grow going from 4-bit to 8-bit precision?

(c) Given (b), explain in one or two sentences why photonic machine-learning hardware deliberately targets the 4–8 bit regime rather than the 16–32 bits of digital arithmetic.

**M28.3 — The Converter Wall**

Model a photonic accelerator whose per-MAC optical energy is $E_\text{opt} = E_0/N$ for an $N\times N$ mesh, while each input and output requires a fixed conversion energy $E_\text{conv}$ (DAC/ADC, amortized per element) that is independent of $N$ (Section 28.2.2).

(a) Write the total energy per MAC as a function of $N$, $E_0$, and $E_\text{conv}$, accounting for the $2N$ conversions that serve $N^2$ MACs.

(b) Find the crossover $N$ at which the conversion term equals the optical term for representative values $E_0 = 1$ pJ and $E_\text{conv} = 5$ pJ per element.

(c) Identify the converter-dominated regime and explain why it, rather than the optical core, sets the achievable efficiency — and how this drives the interconnect-first conclusion.

**M28.4 — Delay-Line Memory Capacity**

A photonic "memory" stores bits as pulses circulating in a waveguide delay line (Section 28.2.3). Let the propagation loss be $\alpha$ (dB/cm) and the group velocity $v_g \approx c/4$.

(a) For a target that no bit may decay below half its launch power, derive the maximum storage time as a function of $\alpha$.

(b) At a line rate $R$, how many bits can be held? Express the result as a bandwidth–storage-time product and comment on what limits it.

(c) Compare the resulting bit capacity and hold time to a DRAM cell, and explain why delay lines do not resolve the memory wall.

---

## Conceptual / Discussion Exercises

**C28.1 — Steelman, Then Stress the Interconnect-First Thesis**

Using the energy accounting of Sections 28.2.2 and Concept 6, first construct the strongest possible case that photonics belongs in the computer as interconnect rather than compute. Then attack your own case: name two workloads or two technology shifts that, if they arrived, would make optical *compute* the rational bet again. What single measurement would most cleanly decide the question?

**C28.2 — Does Squeezing Ever Help a Computer?**

Section 28.1.3 argues that squeezed light beats the standard quantum limit only in one quadrature and only at low loss. (a) Describe a computing or sensing task where operating near the SQL is genuinely the binding constraint. (b) Explain why, for a general-purpose photonic classifier running at 4–8 bits, squeezing typically addresses the wrong bottleneck. (c) State the condition under which your answer to (b) would flip.

**C28.3 — Keep Chasing the Photonic Transistor, or Learn the Lesson?**

Miller's criteria (Section 28.2.1, Concept 4) explain why no all-optical switch qualifies as a cascadable low-energy logic device. Argue both sides: (a) that continued pursuit of a photonic transistor is worthwhile, and (b) that the field's real lesson is to stop imitating digital logic and commit to analog, linear, interconnect-centric optics. Which position do you hold, and what evidence would move you?

**C28.4 — Which Workloads Justify Photonic Compute?**

Applying the matched-baseline discipline of Chapter 25 and Concept 10, propose a concrete criterion — in terms of arithmetic intensity, reuse factor $N$, precision tolerance, and data-movement share — for when a workload should run on optics rather than on a digital accelerator at the same process node. Test your criterion against two real workloads (e.g., a transformer inference layer and a large matrix–matrix product) and report where each falls.

---

## Lab / Research Exercises

**L28.1 — Build an End-to-End Energy Model**

Construct a spreadsheet or notebook model of a photonic matrix accelerator that includes laser wall-plug power, modulator drive, DAC/ADC energy per element, thermal/calibration overhead, and the $1/N$ optical MAC term (Section 28.2.2). (a) Sweep $N$ and plot energy-per-MAC versus problem size. (b) Identify the converter-dominated regime numerically. (c) Report the single parameter your model is most sensitive to, and what measurement would pin it down.

**L28.2 — Audit a "Photonic Advantage" Claim**

Select one published or announced claim of photonic-computing advantage. Applying the Chapter 25 discipline (Concept 10), separate the physics result from the computing claim, identify the baseline actually used and the baseline that *should* have been used, and place the source on the peer-review-to-press-release credibility gradient. Write a one-page memo: what is demonstrated, at what scope, against what baseline, and what remains unproven.

**L28.3 — Phase-Change Memory Endurance Survey**

From the primary literature on phase-change photonic memory (Section 28.2.3; Ríos, Wuttig, Feldmann), compile reported figures for write energy, switching speed, number of resolvable levels, and cycling endurance. (a) Tabulate these against DRAM and SRAM equivalents. (b) Explain, from the material physics, why endurance is the limiting figure of merit for a computing memory. (c) State what a phase-change memory would have to achieve to close the photonic memory wall.
