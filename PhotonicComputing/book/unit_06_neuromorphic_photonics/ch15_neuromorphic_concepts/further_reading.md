# Chapter 15: Further Reading and References

---

## Essential Textbooks

**Prucnal, P.R. & Shastri, B.J. (2017). *Neuromorphic Photonics*. CRC Press.**
The definitive monograph on the field, written by the founders of the Princeton program. It develops the excitable laser neuron, the broadcast-and-weight architecture, and the systems view of photonic spike processing that this entire unit follows. The single most important reference for Units VI as a whole.

**Gerstner, W. & Kistler, W.M. (2002). *Spiking Neuron Models: Single Neurons, Populations, Plasticity*. Cambridge University Press.**
The standard graduate text on spiking-neuron theory. Its treatments of the leaky integrate-and-fire model, neural coding, and spike-timing-dependent plasticity are the clearest available, and they underpin the mathematics of §15.1. Freely readable and rigorous.

**Mead, C. (1989). *Analog VLSI and Neural Systems*. Addison-Wesley.**
The founding text of neuromorphic engineering, in which Mead lays out the philosophy of using device physics — subthreshold transistors — to emulate rather than simulate the nervous system. Historically essential context for why "neuromorphic" means what it does.

---

## Neuroscience Foundations

**Hodgkin, A.L. & Huxley, A.F. (1952). "A quantitative description of membrane current and its application to conduction and excitation in nerve." *Journal of Physiology*, 117(4), 500–544.**
The Nobel-winning biophysical model of the action potential, from which the LIF neuron is a drastic but useful reduction. Read it to appreciate what the simplified models leave out.

**Lapicque, L. (1907). "Recherches quantitatives sur l'excitation électrique des nerfs." *J. Physiol. Pathol. Gen.*, 9, 620–635.**
The original integrate-and-fire neuron, predating Hodgkin–Huxley by nearly half a century. Of historical interest as the ancestor of the model used throughout this unit.

**FitzHugh, R. (1961). "Impulses and physiological states in theoretical models of nerve membrane." *Biophysical Journal*, 1(6), 445–466.**
**Nagumo, J., Arimoto, S. & Yoshizawa, S. (1962). "An active pulse transmission line simulating nerve axon." *Proceedings of the IRE*, 50(10), 2061–2070.**
Together these give the two-variable FitzHugh–Nagumo excitable system — the reduced model of excitability that the excitable lasers of Chapter 16 map onto. Essential background for understanding *why* a laser biased near threshold behaves like a neuron.

**Bi, G.-Q. & Poo, M.-M. (1998). "Synaptic modifications in cultured hippocampal neurons: dependence on spike timing, synaptic strength, and postsynaptic cell type." *Journal of Neuroscience*, 18(24), 10464–10472.**
**Markram, H., Lübke, J., Frotscher, M. & Sakmann, B. (1997). "Regulation of synaptic efficacy by coincidence of postsynaptic APs and EPSPs." *Science*, 275(5297), 213–215.**
The two experimental papers that established spike-timing-dependent plasticity: synaptic change depends on the millisecond-scale ordering of pre- and postsynaptic spikes. The STDP window of §15.1 comes directly from this work.

**Maass, W. (1997). "Networks of spiking neurons: the third generation of neural network models." *Neural Networks*, 10(9), 1659–1671.**
The paper that framed spiking networks as a distinct, more powerful "third generation" of neural models. It is the theoretical charter for treating spiking computation as its own paradigm rather than a biological footnote.

---

## Neuromorphic Hardware

**Mead, C. (1990). "Neuromorphic electronic systems." *Proceedings of the IEEE*, 78(10), 1629–1636.**
The paper that coined the term and set the agenda. Compact and readable; the origin point of the entire field.

**Indiveri, G., et al. (2011). "Neuromorphic silicon neuron circuits." *Frontiers in Neuroscience*, 5, 73.**
A comprehensive survey of analog and mixed-signal silicon neuron circuits, tracing the lineage from Mead's ideas to modern designs. The best single entry point to the electronic-neuron literature.

**Merolla, P.A., et al. (2014). "A million spiking-neuron integrated circuit with a scalable communication network and interface." *Science*, 345(6197), 668–673.**
The IBM TrueNorth chip: one million neurons and 256 million synapses at about 70 mW. A landmark in ultra-low-power digital neuromorphic hardware.

**Davies, M., et al. (2018). "Loihi: a neuromorphic manycore processor with on-chip learning." *IEEE Micro*, 38(1), 82–99.**
Intel's Loihi: an asynchronous manycore chip with programmable on-chip learning rules. The reference for modern digital neuromorphic architecture and its later Loihi 2 successor.

**Furber, S.B., Galluppi, F., Temple, S. & Plana, L.A. (2014). "The SpiNNaker project." *Proceedings of the IEEE*, 102(5), 652–665.**
The Manchester SpiNNaker machine, which simulates large spiking networks in biological real time using a bespoke ARM-based, packet-switched fabric. A different bet from the analog and digital-ASIC approaches.

---

## Neuromorphic Photonics Reviews

**Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D. & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114.**
The current roadmap of photonic AI and neuromorphic computing, spanning optical neural networks, in-memory computing, and spiking photonics. The best place to see how Chapter 15's concepts connect to Chapter 16's hardware and to Unit V.

**Ferreira de Lima, T., Shastri, B.J., Tait, A.N., Nahmias, M.A. & Prucnal, P.R. (2017). "Progress in neuromorphic photonics." *Nanophotonics*, 6(3), 577–599.**
A focused review of photonic spike processing and the excitable-laser-plus-weight-bank paradigm. Reads as the natural companion to the Prucnal & Shastri textbook and a bridge into the primary literature of Chapter 16.
