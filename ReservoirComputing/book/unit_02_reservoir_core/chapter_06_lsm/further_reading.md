# Chapter 6: Further Reading

## Annotated Bibliography

---

### [Maass2002] Maass, W., Natschläger, T., and Markram, H. (2002). Real-time computing without stable states: A new framework for neural computation based on perturbations. *Neural Computation*, 14(11), 2531-2560.

The founding document of liquid state machines. This paper does three things: (1) it introduces the LSM computational framework and states the computation theorem; (2) it implements a biologically realistic spiking LSM using LIF neurons with TM synapses, with parameters taken from the experimental neuroscience literature; and (3) it demonstrates the LSM's computational power on three tasks — speech recognition (spoken digit classification), temporal integration, and spatiotemporal pattern discrimination.

The paper's central claim — that a randomly connected network of spiking neurons can perform useful computation via a linear readout — was greeted with skepticism by some and enthusiasm by others. The skeptics noted that the tasks demonstrated were relatively simple and that the network parameters had been tuned (in a biologically motivated but still somewhat arbitrary way) to produce good results. The enthusiasts recognized that the *principle* was new and important, regardless of the specific implementation.

What makes this paper enduring is not its empirical results (which have been superseded many times) but its conceptual framework: the three conditions (SP, AP, FMP), the kernel quality metric, and the liquid metaphor. These provide the vocabulary for the entire LSM literature.

**What to read:** The full paper. Pay special attention to Section 2 (the computational framework and three conditions), Section 3 (the biological network model), and Section 4 (the numerical experiments). The appendix contains the proof of the computation theorem.

---

### [Bertschinger2004] Bertschinger, N. and Natschläger, T. (2004). Real-time computation at the edge of chaos in recurrent neural networks. *Neural Computation*, 16(7), 1413-1436.

The landmark paper on the edge of chaos in reservoir computing. Bertschinger and Natschläger consider random binary networks and measure the information processing capacity $C = \sum_k I(x_t; u_{t-k})$ as a function of the coupling strength $J$. Their main finding: $C$ peaks sharply at the critical coupling $J_c = 1/\beta$ (the phase transition between ordered and chaotic dynamics).

The paper provides both numerical evidence (from direct simulation) and theoretical insight (from mean-field analysis of the order parameter). It establishes that the edge of chaos is not merely a metaphor but a precise, quantitative concept with measurable consequences for information processing.

This paper is essential reading for understanding *why* $\rho \approx 1$ is the optimal operating point. Without it, the design rule is just an empirical observation. With it, the rule has a principled information-theoretic foundation.

**What to read:** All of it. The setup (Section 2) introduces the model; the numerical results (Section 3) demonstrate the capacity peak; the theoretical analysis (Section 4) provides the mean-field derivation of $J_c$; and the discussion (Section 5) connects the results to biological neural networks. The key figure is Figure 3, which shows the capacity $C$ as a function of $J$ — the peak at $J_c$ is visually dramatic.

---

### [Maass2004] Maass, W. and Markram, H. (2004). On the computational power of circuits of spiking neurons. *Journal of Computer and System Sciences*, 69(4), 593-616.

This theoretical paper examines the computational power of spiking neural networks from a computational complexity perspective. The main result is that circuits of spiking neurons — which encode information in the precise timing of spikes — are computationally more powerful than circuits of rate-coded neurons of the same size, for certain classes of computation. Specifically, the paper shows that spiking networks can implement temporal XOR and other functions of spike timing that rate-coded networks cannot compute with the same resource constraints.

For the reservoir computing reader, the main takeaway is: spiking dynamics are not just a biological detail. They add genuine computational power that is not captured by rate-coded (ESN) approximations. Whether this extra power is relevant for practical applications depends on whether the task requires precise spike timing, which many real-world tasks do not.

**What to read:** The introduction and the main theorems (Theorems 1-3). The proofs are technical and can be skimmed on first reading.

---

### [TsodykMarkram1997] Tsodyks, M. and Markram, H. (1997). The neural code between neocortical pyramidal neurons depends on neurotransmitter release probability. *Proceedings of the National Academy of Sciences*, 94(2), 719-723.

