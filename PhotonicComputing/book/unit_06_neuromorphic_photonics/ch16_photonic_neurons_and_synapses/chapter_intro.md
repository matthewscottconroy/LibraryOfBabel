# Chapter 16: Photonic Neurons and Synapses

> *"A neuron is not a transistor and a synapse is not a memory cell — but a laser biased near threshold behaves remarkably like the former, and a thin film of phase-change glass on a waveguide behaves remarkably like the latter, both a million times faster than the brain they imitate."*
>
> — Paraphrase of a recurring theme in the neuromorphic-photonics literature

---

## From Concepts to Devices

Chapter 15 established *why* one might build a brain-inspired computer out of light and *what* such a machine must compute: leaky integrate-and-fire (LIF) dynamics, spike-timing codes, spike-timing-dependent plasticity (STDP), and the speed, fan-out, and energy arguments that make photonics attractive. This chapter is the engineering complement. It asks the physical question that Chapter 15 deferred: out of lasers, waveguides, resonators, and thin films, how do we actually *build* the neuron, the synapse, the network, and the learning rule?

The encouraging answer is that every abstract element of a spiking neural network (SNN) now has at least one credible photonic embodiment, and several have been demonstrated on silicon and silicon-nitride chips. The purpose of this chapter is to survey those embodiments with enough physical detail that a photonic-computing engineer can reason about their performance, their limits, and how they fit together into a system (Prucnal & Shastri, 2017; Shastri et al., 2021).

---

## The Four Building Blocks

**Section 16.1 — Excitable laser neurons.** A semiconductor laser biased just below its lasing threshold — perturbed by optical injection, or governed by an embedded saturable-absorber section — is *excitable* in exactly the sense of the FitzHugh–Nagumo and LIF models. A sub-threshold input decays away; a super-threshold input triggers a single, all-or-nothing optical pulse; and the device then recovers through a refractory period before it can fire again. Nahmias et al. (2013) showed that a two-section laser with a saturable absorber maps rigorously onto the LIF equations and can spike at gigahertz rates — roughly eight orders of magnitude faster than a biological neuron. We also cover the polarization-switching VCSEL neuron, the excitable semiconductor ring laser, and the underlying rate equations whose fast photon and slow carrier timescales are the origin of excitability.

**Section 16.2 — Photonic synapses.** A synapse must store an analog weight and apply it to a signal. A phase-change material (PCM) — canonical GST, Ge₂Sb₂Te₅, or the low-loss selenide GSST — clad on a waveguide does exactly this: its crystalline fraction sets the waveguide transmission, and hence the weight, and it holds that state indefinitely at *zero* static power. We examine SET/RESET switching physics, the GST-versus-GSST cascadability trade-off, multi-level analog storage and its drift and endurance limits, and the in-memory-computing principle that lets the multiplication happen passively as light traverses the cell (Ríos et al., 2015; Feldmann et al., 2019).

**Section 16.3 — Photonic SNN architectures.** Individual neurons and synapses must be wired into a network. The dominant scheme is WDM broadcast-and-weight: each neuron emits at a distinct wavelength onto a shared bus, and a bank of microring weights at each receiver — read out by balanced photodetection for signed weights — implements the synaptic weighted sum. This is the spiking use of the broadcast-and-weight fabric developed for matrix–vector products in Unit V (§12.4); here we emphasize the spiking substrate and the photonic spiking convolutional layer.

**Section 16.4 — Learning in photonic SNNs.** Finally, weights must be learned. We contrast two philosophies: local, unsupervised optical STDP, in which pulse coincidence at a PCM synapse programs the weight (Feldmann et al., 2019), and global, supervised surrogate-gradient training, in which the non-differentiable spike is replaced by a smooth surrogate so that backpropagation-through-time can train the network offline before its weights are deployed to hardware.

---

## How to Read This Chapter

This is a device- and architecture-oriented chapter: it complements the concepts of Chapter 15 rather than repeating them. Where an idea — LIF dynamics, STDP, broadcast-and-weight, reservoir computing — has already been developed elsewhere in the book, we reference it and move directly to the physics of its photonic realization. The recurring question throughout is quantitative: how fast, how small, how many levels, how much energy, and how far can it be scaled before a physical limit intervenes.

---

## References

- Nahmias, M.A., Shastri, B.J., Tait, A.N. & Prucnal, P.R. (2013). "A leaky integrate-and-fire laser neuron for ultrafast cognitive computing." *IEEE J. Sel. Top. Quantum Electron.*, 19(5), 1800212.
- Ríos, C., Stegmaier, M., Hosseini, P., Wang, D., Scherer, T., Wright, C.D., Bhaskaran, H. & Pernice, W.H.P. (2015). "Integrated all-photonic non-volatile multi-level memory." *Nature Photonics*, 9(11), 725–732.
- Feldmann, J., Youngblood, N., Wright, C.D., Bhaskaran, H. & Pernice, W.H.P. (2019). "All-optical spiking neurosynaptic networks with self-learning capabilities." *Nature*, 569(7755), 208–214.
- Prucnal, P.R. & Shastri, B.J. (2017). *Neuromorphic Photonics*. CRC Press.
- Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D. & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114.
