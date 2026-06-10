# Neuronal Cultures as Reservoirs

## 32.1.1 The Case for Living Reservoirs

All preceding chapters have treated reservoir computing as a computational architecture implemented in silicon (digital), optics, mechanics, or quantum hardware. This chapter asks a different question: can biological neural tissue serve as a reservoir? And if so, what does reservoir computing theory tell us about how neurons compute?

The answer to the first question is: yes, under carefully controlled conditions, biological neurons exhibit the key properties required for reservoir computing — nonlinear dynamics, high-dimensional state space, fading memory, and separation of input-dependent states. The second question is harder and more controversial; we treat it with appropriate care.

## 32.1.2 In Vitro Neuronal Cultures

**In vitro neuronal cultures** are networks of dissociated neurons grown on glass substrates coated with adhesion proteins. When neurons from the cortex, hippocampus, or spinal cord of rodent embryos are dissociated and plated on a surface, they self-organize over days to weeks into connected networks exhibiting spontaneous electrophysiological activity.

**Key properties of neuronal cultures:**

- **Spontaneous bursting:** Cultures exhibit spontaneous synchronous bursting activity — episodes of high-frequency firing involving large fractions of the network, separated by quiescent periods [Mahowald & Douglas 1991]. This reflects the recurrent excitatory connections formed during self-organization.

- **Multi-electrode arrays (MEAs):** Cultures are grown on MEAs with 60–256 electrodes. Each electrode records local field potentials (LFPs) and spike trains from 1–10 nearby neurons. Recordings provide access to 60–256 independent readout channels.

- **Input via electrical stimulation:** Patterns of electrical pulses delivered through MEA electrodes can drive specific neurons or patches of the culture, allowing controlled input delivery.

- **Fading memory:** Cultures show memory of stimulation patterns for 100–500 ms [Bakkum et al. 2008], providing the temporal memory needed for reservoir computing. The memory timescale corresponds to recurrent excitatory dynamics and synaptic vesicle recovery.

## 32.1.3 Reservoir Computing with MEA Cultures

The reservoir computing protocol for neuronal cultures was developed by [Bakkum et al. 2008]:

1. **Input encoding:** The input signal $u(t)$ is encoded as a spatiotemporal pattern of electrical stimulation. Different input values are represented by different stimulation patterns (e.g., stimulation electrode identity, pulse amplitude, or pulse timing).

2. **State collection:** LFPs and spike times from all 60+ recording electrodes are collected during and after stimulation. Each electrode provides one "neuron state" $x_i(t)$.

3. **Readout training:** The state vector $\mathbf{x}(t) = (x_1(t), \ldots, x_M(t)) \in \mathbb{R}^M$ (from $M$ recording channels) is used to train a linear readout via ridge regression, targeting the desired output $y(t)$.

The MEA culture implements a biological equivalent of the echo state network: random recurrent connections (formed by self-organization), fixed weights during testing (no deliberate synaptic plasticity on training timescales), and a linear readout trained offline.

## 32.1.4 Memory and Separation Properties

For the MEA culture reservoir to be useful, it must satisfy two key requirements:

**1. Fading memory.** The culture must respond differently to different recent input histories. [Bakkum et al. 2008] measured the memory capacity of cortical cultures using random binary inputs and found linear memory capacity $C_L \approx 5$–$15$ time steps (for time steps of 100 ms). This is lower than typical ESN memory capacity ($C_L \sim N/4$) but sufficient for short-term temporal pattern recognition.

**2. Separation property (kernel quality).** Distinct input sequences must drive the culture to distinct network states. For MEA cultures, the separation property is satisfied empirically: distinct stimulation patterns produce distinct spatiotemporal LFP patterns, measurable by PCA of the electrode recordings [Mahowald & Douglas 1991].

The fading memory timescale (100–500 ms) places the neuronal culture at the fast end of the biological memory range. For longer memory tasks, additional biological structures (hippocampus, prefrontal cortex) that maintain persistent activity over seconds to minutes would be required.

## 32.1.5 DishBrain: Neurons Playing Pong

The most widely reported biological RC experiment is **DishBrain** [Kagan et al. 2022], in which mouse cortical neurons grown on a MEA were reported to "play" the video game Pong. The experimental setup:

- **Input:** The position of the Pong paddle and ball were encoded as stimulation patterns on different MEA electrode groups.
- **Output:** The firing rate of neurons on specific electrode groups was decoded to produce paddle movement commands.
- **Feedback:** Stimulation patterns providing "reward" (regular stimulation) vs. "punishment" (irregular stimulation) feedback were delivered based on game performance.

The neurons' behavior changed over $\sim 5$ minutes of play: the paddle-ball miss rate decreased, consistent with learning. The paper claimed this demonstrated "sentient behavior" in neuronal cultures.

**The reservoir interpretation.** From the reservoir computing perspective, the DishBrain system is a closed-loop biological reservoir: the culture processes input (game state), the readout produces a control signal (paddle movement), and the feedback (stimulation reward/punishment) modifies the reservoir's internal dynamics. The improvement in game performance is consistent with:
- Spike-timing-dependent plasticity (STDP) modifying synaptic weights in response to the feedback stimulation
- Short-term synaptic facilitation/depression changing effective connectivity transiently

**Epistemic caution.** The DishBrain paper used the word "sentient" in a way that many neuroscientists consider premature and misleading [Smirnova et al. 2023]. The neurons demonstrating fewer misses over time is a result that can be explained by standard forms of non-conscious synaptic plasticity. The paper does not demonstrate consciousness, cognition, or subjective experience in any established scientific sense. The reservoir computing framework provides a more parsimonious explanation: the neurons are an adaptive reservoir whose dynamics change in response to feedback, without any requirement for sentience.

## 32.1.6 Limitations of Neuronal Culture Reservoirs

**Biological variability.** Unlike digital ESNs, neuronal cultures are not reproducible: each plate grows differently, with different connectivity, different spontaneous activity patterns, and different fading memory timescales. This makes systematic comparison across experiments difficult.

**Short operational window.** Cultures are viable for 2–8 weeks before degrading. The reservoir cannot be "stored" and reused like a digital ESN.

**Limited readout channels.** MEAs provide at most 256 recording channels, far fewer than the $N = 10^3$–$10^5$ neurons that are active. The readout is effectively a 256-dimensional projection of a much higher-dimensional state.

**Input bandwidth.** MEA stimulation is limited in bandwidth and spatial precision: individual neurons cannot typically be addressed independently, and stimulation artifacts often contaminate the recording channels.

## References

- Bakkum, D. J., Gamblen, P. M., Ben-Ary, G., Chao, Z. C., and Potter, S. M. (2008). MEART: The semi-living artist. *Frontiers in Neurorobotics*, 2, 5.
- Kagan, B. J., Kitchen, A. C., Tran, N. T., et al. (2022). In vitro neurons learn and exhibit sentience when embodied in a simulated game-world. *Neuron*, 110(23), 3952–3969.
- Mahowald, M. and Douglas, R. (1991). A silicon neuron. *Nature*, 354(6354), 515–518.
- Smirnova, L., Caffo, B. S., Bhattacharya, S., et al. (2023). Organoid intelligence (OI): The new frontier in biocomputing and intelligence-in-a-dish. *Frontiers in Science*, 1, 1017235.
