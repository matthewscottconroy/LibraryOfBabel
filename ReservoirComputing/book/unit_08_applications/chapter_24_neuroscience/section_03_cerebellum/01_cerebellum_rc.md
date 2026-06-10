# 24.3.1 Cerebellum, Working Memory, and the Broader RC–Neuroscience Connection

## The Cerebellum: Established Anatomy and the RC Interpretation

### Established Anatomy

The cerebellum has one of the best-characterized anatomical structures in the brain. The following are **established facts** [Ito1984]:

- The cerebellum contains approximately $10^{11}$ granule cells (the most numerous neurons in the brain) and approximately $10^7$ Purkinje cells.
- **Granule cells** receive mossy fiber inputs from diverse sources (spinal cord, brainstem, cerebral cortex) and project to the molecular layer via parallel fibers.
- **Purkinje cells** integrate input from parallel fibers (from granule cells) and from climbing fibers (from the inferior olive). Purkinje cells are the sole output of the cerebellar cortex.
- **Climbing fibers** make powerful synaptic connections onto Purkinje cells — each Purkinje cell receives input from exactly one climbing fiber, which fires infrequently (~1 Hz) but produces strong complex spike responses.

This anatomy is **not in dispute** and has been confirmed by over 150 years of histological, electrophysiological, and imaging studies.

### Marr-Albus-Ito: The Supervised Learning Hypothesis

Three theorists independently proposed that the cerebellum implements supervised learning [Marr1969, Albus1971, Ito1989]:

- **Marr (1969):** The cerebellum learns to predict the sensory consequences of movements. The climbing fiber acts as a "teacher signal" that drives modification of the synaptic weights from parallel fibers to Purkinje cells.

- **Albus (1971):** Refined Marr's model. Albus proposed a specific error rule: climbing fiber activity represents the *error* in the Purkinje cell's prediction, and synaptic modification occurs at parallel fiber → Purkinje cell synapses in the direction that reduces this error. This is the **perceptron learning rule** applied to a biological circuit.

- **Ito (1989):** Provided substantial experimental evidence for long-term depression (LTD) at parallel fiber → Purkinje cell synapses, induced by the co-activation of climbing and parallel fibers. This is the **established synaptic plasticity result** that supports the Marr-Albus hypothesis.

**Epistemic status:** The existence of LTD at parallel fiber-Purkinje cell synapses and its induction by climbing fiber co-activation is an **established physiological fact** [Ito1989, Linden1994]. The interpretation that this LTD is the mechanism by which the cerebellum implements supervised learning is a **theoretical interpretation** that is widely accepted but not universally so. Alternative models propose different computational roles for LTD and for the cerebellum [Dean2010].

### The Reservoir Computing Interpretation

In the RC interpretation of the cerebellum [Doya1999, Mauk2004]:
- **Granule cells = reservoir.** The $10^{11}$ granule cells, with diverse responses to mossy fiber inputs, constitute a high-dimensional, nonlinear reservoir. Each granule cell receives input from 4–5 mossy fibers (very sparse input), and its output depends nonlinearly on their combination.
- **Purkinje cells = linear readout.** Each Purkinje cell reads out from approximately $10^5$ parallel fibers (each carrying the output of one granule cell). The Purkinje cell's response is approximately linear in its parallel fiber inputs (given the large number of weak inputs).
- **Climbing fibers = error signal.** The climbing fiber provides the teaching signal for adjusting the Purkinje cell's weights (LTD/LTP at parallel fiber synapses).

**What this interpretation adds.** The RC framework provides a unified explanation for why the cerebellum needs so many granule cells: they constitute a high-dimensional reservoir that can represent diverse spatiotemporal patterns of mossy fiber activity, giving the Purkinje cell's linear readout the rich basis it needs to learn arbitrary mappings from input to output.

**What remains uncertain.** Whether granule cells genuinely function as a random, generic reservoir (as assumed in the RC model) or whether their connectivity is structured and task-specific (as suggested by some anatomical data) is not established. The sparse input connectivity (4–5 mossy fibers per granule cell) is consistent with random sampling of the high-dimensional mossy fiber space, but it could also reflect structured wiring.

**Key references for this interpretation:**
- [Doya1999] Doya, K. (1999). What are the computations of the cerebellum, the basal ganglia and the cerebral cortex? *Neural Networks*, 12(7–8), 961–974.
- [Mauk2004] Mauk, M.D. & Buonomano, D.V. (2004). The neural basis of temporal processing. *Annual Review of Neuroscience*, 27, 307–340.

