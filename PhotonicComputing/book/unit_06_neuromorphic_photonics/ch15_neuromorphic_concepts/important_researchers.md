# Chapter 15: Important Researchers

---

## Carver Mead (Caltech)

Carver Mead spent his career at the California Institute of Technology, where he was a pioneer of very-large-scale integration (VLSI) and, with Lynn Conway, co-authored the textbook that taught a generation how to design integrated circuits. In the 1980s he turned to the question of whether silicon could emulate the nervous system not by numerically simulating equations but by exploiting the analog physics of transistors operated in their sub-threshold regime, where the exponential current–voltage relationship echoes the behavior of ion channels. He coined the term *neuromorphic* for this approach, set it out in his book *Analog VLSI and Neural Systems* [Mead, 1989] and his programmatic paper [Mead, *Proc. IEEE*, 1990], and built early silicon retinas and cochleas. Every subsequent neuromorphic platform — electronic or photonic — descends intellectually from this program.

---

## Wolfgang Maass (Graz University of Technology)

Wolfgang Maass, a computer scientist at the Graz University of Technology in Austria, provided much of the theoretical foundation for spiking neural networks. His influential paper [Maass, *Neural Networks*, 1997] classified neural-network models into three generations — McCulloch–Pitts threshold units, analog rate-based (sigmoidal) networks, and *spiking* networks that compute with the timing of individual pulses — and argued that this third generation is, for a given number of units, at least as powerful as the second and strictly more efficient for temporally coded tasks. This "third generation" framing is why spiking networks are studied as a distinct computational paradigm rather than a biological curiosity, and it directly motivates the temporal-coding, event-driven architectures of neuromorphic photonics. Maass also contributed to the theory of reservoir computing through the liquid-state-machine model, connecting this chapter to the reservoir architectures of Unit V.

---

## Paul R. Prucnal (Princeton)

Paul Prucnal is a professor of electrical engineering at Princeton University and the founder of its neuromorphic-photonics program. After earlier foundational work in optical networking, he and his students established the modern paradigm of photonic spike processing: the excitable laser neuron and the broadcast-and-weight architecture for wiring such neurons together. With Bhavin Shastri he co-authored *Neuromorphic Photonics* [Prucnal & Shastri, 2017], the field's definitive monograph. The Princeton group he built — including Shastri, Tait, Nahmias, and Ferreira de Lima — has authored a large fraction of the primary literature this unit draws on.

---

## Bhavin J. Shastri (Queen's University)

Bhavin Shastri, now a professor at Queen's University in Canada after research at Princeton, is a leading figure in photonic spiking neurons and neuromorphic photonics. His experimental and theoretical work spans excitable lasers (including a graphene excitable laser for spike processing), photonic spike computation, and the systems integration of weight banks with laser neurons. He co-authored the 2017 *Neuromorphic Photonics* monograph with Prucnal and is the lead author of the 2021 *Nature Photonics* roadmap, "Photonics for artificial intelligence and neuromorphic computing" [Shastri et al., 2021], the most cited recent survey of the field.

---

## Mitchell A. Nahmias (Princeton; co-founder, Luminous Computing)

Mitchell Nahmias, who did his doctoral work in the Princeton group, is the lead author of the paper that introduced the *leaky integrate-and-fire laser neuron* [Nahmias et al., *IEEE JSTQE*, 2013] — an excitable semiconductor laser with a saturable-absorber section whose dynamics map rigorously onto the LIF model, predicted to spike at GHz rates some eight orders of magnitude faster than biology. He later co-founded Luminous Computing, a startup pursuing large-scale photonic computing hardware. His work established the single most-used device abstraction in this unit: the laser as a physical LIF neuron.

---

## Alexander N. Tait (Queen's University; formerly NIST and Princeton)

Alexander Tait, now a professor at Queen's University after doctoral work at Princeton and a research appointment at the U.S. National Institute of Standards and Technology (NIST), is the principal architect of the *microring weight bank* and the *broadcast-and-weight* network introduced in Unit V, §12.4. His weight banks — arrays of tunable add–drop microrings with balanced photodetection to realize signed weights — are the interconnect fabric that wires photonic spiking neurons together, and his demonstration of a 49-node silicon weight-bank network [Tait et al., *Sci. Rep.*, 2017] is a landmark of the field. In this unit his architecture reappears not as a matrix multiplier but as the synaptic wiring between excitable laser neurons.
