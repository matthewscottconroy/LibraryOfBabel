# Chapter 16: Further Reading and References

---

## Textbooks and Reviews

**Prucnal, P.R. & Shastri, B.J. (2017). *Neuromorphic Photonics*. CRC Press.**
The definitive monograph on the field, written by the Princeton group that pioneered the excitable-laser neuron and the broadcast-and-weight architecture. It develops the LIF laser neuron, the WDM weighting fabric, and the systems arguments in a single coherent treatment; the primary reference for this entire chapter.

**Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D. & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114.**
The community roadmap. It surveys both the deep-learning-accelerator strand (Unit V) and the spiking/neuromorphic strand (this unit), and is the best single entry point for placing photonic neurons and synapses in the wider landscape of photonic AI hardware.

**Ferreira de Lima, T., Shastri, B.J., Tait, A.N., Nahmias, M.A. & Prucnal, P.R. (2017). "Progress in neuromorphic photonics." *Nanophotonics*, 6(3), 577–599.**
A focused review of photonic spike processing, covering excitable lasers, the mapping to LIF dynamics, and network integration. More device-oriented than the 2021 roadmap.

**Prucnal, P.R., Shastri, B.J., Ferreira de Lima, T., Nahmias, M.A. & Tait, A.N. (2016). "Recent progress in semiconductor excitable lasers for photonic spike processing." *Advances in Optics and Photonics*, 8(2), 228–299.**
A comprehensive, tutorial-length review of every major excitable-laser scheme — saturable-absorber lasers, injection-locked lasers, ring and microdisk lasers — with the underlying dynamical-systems theory. Essential for Section 16.1.

**Peng, H.-T., Nahmias, M.A., de Lima, T.F., Tait, A.N. & Shastri, B.J. (2018). "Neuromorphic photonic integrated circuits." *IEEE J. Sel. Top. Quantum Electron.*, 24(6), 6101715.**
Reviews the integration path — how neurons, weight banks, and detectors come together on a chip — and the engineering constraints (thermal tuning, footprint, control) that scaling must overcome.

---

## Excitable Laser Neurons

**Nahmias, M.A., Shastri, B.J., Tait, A.N. & Prucnal, P.R. (2013). "A leaky integrate-and-fire laser neuron for ultrafast cognitive computing." *IEEE J. Sel. Top. Quantum Electron.*, 19(5), 1800212.**
The foundational paper establishing that a two-section laser with a saturable absorber maps rigorously onto the LIF model, with spiking predicted at gigahertz rates — roughly eight orders of magnitude faster than a biological neuron.

**Shastri, B.J., Nahmias, M.A., Tait, A.N., Rodriguez, A.W., Wu, B. & Prucnal, P.R. (2016). "Spike processing with a graphene excitable laser." *Scientific Reports*, 6, 19126.**
An experimental excitable laser using a graphene saturable absorber, demonstrating spike generation, thresholding, and temporal integration — a hardware realization of the 2013 proposal.

**Hurtado, A. & Javaloyes, J. (2015). "Controllable spiking patterns in long-wavelength vertical cavity surface emitting lasers for neuromorphic photonics systems." *Applied Physics Letters*, 107(24), 241103.**
Introduces the spiking VCSEL neuron: a commercial 1300 nm VCSEL whose polarization-mode switching yields controllable sub-nanosecond spikes. Compact, directly modulated, and arrayable in two dimensions.

**Robertson, J., Hejda, M., Bueno, J. & Hurtado, A. (2020). "Ultrafast optical integration and pattern classification for neuromorphic photonics based on spiking VCSEL neurons." *Scientific Reports*, 10, 6098.**
Demonstrates temporal integration, inhibition, and pattern classification with spiking VCSEL neurons, advancing the VCSEL approach from a single spiking device toward functional processing.

**Selmi, F., Braive, R., Beaudoin, G., Sagnes, I., Kuszelewicz, R. & Barbay, S. (2014). "Relative refractory period in an excitable semiconductor laser." *Physical Review Letters*, 112(18), 183902.**
Reports a micropillar laser with saturable absorber exhibiting a *relative refractory period* — a hallmark neuronal property — at nanosecond timescales, strengthening the biological analogy at the device level.

**Coomans, W., Gelens, L., Beri, S., Danckaert, J. & Van der Sande, G. (2011). "Solitary and coupled semiconductor ring lasers as optical spiking neurons." *Physical Review E*, 84(3), 036209.**
Shows that counter-propagating mode competition in a semiconductor ring laser produces excitable spiking, providing an on-chip-integrable neuron distinct from the saturable-absorber approach.

**Van Vaerenbergh, T. et al. (2012). "Cascadable excitability in microrings." *Optics Express*, 20(18), 20292–20308.**
Demonstrates that carrier-induced nonlinearity in a silicon microring produces cascadable excitable spikes at low energy — an early passive-platform route to photonic neurons.

---

## Phase-Change Photonic Synapses and In-Memory Computing

**Ríos, C., Stegmaier, M., Hosseini, P., Wang, D., Scherer, T., Wright, C.D., Bhaskaran, H. & Pernice, W.H.P. (2015). "Integrated all-photonic non-volatile multi-level memory." *Nature Photonics*, 9(11), 725–732.**
The first integrated all-photonic non-volatile memory: GST on a waveguide, storing multiple distinct levels with year-scale retention at zero static power. The device foundation of the photonic synapse.

