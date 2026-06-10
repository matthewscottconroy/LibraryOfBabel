# Chapter 6: Key Researchers

---

## Wolfgang Maass

**Affiliation:** Graz University of Technology (Technische Universität Graz), Austria. Institute of Theoretical Computer Science.

**Role in reservoir computing:** Maass is the inventor of the liquid state machine and the co-founder (with Jaeger) of the reservoir computing paradigm. A theoretical computer scientist by training, Maass brought to reservoir computing a deep familiarity with computational complexity theory, approximation theory, and the neuroscience of cortical circuits. His approach was explicitly biological: he wanted to understand how the brain could compute without the stable attractors and trained recurrent weights that dominated the computational neuroscience models of the 1990s.

**The 2002 LSM paper [Maass2002]:** "Real-time computing without stable states: A new framework for neural computation based on perturbations." *Neural Computation* 14(11), 2531-2560. This is the founding document of liquid state machines. It introduces the three conditions (SP, AP, FMP), states and proves the LSM computation theorem, and presents numerical simulations of biologically realistic spiking networks on classification and integration tasks. The paper is notable for its combination of mathematical rigor and biological grounding: the network parameters are taken from the experimental literature, and the tasks are chosen to reflect the kinds of computations the brain must perform in real time.

**Subsequent theoretical work:** Maass continued to develop the theoretical foundations of the LSM throughout the 2000s. Key contributions include: the analysis of computational power of spiking networks vs. analog networks (showing that spiking can be computationally more powerful under some conditions); the study of the role of short-term plasticity in computation [Maass2002b]; and the generalization of the LSM framework to include feedback and output-dependent dynamics.

**The 2004 perspective [Maass2004]:** "On the computational power of circuits of spiking neurons." *J. Computer and System Sciences* 69(4), 593-616. Examines the relationship between the temporal coding capacity of spiking networks and their computational power, showing that spiking networks can compute functions not computable by rate-coded networks of the same size.

**The Maass-Markram collaboration [Maass2004b]:** "Temporal integration of sensory and motor aspects of corollary discharge..." — one of several papers examining the computational role of cortical dynamics in sensorimotor integration, showing how the LSM framework applies to specific cortical circuits.

**Teaching legacy:** Maass has been a dedicated teacher of theoretical neuroscience and neural computation. His lecture notes and course materials from Graz have influenced a generation of computational neuroscientists. He has also been actively involved in the European Human Brain Project (HBP), applying computational neuroscience tools to the grand challenge of understanding the human brain.

**Recommended papers:**
- [Maass2002] "Real-time computing without stable states": the foundational LSM paper
- [Maass2002b] "Real-time computing and computational complexity in spiking neural networks with dynamic synapses"
- [MaassMarkram2004] "On the computational power of recurrent circuits of spiking neurons"

---

## Thomas Natschläger

**Affiliation:** At the time of the foundational LSM work, Institute of Theoretical Computer Science, Graz University of Technology. Later worked in industry (Numerical Analysis research at Software Competence Center Hagenberg and at Siemens).

**Role in reservoir computing:** Natschläger was Maass's primary collaborator on the computational and simulation work behind the liquid state machine. He implemented the biologically realistic LIF/TM network simulations, developed the kernel quality measurement methodology, and conducted the numerical experiments that demonstrated the LSM's computational power on real-time tasks. His contribution was essential for converting Maass's theoretical framework into a working computational system.

**The Bertschinger-Natschläger paper [Bertschinger2004]:** "Real-time computation at the edge of chaos in recurrent neural networks." *Neural Computation* 16(7), 1413-1436. This is arguably the most cited paper in the edge-of-chaos literature in neural computation. Co-authored with Nils Bertschinger (then at the Max Planck Institute for Mathematics in the Sciences in Leipzig), this paper provided the quantitative demonstration that information processing capacity peaks at the critical connectivity. The paper introduced the order parameter framework for analyzing reservoir dynamics and established the edge of chaos as the optimal operating regime. Natschläger brought the simulation expertise; Bertschinger brought the information-theoretic framework.

**Recommended papers:**
- [Bertschinger2004] "Real-time computation at the edge of chaos": information processing capacity analysis
- [MaassNatschläger2002] The core LSM papers with computational demonstrations

---

## Henry Markram

**Affiliation:** École Polytechnique Fédérale de Lausanne (EPFL), Switzerland. Brain Mind Institute, Laboratory of Neural Microcircuitry.

