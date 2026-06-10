# Chapter 4 Key Researchers

---

## Herbert Jaeger

**Born:** 1961, Germany  
**Affiliation:** Constructor University Bremen (Professor, 2010–present; formerly Jacobs University Bremen); GMD – German National Research Center for Information Technology (2000–2003); previously at Technische Universität Braunschweig, University of Bielefeld  
**Key Contribution:** Inventor of the Echo State Network; developer of the echo state property, the practical training algorithm for reservoir computers, and the theory of memory capacity

Herbert Jaeger is the inventor of the echo state network, the founder of the practical reservoir computing paradigm, and one of the central figures in the development of reservoir computing theory. His intellectual path to the ESN reflects his background as both a dynamical systems theorist and a practical engineer: he was simultaneously interested in the formal mathematical properties of recurrent systems and in building systems that actually worked.

Jaeger joined the GMD (later absorbed into the Fraunhofer Institute) in Bremen around 2000, at a time when the RNN training literature was rich with clever partial solutions to the vanishing gradient problem but none had definitively resolved the fundamental difficulty. His insight — that the recurrent weights could simply be set randomly and the problem collapsed to a linear regression — was, as he has noted, motivated partly by a desire to understand what trained recurrent weights were actually doing, and partly by the pragmatic recognition that gradient-based training was prone to failure in ways that were hard to diagnose.

The 2001 technical report [Jaeger2001] introduced the echo state property, the echo state network, and the basic training procedure. The 2004 *Science* paper with Haas [Jaeger2004] brought the work to a much broader audience by demonstrating state-of-the-art performance on chaotic time series prediction. The 2002 technical report on memory capacity [Jaeger2002mem] introduced the formal definition of linear memory capacity and proved that it is bounded by $N$ for $N$-unit reservoirs — a fundamental theoretical result.

Over the subsequent two decades, Jaeger's theoretical contributions expanded significantly. He developed the **conceptor** framework [Jaeger2014] — a method for training reservoir computers to store and manage multiple dynamical patterns, blending reservoir computing with autoassociative memory. This work, though mathematically complex, is among the most sophisticated developments in the theoretical understanding of what reservoir computers can and cannot represent.

Jaeger's approach to research is notable for its combination of formal rigor and intellectual humility. His papers characteristically acknowledge limitations, open questions, and the connections between his results and prior work by others — a quality not universal in machine learning. He has been a consistent voice for theoretical grounding in a field that sometimes prioritizes empirical results over understanding.

---

## Wolfgang Maass

**Born:** 1954, Austria  
**Affiliation:** Graz University of Technology (Professor of Theoretical Computer Science, 1991–present); Institute of Theoretical Computer Science  
**Key Contribution:** Inventor of the Liquid State Machine; theoretical computer science of neural computation; computational neuroscience of cortical microcircuits

Wolfgang Maass is a theoretical computer scientist whose primary research focus is the computational theory of biological neural networks. He arrived at the Liquid State Machine framework from a direction nearly orthogonal to Jaeger's: while Jaeger was thinking about how to make RNN training tractable, Maass was thinking about how the brain computes.

Maass's background is in formal language theory and computational complexity, and he brought this perspective to computational neuroscience in the 1990s, when he began working on the computational properties of spiking neural networks. His earlier work established that networks of spiking neurons are capable of universal computation [Maass1997], placing neural computation within the landscape of computability theory.

The Liquid State Machine emerged from the collaboration with Thomas Natschläger and Henry Markram at EPFL around 2001–2002. Markram's group had detailed biophysical models of cortical microcircuits derived from actual experimental data, and the question was whether these circuits — with their constant, highly irregular activity and absence of stable attractor states — could nevertheless perform useful computations. Maass's insight was that the irregular dynamics were not a problem to be solved but a resource to be exploited: the high-dimensional, continuously evolving state of the circuit encoded the input history in a way that any readout neuron could exploit.

The 2002 *Neural Computation* paper [Maass2002] presented the LSM framework with a universal approximation theorem, demonstrated on biologically realistic cortical circuit models, and established the two key conditions (separation property and fading memory) that are now standard vocabulary in the field.