**Cheng, Z., Ríos, C., Pernice, W.H.P., Wright, C.D. & Bhaskaran, H. (2017). "On-chip photonic synapse." *Science Advances*, 3(9), e1700160.**
Explicitly frames the PCM-on-waveguide cell as a synapse, demonstrating weighted, plastic optical transmission suitable for neuromorphic use.

**Feldmann, J., Youngblood, N., Wright, C.D., Bhaskaran, H. & Pernice, W.H.P. (2019). "All-optical spiking neurosynaptic networks with self-learning capabilities." *Nature*, 569(7755), 208–214.**
The first all-optical spiking neurosynaptic network on a Si₃N₄ chip, combining PCM synapses with ring resonators and demonstrating an on-chip STDP-like learning rule. The central experimental reference for Sections 16.2 and 16.4.

**Feldmann, J. et al. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589(7840), 52–58.**
The integrated photonic tensor core: a PCM weight matrix fed by a WDM frequency comb, with on-chip germanium detectors, performing parallel convolution at very high MAC throughput. The benchmark demonstration of non-volatile in-memory photonic computing.

**Ríos, C., Youngblood, N., Cheng, Z., Le Gallo, M., Pernice, W.H.P., Wright, C.D., Sebastian, A. & Bhaskaran, H. (2019). "In-memory computing on a photonic platform." *Science Advances*, 5(2), eaau5759.**
Demonstrates scalar multiply–accumulate performed *in* the memory: light passing a PCM cell is automatically weighted, with no separate fetch — a direct optical assault on the von Neumann bottleneck.

**Wright, C.D., Hosseini, P. & Diosdado, J.A.V. (2013). "Beyond von-Neumann computing with nanoscale phase-change memory devices." *Advanced Functional Materials*, 23(18), 2248–2254.**
The conceptual paper articulating how phase-change devices can compute *and* store in the same physical element, motivating the in-memory-computing paradigm later realized photonically.

**Zhang, Y., Chou, J.B., Li, J. et al. (2019). "Broadband transparent optical phase change materials for high-performance nonvolatile photonics." *Nature Communications*, 10, 4279.**
Introduces GSST (Ge₂Sb₂Se₄Te₁), whose low optical loss in *both* states and large index contrast enable cascadable, low-insertion-loss phase-change synapses and switches — overcoming the crystalline-absorption limit of GST.

**Chakraborty, I., Saha, G., Sengupta, A. & Roy, K. (2018). "Toward fast neural computing using all-photonic phase change spiking neurons." *Scientific Reports*, 8, 12980.**
Proposes photonic spiking neurons built from phase-change materials, analyzing how PCM dynamics can implement integrate-and-fire behavior.

**Chakraborty, I., Saha, G. & Roy, K. (2019). "Photonic in-memory computing primitive for spiking neural networks using phase-change materials." *Physical Review Applied*, 11(1), 014063.**
Develops a PCM-based in-memory computing primitive tailored to spiking networks, linking the synapse and neuron functions within a single device paradigm.

---

## Broadcast-and-Weight and Architectures

**Tait, A.N., Nahmias, M.A., Shastri, B.J. & Prucnal, P.R. (2014). "Broadcast and weight: an integrated network for scalable photonic spike processing." *Journal of Lightwave Technology*, 32(21), 4029–4041.**
The architecture paper. Each neuron emits at a unique wavelength on a shared bus; microring weight banks and balanced detection implement the signed synaptic weighted sum. The organizing principle of Section 16.3.

**Tait, A.N., de Lima, T.F., Zhou, E., Wu, A.X., Nahmias, M.A., Shastri, B.J. & Prucnal, P.R. (2017). "Neuromorphic photonic networks using silicon photonics weight banks." *Scientific Reports*, 7, 7430.**
The experimental realization of a silicon-photonic weight-bank network (a 49-node demonstration), showing that the broadcast-and-weight fabric works on a real CMOS-compatible platform.

**Tait, A.N., Wu, A.X., de Lima, T.F., Zhou, E., Shastri, B.J., Nahmias, M.A. & Prucnal, P.R. (2016). "Microring weight banks." *IEEE J. Sel. Top. Quantum Electron.*, 22(6), 312–325.**
The detailed device study of the microring weight bank: tuning, control, precision, and crosstalk — the engineering that sets how many WDM channels a bank can weight.

**Vandoorne, K. et al. (2014). "Experimental demonstration of reservoir computing on a silicon photonics chip." *Nature Communications*, 5, 3541.**
An integrated photonic reservoir; included here as a cross-reference. The reservoir-computing paradigm is treated in depth in Unit V (§13.4), and is revisited in Project 16.4 with spiking nodes.

---

## SNN Learning

**Neftci, E.O., Mostafa, H. & Zenke, F. (2019). "Surrogate gradient learning in spiking neural networks." *IEEE Signal Processing Magazine*, 36(6), 51–63.**
The tutorial reference for surrogate-gradient training: because the spike threshold is non-differentiable, its derivative is replaced by a smooth surrogate so that backpropagation-through-time can train an SNN offline. Underpins the supervised route of Section 16.4.

**Zenke, F. & Ganguli, S. (2018). "SuperSpike: supervised learning in multilayer spiking neural networks." *Neural Computation*, 30(6), 1514–1541.**
Introduces SuperSpike, a concrete surrogate-gradient rule for multilayer SNNs, including the fast-sigmoid surrogate derivative used in this chapter's worked examples.
