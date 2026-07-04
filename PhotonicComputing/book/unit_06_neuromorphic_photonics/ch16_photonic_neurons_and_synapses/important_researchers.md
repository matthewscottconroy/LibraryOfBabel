# Chapter 16: Important Researchers

---

## Mitchell A. Nahmias

Mitchell Nahmias, working in the Princeton neuromorphic-photonics group, is the principal architect of the leaky integrate-and-fire (LIF) laser neuron. His 2013 paper with Shastri, Tait, and Prucnal [Nahmias et al., *IEEE J. Sel. Top. Quantum Electron.*, 2013] showed that a two-section semiconductor laser with a saturable absorber maps *rigorously* onto the LIF model of computational neuroscience, and predicted spiking at gigahertz rates — roughly eight orders of magnitude faster than a biological neuron. This mapping is the theoretical cornerstone of Section 16.1: it lets the entire apparatus of spiking-neuron theory transfer to a photonic device. Nahmias later co-founded Luminous Computing, one of several startups spun out of the photonic-computing research community.

---

## Paul R. Prucnal, Bhavin J. Shastri, and Alexander N. Tait

These three, working together at Princeton and now leading their own efforts, founded much of modern neuromorphic photonics. **Paul Prucnal** (Princeton) established the Princeton program and, with his students, developed both the excitable-laser neuron and the broadcast-and-weight architecture. **Bhavin Shastri** (Queen's University, formerly Princeton) has driven the physics of photonic spiking neurons — including the graphene excitable laser [Shastri et al., *Scientific Reports*, 2016] — and is a lead author of the 2021 *Nature Photonics* roadmap that framed the whole field. **Alexander Tait** (Queen's University, formerly NIST and Princeton) invented the microring weight bank and the broadcast-and-weight network [Tait et al., *Journal of Lightwave Technology*, 2014; *Scientific Reports*, 2017], the WDM fabric that wires photonic neurons into networks and that Section 16.3 is built around. Together with co-author Thomas Ferreira de Lima, they wrote the field's defining monograph and reviews.

---

## Antonio Hurtado

Antonio Hurtado (University of Strathclyde) is the leading figure in spiking VCSEL neurons. His work — beginning with Javaloyes [Hurtado & Javaloyes, *Applied Physics Letters*, 2015] and continued with Robertson and colleagues [Robertson et al., *Scientific Reports*, 2020] — showed that the polarization-mode competition of an ordinary long-wavelength vertical-cavity laser can be harnessed to produce controllable sub-nanosecond optical spikes, and then to perform integration, inhibition, and pattern classification. Because VCSELs are cheap, directly modulated, and fabricable in dense two-dimensional arrays, Hurtado's approach is one of the most practical routes toward scalable photonic spiking hardware.

---

## Sylvain Barbay

Sylvain Barbay, at the Centre de Nanosciences et de Nanotechnologies (C2N, CNRS), studies excitability in micropillar lasers with saturable absorbers. His group's demonstration of a *relative refractory period* in an excitable semiconductor laser [Selmi et al., *Physical Review Letters*, 2014] is a landmark: the refractory period is a defining property of biological neurons, and reproducing it at nanosecond timescales in a compact laser deepened the physical case that these devices are genuine optical neurons rather than mere pulse generators.

---

## Wolfram H.P. Pernice and Harish Bhaskaran

Wolfram Pernice (Münster, later Heidelberg) and Harish Bhaskaran (Oxford) lead the two groups that, together, defined the phase-change photonic synapse. Their collaboration produced the first integrated all-photonic non-volatile multi-level memory [Ríos et al., *Nature Photonics*, 2015], the on-chip photonic synapse [Cheng et al., *Science Advances*, 2017], the first all-optical spiking neurosynaptic network with self-learning [Feldmann et al., *Nature*, 2019], and the integrated photonic tensor core [Feldmann et al., *Nature*, 2021]. Pernice contributed much of the device physics and fabrication of PCM on photonic waveguides; Bhaskaran's group has driven the memory, synapse, and tensor-core demonstrations. Between them they set the experimental state of the art for non-volatile, in-memory photonic computing that Section 16.2 describes.

---

## Johannes Feldmann

Johannes Feldmann (Münster and Oxford) is the lead experimentalist behind the two most influential demonstrations of PCM photonic computing: the all-optical spiking neurosynaptic network with an on-chip STDP-like learning rule [Feldmann et al., *Nature*, 2019], and the WDM-fed integrated photonic tensor core performing parallel convolution [Feldmann et al., *Nature*, 2021]. These two results are the reference points for both the synapse and the architecture sections of this chapter.

---

## Nathan Youngblood

Nathan Youngblood (Oxford, now University of Pittsburgh) has been central to the experimental realization of phase-change photonic memory and computing, as a co-author of the multi-level memory, spiking-network, tensor-core, and in-memory-computing demonstrations. His continuing work addresses the device-level challenges — multi-level precision, drift, and endurance — that determine whether PCM synapses can meet the demands of practical neuromorphic systems.

---

## C. David Wright

C. David Wright (University of Exeter) is a leading authority on phase-change materials and their use for computation. His conceptual work on "beyond von Neumann" computing with nanoscale phase-change devices [Wright et al., *Advanced Functional Materials*, 2013] articulated how a single device can both store and process information, motivating the in-memory-computing paradigm later realized on the photonic platform. He is a co-author of the key Oxford–Münster PCM photonics papers.

---

## Kaushik Roy

Kaushik Roy (Purdue University), a leader in energy-efficient neuromorphic computing, has extended the phase-change approach toward spiking networks through work with Chakraborty and Sengupta on all-photonic phase-change spiking neurons [Chakraborty et al., *Scientific Reports*, 2018] and a photonic in-memory computing primitive for spiking neural networks [Chakraborty et al., *Physical Review Applied*, 2019]. This work connects the device physics of phase-change materials to the algorithmic requirements of spiking neural networks, bridging Sections 16.2 and 16.4.
