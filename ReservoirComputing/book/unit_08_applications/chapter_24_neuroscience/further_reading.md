# Chapter 24 — Further Reading and References

*The annotation for each reference includes its epistemic status: whether it reports data, builds models, or makes theoretical claims.*

---

## Essential References

### [Maass2002] — The LSM Paper

**Maass, W., Natschläger, T., & Markram, H. (2002). Real-time computing without stable states: A new framework for neural computation based on perturbations. *Neural Computation*, 14(11), 2531–2560.**

*Data/model/theory:* Primarily a **computational model** paper. Introduces the liquid state machine, proves theoretical properties (SP and AP), and connects to cortical microcircuit biology. Foundational for this chapter. The biological connections are theoretical proposals, not established facts.

### [Churchland2012] — The Rotational Dynamics Paper

**Churchland, M.M., Cunningham, J.P., Kaufman, M.T., Foster, J.D., Nuyujukian, P., Ryu, S.I., & Shenoy, K.V. (2012). Neural population dynamics during reaching. *Nature*, 487(7405), 51–56.**

*Data/model/theory:* An **experimental data paper** reporting a striking empirical finding (rotational structure in motor cortex population dynamics). The jPCA method is developed in this paper. The rotational structure is the data finding; its interpretation as dynamical-systems computation is theoretical.

### [Sussillo2009] — FORCE Learning

**Sussillo, D. & Abbott, L.F. (2009). Generating coherent patterns of activity from chaotic neural networks. *Neuron*, 63(4), 544–557.**

*Data/model/theory:* A **computational model** paper. Introduces the FORCE algorithm and demonstrates that it can train a chaotic RNN to produce coherent outputs. The connection to motor cortex is proposed, not demonstrated, in this paper.

### [Sussillo2015] — Motor Cortex Model vs. Data

**Sussillo, D., Churchland, M.M., Kaufman, M.T., & Shenoy, K.V. (2015). A neural network that finds a naturalistic solution for the production of muscle activity. *Nature Neuroscience*, 18(7), 1025–1033.**

*Data/model/theory:* A **model fitting to data** paper. Trains an RNN to reproduce actual muscle activity and compares internal dynamics to neural recordings. The quantitative match between model and data is the main contribution. This is the strongest evidence that the dynamical systems account is on the right track, though it is still not proof of mechanism.

---

## Cerebellar Models

### [Marr1969]

**Marr, D. (1969). A theory of cerebellar cortex. *Journal of Physiology*, 202(2), 437–470.**

*Theory:* A foundational **theoretical paper** proposing supervised learning in the cerebellum. Elegant and historically important. The specific plasticity predictions were partially confirmed by Ito; others remain uncertain.

### [Albus1971]

**Albus, J.S. (1971). A theory of cerebellar function. *Mathematical Biosciences*, 10(1–2), 25–61.**

*Theory:* Refines Marr's theory with a perceptron learning rule interpretation. Less well-known than Marr but more precise computationally.

### [Ito1989]

**Ito, M. (1989). Long-term depression. *Annual Review of Neuroscience*, 12(1), 85–102.**

*Data:* An **experimental review** establishing LTD at parallel fiber → Purkinje cell synapses as a real physiological phenomenon. This is one of the best-established experimental facts in the cerebellar literature. Note: establishing LTD is not the same as establishing the Marr-Albus learning rule; the connection from LTD to supervised learning is still theoretical.

### [Doya1999]

**Doya, K. (1999). What are the computations of the cerebellum, the basal ganglia and the cerebral cortex? *Neural Networks*, 12(7–8), 961–974.**

*Theory:* Proposes a mapping from supervised/reinforcement/unsupervised learning to cerebellum/basal ganglia/cortex. The reservoir interpretation of the cerebellum is consistent with (but not identical to) Doya's framework.

---

## Working Memory

### [Fuster1971]

**Fuster, J.M. & Alexander, G.E. (1971). Neuron activity related to short-term memory. *Science*, 173(3997), 652–654.**

*Data:* The original **experimental paper** describing persistent activity in prefrontal cortex during working memory. One of the most cited papers in systems neuroscience.

### [Compte2000]

**Compte, A., Brunel, N., Goldman-Rakic, P.S., & Wang, X.J. (2000). Synaptic mechanisms and network dynamics underlying spatial working memory in a cortical network model. *Cerebral Cortex*, 10(9), 910–923.**

*Model:* A biophysical **computational model** of persistent activity in PFC through E/I balance and attractor dynamics. The bump attractor model is the dominant computational model of working memory; it is distinct from (but related to) the reservoir account.

---

## Review Articles

### [Buonomano2009]

**Buonomano, D.V. & Maass, W. (2009). State-dependent computations: Spatiotemporal processing in cortical networks. *Nature Reviews Neuroscience*, 10(2), 113–125.**

*Review:* The best overview of the state-dependent computation framework in neuroscience. Accessible and comprehensive, with good discussion of what is established vs. theoretical.

### [Gallego2017]

**Gallego, J.A., Perich, M.G., Miller, L.E., & Solla, S.A. (2017). Neural manifolds for the control of movement. *Neuron*, 94(5), 978–984.**

*Data + theory:* A review of the neural manifold concept — the low-dimensional structure of motor cortex population activity — with discussion of its relationship to dynamical systems models.

---

## Critical Perspective

### [Krakauer2017]

**Krakauer, J.W., Ghazanfar, A.A., Gomez-Marin, A., MacIver, M.A., & Poeppel, D. (2017). Neuroscience needs behavior: Correcting a reductionist bias. *Neuron*, 93(3), 480–490.**

A critique of the tendency in systems neuroscience to interpret neural data through computational models without sufficient grounding in behavior. Relevant context for evaluating the RC/motor cortex interpretations: the question "does the data show rotation?" has a clear answer; the question "does motor cortex function as a reservoir?" is a much harder claim.
