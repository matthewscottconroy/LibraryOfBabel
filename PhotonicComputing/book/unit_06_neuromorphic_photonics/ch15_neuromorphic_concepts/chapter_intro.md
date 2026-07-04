# Chapter 15: Neuromorphic Computing Concepts

> *"Ask not what the brain computes, but how. The answer is not floating-point multiply–accumulate — it is a spike, a threshold, and a synapse that remembers when the last spike arrived."*

---

## Why Begin with Biology

Before we can build a neuron out of a laser, we need to know what a neuron *is* — not anatomically, but computationally. This chapter supplies that foundation. It is deliberately hardware-agnostic: the leaky integrate-and-fire equation, the distinction between rate and temporal codes, and the spike-timing-dependent plasticity rule introduced here apply equally to a cortical pyramidal cell, an Intel Loihi core, and an excitable microring. Chapter 16 will make these ideas physical; Chapter 15 makes them precise.

## The Neuromorphic Hardware Landscape

The term *neuromorphic* was coined by Carver Mead in the late 1980s and crystallized in his 1990 paper: the idea of using the physics of analog subthreshold VLSI transistors to *emulate*, rather than numerically simulate, the operations of neural tissue — a lineage of silicon neuron circuits later surveyed by Indiveri and colleagues. Three decades on, neuromorphic engineering is a substantial field with several flagship electronic platforms, each making a different bet:

- **Intel Loihi (2018)** is a digital, asynchronous manycore chip: 128 neuromorphic cores supporting on the order of 130,000 neurons and 130 million synapses, with programmable on-chip learning rules. Its 2021 successor, **Loihi 2**, pushes toward a million neurons per chip and faster spike processing.
- **IBM TrueNorth (2014)** took the low-power extreme: 4,096 cores, one million neurons, and 256 million synapses running a real-time model at roughly 70 mW — a fraction of a watt for a million neurons.
- **SpiNNaker** (Manchester) is a massively parallel machine built from ARM cores, designed to simulate large spiking networks in biological real time by brute software force over a bespoke packet-switched fabric.
- **BrainScaleS** (Heidelberg) is analog and waferscale, running its physical neuron circuits roughly $10^3$–$10^4\times$ faster than biology — an *accelerated* emulation rather than a real-time one.

These machines span an enormous design space, but they share one property: their spikes are electronic events, gated by transistors and moved over metal wires, so their fundamental timescale is microseconds to nanoseconds. Biology sets the slow end of the scale at milliseconds. The thesis of this unit — argued conceptually in §15.2 and physically in Chapter 16 — is that photonics can push the spike timescale three to six orders of magnitude further, into picoseconds, while replacing RC-limited fan-out with passive optical splitting and wavelength multiplexing.

## What This Chapter Covers

**Section 15.1 — Biological Neurons and Spiking Neural Networks** develops the biophysics and its reductions: the action potential, the Hodgkin–Huxley model and its leaky integrate-and-fire simplification, neural coding schemes, and the STDP learning rule. It closes with Maass's framing of spiking networks as the "third generation" of neural models.

**Section 15.2 — Why Photonics for Neuromorphic Computing** makes the quantitative case: the picosecond spike, the WDM fan-out advantage, and the energy-per-spike budget that motivates the sub-femtojoule goal.

By the end of the chapter you should be able to write down the equation of a spiking neuron, explain what a synapse learns and when, and state — in numbers — why one might want to build such a system out of light.

---

## References

[1] Mead, C. (1990). "Neuromorphic electronic systems." *Proceedings of the IEEE*, 78(10), 1629–1636.

[2] Davies, M., et al. (2018). "Loihi: a neuromorphic manycore processor with on-chip learning." *IEEE Micro*, 38(1), 82–99.

[3] Merolla, P.A., et al. (2014). "A million spiking-neuron integrated circuit with a scalable communication network and interface." *Science*, 345(6197), 668–673.

[4] Furber, S.B., Galluppi, F., Temple, S., & Plana, L.A. (2014). "The SpiNNaker project." *Proceedings of the IEEE*, 102(5), 652–665.

[5] Indiveri, G., et al. (2011). "Neuromorphic silicon neuron circuits." *Frontiers in Neuroscience*, 5, 73.

[6] Shastri, B.J., et al. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114.
