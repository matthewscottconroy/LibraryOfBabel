# Section 4: Neuromorphic Computing

## The Problem with How We Compute

There is something deeply strange about modern computing. A graphics processing unit running a deep neural network performs trillions of floating-point operations per second, consuming hundreds of watts of power, to do something that a child does effortlessly — recognize a face. A child's brain performs this feat using roughly twenty watts across its entire operation, including the portions not involved in face recognition. The GPU approach is, by the metric of operations per watt, many orders of magnitude less efficient than the biological system it ostensibly emulates.

The inefficiency is architectural. Standard digital computers — GPUs included — use the von Neumann architecture: a central processing unit separated from memory, connected by a bus that must transfer data for every operation. When a neural network runs on this hardware, every multiplication and addition requires fetching weights from memory, performing the operation, and writing the result back. The memory-processor bottleneck — the von Neumann bottleneck — is a fundamental throughput limit, and moving data across the bus is far more expensive in energy than the computation itself (Indiveri et al., 2011).

The brain has no such separation. Memory and processing are co-located in the synapse: the connection between neurons stores a weight (the synaptic strength) and is also the site where the computation (multiplication of input spike by weight) occurs. There is no data bus to cross. Computation happens where memory lives. This in-memory computing architecture is fundamentally different from von Neumann computing, and it is one of the main reasons the brain is so much more energy-efficient than silicon hardware doing nominally similar computations.

Neuromorphic computing is the project of building silicon hardware that implements something closer to in-memory computation with spiking dynamics — computing like a brain rather than computing on a von Neumann architecture that models a brain.

## Spiking Neural Networks

The neurons in the brain do not communicate by sending continuous real-valued signals. They communicate with discrete action potentials — spikes — that propagate along axons and trigger synaptic release at their terminals. A spike is an all-or-nothing event, roughly 1 millisecond in duration, at roughly +40 mV. The information carried by a neuron is encoded not in the amplitude of the spike (all spikes are essentially identical) but in the timing and rate of spiking.

Artificial neural networks as used in deep learning abstract away this spike-based communication. Each artificial neuron computes a real-valued activation (a weighted sum of inputs, passed through a nonlinearity) and passes this real value to its downstream neighbors. This is computationally convenient — it allows gradient-based training using backpropagation — but it is biologically unrealistic and, more importantly for our purposes, inefficient to implement in hardware.

Spiking neural networks (SNNs) use neurons that communicate with discrete spikes, as in the biological case (Maass, 1997). Each neuron integrates input spikes over time, maintains an internal "membrane potential," and fires a spike when that potential crosses a threshold, after which it resets. The key advantage for hardware implementation is sparsity: in any given time step, most neurons are silent. A spike-based communication system only needs to transmit information when neurons fire, which at any moment is a small fraction of the network. In the brain, neurons fire at average rates of perhaps 1–10 Hz, while the network could theoretically fire at rates of hundreds of Hz. The sparsity of neural activity is a major source of energy efficiency.

Training spiking neural networks is harder than training standard neural networks, because the spike-firing nonlinearity is discontinuous and therefore not directly differentiable. Multiple approaches have been developed: converting trained standard networks to SNN format, surrogate gradient methods, and biologically inspired local learning rules like spike-timing-dependent plasticity (STDP) (Pfeiffer & Pfeil, 2018). None of these fully matches the ease and performance of backpropagation on standard deep networks, which remains a significant limitation. But the energy efficiency of SNNs on specialized hardware can be orders of magnitude better than equivalent standard networks, which motivates continued research.

## Intel Loihi and IBM TrueNorth

Two of the most prominent neuromorphic chips developed to date are Intel's Loihi family and IBM's TrueNorth, both of which implement on-chip populations of spiking neurons with local learning rules.

IBM's TrueNorth chip, described in detail by Merolla and colleagues (2014), implements approximately one million programmable spiking neurons and 256 million synaptic connections on a chip consuming roughly 65 milliwatts at its core. This is a specific demonstration that spiking neural computation can be implemented in silicon at low power. The chip was designed for inference (running a pre-trained network) rather than on-chip learning, but it demonstrated the feasibility of massively parallel, brain-inspired computation at biologically plausible power levels.

Intel's Loihi chip (Davies et al., 2018), and its successor Loihi 2, went further by implementing on-chip learning: synaptic weights on the chip can be modified based on spike timing patterns, allowing the chip to learn in response to its inputs without sending data off-chip for training. This is qualitatively different from TrueNorth and far closer to the biological paradigm, where learning and computation are co-located in the synapse. Loihi 2 uses programmable learning rules, allowing researchers to implement STDP and other local learning algorithms in hardware. Demonstrated applications include adaptive sensory processing, constraint satisfaction problems, and sparse coding of sensory data, in each case with energy efficiency substantially better than GPU equivalents.

The theoretical maximum efficiency of neuromorphic chips, if the biological paradigm is faithfully implemented at the physical level, is estimated at several orders of magnitude better than current GPU-based deep learning (Furber, 2016). We are not close to that limit yet — both TrueNorth and Loihi are still substantially less efficient than the brain per useful computation — but the trajectory is encouraging.

## In-Memory Computing

A related but distinct direction addresses the von Neumann bottleneck more directly, without necessarily committing to spiking dynamics. In-memory computing performs analog computation within memory arrays, using the physical properties of memory cells to implement mathematical operations. Resistive memory technologies — including phase-change memory, spin-transfer torque magnetic RAM, and memristors — allow the resistance of a memory cell to encode a synaptic weight, and Ohm's law (current = voltage × conductance) to implement the multiplication of an input signal by that weight as a physical process within the memory array (Ielmini & Wong, 2018).