The original TM synapse model paper. Based on patch-clamp recordings from pairs of connected pyramidal neurons in rat somatosensory cortex, Tsodyks and Markram documented the history-dependent nature of synaptic transmission: a neuron's response to a presynaptic action potential depends strongly on the recent firing history of the presynaptic neuron. They introduced the phenomenological model (the TM model) to describe this history dependence in terms of vesicle depletion and utilization factors.

This paper is foundational for understanding short-term synaptic plasticity in cortical networks. For the LSM reader, it provides the biological motivation and parameter values for the TM synapse model used in the Maass et al. 2002 LSM simulations.

**What to read:** The first half (experimental results) explains what is observed in recordings. The second half (model) introduces the mathematical description. Both are clearly written and accessible to readers without deep electrophysiology background.

---

### [DayanAbbott2001] Dayan, P. and Abbott, L.F. (2001). *Theoretical Neuroscience: Computational and Mathematical Modeling of Neural Systems*. MIT Press.

The standard graduate-level textbook on computational neuroscience. For readers approaching LSMs from a machine learning background, this book provides the essential background on:
- Neural encoding (Chapter 1): how neurons represent information in their spike trains.
- Neural decoding (Chapter 3): how to read out neural population activity (directly relevant to the LSM readout problem).
- The Hodgkin-Huxley model (Chapter 5) and the integrate-and-fire model (Chapter 5): the biophysical models that underlie the LIF neuron.
- Network models (Chapter 7): mean-field analysis of recurrent networks, stability analysis, attractor dynamics.

For the reservoir computing reader, Chapters 5, 7, and 8 (synaptic plasticity) are most directly relevant.

**What to read:** Chapter 5 (LIF derivation and f-I curve), Chapter 7 (network dynamics and stability), and Chapter 3 (neural decoding — the readout problem from a neuroscience perspective). At 420 pages, the full book is worth reading for anyone serious about the neuroscience connection.

---

### [Beggs2003] Beggs, J.M. and Plenz, D. (2003). Neuronal avalanches in neocortical circuits. *Journal of Neuroscience*, 23(35), 11167-11177.

The landmark paper demonstrating neural avalanches — power-law distributed cascades of neural activity — in rat cortical slices. Beggs and Plenz recorded spontaneous activity from local field potentials (LFPs) in organotypic cortical slices using 60-electrode arrays, and found that the distribution of "avalanche" sizes (defined as consecutive time bins with above-threshold LFP amplitude in a spatially connected cluster) follows a power law $P(s) \propto s^{-3/2}$.

This $s^{-3/2}$ exponent is the prediction of a critical branching process (branching ratio $\sigma = 1$), providing direct experimental evidence that cortical networks operate near the critical branching ratio. The paper was a major driver of subsequent theoretical work on criticality in neural networks and its connection to computational capacity.

**What to read:** The experimental results (Figures 1-4) are accessible without deep technical background. The discussion section carefully evaluates alternative interpretations and addresses potential criticisms. Follow-up work by Beggs and others (including in human MEG and EEG recordings) provides broader validation.

---

### [Brunel2000] Brunel, N. (2000). Dynamics of sparsely connected networks of excitatory and inhibitory spiking neurons. *Journal of Computational Neuroscience*, 8(3), 183-208.

A theoretical paper analyzing the steady-state dynamics of large networks of leaky integrate-and-fire neurons with a mixture of excitatory (E) and inhibitory (I) connections. Brunel derived the mean-field equations for the E/I network and computed the phase diagram as a function of the E/I balance and network coupling strength. The key result is a phase diagram with four regions: synchronous regular firing, asynchronous irregular firing (the biologically realistic regime), and two types of synchronized oscillatory states.

The asynchronous irregular (AI) regime — corresponding to E/I balance and near-critical connectivity — is the state most consistent with experimental observations of cortical activity in behaving animals. It is also, not coincidentally, the state most consistent with the edge-of-chaos hypothesis for optimal reservoir computation.

**What to read:** The introduction (motivation and summary of results), the phase diagram (Figure 7), and the discussion of the AI regime. The derivations are detailed but follow standard mean-field methods.