Maass's subsequent work has been equally influential. His theoretical work on the computational principles underlying cortical computation — particularly the role of recurrent circuits in working memory, motor control, and prediction — has shaped how computational neuroscientists think about the function of cortical dynamics. His collaborations with Markram on the Blue Brain Project have given his theoretical framework direct contact with experimental neuroscience at the level of specific circuits and synapses.

Maass received the Austrian Science Prize in 2005 and has been recognized by multiple national academies. He is perhaps the figure who most clearly exemplifies the convergence between machine learning and computational neuroscience that characterizes the reservoir computing paradigm.

---

## Thomas Natschläger

**Born:** 1969, Austria  
**Affiliation:** Software Competence Center Hagenberg (SCCH), Austria (2004–present); formerly at Graz University of Technology  
**Key Contribution:** Co-inventor of the Liquid State Machine; early empirical studies of LSM properties; bridge between LSM theory and simulation

Thomas Natschläger was Maass's primary collaborator on the Liquid State Machine papers and contributed both to the theoretical formulation and to the numerical simulations that demonstrated the LSM's computational properties. His technical role was to implement and run the large-scale simulations of biologically realistic cortical circuits that served as the empirical backbone of the 2002 paper [Maass2002].

Natschläger's subsequent work at SCCH has focused on applying machine learning methods to industrial software engineering problems, but his early contributions to the LSM framework — particularly the computational experiments that bridged the gap between the abstract theory and concrete dynamical systems — were essential to the field's early development. He is also a co-developer of the **NEMO** simulation framework for LSMs, which enabled other researchers to build on the original results.

---

## Henry Markram

**Born:** 1962, South Africa  
**Affiliation:** École Polytechnique Fédérale de Lausanne (EPFL) (Professor, 1995–present); founder and director of the Blue Brain Project (2005–present) and the Human Brain Project (2013–2023)  
**Key Contribution:** Co-inventor of the Liquid State Machine; experimental and computational characterization of cortical microcircuit connectivity; spike-timing-dependent plasticity

Henry Markram is one of the most ambitious and controversial figures in contemporary neuroscience. His experimental work on cortical circuits — characterizing the precise connectivity, synaptic dynamics, and firing properties of neurons in the neocortex — provided the biophysical foundation for the LSM simulations in the 2002 paper.

Markram's specific contribution to the LSM was the detailed biological data: the connectivity rules, synaptic time constants, and neuron type distributions that Natschläger's simulations used to construct the "liquid." Maass provided the mathematical framework; Markram provided the biological substrate. The result was a demonstration that actual cortical circuits — not just abstract random networks — satisfy the conditions for universal temporal computation.

Beyond the LSM, Markram is known for his discovery of spike-timing-dependent plasticity (STDP) [Markram1997] — the synaptic learning rule by which the relative timing of pre- and post-synaptic spikes determines the sign and magnitude of synaptic weight changes. STDP is now a central concept in computational and experimental neuroscience and is relevant to reservoir computing as a potential mechanism for learning the readout weights in biological systems.

Markram's Blue Brain Project — which aims to construct a detailed digital reconstruction of a mammalian brain — has been both celebrated as visionary and criticized as premature. Whatever the long-term outcome, the project has generated detailed anatomical and physiological data about cortical circuits that continue to inform computational models, including those in the reservoir computing tradition.

---

## Dean Buonomano

**Affiliation:** University of California, Los Angeles (Professor of Neurobiology and Psychology)  
**Key Contribution:** Computational theory of timing in the brain; state-dependent network computation; early theoretical work connecting neural dynamics to temporal processing

Dean Buonomano's research centers on how the brain processes time — how neural circuits encode duration, sequence, and temporal structure. His theoretical framework, developed through the 1990s and 2000s [Buonomano1995, Buonomano2000], proposed that timing in the brain is not achieved by a dedicated "clock" but by the evolving states of recurrent neural networks: the current network state carries information about when an event occurred, because the state encodes how much time has passed since the event perturbed the network.

This framework — **state-dependent network computation** — is conceptually very close to the reservoir computing paradigm, and Buonomano has explicitly acknowledged the connection [Buonomano2009]. The key parallel: in state-dependent networks, the "memory" of past events is stored not in stable attractors but in the transient dynamics of the network's state trajectory. The readout of timing information is performed by output neurons that receive input from the recurrent network's current state — exactly the reservoir readout.