The result is that a matrix-vector multiply — the fundamental operation of a neural network layer — can be implemented in a single step in a memory array, with no data transfers across a bus. The energy cost is the cost of applying voltages to the array and reading the resulting currents, which can be small. Prototype in-memory computing chips for neural network inference have demonstrated energy efficiency in the range of tens of tera-operations per second per watt, compared to roughly tens of giga-operations per second per watt for GPU-based neural network inference — a roughly three-order-of-magnitude advantage (Burr et al., 2017).

In-memory computing does not implement the full biological paradigm — it uses analog continuous signals rather than spikes, and the memory cells encoding weights are typically updated by external training rather than local learning rules. But it addresses the same fundamental problem that motivated neuromorphic computing: the energy cost of moving data between memory and processing. In this sense, it implements a key insight from biological computation — that memory and processing should be co-located — in a form that is more immediately compatible with existing deep learning workflows.

## The Biological Comparison

Throughout this book, we have examined the computational properties of cells and cellular systems. What emerges from this examination, relevant to neuromorphic computing, are several principles:

**Local learning.** Cells update their behaviors based on local signals — the concentration of ligands at the receptor, the pattern of inputs to a signaling network, the history of membrane potential. There is no global supervisor. Spike-timing-dependent plasticity, Hebbian learning, and other local learning rules are attempts to capture this principle in silicon. The success of these rules in biological systems — and the fact that they can, in principle, implement gradient descent (Lillicrap et al., 2016) — suggests that local learning is not a compromise but a viable computational principle.

**Temporal coding.** Biological neurons use the precise timing of spikes, not just their rate, to encode and process information. The temporal dimension of neural computation is largely absent from standard deep learning, which processes data in a fixed number of forward passes rather than continuously over time. Implementing temporal coding in hardware is technically challenging but potentially essential for the kind of real-time, continuous processing that biological systems perform.

**Plasticity across timescales.** Biological synapses change their strength on timescales ranging from milliseconds (short-term facilitation and depression) to years (long-term potentiation). This multi-timescale plasticity allows the system to maintain both rapid adaptation and long-term memory simultaneously — a capacity that standard deep learning, with a single weight matrix updated by batch gradient descent, does not naturally implement.

**Metabolic coupling.** Biological neurons are tightly coupled to their metabolic state. Synaptic transmission and plasticity depend on ATP availability, oxygen tension, and the activity of glial cells that maintain the ionic environment. This metabolic coupling means that the "energy available for computation" and "the computation performed" are not separated; they are intimately linked. Neuromorphic chips that implement metabolic constraints — that reduce computation when energy is scarce and allocate it efficiently when it is available — would be implementing something that biological brains do naturally.

## The Distance Remaining

It would be dishonest to end this section without acknowledging how far neuromorphic computing remains from the biological ideal. The best current neuromorphic chips implement a few million neurons and a few billion synapses — roughly the scale of a bee's nervous system, not a human brain, and with far lower functional sophistication than a bee. Training methods for spiking neural networks remain substantially less effective than backpropagation. In-memory computing devices suffer from device-to-device variability and drift that make precision computation difficult. The on-chip learning rules available on current hardware capture only a fraction of the plasticity mechanisms present in biological synapses.

Most fundamentally, we do not understand in detail how the brain's computation works — we do not have a precise account of how specific spike patterns represent specific information, how local learning rules implement useful global computations, or how the brain's many plasticity mechanisms interact. Without this understanding, the goal of building a brain-inspired computer is constrained by our incomplete blueprint.

These limitations are real, and they counsel appropriate humility. But the direction is right: toward distributed, in-memory, spiking, locally-learning architectures that exploit the physical properties of their substrate for computation. This is the direction that four billion years of evolution found optimal. That it requires substantial engineering effort to implement in silicon does not diminish the promise.

---

## References

Burr, G. W., Shelby, R. M., Sebastian, A., Kim, S., Kim, S., Sidler, S., ... & Narayanan, P. (2017). Neuromorphic computing and engineering. *IEEE Transactions on Electron Devices*, 64(10), 4137–4156.

Davies, M., Srinivasa, N., Lin, T. H., Chinya, G., Cao, Y., Choday, S. H., ... & Wang, H. (2018). Loihi: A neuromorphic manycore processor with on-chip learning. *IEEE Micro*, 38(1), 82–99.

Furber, S. (2016). Large-scale neuromorphic computing systems. *Journal of Neural Engineering*, 13(5), 051001.

Ielmini, D., & Wong, H.-S. P. (2018). In-memory computing with resistive switching devices. *Nature Electronics*, 1(6), 333–343.

Indiveri, G., Linares-Barranco, B., Hamilton, T. J., van Schaik, A., Etienne-Cummings, R., Delbruck, T., ... & Chicca, E. (2011). Neuromorphic silicon neuron circuits. *Frontiers in Neuroscience*, 5, 73.

Lillicrap, T. P., Cownden, D., Tweed, D. B., & Akerman, C. J. (2016). Random synaptic feedback weights support error backpropagation for deep learning. *Nature Communications*, 7, 13276.

Maass, W. (1997). Networks of spiking neurons: The third generation of neural network models. *Neural Networks*, 10(9), 1659–1671.

Merolla, P. A., Arthur, J. V., Alvarez-Icaza, R., Cassidy, A. S., Sawada, J., Akopyan, F., ... & Modha, D. S. (2014). A million spiking-neuron integrated circuit with a scalable communication network and interface. *Science*, 345(6197), 668–673.

Pfeiffer, M., & Pfeil, T. (2018). Deep learning with spiking neurons: Opportunities and challenges. *Frontiers in Neuroscience*, 12, 774.