**Role in reservoir computing:** Markram is one of the world's leading experimental neuroscientists, best known for his work characterizing the detailed anatomy and physiology of cortical microcircuits. His contribution to the LSM framework came primarily through the Tsodyks-Markram synapse model [TsodykMarkram1997], which provides the biological substrate for short-term synaptic plasticity in the LSM. Without the TM model, the LSM would use static synapses, significantly reducing its biological realism and (arguably) its computational richness.

**The Tsodyks-Markram model [TsodykMarkram1997]:** "The neural code between neocortical pyramidal neurons depends on neurotransmitter release probability." *PNAS* 94(2), 719-723. This paper introduced the phenomenological model of short-term synaptic dynamics that now bears Tsodyks and Markram's names. Based on patch-clamp recordings from pyramidal neurons in rat somatosensory cortex, the paper showed that synaptic efficacy varies dramatically with firing rate in a manner consistent with the depletion-recovery model. The parameters of the TM model were fit to these experimental recordings and were later used by Maass et al. as the synaptic model for the LSM.

**The Blue Brain Project:** Markram founded the Blue Brain Project at EPFL in 2005, with the audacious goal of creating a detailed computational model of a cortical column. The project has produced some of the most detailed simulations of neural microcircuits ever undertaken. While not directly a reservoir computing project, it has generated an enormous amount of data and validated simulation methods that are directly relevant to building realistic LSMs.

**The Human Brain Project:** Markram was the principal investigator of the European Human Brain Project (HBP), a €1 billion, ten-year project to create a comprehensive simulation of the human brain. The HBP has been controversial — both in its scientific scope and its organizational management — but it represents the most ambitious attempt to date to understand the brain as a computing machine.

**Recommended papers:**
- [TsodykMarkram1997] "The neural code between neocortical pyramidal neurons depends on neurotransmitter release probability"
- [Markram1997] "Regulation of synaptic efficacy by coincidence of postsynaptic APs and EPSPs" (*Science*, spike-timing-dependent plasticity)

---

## Nils Bertschinger

**Affiliation:** At the time of the edge-of-chaos paper, Max Planck Institute for Mathematics in the Sciences, Leipzig, Germany. Later at Frankfurt Institute for Advanced Studies and other institutions.

**Role in reservoir computing:** Bertschinger made the key theoretical contribution of connecting the edge of chaos in neural networks to information processing capacity [Bertschinger2004], giving a precise, quantitative reason why $\rho \approx 1$ (or $J \approx J_c$) is the optimal operating point. His approach brought tools from information theory (mutual information, capacity) and statistical physics (order parameters, phase transitions) to bear on the reservoir computing problem.

**The edge-of-chaos capacity result:** The key insight of the Bertschinger-Natschläger paper is that information processing capacity $C = \sum_k I(x_t; u_{t-k})$ peaks exactly at the phase transition. This is not obvious: one might expect that the chaotic phase, which is more "sensitive" to inputs, would also be better at retaining information about them. Bertschinger showed that chaotic sensitivity is actually detrimental: while the chaotic network is sensitive to recent inputs, it is unreliable — the same input produces different outputs on different trials due to the exponential amplification of noise. Only at the critical point is the network both sensitive and reliable.

**Recommended papers:**
- [Bertschinger2004] "Real-time computation at the edge of chaos in recurrent neural networks": the foundational capacity analysis

---

## Peter Tino

**Affiliation:** University of Birmingham, School of Computer Science, UK.

**Role in reservoir computing:** Tino has made significant contributions to the theoretical foundations of reservoir computing, particularly to the analysis of memory capacity, the geometry of reservoir state space, and the relationship between reservoir computing and kernel methods. His work provides some of the most rigorous theoretical underpinnings available for the field.

**Geometric analysis of reservoir computing [Tino2020]:** "Dynamical Systems as Temporal Feature Machines." Provides a geometric perspective on reservoir computing, analyzing the structure of the reservoir's state manifold and how it relates to computational capacity. This work connects reservoir computing to the theory of random feature maps and kernel methods.

**Memory capacity bounds:** Tino and collaborators have extended the memory capacity analysis of Jaeger (linear case, $MC \leq N$) to nonlinear reservoirs, deriving bounds on capacity that depend on the topology and dynamics of the reservoir. This theoretical work is directly relevant to understanding how to design reservoirs for maximum capacity.

**Recommended papers:**
- [Tino2020] "Dynamical Systems as Temporal Feature Machines"
- [Tino2001] "Spatial representation of symbolic sequences through iterative function systems": early work on reservoir-like dynamical systems for sequence processing
