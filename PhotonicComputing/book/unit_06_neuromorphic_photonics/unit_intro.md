# Unit VI: Neuromorphic Photonics — Spiking, Excitable, and In-Memory Brain-Inspired Optics

> *"The human brain runs on about 20 watts — dimmer than the bulb in a desk lamp — yet it out-computes any supercomputer at perception, learning, and control. It does so not with clocked arithmetic but with spikes: brief, all-or-nothing electrical pulses exchanged among roughly a hundred billion neurons on a millisecond clock. A photonic neuron can emit the same kind of spike a million times faster, in picoseconds. This unit asks what happens when we keep the brain's architecture and give it light for a nervous system."*
>
> — The thesis of neuromorphic photonics

---

## What This Unit Is About

Unit V built photonic computers in the image of linear algebra. Its optical neural networks (ONNs) encode a vector in the amplitudes of a set of optical modes, pass them through a mesh of Mach–Zehnder interferometers that enacts a matrix multiply, and read out the result — a feedforward, largely linear, synchronous machine. We studied MZI meshes (Reck and Clements decompositions), broadcast-and-weight matrix multipliers, optical activation functions, diffractive networks (D²NN), the training of ONNs, and reservoir computing. Those architectures are powerful, but they are *not* how the brain computes.

Neuromorphic computing takes the opposite starting point. Instead of a matrix multiply followed by a pointwise nonlinearity, it builds the nonlinearity and the dynamics in from the beginning: each "neuron" is an excitable dynamical system that integrates its inputs over time and, when a threshold is crossed, emits a spike. Information lives not in a static vector of amplitudes but in *when* the spikes happen. Learning is often not a global gradient step but a local rule that adjusts a synapse based on the relative timing of the spikes that pass through it. This is analog, event-driven, brain-inspired computing, and it is the subject of Unit VI.

The word to keep in mind throughout is *spike*. A biological action potential lasts one to two milliseconds; the neuron then falls silent for a refractory period of comparable length. The brain's astonishing energy efficiency — roughly 20 W for the whole organ — comes precisely from this sparsity: a neuron consumes energy only when it fires, and most neurons are silent most of the time. Photonics does not change the event-driven principle; it changes the clock. An excitable laser can emit an optical spike in a few picoseconds, some eight orders of magnitude faster than its biological counterpart, while a single waveguide can carry dozens of independent wavelength channels that fan out passively to many downstream neurons. Speed, fan-out, and (potentially) sub-femtojoule energy per spike are the three arguments for doing neuromorphic computing in light rather than in silicon transistors.

---

## Two Chapters

**Chapter 15 — Neuromorphic Computing Concepts** builds the conceptual foundation. It reviews the biology of the neuron and the action potential, reduces that biophysics to the leaky integrate-and-fire (LIF) model that recurs throughout the unit, and introduces the vocabulary of neural coding (rate, temporal, and population codes) and of synaptic plasticity (spike-timing-dependent plasticity, STDP). It surveys the electronic neuromorphic landscape — Mead's analog VLSI, Intel's Loihi, IBM's TrueNorth, Manchester's SpiNNaker, Heidelberg's BrainScaleS — and then makes the quantitative case for photonics: why the spike timescale can fall from milliseconds to picoseconds, why optical fan-out via wavelength-division multiplexing (WDM) beats RC-limited electronic interconnect, and how the energy-per-spike budget is set.

**Chapter 16 — Photonic Neurons and Synapses** turns concepts into devices and architectures. It presents the excitable semiconductor laser as a physical neuron — biased near threshold, with a saturable absorber or optical injection, mapping rigorously onto FitzHugh–Nagumo and LIF dynamics — in its injection-locked, VCSEL, and microring forms. It develops the phase-change-material (PCM) synapse (GST, and low-loss GSST, clad on a waveguide) as a non-volatile, multi-level weight, and the in-memory computing principle that lets the multiply happen as light traverses the cell. It assembles these into WDM broadcast-and-weight *spiking* networks and photonic spiking convolutional layers, and closes with learning: all-optical STDP on PCM synapses, and offline surrogate-gradient training of spiking networks whose weights are then deployed to the hardware.

---

## The Cousins in Unit V

Neuromorphic photonics is not disconnected from the classical photonic computing of Unit V; it is its dynamical sibling. Two links are worth naming now. First, **reservoir computing** (Unit V, §13.4) is in a sense the fixed-weight, linear-readout relative of a spiking recurrent network: it exploits the transient dynamics of a nonlinear optical system without training the internal connections. Second, and more directly, the **broadcast-and-weight** architecture (Unit V, §12.4) — Tait's silicon microring weight banks summing wavelength-multiplexed signals by balanced photodetection — was invented precisely to wire spiking photonic neurons together. In Unit V we used it for matrix–vector products; here we will use the very same weight banks to route spikes between excitable laser neurons. We therefore *reference* broadcast-and-weight and reservoir computing rather than re-deriving them, and concentrate on what is genuinely new: excitability, plasticity, and the timing of spikes.

---

## The Stakes

Neuromorphic photonics is the youngest and most speculative subject in this book. No one has yet built a large, general-purpose photonic spiking computer, and formidable obstacles remain: cascadability, device-to-device variability, the energy and endurance of PCM writes, and the absence of a mature training theory for hardware spiking networks. But the underlying arithmetic is compelling. If a neuron can fire in picoseconds at sub-femtojoule cost and fan out over dozens of wavelengths, then a brain-inspired photonic processor could occupy a region of the speed–energy plane that neither GPUs nor electronic neuromorphic chips can reach. This unit is a map of that frontier: the concepts in Chapter 15, the devices and architectures in Chapter 16, and — throughout — an honest accounting of what has been demonstrated versus what has merely been proposed.

---

## References for the Unit Introduction

[1] Prucnal, P.R., & Shastri, B.J. (2017). *Neuromorphic Photonics*. CRC Press. [The definitive monograph on the field, written by the founders of the Princeton program; the canonical reference for excitable laser neurons and broadcast-and-weight spiking networks.]

[2] Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D., & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114. [The current roadmap of the field, spanning ONNs, in-memory computing, and spiking photonics.]

[3] Ferreira de Lima, T., Shastri, B.J., Tait, A.N., Nahmias, M.A., & Prucnal, P.R. (2017). "Progress in neuromorphic photonics." *Nanophotonics*, 6(3), 577–599. [A focused review of photonic spike processing and the excitable-laser-plus-weight-bank paradigm.]
