# 24.1.1 The Cortical Microcircuit as a Reservoir

## Epistemic Status of This Section

The claims in this section have the following statuses:

- **Established fact:** The neocortex contains recurrently connected networks of neurons with diverse time constants and nonlinear activation functions. Cortical neurons receive convergent input from many sources and project back to earlier processing stages.
- **Computational model:** Maass, Natschläger, and Markram [Maass2002] proposed that cortical microcircuits, modeled as liquid state machines (LSMs), can compute real-time temporal functions of their inputs.
- **Theoretical interpretation:** The idea that the cortex *functions as* a reservoir in the computational sense of Chapters 5–6 is a theoretical interpretation. It is not established that the specific mechanisms of LSM/ESN operation are the correct description of cortical computation.

We present the Maass et al. proposal in its full detail, while being precise about what the model claims and what it does not.

## The Cortical Microcircuit: Established Anatomy

The neocortex is organized into six layers (I–VI) with characteristic cell types and projection patterns. This is an **established anatomical fact** [Douglas1989]:

- **Layer II/III:** Pyramidal neurons that project to other cortical areas (cortico-cortical connections)
- **Layer IV:** Spiny stellate cells that receive thalamic input
- **Layer V:** Large pyramidal neurons projecting subcortically (to basal ganglia, spinal cord)
- **Layer VI:** Neurons projecting back to thalamus (feedback connections)
- **Interneurons (all layers):** GABAergic inhibitory neurons with diverse morphologies (basket cells, chandelier cells, Martinotti cells)

The microcircuit contains both excitatory (glutamatergic) and inhibitory (GABAergic) neurons, with recurrent connections within and between layers. The recurrent connectivity is dense: within a cortical column, each neuron may receive input from thousands of other nearby neurons.

**Why this looks like a reservoir.** The anatomical organization has several features that are consistent with (but not exclusive to) a reservoir interpretation:

1. **High dimensionality:** A cortical column contains approximately $10^4$–$10^5$ neurons (this is an **established order-of-magnitude estimate** from histology [Herculano-Houzel2017]).
2. **Recurrent connectivity:** The extensive lateral and feedback connections provide the recurrent dynamics necessary for temporal computation.
3. **Diverse time constants:** Different neuron types have different membrane time constants, ranging from $\sim 5$ ms (fast-spiking interneurons) to $\sim 50$ ms (regular-spiking pyramidal cells). This is an **established physiological fact** [McCormick1985].
4. **Nonlinear activation:** Neural spiking is a threshold-and-reset nonlinearity, distinct from (but functionally similar to) the tanh nonlinearity in digital reservoirs.

## The Maass et al. Liquid State Machine

Maass, Natschläger, and Markram [Maass2002] proposed the LSM as a computational model of cortical microcircuits, published in *Neural Computation* in 2002.

**Key claims of the paper:**

1. A randomly connected network of leaky integrate-and-fire neurons ("the liquid") can compute complex temporal functions of its inputs, using only a linear readout (the "readout map").

2. This is formalized through the **separation property** (SP) and **approximation property** (AP):
   - **SP:** Different input histories produce measurably different liquid states. This is the LSM analog of the echo state property.
   - **AP:** The readout map can approximate any function of the liquid state that can be computed from the input history.

3. A randomly constructed liquid (cortical microcircuit) generically satisfies SP and AP, making it a universal temporal processor.

**What this means biologically.** The Maass et al. model makes a **theoretical claim**: if the cortical microcircuit has the dynamics of a randomly connected leaky integrate-and-fire network, then it can compute real-time temporal functions using a linear readout (which might correspond to a downstream "readout" area). This is a testable hypothesis, not an established fact.

**What the data suggest.** Several lines of evidence are consistent with the LSM hypothesis [Maass2002, cited by Buonomano2009]:
- Cortical networks show diverse, high-dimensional responses to identical stimuli (consistent with SP)
- Downstream readout areas (e.g., prefrontal cortex) receive convergent input from widespread cortical regions (consistent with the linear readout architecture)
- Cortical responses contain information about past inputs for several hundred milliseconds (consistent with fading memory)

**Caveats and competing interpretations:**
- The LSM hypothesis does not predict or explain why specific cortical computations are implemented in specific areas
- Many cortical features (laminar specificity, dendritic computation, synaptic plasticity) are absent from the basic LSM model
- Alternative models (attractor networks, sequence generators, predictive coding) make different predictions about cortical dynamics

One interpretation is that the LSM/reservoir framework is a useful first approximation to cortical computation — capturing the key features of high-dimensionality and fading memory — while missing the learned structure that gives specific cortical areas their computational specialization.

## From LSM to ESN: The Modeling Simplification

The ESN (Chapter 5) is a simplified version of the LSM: it uses rate-coded neurons (continuous activations) rather than spiking neurons, and it uses fixed random weights rather than the structured cortical microcircuit anatomy. The simplification trades biological realism for mathematical tractability.

The echo state property of the ESN corresponds exactly to the separation property of the LSM: both require that the reservoir state converge to a unique functional of the input history. The sufficient conditions for ESP (Chapter 5) are the ESN version of the conditions for SP in the LSM.

**What the simplification loses:**
- Spike timing: spiking neurons can encode information in precise temporal patterns, not just rates
- Synaptic plasticity: real cortical circuits modify their connections in response to experience
- Detailed anatomy: layer-specific and cell-type-specific connectivity

**What the simplification retains:**
- The key computational principle: rich nonlinear dynamics can support universal temporal computation with a linear readout
- The order of magnitude scaling: the computational capacity scales with reservoir size

---

## References

- [Maass2002] Maass, W., Natschläger, T., & Markram, H. (2002). Real-time computing without stable states: A new framework for neural computation based on perturbations. *Neural Computation*, 14(11), 2531–2560.
- [Douglas1989] Douglas, R.J. & Martin, K.A.C. (1989). A functional microcircuit for cat visual cortex. *Journal of Physiology*, 440(1), 735–769.
- [Herculano-Houzel2017] Herculano-Houzel, S. (2017). Numbers of neurons as biological correlates of cognitive capability. *Current Opinion in Behavioral Sciences*, 16, 1–7.
- [McCormick1985] McCormick, D.A. et al. (1985). Comparative electrophysiology of pyramidal and sparsely spiny stellate neurons of the neocortex. *Journal of Neurophysiology*, 54(4), 782–806.
- [Buonomano2009] Buonomano, D.V. & Maass, W. (2009). State-dependent computations: Spatiotemporal processing in cortical networks. *Nature Reviews Neuroscience*, 10(2), 113–125.
