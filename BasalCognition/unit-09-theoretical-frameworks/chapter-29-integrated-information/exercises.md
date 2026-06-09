# Chapter 29 Exercises: Integrated Information Theory

---

## Part I: Reflection and Discussion Questions

**1. The Axiom Challenge**
IIT grounds its five postulates in five axioms about the essential features of consciousness. The axioms are presented as self-evident: any conscious system must exhibit existence, composition, information, integration, and exclusion. Examine each axiom critically. Can you think of putative counterexamples to any of them — cases where something that seems conscious lacks one of these features, or cases where something seems to lack experience but satisfies all five axioms? Pay particular attention to the integration axiom: split-brain patients appear to have divided consciousness in some sense — does this show that the unity of consciousness can be disrupted, and if so, does this undermine the axiom?

**2. The Identity Thesis and Explanation**
IIT proposes that consciousness *is* integrated information — not that high Φ causes consciousness, or correlates with it, or implements it, but that they are identical. Evaluate whether this identity thesis constitutes an explanation of consciousness or merely a redescription. Compare: when we say "water is H₂O," this is an explanation because we can independently characterize both water (by its phenomenal properties) and H₂O (by its molecular structure) and show they are the same thing. Can we independently characterize consciousness and integrated information in the way that allows the identity claim to be informative? Or is Tononi defining consciousness in terms of Φ, which makes the identity trivially true?

**3. Panpsychism and the Combination Problem**
If IIT implies that any system with Φ > 0 has some degree of consciousness, then electrons, rocks, and cells all have some form of experience. Many philosophers have argued that this kind of panpsychism faces an intractable "combination problem": how do the experiences of elementary particles or cells combine into the rich, unified experience of a person? Present IIT's response to the combination problem (via the exclusion postulate) and evaluate whether it is adequate. Does excluding subsets of the maximum-Φ system from consciousness solve the combination problem or merely relocate it?

**4. Substrate Independence and Substrate Specificity**
Computationalism holds that mental states are defined by computational properties and are substrate-independent: the same program running on different hardware implements the same mental states. IIT is anti-computationalist in holding that Φ depends on the physical causal structure, not just the abstract computation. But IIT is also, in a sense, substrate-independent: any physical system with the same causal topology has the same Φ, regardless of whether it is made of neurons, silicon, or any other material. Is IIT really anti-computationalist, or does it simply replace "abstract computation" with "abstract causal topology"? How is "having the same causal structure" different from "performing the same computation"?

**5. The Scientific Status of IIT**
Over 100 scientists signed a letter in 2023 expressing concern that IIT had achieved scientific prominence without sufficient empirical support. Evaluate this concern. What would it take for IIT to be empirically confirmed or disconfirmed? Can you identify at least two specific, testable predictions that IIT makes which differ from the predictions of rival theories? If IIT cannot generate such predictions, what is its status as a scientific theory? Compare to the FEP discussion from Chapter 28: is IIT in a similar situation (a framework rather than a theory) or a different one?

---

## Part II: Thought Experiments

**1. The Φ-Maximizer**
Suppose bioengineers design a small biological system with the explicit goal of maximizing Φ. They start with a neuron culture and experimentally manipulate the connectivity, introducing recurrent connections, eliminating feedforward-only pathways, and ensuring dense cross-coupling until Φ is maximized for a system of a given size. The resulting system has higher Φ than any region of a mammalian brain. Does this system have more consciousness than any part of your brain? Does your intuition rebel against this conclusion? If so, does your intuition reveal a flaw in IIT, or does it reveal a bias toward "familiar" biological systems? What would it mean to take IIT's implications seriously?

**2. The Duplicated Brain**
Suppose we could perfectly duplicate a human brain — creating an exact physical replica, atom for atom, with identical connectivity and state. The original brain is conscious; the duplicate has identical Φ. Does the duplicate have identical consciousness? Now suppose instead of duplicating the whole brain, we duplicate just a single hemisphere. That hemisphere has some Φ (lower than the whole brain). Does the duplicated hemisphere have some consciousness? Now suppose we gradually remove elements from the duplicate — neurons, one by one — while tracking Φ. At what point does consciousness disappear? Does it vanish gradually as Φ decreases, or is there a threshold? Does IIT's continuity claim (consciousness admits of degrees) sit comfortably with the phenomenology of experience, which seems to be an all-or-nothing presence?