## Working Memory: Persistent Activity and Reservoir Models

**Established finding.** Neurons in the prefrontal cortex (PFC) and other association areas maintain elevated firing rates during the delay period of working memory tasks — after the stimulus has been removed but before the response is required. This **persistent activity** is an **established physiological finding**, replicated across many laboratories and species [Fuster1971, Goldman-Rakic1995].

**Computational model: Compte et al. 2000.** Compte, Brunel, Goldman-Rakic, and Wang published a biologically detailed model of persistent activity in PFC [Compte2000]. Their model:
- A network of excitatory (pyramidal) and inhibitory (interneuron) neurons with realistic synaptic dynamics
- With appropriate E/I balance, the network can maintain elevated activity during the delay period (a stable "bump attractor")
- The persistent activity encodes the content of working memory (e.g., the remembered stimulus location)

This model is a **computational model** of PFC dynamics, not a reservoir computer. But the reservoir framework provides a useful perspective:

**The reservoir interpretation of working memory.** In the RC framework, working memory corresponds to the reservoir state encoding a past input after the input has been removed. Specifically, if the input was $u_{t_0}$ (the stimulus) and the current time is $t > t_0$ (the delay period), then the reservoir state $\mathbf{x}_t$ is a decaying function of $u_{t_0}$.

The difference from fading memory: working memory requires *persistent* encoding — the activity must not fade significantly over the delay period. This corresponds to a reservoir operating near marginal stability (spectral radius $\rho \approx 1$), where some modes decay very slowly. The Compte et al. model achieves this through the specific E/I dynamics; the ESN achieves it by choosing $\rho$ close to 1.

**What the data suggest.** The data suggest that PFC activity during working memory tasks encodes relevant task variables in a distributed, high-dimensional manner [Rigotti2013]. This is consistent with a reservoir interpretation but does not exclude attractor-based or low-dimensional models.

**One interpretation is** that working memory capacity is limited by the reservoir's memory capacity (Chapter 7): the number of past stimuli that can be simultaneously decoded from the current state. This connects the RC memory capacity measure to behavioral measurements of working memory capacity.

## Ganguli et al. and the Compressibility of Neural Population Codes

Surya Ganguli and colleagues [Ganguli2012] analyzed the geometry of neural population activity in rodent hippocampus and found that the population activity is well-described by a low-dimensional smooth manifold — not a random, high-dimensional cloud. The low-dimensional manifold reflects the structured nature of the task (navigating an environment) and the structured connectivity of the hippocampus.

This finding is **interesting for the RC framework** because it suggests that biological "reservoirs" are not random. The hippocampus appears to have learned (through synaptic plasticity) a structured state representation suited to its computational tasks. The ESN's random connectivity is a first approximation — one that works surprisingly well computationally but misses the structured geometry of real neural circuits.

---

## References

- [Marr1969] Marr, D. (1969). A theory of cerebellar cortex. *Journal of Physiology*, 202(2), 437–470.
- [Albus1971] Albus, J.S. (1971). A theory of cerebellar function. *Mathematical Biosciences*, 10(1–2), 25–61.
- [Ito1989] Ito, M. (1989). Long-term depression. *Annual Review of Neuroscience*, 12(1), 85–102.
- [Linden1994] Linden, D.J. & Connor, J.A. (1994). Long-term synaptic depression. *Annual Review of Neuroscience*, 17(1), 341–366.
- [Doya1999] Doya, K. (1999). What are the computations of the cerebellum, the basal ganglia and the cerebral cortex? *Neural Networks*, 12(7–8), 961–974.
- [Compte2000] Compte, A., Brunel, N., Goldman-Rakic, P.S., & Wang, X.J. (2000). Synaptic mechanisms and network dynamics underlying spatial working memory in a cortical network model. *Cerebral Cortex*, 10(9), 910–923.
- [Fuster1971] Fuster, J.M. & Alexander, G.E. (1971). Neuron activity related to short-term memory. *Science*, 173(3997), 652–654.
- [Goldman-Rakic1995] Goldman-Rakic, P.S. (1995). Cellular basis of working memory. *Neuron*, 14(3), 477–485.
- [Ganguli2012] Ganguli, S. & Sompolinsky, H. (2012). Compressed sensing, sparsity, and dimensionality in neuronal information processing and data analysis. *Annual Review of Neuroscience*, 35, 485–508.
- [Rigotti2013] Rigotti, M. et al. (2013). The importance of mixed selectivity in complex cognitive tasks. *Nature*, 497(7451), 585–590.
