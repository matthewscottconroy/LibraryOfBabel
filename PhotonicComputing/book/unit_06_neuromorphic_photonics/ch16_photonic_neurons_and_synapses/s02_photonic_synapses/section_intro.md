# Section 16.2: Photonic Synapses — Nonvolatile Optical Weights

A spiking neural network is defined as much by its synapses as by its neurons. Where §16.1 built the excitable optical neuron — the element that integrates and fires — this section addresses the element that *connects* neurons: the synapse, whose job is to multiply a signal by a stored weight before it reaches the next cell. A useful neuromorphic processor needs these weights by the thousands, and it needs them to be **programmable** (so the network can be trained or reconfigured) and, ideally, **non-volatile** (so each weight persists without a continuous supply of energy to hold it).

Non-volatility is not a mere convenience. In an analog optical network a weight held by a thermally tuned microring or a biased modulator must be powered and refreshed continuously; the static power needed to maintain thousands of such weights quickly dominates the energy budget. A phase-change material (PCM) patch sitting on a waveguide sidesteps this entirely: it encodes its weight in the *structural state* of a few femtolitres of chalcogenide glass, which holds for years at zero standby power. The weight becomes a passive optical property of the waveguide — light that passes is automatically multiplied by a number frozen into the material.

This section develops photonic synapses in four steps. §16.2.1 introduces the canonical GST phase-change synapse and its all-optical write/read physics. §16.2.2 examines GSST, a selenium-alloyed PCM engineered for low loss and deep cascadability. §16.2.3 shows how storing the weight *in* the cell breaks the von Neumann bottleneck, turning propagation into computation. §16.2.4 treats the multi-level, analog nature of PCM storage — how many bits a synapse can hold, and what limits that precision.

---

## References

- Ríos, C., Stegmaier, M., Hosseini, P., Wang, D., Scherer, T., Wright, C.D., Bhaskaran, H. & Pernice, W.H.P. (2015). "Integrated all-photonic non-volatile multi-level memory." *Nature Photonics*, 9(11), 725–732.
- Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D. & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114.
