# Section 15.1: Biological Neurons and Spiking Neural Networks

The brain is the existence proof that motivates neuromorphic engineering: on the order of $10^{11}$ neurons and $10^{14}$ synapses perform robust perception, learning, and control on roughly 20 W — less than a household light bulb, and far more energy-efficient than any digital machine at the same tasks. Understanding *how* it does this is where photonic neuromorphic design begins.

The functional unit is the neuron. Its **dendrites** form a branching tree that collects synaptic input from thousands of upstream cells; each input is *excitatory* (driving the cell toward firing) or *inhibitory* (driving it away). The **soma** integrates these inputs across its membrane in space and time. When the membrane voltage crosses a threshold, the soma emits an **action potential** — a stereotyped, all-or-nothing spike about 1 ms wide. The **axon** carries that spike, without attenuation, to downstream targets, where each **synapse** converts it into a postsynaptic current scaled by the synapse's *strength*. That strength is the biological weight, and adjusting it is how the brain learns.

Three properties make this style of computation compelling for hardware. It is **spike-based**: information rides on discrete, identical events, so signaling tolerates amplitude noise. It is **event-driven**: a neuron burns energy only when it spikes, so idle elements cost almost nothing. And it is **co-located and parallel**: weights (memory) and integration (computation) sit together, sidestepping the von Neumann bottleneck. These are exactly the properties photonic neuromorphic systems chase — with optical pulses replacing action potentials at picosecond rather than millisecond scales.

This section builds the vocabulary. Subsection 15.1.1 introduces the **leaky integrate-and-fire (LIF)** model. Subsection 15.1.2 contrasts **rate and temporal coding**. Subsection 15.1.3 presents **spike-timing-dependent plasticity (STDP)**, the learning rule that Chapter 16 realizes in phase-change photonic synapses.

## References

Gerstner, W. & Kistler, W.M. (2002). *Spiking Neuron Models: Single Neurons, Populations, Plasticity*. Cambridge University Press.

Hodgkin, A.L. & Huxley, A.F. (1952). "A quantitative description of membrane current and its application to conduction and excitation in nerve." *Journal of Physiology*, 117(4), 500–544.