Buonomano's work provides an important bridge between the computational neuroscience and the engineering communities: his state-dependent networks are not just a theoretical abstraction but are grounded in specific predictions about the behavior of real neural circuits, with experimental evidence from both in vitro and in vivo recordings.

---

## Michael Mauk

**Affiliation:** University of Texas at Austin (Professor of Neuroscience)  
**Key Contribution:** Theory of cerebellar timing and learning; long-term synaptic plasticity; neural mechanisms of classical conditioning

Michael Mauk's research focuses on the cerebellum as a model system for understanding how the brain learns and times motor behaviors. His computational models of cerebellar learning [Mauk1997] propose that the granule cell layer of the cerebellum acts as a sparse, high-dimensional representation of motor state — essentially a reservoir — while the Purkinje cells act as a trained linear readout.

This parallel was noted explicitly by Maass and colleagues and is one of the strongest pieces of evidence that the reservoir computing paradigm may not just be a convenient engineering simplification but may reflect actual computational principles at work in biological neural systems. The cerebellum's anatomy — a large, randomly-connected mossy fiber-granule cell layer that expands a low-dimensional input into a high-dimensional state space, with adjustable synaptic weights on Purkinje cell dendrites — is strikingly similar to the reservoir + readout architecture.

Mauk's work suggests that the reservoir computing paradigm may be, in this sense, "discovered by evolution" as an efficient solution to the problem of temporal computation under biological constraints.

---

## Danil Verstraeten

**Affiliation:** Ghent University, Department of Electronics and Information Systems (ELIS); PhD completed 2009  
**Key Contribution:** Unification of ESN and LSM frameworks under the "reservoir computing" label; systematic experimental comparisons; reservoir computing for speech recognition

Danil Verstraeten is primarily responsible for the name and the conceptual unification that created "reservoir computing" as a coherent field. His 2007 *Neural Networks* paper [Verstraeten2007], co-authored with Benjamin Schrauwen, Michiel d'Haene, and Dirk Stroobandt, was the first to systematically compare the ESN and LSM frameworks, demonstrate their mathematical equivalence, and propose "reservoir computing" as the unifying term.

The paper is notable not just for the naming but for its experimental methodology: Verstraeten et al. ran both ESN and LSM on a common set of benchmarks, showed that their performance was comparable, and identified the key design principles that determined performance in both cases. This experimental unification was essential to the field's development, because it meant that results from the ESN community and the LSM community were mutually applicable.

Verstraeten's PhD thesis and subsequent papers also made important contributions to reservoir computing for speech recognition and audio processing — one of the domains where the temporal memory and nonlinear mixing properties of reservoirs are most naturally suited to the task.

---

## References

- [Jaeger2001] Jaeger, H. (2001). The "echo state" approach to analysing and training recurrent neural networks. *GMD Report 148*.
- [Jaeger2002mem] Jaeger, H. (2002). Short term memory in echo state networks. *GMD Report 152*.
- [Jaeger2004] Jaeger, H., & Haas, H. (2004). Harnessing nonlinearity: Predicting chaotic systems and saving energy in wireless communication. *Science*, 304(5667), 78–80.
- [Jaeger2014] Jaeger, H. (2014). Controlling recurrent neural networks by conceptors. *arXiv:1403.3369*.
- [Maass2002] Maass, W., Natschläger, T., & Markram, H. (2002). Real-time computing without stable states. *Neural Computation*, 14(11), 2531–2560.
- [Maass1997] Maass, W. (1997). Networks of spiking neurons: The third generation of neural network models. *Neural Networks*, 10(9), 1659–1671.
- [Markram1997] Markram, H., Lübke, J., Frotscher, M., & Sakmann, B. (1997). Regulation of synaptic efficacy by coincidence of postsynaptic APs and EPSPs. *Science*, 275(5297), 213–215.
- [Buonomano2000] Buonomano, D. V., & Mauk, M. D. (1994). Neural network model of the cerebellum: Temporal discrimination and the timing of motor responses. *Neural Computation*, 6(1), 38–55.
- [Buonomano2009] Buonomano, D. V. (2009). Harnessing timing mechanisms for neural computation. *Nature Neuroscience*, 12(12), 1455–1456.
- [Verstraeten2007] Verstraeten, D., Schrauwen, B., d'Haene, M., & Stroobandt, D. (2007). An experimental unification of reservoir computing methods. *Neural Networks*, 20(3), 391–403.