**3. The Colonial Consciousness**
Consider a large coral colony consisting of millions of genetically identical polyps, all physically connected through a shared organic skeleton and able to communicate through chemical and electrical signals across the colony. Each polyp has some Φ; the whole colony might have higher Φ than any individual polyp. Does IIT predict that the colony is a unified conscious subject — that there is "something it is like" to be the colony as a whole, not merely something it is like to be each individual polyp? If so, how does this relate to the exclusion postulate — does the colony's Φ exclude the polyp-level consciousness? Would the colony's experience be different in character from the experiences of its constituent polyps?

---

## Part III: Laboratory and Computational Investigations

**1. Computing Φ for Simple Signaling Networks**
*Rationale*: Develop a computational tool to calculate Φ (or a tractable approximation such as Φ*) for small network models. Implement a network model of a simplified bacterial chemotaxis signaling cascade — perhaps with 5–10 nodes representing key components (receptor complex, CheA, CheY, CheB, CheR, and flagellar motor). Assign Boolean or probabilistic state transitions to each node. Calculate Φ for this network and compare it to: (a) a shuffled version of the network with the same nodes but randomized connections; (b) a feedforward version with the same nodes but no feedback connections; (c) a fully connected random network of the same size. How does the chemotaxis network's Φ compare to these baselines? Does its biological architecture produce more or less integration than expected by chance?

**2. Measuring Functional Integration in Neural Organoids**
*Rationale*: Cerebral organoids — three-dimensional clusters of neural tissue grown from human stem cells — provide a model system for studying the development of neural integration. Design an experiment using multi-electrode arrays to measure electrical activity in organoids at different stages of development (early, 30 days; middle, 60 days; late, 90+ days). Use an approximation method for Φ (such as the geometric integrated information Φ_G or the perturbational complexity index, PCI) to track how integrated information changes as the organoid matures and develops more complex connectivity. Compare integration measures to behavioral measures (e.g., complexity of spontaneous activity patterns). Does integration increase monotonically with development, or are there phase transitions?

**3. IIT-Motivated Analysis of Plant Electrical Networks**
*Rationale*: Apply IIT concepts to analyze the causal integration of electrical signaling in plants. Obtain multi-site electrical recording data from *Arabidopsis thaliana* or *Mimosa pudica* during and after stimulation (wound response, touch response, light exposure). Compute a measure of effective connectivity (Granger causality or transfer entropy) between recording sites to construct a functional connectivity matrix. Analyze how this matrix changes during signaling events: does integration (as measured by network properties related to Φ) increase during active signaling? Does the pattern of integration differ between plants with different levels of ecological complexity or evolutionary history? This investigation uses IIT-motivated concepts without requiring full Φ computation.

---

## Bibliography for Chapter 29

Aaronson, S. (2014). Why I am not an integrated information theorist (or, the unconscious expander). *Blog post*. Retrieved from https://scottaaronson.com/blog/?p=1799

Bayne, T. (2010). *The Unity of Consciousness*. Oxford University Press.

Cerullo, M.A. (2015). The problem with phi: A critique of integrated information theory. *PLOS Computational Biology*, 11(9), e1004286.

Chalmers, D.J. (1996). *The Conscious Mind: In Search of a Fundamental Theory*. Oxford University Press.

Koch, C. (2019). *The Feeling of Life Itself: Why Consciousness Is Widespread but Can't Be Computed*. MIT Press.

Oizumi, M., Albantakis, L., & Tononi, G. (2014). From the phenomenology to the mechanisms of consciousness: Integrated information theory 3.0. *PLOS Computational Biology*, 10(5), e1003588.

Tononi, G. (2004). An information integration theory of consciousness. *BMC Neuroscience*, 5(1), 42.

Tononi, G., Boly, M., Massimini, M., & Koch, C. (2016). Integrated information theory: From consciousness to its physical substrate. *Nature Reviews Neuroscience*, 17(7), 450–461.
